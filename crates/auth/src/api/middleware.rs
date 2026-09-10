use crate::Role;
use crate::api::AuthUser;
use crate::domain::apikey::ApiKey;
use crate::domain::permission::Permission;
use crate::domain::revoked_token::RevokedToken;
use crate::{TokenService, util};
use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::http::header::AUTHORIZATION;
use axum::middleware;
use axum::middleware::Next;
use axum::response::Response;
use hulu_core::claims::{AuthenticationType, UserContext};
use std::sync::Arc;
use tracing::debug;
use utoipa_axum::router::OpenApiRouter;

/// All header names that may carry a credential, in priority order.
///
/// The standard `Authorization` header is checked first (JWT + `hp_` API
/// keys). Then each provider's own API-key header is scanned — a request
/// to `/v1/transaction/initialize` may arrive with `x-arifpay-key` or
/// `x-simulation-key` instead of the Bearer scheme.
const PROVIDER_APIKEY_HEADERS: &[&str] = &["x-arifpay-key", "x-simulation-key", "x-chapa-key", "x-api-key"];

/// Extract the first credential we can find from the request headers.
///
/// Checks, in order:
/// 1. `Authorization: Bearer <token>` → JWT or `hp_` API key
/// 2. Provider-specific headers (`x-arifpay-key`, `x-simulation-key`, etc.)
///    → treated as raw API keys (may or may not have the `hp_` prefix)
///
/// Returns `None` when no credential is present (anonymous request).
fn extract_credential(req: &Request<Body>) -> Option<String> {
    // 1. Standard Authorization: Bearer <token>
    if let Some(cred) = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer ").map(|c| c.to_string()))
    {
        return Some(cred);
    }

    // 2. Provider-specific API-key headers
    for header_name in PROVIDER_APIKEY_HEADERS {
        if let Some(val) = req.headers().get(*header_name) {
            if let Ok(s) = val.to_str() {
                let trimmed = s.strip_prefix("Bearer ").unwrap_or(s).trim();
                if !trimmed.is_empty() {
                    return Some(trimmed.to_string());
                }
            }
        }
    }

    None
}

/// Authentication entry point.
///
/// Resolves the caller's identity from any known credential header:
/// - `Authorization: Bearer hp_...` or provider header → API key flow
/// - `Authorization: Bearer <jwt>`                       → JWT flow
/// - absent                                               → anonymous
///
/// On success, inserts `AuthUser(UserContext)` into request extensions.
pub async fn authentication(mut req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let credential = extract_credential(&req);

    let Some(credential) = credential else {
        return Ok(next.run(req).await); // anonymous allowed
    };

    // ── Branch on credential type ───────────────────────────────
    if is_api_key(&credential) {
        authenticate_api_key(&mut req, &credential).await?;
    } else {
        authenticate_jwt(&mut req, &credential).await?;
    }

    Ok(next.run(req).await)
}

/// A credential is treated as an API key if it starts with `hp_` OR if it
/// arrived via a provider-specific header (which we can detect by checking
/// that it's not a valid JWT — JWTs contain dots, API keys don't).
fn is_api_key(credential: &str) -> bool {
    credential.starts_with("hp_") || !credential.contains('.')
}

/// Validate a JWT, check the revoked-token blocklist, and insert the
/// resulting `AuthUser` into extensions.
async fn authenticate_jwt(req: &mut Request<Body>, token: &str) -> Result<(), StatusCode> {
    let token_service = req
        .extensions()
        .get::<Arc<dyn TokenService>>()
        .cloned()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let claims = token_service
        .validate_access(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Check if the token has been revoked (server-side logout).
    let db = req
        .extensions()
        .get::<toasty::Db>()
        .cloned()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut db = db;

    let is_revoked = RevokedToken::filter_by_id(claims.jti)
        .first()
        .exec(&mut db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .is_some();

    if is_revoked {
        return Err(StatusCode::UNAUTHORIZED);
    }

    req.extensions_mut().insert(AuthUser(claims));
    Ok(())
}

/// Resolve an API key to an `AuthUser` and insert it.
///
/// 1. Extract the prefix (first 12 chars) and look up the row.
/// 2. Verify the full key against the stored argon2 hash.
/// 3. Check the key is active and not expired.
/// 4. Build a `UserContext` from the key + its scopes.
/// 5. Fire-and-forget update of `last_used_at`.
async fn authenticate_api_key(req: &mut Request<Body>, raw_key: &str) -> Result<(), StatusCode> {
    let db = req
        .extensions()
        .get::<toasty::Db>()
        .cloned()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    // Strip the `hp_` prefix if present — the stored prefix is always 12 chars
    // of the key body (without the `hp_`).
    // let key_body = raw_key.strip_prefix("hp_").unwrap_or(raw_key);

    // Prefix is the first 12 chars of the key body.
    let prefix = raw_key
        .get(..12)
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_string();
    println!("{prefix}");
    debug!(prefix = %prefix, "looking up API key");

    let mut query_db = db.clone();
    let mut row = ApiKey::filter(ApiKey::fields().prefix().eq(&prefix))
        .filter(ApiKey::fields().is_active().eq(true))
        .first()
        .exec(&mut query_db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Verify the full key against the stored hash.
    let valid = crate::application::password::verify_password(&row.hash, raw_key);
    if !valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Check expiry.
    let now = util::now_jiff();
    if row.expires_at <= now {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Build the user context from the API key.
    let permissions = row.scopes.clone();
    let key_id = row.id;
    let merchant_id = row.merchant_id;

    let context = UserContext {
        sub: key_id,
        email: String::new(),
        merchant_id,
        role: "apikey".to_string(),
        permissions,
        exp: 0,
        auth_type: AuthenticationType::ApiKey,
        auth_value: Some(raw_key.to_string()),
        jti: uuid::Uuid::now_v7(),
        typ: Default::default(),
    };
    req.extensions_mut().insert(AuthUser(context));

    // Fire-and-forget: update last_used_at.
    tokio::spawn(async move {
        let mut db = db.clone();
        let _ = toasty::update!(row {
            last_used_at: util::now_jiff()
        })
        .exec(&mut db)
        .await;
    });

    Ok(())
}

/// Authorization gate applied per-route via `AuthRouterExt`.
pub async fn authorization(
    State(policy): State<AuthorizationPolicy>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let user = req
        .extensions()
        .get::<AuthUser>()
        .cloned()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    match &policy {
        AuthorizationPolicy::Authenticated => {}
        AuthorizationPolicy::Role(required_role) => {
            if user.0.role != required_role.to_string() {
                return Err(StatusCode::FORBIDDEN);
            }
        }
        AuthorizationPolicy::Permission(required) => {
            let needed = required.to_string();
            if !user.0.permissions.contains(&needed) {
                return Err(StatusCode::FORBIDDEN);
            }
        }
        AuthorizationPolicy::Permissions(any_of) => {
            // Caller must hold at least one of the listed permissions
            let has_any = any_of
                .iter()
                .any(|p| user.0.permissions.contains(&p.to_string()));
            if !has_any {
                return Err(StatusCode::FORBIDDEN);
            }
        }
    }

    Ok(next.run(req).await)
}

#[derive(Clone)]
pub enum AuthorizationPolicy {
    /// Any authenticated identity (JWT or API key).
    Authenticated,
    /// Exact role match (JWT only — API keys report role `"apikey"`).
    Role(Role),
    /// Caller must hold this single permission.
    Permission(Permission),
    /// Caller must hold at least one of these permissions.
    Permissions(Vec<Permission>),
}

pub trait AuthRouterExt {
    fn require_auth(self) -> Self;
    fn require_role(self, role: Role) -> Self;
    /// Require the caller to hold `permission`.
    fn require_permission(self, permission: Permission) -> Self;
    /// Require the caller to hold at least one of `permissions`.
    fn require_any_permission(self, permissions: Vec<Permission>) -> Self;
}

impl<S> AuthRouterExt for OpenApiRouter<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn require_auth(self) -> Self {
        self.layer(middleware::from_fn_with_state(
            AuthorizationPolicy::Authenticated,
            authorization,
        ))
    }

    fn require_role(self, role: Role) -> Self {
        self.layer(middleware::from_fn_with_state(
            AuthorizationPolicy::Role(role),
            authorization,
        ))
    }

    fn require_permission(self, permission: Permission) -> Self {
        self.layer(middleware::from_fn_with_state(
            AuthorizationPolicy::Permission(permission),
            authorization,
        ))
    }

    fn require_any_permission(self, permissions: Vec<Permission>) -> Self {
        self.layer(middleware::from_fn_with_state(
            AuthorizationPolicy::Permissions(permissions),
            authorization,
        ))
    }
}

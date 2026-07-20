use crate::api::AuthUser;
use crate::application::{AuthenticationType, TokenService, UserContext};
use crate::domain::apikey;
use crate::domain::permission::Permission;
use crate::domain::revoked_token;
use crate::Role;
use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::header::AUTHORIZATION;
use axum::http::StatusCode;
use axum::middleware;
use axum::middleware::Next;
use axum::response::Response;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;

/// Authentication entry point.
///
/// Resolves the caller's identity from the `Authorization` header:
/// - `Bearer hp_...` → API key flow (lookup + hash verify)
/// - `Bearer <jwt>`  → JWT flow (existing)
/// - absent          → anonymous (allowed through; protected routes
///                     will reject via the `authorization` layer)
///
/// On success, inserts `AuthUser(UserContext)` into request extensions.
pub async fn authentication(mut req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    // Clone the credential out of the header so we end the immutable
    // borrow of `req` before the mutable borrows below.
    let credential: Option<String> = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer ").map(|c| c.to_string()));

    let Some(credential) = credential else {
        return Ok(next.run(req).await); // anonymous allowed
    };

    // ── Branch on credential type ───────────────────────────────
    if credential.starts_with("hp_") {
        authenticate_api_key(&mut req, &credential).await?;
    } else {
        authenticate_jwt(&mut req, &credential).await?;
    }

    Ok(next.run(req).await)
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
        .get::<DatabaseConnection>()
        .cloned()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let is_revoked = revoked_token::Entity::find_by_id(claims.jti)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .is_some();

    if is_revoked {
        return Err(StatusCode::UNAUTHORIZED);
    }

    req.extensions_mut().insert(AuthUser(claims));
    Ok(())
}

/// Resolve an API key (`hp_...`) to an `AuthUser` and insert it.
///
/// 1. Extract the prefix (first 12 chars after `hp_`) and look up the row.
/// 2. Verify the full key against the stored argon2 hash.
/// 3. Check the key is active and not expired.
/// 4. Build a `UserContext` from the key + its scopes.
/// 5. Fire-and-forget update of `last_used_at`.
async fn authenticate_api_key(req: &mut Request<Body>, raw_key: &str) -> Result<(), StatusCode> {
    let db = req
        .extensions()
        .get::<DatabaseConnection>()
        .cloned()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    // Prefix is the first 12 chars after "hp_"
    let prefix = raw_key
        .get(3..15)
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_string();

    let row = apikey::Entity::find()
        .filter(apikey::Column::Prefix.eq(&prefix))
        .filter(apikey::Column::IsActive.eq(true))
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Verify the full key against the stored hash
    let valid = crate::application::password::verify_password(&row.hash, raw_key);
    if !valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Check expiry
    let now = sqlx::types::chrono::Utc::now();
    if row.expires_at <= now {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Build the user context from the API key
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
        jti: uuid::Uuid::now_v7(),
        typ: Default::default(),
    };
    req.extensions_mut().insert(AuthUser(context));

    // Fire-and-forget: update last_used_at
    tokio::spawn(async move {
        let mut am: apikey::ActiveModel = row.into();
        am.last_used_at = sea_orm::Set(Some(sqlx::types::chrono::Utc::now()));
        let _ = am.update(&db).await;
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

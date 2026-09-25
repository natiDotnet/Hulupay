//! Internal service-to-service endpoints.
//!
//! `POST /internal/keys/verify` — lets the PayBridge microservice resolve an
//! `hp_…` API key it received from a caller back to a merchant identity.
//! The Rust platform stays the single source of truth for API keys; PayBridge
//! never stores or verifies keys itself.
//!
//! Auth: `X-Internal-Service-Token` header compared against the
//! `INTERNAL_SERVICE_TOKEN` env var. The endpoint is disabled (404) when the
//! token is not configured.

use crate::application::password::verify_password;
use crate::domain::apikey::ApiKey;
use crate::util;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::Json;
use axum::routing::post;
use utoipa_axum::router::OpenApiRouter;

/// Lightweight state for the verify-key handler — only carries the DB connection.
#[derive(Clone)]
pub struct VerifyKeyState {
    pub db: toasty::Db,
}

/// Constant-time byte equality.
fn constant_time_eq(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.bytes()
        .zip(b.bytes())
        .fold(0u8, |acc, (x, y)| acc | (x ^ y))
        == 0
}

#[derive(serde::Deserialize)]
pub struct VerifyKeyRequest {
    pub key: String,
}

#[derive(serde::Serialize)]
pub struct VerifyKeyResponse {
    pub valid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<uuid::Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<uuid::Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
}

impl VerifyKeyResponse {
    fn invalid() -> Self {
        Self {
            valid: false,
            key_id: None,
            merchant_id: None,
            scopes: None,
        }
    }
}

/// Resolve an `hp_…` API key to its merchant and scopes — the same validation
/// as `authenticate_api_key`, exposed for sibling services. An invalid key is
/// a 200 with `valid: false` so callers may cache the negative result;
/// service-auth failures are HTTP errors.
pub async fn verify_key_handler(
    State(state): State<VerifyKeyState>,
    headers: HeaderMap,
    Json(req): Json<VerifyKeyRequest>,
) -> Result<Json<VerifyKeyResponse>, axum::http::StatusCode> {
    let expected = std::env::var("INTERNAL_SERVICE_TOKEN").unwrap_or_default();
    if expected.is_empty() {
        // Endpoint deliberately disabled when no service token is configured.
        return Err(axum::http::StatusCode::NOT_FOUND);
    }
    let provided = headers
        .get("x-internal-service-token")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if !constant_time_eq(&expected, provided) {
        return Err(axum::http::StatusCode::UNAUTHORIZED);
    }

    let key = req.key.trim();
    let Some(prefix) = key.strip_prefix("hp_").and_then(|rest| rest.get(..12)) else {
        return Ok(Json(VerifyKeyResponse::invalid()));
    };

    let mut db = state.db.clone();
    let row = ApiKey::filter(ApiKey::fields().prefix().eq(prefix))
        .filter(ApiKey::fields().is_active().eq(true))
        .first()
        .exec(&mut db)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let Some(row) = row else {
        return Ok(Json(VerifyKeyResponse::invalid()));
    };

    if !verify_password(&row.hash, key) {
        return Ok(Json(VerifyKeyResponse::invalid()));
    }

    let now = util::now_jiff();
    if row.expires_at <= now {
        return Ok(Json(VerifyKeyResponse::invalid()));
    }

    Ok(Json(VerifyKeyResponse {
        valid: true,
        key_id: Some(row.id),
        merchant_id: Some(row.merchant_id),
        scopes: Some(row.scopes),
    }))
}

/// Router for internal endpoints; mounted at the root by the host app.
pub fn internal_router(db: toasty::Db) -> OpenApiRouter {
    OpenApiRouter::new()
        .route("/internal/keys/verify", post(verify_key_handler))
        .with_state(VerifyKeyState { db })
}
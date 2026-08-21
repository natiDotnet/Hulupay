use crate::api::AuthUser;
use crate::api::state::AuthState;
use crate::application::LogoutRequest;
use crate::domain::refresh_token::RefreshToken;
use crate::domain::revoked_token::RevokedToken;
use crate::util;
use axum::{Json, extract::State, http::StatusCode};
use chrono::Utc;
use hulu_core::claims::UserContext;

/// POST /auth/logout
///
/// Revokes the current access token (writes to `revoked_tokens`) and
/// optionally revokes the refresh token family.
#[utoipa::path(
    post,
    tag = "auth",
    path = "/auth/logout",
    request_body = LogoutRequest,
    responses((status = NO_CONTENT))
)]
pub async fn logout_handler(
    State(state): State<AuthState>,
    user: AuthUser,
    Json(payload): Json<LogoutRequest>,
) -> Result<StatusCode, StatusCode> {
    let UserContext { jti, sub, exp, .. } = user.0;
    let mut db = state.db.clone();
    let now_chrono = Utc::now();

    // Revoke the access token (blocklist). `expires_at` mirrors the
    // original access token's expiry so the row can be cleaned up later.
    let remaining = (exp as i64) - now_chrono.timestamp();
    let expires_at = if remaining > 0 {
        util::to_jiff(now_chrono + chrono::Duration::seconds(remaining))
    } else {
        util::to_jiff(now_chrono)
    };
    let revoked_at = util::to_jiff(now_chrono);

    toasty::create!(RevokedToken {
        id: jti,
        user_id: sub,
        expires_at,
        revoked_at,
    })
    .exec(&mut db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Optionally revoke the refresh token family.
    if let Some(refresh_token_str) = payload.refresh_token {
        if let Ok(claims) = state.token_service.validate_refresh(&refresh_token_str) {
            let rows = RefreshToken::filter(RefreshToken::fields().family_id().eq(claims.jti))
                .filter(RefreshToken::fields().revoked_at().is_none())
                .exec(&mut db)
                .await
                .unwrap_or_default();

            let now = util::now_jiff();
            for mut row in rows {
                let _ = toasty::update!(row { revoked_at: now }).exec(&mut db).await;
            }
        }
    }

    Ok(StatusCode::NO_CONTENT)
}

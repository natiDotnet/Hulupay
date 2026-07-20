use crate::api::state::AuthState;
use crate::api::AuthUser;
use crate::application::{LogoutRequest, UserContext};
use crate::domain::refresh_token;
use crate::domain::revoked_token;
use axum::{extract::State, http::StatusCode, Json};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

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
    let db = &state.db;
    let now = Utc::now();

    // Revoke the access token (blocklist). `expires_at` mirrors the
    // original access token's expiry so the row can be cleaned up later.
    let remaining = (exp as i64) - now.timestamp();
    let expires_at = if remaining > 0 {
        now + chrono::Duration::seconds(remaining)
    } else {
        now
    };

    let revoked = revoked_token::ActiveModel {
        id: Set(jti),
        user_id: Set(sub),
        expires_at: Set(expires_at),
        revoked_at: Set(now),
    };
    revoked
        .insert(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Optionally revoke the refresh token family.
    if let Some(refresh_token_str) = payload.refresh_token {
        if let Ok(claims) = state.token_service.validate_refresh(&refresh_token_str) {
            let rows = refresh_token::Entity::find()
                .filter(refresh_token::Column::FamilyId.eq(claims.jti))
                .filter(refresh_token::Column::RevokedAt.is_null())
                .all(db)
                .await
                .unwrap_or_default();

            for row in rows {
                let mut am: refresh_token::ActiveModel = row.into();
                am.revoked_at = Set(Some(now));
                let _ = am.update(db).await;
            }
        }
    }

    Ok(StatusCode::NO_CONTENT)
}

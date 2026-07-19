use crate::api::state::AuthState;
use crate::application::ResetPasswordRequest;
use axum::{extract::State, http::StatusCode, Json};

#[utoipa::path(
    post,
    tag = "auth",
    path = "/auth/reset-password",
    request_body = ResetPasswordRequest,
    responses((status = OK))
)]
pub async fn reset_password_handler(
    State(state): State<AuthState>,
    Json(payload): Json<ResetPasswordRequest>,
) -> Result<StatusCode, StatusCode> {
    state
        .reset_password_use_case
        .execute(payload)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|_| StatusCode::BAD_REQUEST)
}

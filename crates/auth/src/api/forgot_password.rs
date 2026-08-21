use crate::api::state::AuthState;
use crate::application::ForgotPasswordRequest;
use axum::{Json, extract::State, http::StatusCode};

#[utoipa::path(
    post,
    tag = "auth",
    path = "/auth/forgot-password",
    request_body = ForgotPasswordRequest,
    responses((status = OK))
)]
pub async fn forgot_password_handler(
    State(state): State<AuthState>,
    Json(payload): Json<ForgotPasswordRequest>,
) -> Result<StatusCode, StatusCode> {
    state
        .forgot_password_use_case
        .execute(payload)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

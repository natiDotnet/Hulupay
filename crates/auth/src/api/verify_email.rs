use crate::api::state::AuthState;
use crate::application::{VerifyEmailRequest, VerifyEmailResponse};
use axum::{extract::State, http::StatusCode, Json};

#[utoipa::path(
    post,
    tag = "auth",
    path = "/auth/verify-email",
    request_body = VerifyEmailRequest,
    responses((status = OK, body = VerifyEmailResponse))
)]
pub async fn verify_email_handler(
    State(state): State<AuthState>,
    Json(payload): Json<VerifyEmailRequest>,
) -> Result<Json<VerifyEmailResponse>, StatusCode> {
    state
        .verify_email_use_case
        .execute(payload)
        .await
        .map(Json)
        .map_err(|_| StatusCode::BAD_REQUEST)
}

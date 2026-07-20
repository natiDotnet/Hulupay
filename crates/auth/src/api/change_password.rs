use crate::api::state::AuthState;
use crate::api::AuthUser;
use crate::application::ChangePasswordRequest;
use axum::{extract::State, http::StatusCode, Json};

#[utoipa::path(
    post,
    tag = "auth",
    path = "/auth/change-password",
    request_body = ChangePasswordRequest,
    responses((status = OK))
)]
pub async fn change_password_handler(
    State(state): State<AuthState>,
    user: AuthUser,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<StatusCode, StatusCode> {
    state
        .change_password_use_case
        .execute(user.0.sub, payload)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|_| StatusCode::BAD_REQUEST)
}

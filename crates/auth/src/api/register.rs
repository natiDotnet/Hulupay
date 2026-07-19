use crate::api::state::AuthState;
use crate::application::{RegisterUserRequest, RegisterUserResponse};
use axum::{Json, extract::State, http::StatusCode};

#[utoipa::path(
    post,
    tag = "auth",
    path = "/auth/register",
    request_body = RegisterUserRequest,
    responses((status = OK, body = RegisterUserResponse))
)]
pub async fn register_user_handler(
    State(state): State<AuthState>,
    Json(payload): Json<RegisterUserRequest>,
) -> Result<Json<RegisterUserResponse>, StatusCode> {
    let res = state
        .register_use_case
        .execute(payload)
        .await
        .map_err(|e| {
            eprintln!("Error registering user: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(res))
}

use crate::api::state::AuthState;
use crate::{LoginRequest, LoginResponse};
use axum::{Json, extract::State, http::StatusCode};

#[utoipa::path(
    post,
    tag = "auth",
    path = "/auth/login",
    request_body = LoginRequest,
    responses((status = OK, body = LoginResponse))
)]
pub async fn login_user_handler(
    State(state): State<AuthState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let res = state.login_use_case.execute(payload).await.map_err(|e| {
        eprintln!("Error logging in user: {:?}", e);
        StatusCode::BAD_REQUEST
    })?;
    Ok(Json(res))
}

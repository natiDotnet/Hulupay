use crate::api::state::AuthState;
use crate::application::{RefreshRequest, RefreshResponse};
use axum::{Json, extract::State, http::StatusCode};

#[utoipa::path(
    post,
    tag = "auth",
    path = "/auth/refresh",
    request_body = RefreshRequest,
    responses((status = OK, body = RefreshResponse))
)]
pub async fn refresh_handler(
    State(state): State<AuthState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<RefreshResponse>, StatusCode> {
    state
        .refresh_use_case
        .execute(payload)
        .await
        .map(Json)
        .map_err(|_| StatusCode::UNAUTHORIZED)
}

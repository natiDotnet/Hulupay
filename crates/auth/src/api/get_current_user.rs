use crate::api::AuthUser;
use crate::api::state::AuthState;
use crate::application::CurrentUserResponse;
use axum::{Json, extract::State, http::StatusCode};

#[utoipa::path(
    get,
    tag = "auth",
    path = "/auth/me",
    responses((status = OK, body = CurrentUserResponse))
)]
pub async fn get_current_user_handler(
    State(state): State<AuthState>,
    user: AuthUser,
) -> Result<Json<CurrentUserResponse>, StatusCode> {
    state
        .get_current_user_use_case
        .execute(user.0.sub)
        .await
        .map(Json)
        .map_err(|_| StatusCode::UNAUTHORIZED)
}

use crate::application::LoginUser;
use crate::{LoginRequest, LoginResponse};
use axum::{Json, extract::State, http::StatusCode};

#[utoipa::path(
    post,
    tag = "auth",
    path = "/auth/login",
    responses((status = OK, body = LoginResponse))
)]
pub async fn login_user_handler(
    State(usecase): State<LoginUser>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let res = usecase.execute(payload).await.map_err(|e| {
        eprintln!("Error logging in user: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(res))
}

use crate::application::LoginUser;
use crate::{LoginRequest, LoginResponse};
use axum::{extract::State, http::StatusCode, Json};

#[utoipa::path(
    post,
    tag = "auth",
    path = "/auth/login",
    responses((status = OK, body = LoginResponse))
)]
pub async fn checkout_handler(
    State(usecase): State<LoginUser>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let res = usecase.execute(payload).await.map_err(|e| {
        eprintln!("Error logging in user: {:?}", e);
        StatusCode::BAD_REQUEST
    })?;
    Ok(Json(res))
}

use crate::application::RegisterUser;
use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use crate::api::extractor::AuthUser;
use crate::{RegisterUserRequest, RegisterUserResponse};

#[utoipa::path(
    post,
    tag = "auth",
    path = "/auth/register",
    responses((status = OK, body = RegisterUserResponse))
)]
pub async fn register_user_handler(
    State(usecase): State<RegisterUser>,
    Json(payload): Json<RegisterUserRequest>,
) -> Result<Json<RegisterUserResponse>, StatusCode> {
    let res = usecase.execute(payload)
        .await
        .map_err(|e| {
            eprintln!("Error registering user: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(res))
}

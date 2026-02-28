use application::auth::login_request::{RegisterUserRequest, RegisterUserResponse};
use application::auth::register_user::RegisterUser;
use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

#[utoipa::path(post, tag="auth", path = "/auth/register", responses((status = OK, body = RegisterUserResponse)))]
pub async fn register_user_handler(
    State(usecase): State<RegisterUser>,
    Json(payload): Json<RegisterUserRequest>,
) -> Result<Json<RegisterUserResponse>, StatusCode> {
    let res = usecase.execute(payload)
        .await
        .map_err(|e| {
            eprintln!("Error creating merchant: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(res))
}
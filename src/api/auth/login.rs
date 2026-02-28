use application::auth::login::LoginUser;
use application::auth::login_request::{LoginRequest, LoginResponse, RegisterUserResponse};
use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

#[utoipa::path(post, tag="auth", path = "/auth/login", responses((status = OK, body = LoginResponse)))]
pub async fn login_user_handler(
    State(usecase): State<LoginUser>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let res = usecase.execute(payload).await
        .map_err(|e| {
            eprintln!("Error creating merchant: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(res))
}
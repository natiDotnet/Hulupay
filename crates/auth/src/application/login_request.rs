use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct RegisterUserRequest {
    pub email: String,
    pub password: String,
    pub role: String,
    pub merchant_id: Option<Uuid>,
}

#[derive(Serialize, ToSchema)]
pub struct RegisterUserResponse {
    pub id: Uuid,
    pub email: String,
    pub role: String,
}

#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub access_token: String,
}

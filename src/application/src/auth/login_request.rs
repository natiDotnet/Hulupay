use serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct RegisterUserRequest {
    pub email: String,
    pub password: String,
    pub role: String,              // "MASTER_ADMIN" | "MERCHANT_ADMIN"
    pub merchant_id: Option<Uuid>, // required if MERCHANT_ADMIN
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
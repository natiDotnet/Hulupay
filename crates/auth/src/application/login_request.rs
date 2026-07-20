use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct RegisterUserRequest {
    pub email: String,
    pub password: String,
    pub role: String,
    pub merchant_id: Uuid,
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
    pub refresh_token: String,
}

#[derive(Deserialize, ToSchema)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Serialize, ToSchema)]
pub struct RefreshResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LogoutRequest {
    /// Optional — if provided, also revokes the refresh token family.
    pub refresh_token: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Deserialize, ToSchema)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub new_password: String,
}

#[derive(Deserialize, ToSchema)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Deserialize, ToSchema)]
pub struct VerifyEmailRequest {
    pub token: String,
}

#[derive(Serialize, ToSchema)]
pub struct VerifyEmailResponse {
    pub verified: bool,
}

#[derive(Serialize, ToSchema)]
pub struct CurrentUserResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: String,
    pub merchant_id: Uuid,
    pub email_verified: bool,
    pub created_at: String,
}

pub mod api;
pub mod application;
pub mod domain;
pub mod infrastructure;

pub use api::{router, AuthState};
pub use application::{
    LoginRequest, LoginResponse, LoginUser, RegisterUser, RegisterUserRequest,
    RegisterUserResponse, TokenService, UserContext,
};
pub use domain::{AuthError as DomainAuthError, Role};
use hulu_core::gateway_response::CheckoutResponse;
use hulu_core::hulu_error::HuluError;
pub use infrastructure::JwtTokenService;
use sea_orm::sea_query::prelude::serde_json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HuluResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
    // pub status_code: u16,
}
impl From<HuluError> for HuluResponse<serde_json::Value> {
    fn from(value: HuluError) -> Self {
        Self {
            success: false,
            data: None,
            message: value.to_string(),
            // status_code: 400,
        }
    }
}

impl From<CheckoutResponse> for HuluResponse<CheckoutResponse> {
    fn from(value: CheckoutResponse) -> Self {
        Self {
            success: true,
            data: Some(value),
            message: "success".to_string(),
            // status_code: 200,
        }
    }
}

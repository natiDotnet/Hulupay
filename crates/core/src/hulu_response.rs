use crate::gateway_response::CheckoutResponse;
use crate::hulu_error::HuluError;
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

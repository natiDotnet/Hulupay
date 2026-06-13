use hulu_core::gateway_response::CheckoutResponse;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ChapaResponse<T> {
    pub message: String,
    pub status: String,
    pub data: Option<T>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChapaCheckoutResponse {
    pub checkout_url: String,
}

impl From<CheckoutResponse> for ChapaResponse<ChapaCheckoutResponse> {
    fn from(value: CheckoutResponse) -> Self {
        Self {
            message: "success".to_string(),
            status: "success".to_string(),
            data: Some(ChapaCheckoutResponse {
                checkout_url: value.checkout_url,
            }),
        }
    }
}

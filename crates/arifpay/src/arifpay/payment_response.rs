use gateway_response::CheckoutResponse;
use hulu_core::gateway_response;
use hulu_core::hulu_error::HuluError;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ArifPayInitializeResponse {
    pub error: bool,
    pub msg: String,
    pub data: Option<ArifPayInitializeData>,
}
#[derive(Debug, Deserialize, Clone, Serialize, ToSchema)]
#[serde(untagged)]
pub enum ResponseData {
    Success(ArifPayInitializeData),
    Error(serde_json::Value),
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ArifPayInitializeData {
    pub session_id: String,
    pub payment_url: String,
    pub cancel_url: String,
    pub total_amount: Decimal,
}

impl From<CheckoutResponse> for ArifPayInitializeResponse {
    fn from(value: CheckoutResponse) -> Self {
        Self {
            error: false,
            msg: "success".to_string(),
            data: Some(ArifPayInitializeData {
                payment_url: value.checkout_url,
                session_id: value.reference.clone(),
                cancel_url: format!(
                    "https://gateway.arifpay.org/v0/checkout/session/cancel{}",
                    value.reference
                ),
                total_amount: value.amount,
            }),
        }
    }
}

impl From<HuluError> for ArifPayInitializeResponse {
    fn from(value: HuluError) -> Self {
        Self {
            error: true,
            msg: "error".to_string(),
            data: None,
        }
    }
}

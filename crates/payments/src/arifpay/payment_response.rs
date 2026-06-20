use gateway_response::CheckoutResponse;
use hulu_core::gateway_response;
use hulu_core::gateway_response::VerifyResponse;
use hulu_core::hulu_error::HuluError;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ArifResponse<T> {
    pub error: bool,
    pub msg: String,
    pub data: Option<T>,
}
#[derive(Debug, Deserialize, Clone, Serialize, ToSchema)]
#[serde(untagged)]
pub enum ResponseData {
    Success(ArifInitializeData),
    Error(serde_json::Value),
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ArifInitializeData {
    pub session_id: String,
    pub payment_url: String,
    pub cancel_url: String,
    pub total_amount: Decimal,
}

impl From<CheckoutResponse> for ArifResponse<ArifInitializeData> {
    fn from(value: CheckoutResponse) -> Self {
        Self {
            error: false,
            msg: "success".to_string(),
            data: Some(ArifInitializeData {
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

impl From<HuluError> for ArifResponse<serde_json::Value> {
    fn from(value: HuluError) -> Self {
        Self {
            error: true,
            msg: "error".to_string(),
            data: None,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ArifVerifyResponse {
    pub transaction_status: String,
    pub transaction_id: Option<String>,
    pub session_id: String,
}

impl From<VerifyResponse> for ArifResponse<ArifVerifyResponse> {
    fn from(value: VerifyResponse) -> Self {
        Self {
            error: false,
            msg: "payment status".to_string(),
            data: Some(ArifVerifyResponse {
                session_id: value.transaction.reference,
                transaction_id: value.transaction.id,
                transaction_status: value.transaction.status,
            }),
        }
    }
}

use hulu_core::gateway_response::CheckoutResponse;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::chapa::chapa_webhook::ChapaPaymentMethod;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ChapaResponse<T> {
    pub message: String,
    pub status: String,
    pub data: Option<T>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
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

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Customization {
    pub title: String,
    pub description: String,
    pub logo: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ChapaVerifyResponse {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub currency: String,
    pub amount: Decimal,
    pub charge: Decimal,
    pub mode: String,
    pub method: ChapaPaymentMethod,
    #[serde(rename = "type")]
    pub r#type: String,
    pub status: String,
    pub reference: Option<String>,
    pub tx_ref: String,
    pub customization: Customization,
    pub meta: Option<serde_json::Value>,
    #[schema(value_type = String)]
    pub created_at: jiff::Timestamp,
    #[schema(value_type = String)]
    pub updated_at: jiff::Timestamp,
}

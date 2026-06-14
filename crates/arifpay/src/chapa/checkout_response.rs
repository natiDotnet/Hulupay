use chrono::{DateTime, Utc};
use hulu_core::gateway_response::{CheckoutResponse, VerifyResponse};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
    pub method: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub status: String,
    pub reference: String,
    pub tx_ref: String,
    pub customization: Customization,
    pub meta: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<VerifyResponse> for ChapaResponse<ChapaVerifyResponse> {
    fn from(value: VerifyResponse) -> Self {
        let customer = value.customer.unwrap();
        let pay = value.payment.unwrap();
        let tnx = value.transaction;
        let custom = value.items.iter().next().unwrap().clone();
        let mut name = customer.name.split_whitespace();
        Self {
            status: "success".to_string(),
            message: "payment details".to_string(),
            data: Some(ChapaVerifyResponse {
                first_name: name.next().unwrap_or("").to_string(),
                last_name: name.next().unwrap_or("").to_string(),
                email: customer.email,
                currency: pay.currency,
                amount: pay.amount,
                charge: tnx.charge.unwrap_or(Decimal::ZERO),
                mode: "".to_string(),
                method: "test".to_string(),
                r#type: "API".to_string(),
                status: tnx.status,
                reference: tnx.id.unwrap_or("".to_string()),
                tx_ref: tnx.reference,
                customization: Customization {
                    title: custom.name,
                    logo: custom.image,
                    description: custom.description,
                },
                meta: None,
                created_at: tnx.created_at,
                updated_at: tnx.updated_at,
            }),
        }
    }
}

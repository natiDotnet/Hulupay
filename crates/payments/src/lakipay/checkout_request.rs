use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use hulu_core::payment_request::PaymentRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct Redirects {
    pub success: String,
    pub failed: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LakiCheckoutRequest {
    pub amount: Decimal,
    pub currency: String,
    pub phone_number: String,
    pub reference: String,
    pub description: String,
    pub callback_url: String,
    pub redirects: Redirects,
    pub supported_mediums: Vec<String>,
}

impl From<&PaymentRequest> for LakiCheckoutRequest {
    fn from(value: &PaymentRequest) -> Self {
        Self {
            amount: value.payment.amount,
            currency: value.payment.currency.clone(),
            phone_number: value.customer.phone.clone(),
            reference: value.payment.reference.clone(),
            description: value.items.first().unwrap().description.clone(),
            callback_url: value.callbacks.notify_url.clone(),
            redirects: Redirects {
                failed: value.callbacks.error_url.clone(),
                success: value.callbacks.success_url.clone(),
            },
            supported_mediums: value.payment.payment_methods.clone(),
        }
    }
}
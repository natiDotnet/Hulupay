use crate::payment_method::PaymentMethod;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckoutResponse {
    pub reference: String,
    pub checkout_url: String,
    pub amount: Decimal,
}

pub struct VerifyResponse {
    pub id: Option<String>,
    pub reference: String,
    pub status: String,
    pub amount: Decimal,
    pub payment_method: PaymentMethod,
    pub charge: Decimal,
    pub created_at: jiff::Timestamp,
    pub updated_at: jiff::Timestamp,
}

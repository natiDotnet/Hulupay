use crate::payment_method::PaymentMethod;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

pub struct CheckoutResponse {
    pub reference: String,
    pub checkout_url: String,
    pub amount: rust_decimal::Decimal,
}

pub struct VerifyResponse {
    pub id: Option<String>,
    pub reference: String,
    pub status: String,
    pub amount: Decimal,
    pub payment_method: PaymentMethod,
    pub charge: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

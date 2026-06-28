use crate::payment_request::{Beneficiary, CallbackUrls, CustomerInfo, Item, PaymentOptions};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::collections::HashMap;

pub struct CheckoutResponse {
    pub reference: String,
    pub checkout_url: String,
    pub amount: rust_decimal::Decimal,
}

pub struct VerifyResponse {
    pub id: Option<String>,
    pub reference: String,
    pub status: String,
    pub charge: Option<Decimal>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

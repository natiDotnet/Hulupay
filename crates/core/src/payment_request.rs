use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::collections::HashMap;
#[derive(Clone)]
pub struct CustomerInfo {
    pub phone: String,
    pub email: String,
    pub name: String,
}
#[derive(Clone)]
pub struct PaymentOptions {
    pub amount: Decimal,
    pub reference: String,
    pub currency: String,
    pub payment_methods: Vec<String>,
    pub expire_date: Option<DateTime<Utc>>,
    pub lang: Option<String>,
}
#[derive(Clone)]
pub struct CallbackUrls {
    pub success_url: String,
    pub error_url: String,
    pub cancel_url: String,
    pub notify_url: String,
}
#[derive(Clone)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub quantity: u32,
    pub image: Option<String>,
    pub price: Decimal,
}
#[derive(Clone)]
pub struct Beneficiary {
    pub account_number: String,
    pub bank: String,
    pub amount: Decimal,
}

#[derive(Clone)]
pub struct PaymentRequest {
    pub customer: CustomerInfo,
    pub payment: PaymentOptions,
    pub items: Vec<Item>,
    pub beneficiaries: Vec<Beneficiary>,
    pub callbacks: CallbackUrls,
    pub metadata: HashMap<String, serde_json::Value>,
}

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::collections::HashMap;
#[derive(Clone, Debug)]
pub struct HuluCustomer {
    pub phone: String,
    pub email: String,
    pub name: String,
}
#[derive(Clone, Debug)]
pub struct HuluPayment {
    pub amount: Decimal,
    pub reference: String,
    pub currency: String,
    pub payment_methods: Vec<String>,
    pub expire_date: Option<DateTime<Utc>>,
    pub lang: Option<String>,
}
#[derive(Clone, Debug)]
pub struct HuluCallbackUrls {
    pub success_url: String,
    pub error_url: String,
    pub cancel_url: String,
    pub notify_url: String,
}
#[derive(Clone, Debug)]
pub struct HuluItem {
    pub name: String,
    pub description: String,
    pub quantity: u32,
    pub image: Option<String>,
    pub price: Decimal,
}

#[derive(Clone, Debug)]
pub struct HuluPaymentRequest {
    pub customer: HuluCustomer,
    pub payment: HuluPayment,
    pub items: Vec<HuluItem>,
    pub callbacks: HuluCallbackUrls,
    pub metadata: HashMap<String, serde_json::Value>,
}

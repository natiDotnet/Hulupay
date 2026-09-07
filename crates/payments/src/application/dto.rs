use crate::PaymentMethod;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone)]
pub struct InitializePaymentCommand {
    pub merchant_id: Uuid,
    pub phone: String,
    pub email: String,
    pub amount: Decimal,
    pub currency: String,
}

#[derive(Clone)]
pub struct CustomerInfo {
    pub phone: String,
    pub email: String,
    pub name: String,
}

#[derive(Clone)]
pub struct PaymentOptions {
    pub reference: String,
    pub currency: String,
    pub payment_methods: Vec<String>,
    pub expire_date: jiff::Timestamp,
    pub lang: String,
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
    pub quantity: u32,
    pub price: Decimal,
    pub description: String,
}
#[derive(Debug, Clone)]
pub struct DirectPaymentRequest {
    pub merchant_id: Uuid,
    pub payment_method: PaymentMethod,
    pub amount: Decimal,
    pub phone_number: String,
    pub email: Option<String>,
    pub reference: String,
    pub currency: String,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArifPayInitializeRequest {
    pub cancel_url: String,
    pub phone: String,
    pub email: String,
    pub nonce: String,
    pub success_url: String,
    pub error_url: String,
    pub notify_url: String,
    pub payment_methods: Vec<String>,
    pub expire_date: jiff::Timestamp,
    pub items: Vec<ArifPayItem>,
    pub beneficiaries: Vec<ArifPayBeneficiary>,
    pub lang: String,
}

#[derive(Debug, Serialize)]
pub struct ArifPayItem {
    pub name: String,
    pub quantity: u32,
    pub price: Decimal,
    pub description: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArifPayBeneficiary {
    pub account_number: String,
    pub bank: String,
    pub amount: Decimal,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ArifPayInitializeResponse {
    pub error: bool,
    pub msg: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArifPayInitializeData {
    pub session_id: String,
    pub payment_url: String,
    pub cancel_url: String,
    pub total_amount: f64,
}

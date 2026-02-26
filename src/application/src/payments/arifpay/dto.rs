use serde::{Deserialize, Serialize};

#[derive(Serialize)]
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
    pub expire_date: String,
    pub items: Vec<ArifPayItem>,
    pub beneficiaries: Vec<ArifPayBeneficiary>,
    pub lang: String,
}

#[derive(Serialize)]
pub struct ArifPayItem {
    pub name: String,
    pub quantity: u32,
    pub price: f64,
    pub description: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArifPayBeneficiary {
    pub account_number: String,
    pub bank: String,
    pub amount: f64,
}

use uuid::Uuid;

pub struct InitializePaymentCommand {
    pub merchant_id: Uuid,
    pub phone: String,
    pub email: String,
    pub amount: f64,
    pub currency: String,
}

#[derive(Deserialize)]
pub struct ArifPayInitializeResponse {
    pub error: bool,
    pub msg: String,
    pub data: Option<ArifPayInitializeData>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArifPayInitializeData {
    pub session_id: String,
    pub payment_url: String,
    pub cancel_url: String,
    pub total_amount: f64,
}
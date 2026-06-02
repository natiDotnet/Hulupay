use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentRequest {
    pub cancel_url: String,
    pub phone: String,
    pub email: String,
    pub nonce: String,
    pub success_url: String,
    pub error_url: String,
    pub notify_url: String,
    pub payment_methods: Vec<String>,
    pub expire_date: String,
    pub items: Vec<Item>,
    pub beneficiaries: Vec<Beneficiary>,
    pub lang: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub name: String,
    pub quantity: u32,
    pub price: u64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Beneficiary {
    pub account_number: String,
    pub bank: String,
    pub amount: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtpRequest {
    pub session_id: String,
    pub phone: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyOtpRequest {
    pub session_id: String,
    pub otp: String,
}

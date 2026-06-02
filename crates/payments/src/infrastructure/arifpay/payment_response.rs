use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArifPayInitializeResponse {
    pub error: bool,
    pub msg: String,
    pub data: Option<ResponseData>,
}
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum ResponseData {
    Success(ArifPayInitializeData),
    Error(serde_json::Value),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArifPayInitializeData {
    pub session_id: String,
    pub payment_url: String,
    pub cancel_url: String,
    pub total_amount: Decimal,
}

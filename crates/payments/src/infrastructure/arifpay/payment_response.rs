use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ArifPayInitializeResponse {
    pub error: bool,
    pub msg: String,
    pub data: Option<ResponseData>,
}
#[derive(Debug, Deserialize, Clone, Serialize, ToSchema)]
#[serde(untagged)]
pub enum ResponseData {
    Success(ArifPayInitializeData),
    Error(serde_json::Value),
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ArifPayInitializeData {
    pub session_id: String,
    pub payment_url: String,
    pub cancel_url: String,
    pub total_amount: Decimal,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LakiCheckoutResponse {
    pub success: bool,
    pub status: Option<String>,
    pub message: String,
    pub payment_url: Option<String>,
    pub reference_id: Option<String>,
    pub lakipay_transaction_id: Option<String>,
    pub merchant_pays_fee: Option<bool>,
}
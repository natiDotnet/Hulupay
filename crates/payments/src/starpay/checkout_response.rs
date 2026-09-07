use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StarData {
    pub order_id: String,
    pub status: String,
    pub amount: String,
    pub currency: String,
    pub payment_url: String,
    #[serde(rename = "redirectUrl")]
    pub redirect_url: String,
    pub expires_at: jiff::Timestamp,
    metadata: HashMap<String, serde_json::Value>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StarError {
    pub code: String,
    pub message: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StarCheckoutResponse {
    pub status: String,
    pub timestamp: String,
    pub path: Option<String>,
    pub message: Option<String>,
    pub data: Option<StarData>,
    pub error: Option<StarError>,
}
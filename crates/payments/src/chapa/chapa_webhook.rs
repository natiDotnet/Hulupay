use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Customization {
    pub title: Option<String>,
    pub description: Option<String>,
    pub logo: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ChapaWebhook {
    pub event: String,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub mobile: String,
    pub currency: String,
    pub amount: String,
    pub charge: String,
    pub status: String,
    pub mode: String,
    pub reference: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub tx_ref: String,
    pub payment_method: String,
    pub customization: Customization,
    pub meta: Option<serde_json::Value>,
}

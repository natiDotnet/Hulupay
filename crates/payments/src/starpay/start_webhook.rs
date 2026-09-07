use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use hulu_core::payment_gateway::PaymentStatus;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StarWebhook {
    pub bill_ref_no: String,
    pub status: StarWebhookStatus,
    pub timestamp: Option<jiff::Timestamp>,
    pub message: String,
    pub merchant_id: Option<String>,
    pub customer_id: Option<String>,
    pub external_reference_id: Option<String>,
    pub amount: Option<Decimal>,
    pub payment_type: Option<String>,
    pub receipt_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub enum StarWebhookStatus {
    Paid,
    Failed,
}

impl From<StarWebhookStatus> for PaymentStatus {
    fn from(value: StarWebhookStatus) -> Self {
         match value {
             StarWebhookStatus::Paid => Self::Success,
             StarWebhookStatus::Failed => Self::Failed
         }
    }
}
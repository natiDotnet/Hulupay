use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use hulu_core::payment_gateway::PaymentStatus;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LakiWebhook {
    pub event: String,
    pub reference_id: String,
    pub lakipay_txn_id: String,
    pub status: LakiWebhookStatus,
    pub amount: Decimal,
    pub callback_url: String,
    pub message: String,
    pub provider_tx_id: String,
    pub timestamp: jiff::Timestamp,
    pub merchant_id: String,
    pub user_id: String,
    pub signature: String,
}

#[derive(Serialize, Deserialize)]
pub enum LakiWebhookStatus {
    Success,
    Failed,
    Pending,
}

impl From<LakiWebhookStatus> for PaymentStatus {
    fn from(value: LakiWebhookStatus) -> Self {
        match value {
            LakiWebhookStatus::Success => Self::Success,
            LakiWebhookStatus::Failed => Self::Failed,
            LakiWebhookStatus::Pending => Self::Pending,
        }
    }
}
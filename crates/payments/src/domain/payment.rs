use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArifPayment {
    pub uuid: Uuid,
    pub nonce: String,
    pub phone: String,
    pub payment_method: String,
    pub total_amount: f64,
    pub transaction_status: ArifTransactionStatus,
    pub transaction: Option<ArifPaymentTransaction>,
    pub notification_url: String,
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ArifTransactionStatus {
    Success,
    Pending,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArifPaymentTransaction {
    pub transaction_id: String,
    pub transaction_status: ArifTransactionStatus,
}

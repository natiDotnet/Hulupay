use crate::arifpay::arif_payment_method::ArifPaymentMethod;
use hulu_core::payment_gateway::PaymentStatus;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Serialize, Deserialize)]
pub struct ArifTransaction {
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    #[serde(rename = "transactionStatus")]
    pub transaction_status: ArifTransactionStatus,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArifWebhook {
    pub uuid: String,
    pub nonce: String,
    pub phone: String,
    pub payment_method: ArifPaymentMethod,
    pub total_amount: Decimal,
    pub transaction_status: ArifTransactionStatus,
    pub transaction: ArifTransaction,
    pub notification_url: String,
    pub session_id: String,
}

#[derive(Clone, Serialize, Deserialize, Display)]
#[serde(rename_all = "UPPERCASE")]
#[strum(serialize_all = "UPPERCASE")]
pub enum ArifTransactionStatus {
    Success,
    Failed,
    Pending,
}

impl From<ArifTransactionStatus> for PaymentStatus {
    fn from(value: ArifTransactionStatus) -> Self {
        match value {
            ArifTransactionStatus::Success => PaymentStatus::Success,
            ArifTransactionStatus::Failed => PaymentStatus::Failed,
            ArifTransactionStatus::Pending => PaymentStatus::Pending,
        }
    }
}

impl From<PaymentStatus> for ArifTransactionStatus {
    fn from(value: PaymentStatus) -> Self {
        match value {
            PaymentStatus::Success => ArifTransactionStatus::Success,
            PaymentStatus::Failed => ArifTransactionStatus::Failed,
            PaymentStatus::Pending => ArifTransactionStatus::Pending,
            _ => ArifTransactionStatus::Pending,
        }
    }
}

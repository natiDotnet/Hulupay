use crate::domain::payment_status::{TxDirection, TxStatus};
use crate::domain::provider::Provider;
use rust_decimal::Decimal;
use uuid::Uuid;

/// PaymentTransaction — what actually happened on the wire, per attempt.
#[derive(Debug, Clone, toasty::Model)]
pub struct PaymentTransaction {
    #[key]
    #[auto]
    pub id: Uuid,
    pub payment_order_id: Uuid,
    pub provider: Provider,
    /// None when the provider call never reached the provider (local timeout)
    pub provider_tx_id: Option<String>,
    pub direction: TxDirection,
    #[column(type = "text")]
    pub amount: Decimal,
    pub currency: String,
    pub status: TxStatus,
    /// Full raw JSON response from the provider — never discard this.
    #[column(type = "jsonb")]
    pub provider_response: Option<serde_json::Value>,
    pub created_at: jiff::Timestamp,
    pub updated_at: jiff::Timestamp,
}

// ---------------------------------------------------------------------------
// PaymentTransaction — what actually happened on the wire, per attempt
// ---------------------------------------------------------------------------

use crate::domain::payment_order::PaymentOrder;
use crate::domain::payment_status::{TxDirection, TxStatus};
use crate::domain::provider::Provider;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentTransaction {
    pub id: Uuid,
    pub payment_order_id: Uuid,
    pub provider: Provider,
    /// None when the provider call never reached the provider (local timeout)
    pub provider_tx_id: Option<String>,
    pub direction: TxDirection,
    pub amount: Decimal,
    pub currency: String,
    pub status: TxStatus,
    /// Full raw JSON response from the provider — never discard this
    pub provider_response: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PaymentTransaction {
    pub fn new_charge(order: &PaymentOrder) -> Self {
        Self {
            id: Uuid::new_v4(),
            payment_order_id: order.id,
            provider: order.provider.clone(),
            provider_tx_id: None,
            direction: TxDirection::Charge,
            amount: order.amount,
            currency: order.currency.clone(),
            status: TxStatus::Pending,
            provider_response: serde_json::Value::Null,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn new_refund(order: &PaymentOrder, amount: Decimal) -> Self {
        Self {
            id: Uuid::new_v4(),
            payment_order_id: order.id,
            provider: order.provider.clone(),
            provider_tx_id: None,
            direction: TxDirection::Refund,
            amount,
            currency: order.currency.clone(),
            status: TxStatus::Pending,
            provider_response: serde_json::Value::Null,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn mark_success(&mut self, provider_tx_id: String, response: serde_json::Value) {
        self.provider_tx_id = Some(provider_tx_id);
        self.status = TxStatus::Success;
        self.provider_response = response;
        self.updated_at = Utc::now();
    }

    pub fn mark_failed(&mut self, response: serde_json::Value) {
        self.status = TxStatus::Failed;
        self.provider_response = response;
        self.updated_at = Utc::now();
    }
}

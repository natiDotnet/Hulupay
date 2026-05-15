// domain/src/events.rs — what gets published to the event bus after every transition

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PaymentEvent {
    Created     { payment_id: Uuid, order_ref: String, occurred_at: DateTime<Utc> },
    Processing  { payment_id: Uuid, provider_tx_id: String, occurred_at: DateTime<Utc> },
    Completed   { payment_id: Uuid, occurred_at: DateTime<Utc> },
    Failed      { payment_id: Uuid, reason: String, retryable: bool, occurred_at: DateTime<Utc> },
    Retrying    { payment_id: Uuid, attempt: i32, occurred_at: DateTime<Utc> },
    Cancelled   { payment_id: Uuid, occurred_at: DateTime<Utc> },
    RefundInitiated { payment_id: Uuid, amount: String, occurred_at: DateTime<Utc> },
    RefundCompleted { payment_id: Uuid, occurred_at: DateTime<Utc> },
}
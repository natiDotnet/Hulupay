// domain/src/events.rs — what gets published to the event bus after every transition

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PaymentEvent {
    Created {
        payment_id: Uuid,
        order_ref: String,
        occurred_at: jiff::Timestamp,
    },
    Processing {
        payment_id: Uuid,
        provider_tx_id: String,
        occurred_at: jiff::Timestamp,
    },
    Completed {
        payment_id: Uuid,
        occurred_at: jiff::Timestamp,
    },
    Failed {
        payment_id: Uuid,
        reason: String,
        retryable: bool,
        occurred_at: jiff::Timestamp,
    },
    Retrying {
        payment_id: Uuid,
        attempt: i32,
        occurred_at: jiff::Timestamp,
    },
    Cancelled {
        payment_id: Uuid,
        occurred_at: jiff::Timestamp,
    },
    RefundInitiated {
        payment_id: Uuid,
        amount: String,
        occurred_at: jiff::Timestamp,
    },
    RefundCompleted {
        payment_id: Uuid,
        occurred_at: jiff::Timestamp,
    },
}

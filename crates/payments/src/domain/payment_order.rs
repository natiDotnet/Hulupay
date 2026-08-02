use crate::domain::payment_status::{PaymentStatus, TransitionError};
use crate::domain::provider::Provider;
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug, Clone, toasty::Model)]
pub struct PaymentOrder {
    #[key]
    #[auto]
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub customer_id: Uuid,
    #[unique]
    pub order_ref: String,
    #[column(type = "text")]
    pub amount: Decimal,
    pub currency: String,
    pub status: PaymentStatus,
    pub request_provider: Provider,
    pub provider: Provider,
    #[unique]
    pub idempotency_key: String,
    pub retry_count: i32,
    pub created_at: jiff::Timestamp,
    pub updated_at: jiff::Timestamp,
}

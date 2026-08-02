use crate::domain::provider::Provider;
use uuid::Uuid;

#[derive(Debug, Clone, toasty::Model)]
pub struct PaymentWebhook {
    #[key]
    #[auto]
    pub id: Uuid,
    pub payment_order_id: Uuid,
    #[column(type = "jsonb")]
    pub body: serde_json::Value,
    pub status: String,
    #[column(type = "jsonb")]
    pub headers: serde_json::Value,
    pub last_error: Option<String>,
    pub retry_count: u32,
    pub next_retry_at: Option<jiff::Timestamp>,
    pub provider: Provider,
    pub created_at: jiff::Timestamp,
    pub updated_at: jiff::Timestamp,
}

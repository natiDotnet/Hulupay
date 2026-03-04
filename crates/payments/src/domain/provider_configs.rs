use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PaymentProviderConfig {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub provider_id: Uuid,
    pub is_test_mode: bool,
    pub config: serde_json::Value,
    pub is_active: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone)]
pub struct PaymentProvider {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub is_active: bool,
    pub created_at: OffsetDateTime,
}

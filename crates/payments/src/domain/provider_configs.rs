use crate::domain::environment::Environment;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PaymentProvider {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub logo: String,
    pub is_active: bool,
    pub created_at: jiff::Timestamp,
}

#[derive(Debug, Clone)]
pub struct PaymentProviderConfig {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub provider_id: Uuid,
    pub priority: i32,
    pub environment: Environment,
    pub config: serde_json::Value,
    pub is_active: bool,
    pub is_default: bool,
    pub created_at: jiff::Timestamp,
    pub updated_at: jiff::Timestamp,
}

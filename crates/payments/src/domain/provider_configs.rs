use crate::domain::environment::Environment;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PaymentProvider {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub logo: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

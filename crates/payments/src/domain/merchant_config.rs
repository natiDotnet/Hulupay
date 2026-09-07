use crate::domain::environment::Environment;
use uuid::Uuid;

/// A merchant's provider-specific configuration (API keys, settings).
#[derive(Debug, Clone, toasty::Model)]
pub struct MerchantConfig {
    #[key]
    #[auto]
    pub id: Uuid,
    #[index]
    pub merchant_id: Uuid,
    #[index]
    pub provider_id: Uuid,
    pub is_test_mode: bool,
    pub environment: Environment,
    pub priority: i32,
    pub is_default: bool,
    pub is_active: bool,
    #[column(type = "jsonb")]
    pub config: serde_json::Value,
    pub created_at: jiff::Timestamp,
    pub updated_at: jiff::Timestamp,
}

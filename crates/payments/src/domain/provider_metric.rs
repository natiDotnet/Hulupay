use crate::domain::provider_status::ProviderStatus;
use sea_orm::entity::prelude::*;
use sea_orm::prelude::DateTimeUtc;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel};
use uuid::Uuid;

/// Live health/metrics snapshot for a payment provider.
#[sea_orm::model]
#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "provider_metric")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(unique)]
    #[sea_orm(indexed)]
    pub provider_id: Uuid,
    pub current_status: ProviderStatus,
    /// Fraction in [0.0, 1.0].
    #[sea_orm(default_value = "1.0")]
    pub success_rate: f64,
    #[sea_orm(default_value = "0")]
    pub average_latency_ms: i64,
    #[sea_orm(default_value = "0")]
    pub error_rate: f64,
    pub last_health_check: Option<DateTimeUtc>,
    pub updated_at: DateTimeUtc,
}

impl ActiveModelBehavior for ActiveModel {}

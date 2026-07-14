use crate::domain::routing_rule::{ConditionOperator, ConditionType};
use sea_orm::entity::prelude::*;
use sea_orm::prelude::DateTimeUtc;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel};
use uuid::Uuid;

/// A merchant's explicit, ordered routing rule. Lower `priority` runs first.
#[sea_orm::model]
#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "merchant_routing_rule")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(indexed)]
    pub merchant_id: Uuid,
    /// Lower number = higher priority (runs first).
    pub priority: i32,
    #[sea_orm(default_value = "true")]
    pub enabled: bool,
    pub condition_type: ConditionType,
    pub operator: ConditionOperator,
    /// Free-form value compared against the condition (e.g. "TELEBIRR", "ETB", "5000").
    pub condition_value: String,
    /// Provider to route to when the rule matches.
    pub target_provider_id: Uuid,
    /// Optional fallback provider if the target is unavailable.
    pub fallback_provider_id: Option<Uuid>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

impl ActiveModelBehavior for ActiveModel {}

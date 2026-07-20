use crate::domain::routing_strategy::RoutingStrategy;
use sea_orm::entity::prelude::*;
use sea_orm::prelude::DateTimeUtc;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel};
use uuid::Uuid;

/// One row per merchant: the active high-level routing strategy.
#[sea_orm::model]
#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "merchant_routing_strategy")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(unique)]
    #[sea_orm(indexed)]
    pub merchant_id: Uuid,
    pub strategy: RoutingStrategy,
    #[sea_orm(default_value = "true")]
    pub enabled: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

impl ActiveModelBehavior for ActiveModel {}

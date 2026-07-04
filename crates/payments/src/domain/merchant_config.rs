use crate::domain;
use crate::domain::environment::Environment;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveModelBehavior;
use sea_orm::DeriveEntityModel;
use uuid::Uuid;

#[sea_orm::model]
#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "merchant_configs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(unique_key = "merchant_provider")]
    #[sea_orm(indexed)]
    pub merchant_id: Uuid,
    #[sea_orm(unique_key = "merchant_provider")]
    pub provider_id: Uuid,
    pub is_test_mode: bool,
    pub environment: Environment,
    pub priority: i32,
    #[sea_orm(default_value = "false")]
    pub is_default: bool,
    pub is_active: bool,

    pub config: serde_json::Value,

    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,

    #[sea_orm(belongs_to, from = "provider_id", to = "id")]
    pub payment: HasOne<domain::payment_provider::Entity>,
    #[sea_orm(belongs_to, from = "merchant_id", to = "id")]
    pub merchant: HasOne<merchant::domain::merchant::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

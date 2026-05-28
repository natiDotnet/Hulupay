use crate::domain;
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
    #[sea_orm(index)]
    pub merchant_id: Uuid,
    #[sea_orm(unique_key = "merchant_provider")]
    pub provider_id: Uuid,
    pub is_test_mode: bool,
    pub config: serde_json::Value,
    pub is_active: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    #[sea_orm(belongs_to, from = "provider_id", to = "id")]
    pub payment: HasOne<domain::payment_provider::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

use crate::domain;
use crate::domain::provider;
use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::{DeriveEntityModel, Set};
use uuid::Uuid;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "payment_webhooks")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub payment_order_id: Uuid,
    pub body: serde_json::Value,
    pub status: String,
    pub headers: serde_json::Value,
    pub last_error: Option<String>,
    pub retry_count: u32,
    pub next_retry_at: Option<DateTimeUtc>,
    pub provider: provider::Provider,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,

    #[sea_orm(belongs_to, from = "payment_order_id", to = "id")]
    pub payment_order: HasOne<domain::payment_order::Entity>,
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(Uuid::now_v7()),
            retry_count: Set(0),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..ActiveModelTrait::default()
        }
    }
}

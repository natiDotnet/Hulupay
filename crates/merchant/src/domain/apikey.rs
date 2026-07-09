use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[sea_orm::model]
#[derive(Debug, Deserialize, Serialize, DeriveEntityModel, Clone)]
#[sea_orm(table_name = "apikeys")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub merchant_id: Uuid,
    #[sea_orm(unique)]
    pub name: String,
    pub prefix: String,
    pub hash: String,
    pub scopes: Vec<u8>,
    pub is_active: bool,
    pub expires_at: DateTimeUtc,
    pub last_used_at: Option<DateTimeUtc>,
    pub created_at: DateTimeUtc,
    pub updated_at: Option<DateTimeUtc>,
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(Uuid::now_v7()),
            is_active: Set(true),
            ..ActiveModelTrait::default()
        }
    }
}

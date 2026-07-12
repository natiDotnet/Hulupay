use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::prelude::DateTimeUtc;
use sea_orm::{DeriveEntityModel, Set};
use uuid::Uuid;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "payment_providers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub code: String,
    pub name: String,
    pub logo: String,
    pub is_active: bool,
    pub created_at: DateTimeUtc,
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(Uuid::now_v7()),
            is_active: Set(true),
            created_at: Set(Utc::now()),
            ..ActiveModelTrait::default()
        }
    }
}

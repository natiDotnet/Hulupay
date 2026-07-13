use sea_orm::DeriveEntityModel;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[sea_orm::model]
#[derive(Debug, Deserialize, Serialize, DeriveEntityModel, Clone)]
#[sea_orm(table_name = "nati")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub test: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: Option<DateTimeUtc>,
}

impl ActiveModelBehavior for ActiveModel {}

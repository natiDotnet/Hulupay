use super::Role;
use crate::domain::status::AccountStatus;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, Set};
use sqlx::types::chrono;
use sqlx::types::chrono::Utc;
use uuid::Uuid;

#[sea_orm::model]
#[derive(Debug, DeriveEntityModel, Clone)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub email: String,
    pub name: String,
    pub password_hash: String,
    pub merchant_id: Uuid,
    pub role: Role,
    pub is_active: bool,
    pub status: AccountStatus,
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

pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub password_hash: String,
    pub merchant_id: Uuid,
    pub role: Role,
    pub is_active: bool,
    pub status: AccountStatus,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
}

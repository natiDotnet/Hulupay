use super::Role;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, Set};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[sea_orm::model]
#[derive(Debug, Deserialize, Serialize, DeriveEntityModel, Clone)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub email: String,
    pub password_hash: String,
    pub merchant_id: Option<Uuid>,
    pub role: Role,
    pub is_active: bool,
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
    pub password_hash: String,
    pub merchant_id: Option<Uuid>,
    pub role: Role,
    pub is_active: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}
impl User {
    pub fn new(
        email: String,
        password_hash: String,
        role: Role,
        merchant_id: Option<Uuid>,
    ) -> Self {
        let now = OffsetDateTime::now_utc();
        Self {
            id: Uuid::now_v7(),
            email,
            password_hash,
            role,
            merchant_id,
            is_active: true,
            created_at: now,
            updated_at: None,
        }
    }
}

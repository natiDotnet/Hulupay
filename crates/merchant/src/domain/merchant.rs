use crate::domain::error::DomainError;
use sea_orm::entity::prelude::*;
use sea_orm::prelude::DateTimeUtc;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, Set};
use serde::{Deserialize, Serialize};
use sqlx::encode::IsNull::No;
use sqlx::types::chrono::Utc;
use time::OffsetDateTime;
use uuid::Uuid;

#[sea_orm::model]
#[derive(Debug, Deserialize, Serialize, DeriveEntityModel, Clone)]
#[sea_orm(table_name = "merchants")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub name: String,
    pub is_active: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: Option<DateTimeUtc>,
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(Uuid::now_v7()),
            is_active: Set(true),
            created_at: Set(Utc::now()),
            updated_at: Set(None),
            ..ActiveModelTrait::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Merchant {
    pub id: Uuid,
    pub name: String,
    pub is_active: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

impl Merchant {
    pub fn new(name: String, is_active: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            is_active,
            created_at: OffsetDateTime::now_utc(),
            updated_at: Some(OffsetDateTime::now_utc()),
        }
    }

    pub fn ensure_active(&self) -> Result<(), DomainError> {
        if !self.is_active {
            return Err(DomainError::ProviderUnavailable);
        }
        Ok(())
    }
}

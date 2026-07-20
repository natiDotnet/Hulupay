use crate::domain::error::DomainError;
use crate::domain::merchant_status::MerchantStatus;
use sea_orm::entity::prelude::*;
use sea_orm::prelude::DateTimeUtc;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, Set};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono;
use sqlx::types::chrono::Utc;
use uuid::Uuid;

#[sea_orm::model]
#[derive(Debug, DeriveEntityModel, Clone)]
#[sea_orm(table_name = "merchants")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub name: String,
    #[sea_orm(unique)]
    pub slug: String,
    #[sea_orm(unique)]
    pub email: String,
    pub phone: String,
    pub website: String,
    pub is_active: bool,
    pub status: MerchantStatus,
    pub created_at: DateTimeUtc,
    pub updated_at: Option<DateTimeUtc>,
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(Uuid::now_v7()),
            is_active: Set(true),
            status: Set(MerchantStatus::Active),
            created_at: Set(Utc::now()),
            updated_at: Set(None),
            ..ActiveModelTrait::default()
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Merchant {
    pub id: Uuid,
    pub name: String,
    pub is_active: bool,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
}

impl Merchant {
    pub fn new(name: String, is_active: bool) -> Self {
        Self {
            id: Uuid::now_v7(),
            name,
            is_active,
            created_at: Utc::now(),
            updated_at: Some(Utc::now()),
        }
    }

    pub fn ensure_active(&self) -> Result<(), DomainError> {
        if !self.is_active {
            return Err(DomainError::ProviderUnavailable);
        }
        Ok(())
    }
}

impl From<Model> for Merchant {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            is_active: value.is_active,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

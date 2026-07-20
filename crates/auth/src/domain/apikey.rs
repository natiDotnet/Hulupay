use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Merchant API keys used for `Authorization: Bearer hp_...` auth.
///
/// `scopes` stores dot-notation permission strings (JSON array column)
/// such as `["payment.create", "payment.read"]`, reusing the same
/// `Permission` model as roles.
#[sea_orm::model]
#[derive(Debug, Deserialize, Serialize, DeriveEntityModel, Clone)]
#[sea_orm(table_name = "apikeys")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub merchant_id: Uuid,
    #[sea_orm(unique)]
    pub name: String,
    /// First 12 chars of the generated key — used for O(1) lookup.
    pub prefix: String,
    /// Argon2 hash of the full key.
    pub hash: String,
    /// Dot-notation permission strings granted to this key.
    pub scopes: Vec<String>,
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

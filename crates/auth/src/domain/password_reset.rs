use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, Set};
use uuid::Uuid;

/// Password-reset tokens. Created by the forgot-password flow and consumed
/// (single-use) by reset-password. The raw token is delivered by email;
/// only its SHA-256 hash is stored here.
#[sea_orm::model]
#[derive(Debug, DeriveEntityModel, Clone)]
#[sea_orm(table_name = "password_resets")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub user_id: Uuid,
    /// SHA-256 hex of the raw token.
    pub token_hash: String,
    pub expires_at: DateTimeUtc,
    /// `Some` once the token has been used to reset a password.
    pub used_at: Option<DateTimeUtc>,
    pub created_at: DateTimeUtc,
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(Uuid::now_v7()),
            ..ActiveModelTrait::default()
        }
    }
}

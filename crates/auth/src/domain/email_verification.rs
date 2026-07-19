use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, Set};
use uuid::Uuid;

/// Email-verification tokens. Created on registration and consumed
/// (single-use) by the verify-email endpoint. The raw token is delivered
/// by email; only its SHA-256 hash is stored here.
#[sea_orm::model]
#[derive(Debug, DeriveEntityModel, Clone)]
#[sea_orm(table_name = "email_verifications")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub user_id: Uuid,
    /// SHA-256 hex of the raw token.
    pub token_hash: String,
    pub expires_at: DateTimeUtc,
    /// `Some` once the email has been verified.
    pub verified_at: Option<DateTimeUtc>,
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

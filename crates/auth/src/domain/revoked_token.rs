use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, Set};
use uuid::Uuid;

/// Access-token blocklist for server-side logout. `id` is the JWT `jti`.
/// Rows are only retained until `expires_at` (the original access token's
/// expiry), so they can be periodically cleaned up.
#[sea_orm::model]
#[derive(Debug, DeriveEntityModel, Clone)]
#[sea_orm(table_name = "revoked_tokens")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub user_id: Uuid,
    /// When the original access token would have expired (for cleanup).
    pub expires_at: DateTimeUtc,
    pub revoked_at: DateTimeUtc,
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(Uuid::now_v7()),
            ..ActiveModelTrait::default()
        }
    }
}

use super::permission::Permission;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, Set};
use uuid::Uuid;

/// Junction table mapping role names to permissions.
///
/// Built-in roles (`owner`, `admin`, `developer`, `finance`, `viewer`,
/// `master_admin`, `merchant_admin`) are seeded at startup. Custom roles
/// (future) will also store their mappings here.
#[sea_orm::model]
#[derive(Debug, DeriveEntityModel, Clone)]
#[sea_orm(table_name = "role_permissions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    /// Role name in lowercase: `"owner"`, `"admin"`, `"developer"`, etc.
    pub role_name: String,
    /// The permission granted by this role.
    pub permission: Permission,
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(Uuid::now_v7()),
            ..ActiveModelTrait::default()
        }
    }
}

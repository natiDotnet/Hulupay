use super::permission::Permission;
use uuid::Uuid;

/// Junction table mapping role names to permissions.
///
/// Built-in roles (`owner`, `admin`, `developer`, `finance`, `viewer`,
/// `master_admin`, `merchant_admin`) are seeded at startup. Custom roles
/// (future) will also store their mappings here.
#[derive(Debug, Clone, toasty::Model)]
pub struct RolePermission {
    #[key]
    #[auto]
    pub id: Uuid,
    /// Role name in lowercase: `"owner"`, `"admin"`, `"developer"`, etc.
    pub role_name: String,
    /// The permission granted by this role.
    pub permission: Permission,
}

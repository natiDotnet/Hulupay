use crate::Role;
use crate::domain::permission::Permission;
use crate::domain::role_permission::RolePermission;

/// Service responsible for resolving permissions from roles and
/// seeding the built-in role→permission mappings at startup.
#[derive(Clone)]
pub struct PermissionService {
    db: toasty::Db,
}

impl PermissionService {
    pub fn new(db: toasty::Db) -> Self {
        Self { db }
    }

    /// Load the set of permissions granted by a built-in role.
    ///
    /// Queries the `role_permissions` table. Returns an empty set if
    /// the role has no mappings (e.g. a custom role not yet seeded).
    pub async fn get_permissions_for_role(&self, role: &Role) -> Vec<String> {
        let role_name = role_to_name(role);
        self.get_permissions_for_role_str(&role_name).await
    }

    /// Same as `get_permissions_for_role` but accepts a raw role name string
    /// (e.g. from a JWT claim).
    pub async fn get_permissions_for_role_str(&self, role_name: &str) -> Vec<String> {
        let mut db = self.db.clone();
        let rows = RolePermission::filter(RolePermission::fields().role_name().eq(role_name))
            .exec(&mut db)
            .await
            .unwrap_or_default();

        rows.iter().map(|r| r.permission.to_string()).collect()
    }

    /// Seed all built-in role→permission mappings (idempotent).
    ///
    /// Deletes existing mappings for each role and re-inserts so the
    /// mappings always match the compiled-in definitions. Safe to call
    /// on every startup.
    pub async fn seed_builtin_roles(&self) {
        let roles = [
            (Role::MasterAdmin, master_admin_permissions()),
            (Role::MerchantAdmin, merchant_admin_permissions()),
            (Role::Owner, owner_permissions()),
            (Role::Admin, admin_permissions()),
            (Role::Developer, developer_permissions()),
            (Role::Finance, finance_permissions()),
            (Role::Viewer, viewer_permissions()),
        ];

        for (role, permissions) in &roles {
            let role_name = role_to_name(role);
            let mut db = self.db.clone();

            // Delete existing mappings for this role
            if let Err(e) =
                RolePermission::filter(RolePermission::fields().role_name().eq(&role_name))
                    .delete()
                    .exec(&mut db)
                    .await
            {
                tracing::warn!(
                    role = %role_name,
                    error = %e,
                    "failed to clear existing role permissions during seed"
                );
                continue;
            }

            // Insert fresh mappings
            for perm in permissions {
                let mut db = self.db.clone();
                if let Err(e) = toasty::create!(RolePermission {
                    role_name: role_name.clone(),
                    permission: perm.clone(),
                })
                .exec(&mut db)
                .await
                {
                    tracing::warn!(
                        role = %role_name,
                        permission = %perm,
                        error = %e,
                        "failed to seed role permission"
                    );
                }
            }
        }

        tracing::info!("built-in role permissions seeded");
    }
}

/// Convert a `Role` enum variant to the lowercase string stored in `role_permissions`.
fn role_to_name(role: &Role) -> String {
    match role {
        Role::MasterAdmin => "master_admin".to_string(),
        Role::MerchantAdmin => "merchant_admin".to_string(),
        Role::Owner => "owner".to_string(),
        Role::Admin => "admin".to_string(),
        Role::Developer => "developer".to_string(),
        Role::Finance => "finance".to_string(),
        Role::Viewer => "viewer".to_string(),
    }
}

// ── Built-in role permission sets ────────────────────────────────

fn master_admin_permissions() -> Vec<Permission> {
    vec![
        // Platform
        Permission::PlatformManage,
        Permission::PlatformSuspend,
        Permission::PlatformAnalytics,
        // Merchant (everything)
        Permission::MerchantRead,
        Permission::MerchantUpdate,
        Permission::MerchantDelete,
        // Payments (everything)
        Permission::PaymentCreate,
        Permission::PaymentRead,
        Permission::PaymentRefund,
        Permission::PaymentExport,
        // Providers (everything)
        Permission::ProviderRead,
        Permission::ProviderUpdate,
        Permission::ProviderDelete,
        // Routing (everything)
        Permission::RoutingRead,
        Permission::RoutingUpdate,
        // API Keys (everything)
        Permission::ApikeyCreate,
        Permission::ApikeyRotate,
        Permission::ApikeyDelete,
        Permission::ApikeyRead,
        // Webhooks (everything)
        Permission::WebhookRead,
        Permission::WebhookUpdate,
        // Users (everything)
        Permission::UsersRead,
        Permission::UsersInvite,
        Permission::UsersDelete,
        // Audit
        Permission::AuditRead,
    ]
}

fn merchant_admin_permissions() -> Vec<Permission> {
    owner_permissions()
}

fn owner_permissions() -> Vec<Permission> {
    vec![
        Permission::MerchantRead,
        Permission::MerchantUpdate,
        Permission::MerchantDelete,
        Permission::PaymentCreate,
        Permission::PaymentRead,
        Permission::PaymentRefund,
        Permission::PaymentExport,
        Permission::ProviderRead,
        Permission::ProviderUpdate,
        Permission::ProviderDelete,
        Permission::RoutingRead,
        Permission::RoutingUpdate,
        Permission::ApikeyCreate,
        Permission::ApikeyRotate,
        Permission::ApikeyDelete,
        Permission::ApikeyRead,
        Permission::WebhookRead,
        Permission::WebhookUpdate,
        Permission::UsersRead,
        Permission::UsersInvite,
        Permission::UsersDelete,
        Permission::AuditRead,
    ]
}

fn admin_permissions() -> Vec<Permission> {
    // Same as owner minus merchant.delete
    owner_permissions()
        .into_iter()
        .filter(|p| *p != Permission::MerchantDelete)
        .collect()
}

fn developer_permissions() -> Vec<Permission> {
    vec![
        Permission::PaymentRead,
        Permission::ProviderRead,
        Permission::ProviderUpdate,
        Permission::ApikeyCreate,
        Permission::ApikeyRotate,
        Permission::WebhookRead,
        Permission::WebhookUpdate,
        Permission::RoutingRead,
    ]
}

fn finance_permissions() -> Vec<Permission> {
    vec![
        Permission::PaymentRead,
        Permission::PaymentRefund,
        Permission::PaymentExport,
    ]
}

fn viewer_permissions() -> Vec<Permission> {
    vec![
        Permission::PaymentRead,
        Permission::MerchantRead,
        Permission::ProviderRead,
        Permission::RoutingRead,
        Permission::WebhookRead,
        Permission::ApikeyRead,
        Permission::UsersRead,
        Permission::AuditRead,
    ]
}

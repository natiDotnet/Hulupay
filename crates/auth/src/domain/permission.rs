use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

/// Fine-grained permissions that control access to every resource and action.
///
/// Roles are just collections of permissions. Built-in roles map to
/// predefined sets; custom roles (future) will also resolve to these
/// same permissions.
///
/// Stored as a native PostgreSQL enum type. Each variant uses `#[column(variant = "...")]`
/// to preserve the dot-notation discriminant values (e.g. `"payment.create"`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, toasty::Embed)]
pub enum Permission {
    // ── Merchant ──────────────────────────────────────────────
    #[column(variant = "merchant.read")]
    MerchantRead,
    #[column(variant = "merchant.update")]
    MerchantUpdate,
    #[column(variant = "merchant.delete")]
    MerchantDelete,

    // ── Payments ───────────────────────────────────────────────
    #[column(variant = "payment.create")]
    PaymentCreate,
    #[column(variant = "payment.read")]
    PaymentRead,
    #[column(variant = "payment.refund")]
    PaymentRefund,
    #[column(variant = "payment.export")]
    PaymentExport,
    CheckoutCreate,

    // ── Providers ───────────────────────────────────────────────
    #[column(variant = "provider.read")]
    ProviderRead,
    #[column(variant = "provider.update")]
    ProviderUpdate,
    #[column(variant = "provider.delete")]
    ProviderDelete,

    // ── Routing ─────────────────────────────────────────────────
    #[column(variant = "routing.read")]
    RoutingRead,
    #[column(variant = "routing.update")]
    RoutingUpdate,

    // ── API Keys ────────────────────────────────────────────────
    #[column(variant = "apikey.create")]
    ApikeyCreate,
    #[column(variant = "apikey.rotate")]
    ApikeyRotate,
    #[column(variant = "apikey.delete")]
    ApikeyDelete,
    #[column(variant = "apikey.read")]
    ApikeyRead,

    // ── Webhooks ────────────────────────────────────────────────
    #[column(variant = "webhook.read")]
    WebhookRead,
    #[column(variant = "webhook.update")]
    WebhookUpdate,

    // ── Users ───────────────────────────────────────────────────
    #[column(variant = "users.read")]
    UsersRead,
    #[column(variant = "users.invite")]
    UsersInvite,
    #[column(variant = "users.delete")]
    UsersDelete,

    // ── Audit ───────────────────────────────────────────────────
    #[column(variant = "audit.read")]
    AuditRead,

    // ── Platform (admin only) ─────────────────────────────────
    #[column(variant = "platform.manage")]
    PlatformManage,
    #[column(variant = "platform.suspend")]
    PlatformSuspend,
    #[column(variant = "platform.analytics")]
    PlatformAnalytics,
}

impl Permission {
    /// Dot-notation string representation: `"merchant.read"`, `"payment.create"`, etc.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MerchantRead => "merchant.read",
            Self::MerchantUpdate => "merchant.update",
            Self::MerchantDelete => "merchant.delete",
            Self::PaymentCreate => "payment.create",
            Self::PaymentRead => "payment.read",
            Self::PaymentRefund => "payment.refund",
            Self::PaymentExport => "payment.export",
            Self::CheckoutCreate => "checkout.create",
            Self::ProviderRead => "provider.read",
            Self::ProviderUpdate => "provider.update",
            Self::ProviderDelete => "provider.delete",
            Self::RoutingRead => "routing.read",
            Self::RoutingUpdate => "routing.update",
            Self::ApikeyCreate => "apikey.create",
            Self::ApikeyRotate => "apikey.rotate",
            Self::ApikeyDelete => "apikey.delete",
            Self::ApikeyRead => "apikey.read",
            Self::WebhookRead => "webhook.read",
            Self::WebhookUpdate => "webhook.update",
            Self::UsersRead => "users.read",
            Self::UsersInvite => "users.invite",
            Self::UsersDelete => "users.delete",
            Self::AuditRead => "audit.read",
            Self::PlatformManage => "platform.manage",
            Self::PlatformSuspend => "platform.suspend",
            Self::PlatformAnalytics => "platform.analytics",
        }
    }
}

impl Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Parse a dot-notation permission string back into the enum.
impl FromStr for Permission {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "merchant.read" => Ok(Self::MerchantRead),
            "merchant.update" => Ok(Self::MerchantUpdate),
            "merchant.delete" => Ok(Self::MerchantDelete),
            "payment.create" => Ok(Self::PaymentCreate),
            "payment.read" => Ok(Self::PaymentRead),
            "payment.refund" => Ok(Self::PaymentRefund),
            "payment.export" => Ok(Self::PaymentExport),
            "checkout.create" => Ok(Self::CheckoutCreate),
            "provider.read" => Ok(Self::ProviderRead),
            "provider.update" => Ok(Self::ProviderUpdate),
            "provider.delete" => Ok(Self::ProviderDelete),
            "routing.read" => Ok(Self::RoutingRead),
            "routing.update" => Ok(Self::RoutingUpdate),
            "apikey.create" => Ok(Self::ApikeyCreate),
            "apikey.rotate" => Ok(Self::ApikeyRotate),
            "apikey.delete" => Ok(Self::ApikeyDelete),
            "apikey.read" => Ok(Self::ApikeyRead),
            "webhook.read" => Ok(Self::WebhookRead),
            "webhook.update" => Ok(Self::WebhookUpdate),
            "users.read" => Ok(Self::UsersRead),
            "users.invite" => Ok(Self::UsersInvite),
            "users.delete" => Ok(Self::UsersDelete),
            "audit.read" => Ok(Self::AuditRead),
            "platform.manage" => Ok(Self::PlatformManage),
            "platform.suspend" => Ok(Self::PlatformSuspend),
            "platform.analytics" => Ok(Self::PlatformAnalytics),
            _ => Err(format!("unknown permission: {s}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn roundtrips_every_permission_through_str() {
        // The full set of dot-notation strings — as_str() must match the
        // FromStr discriminants exactly or JWT/API-key permission checks break.
        let all = [
            "merchant.read",
            "merchant.update",
            "merchant.delete",
            "payment.create",
            "payment.read",
            "payment.refund",
            "payment.export",
            "provider.read",
            "provider.update",
            "provider.delete",
            "routing.read",
            "routing.update",
            "apikey.create",
            "apikey.rotate",
            "apikey.delete",
            "apikey.read",
            "webhook.read",
            "webhook.update",
            "users.read",
            "users.invite",
            "users.delete",
            "audit.read",
            "platform.manage",
            "platform.suspend",
            "platform.analytics",
        ];

        for s in all {
            let perm =
                Permission::from_str(s).unwrap_or_else(|e| panic!("failed to parse {s}: {e}"));
            assert_eq!(perm.as_str(), s, "as_str mismatch for {s}");
            assert_eq!(perm.to_string(), s, "Display mismatch for {s}");
        }
    }

    #[test]
    fn rejects_malformed_permissions() {
        assert!(Permission::from_str("payment").is_err());
        assert!(Permission::from_str("payment.creat").is_err());
        assert!(Permission::from_str("PAYMENT.CREATE").is_err());
        assert!(Permission::from_str("").is_err());
        assert!(Permission::from_str("payment.create.extra").is_err());
    }
}

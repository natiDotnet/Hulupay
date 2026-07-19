use sea_orm::sea_query::StringLen;
use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

/// Fine-grained permissions that control access to every resource and action.
///
/// Roles are just collections of permissions. Built-in roles map to
/// predefined sets; custom roles (future) will also resolve to these
/// same permissions.
#[derive(
    EnumIter,
    DeriveActiveEnum,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
pub enum Permission {
    // ── Merchant ──────────────────────────────────────────────
    MerchantRead,
    MerchantUpdate,
    MerchantDelete,

    // ── Payments ───────────────────────────────────────────────
    PaymentCreate,
    PaymentRead,
    PaymentRefund,
    PaymentExport,

    // ── Providers ───────────────────────────────────────────────
    ProviderRead,
    ProviderUpdate,
    ProviderDelete,

    // ── Routing ─────────────────────────────────────────────────
    RoutingRead,
    RoutingUpdate,

    // ── API Keys ────────────────────────────────────────────────
    ApikeyCreate,
    ApikeyRotate,
    ApikeyDelete,
    ApikeyRead,

    // ── Webhooks ────────────────────────────────────────────────
    WebhookRead,
    WebhookUpdate,

    // ── Users ───────────────────────────────────────────────────
    UsersRead,
    UsersInvite,
    UsersDelete,

    // ── Audit ───────────────────────────────────────────────────
    AuditRead,

    // ── Platform (admin only) ─────────────────────────────────
    PlatformManage,
    PlatformSuspend,
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
///
/// ```
/// Permission::from_str("payment.create") // => Ok(Permission::PaymentCreate)
/// ```
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

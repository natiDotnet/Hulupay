use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

/// User roles within a merchant or across the platform.
///
/// Stored as a native PostgreSQL enum type. The discriminant uses
/// snake_case (`master_admin`, `merchant_admin`, etc.) which matches
/// the previous SeaORM `rename_all = "snake_case"` convention.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, toasty::Embed)]
pub enum Role {
    MasterAdmin,
    MerchantAdmin,
    Owner,
    Admin,
    Developer,
    Finance,
    Viewer,
}

impl Role {
    pub fn from_string(s: &str) -> Option<Self> {
        Self::from_str(s).ok()
    }
}

impl FromStr for Role {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Accept both snake_case (Toasty DB value) and UPPERCASE (legacy)
        match s {
            "master_admin" | "MASTER_ADMIN" => Ok(Role::MasterAdmin),
            "merchant_admin" | "MERCHANT_ADMIN" => Ok(Role::MerchantAdmin),
            "owner" | "OWNER" => Ok(Role::Owner),
            "admin" | "ADMIN" => Ok(Role::Admin),
            "developer" | "DEVELOPER" => Ok(Role::Developer),
            "finance" | "FINANCE" => Ok(Role::Finance),
            "viewer" | "VIEWER" => Ok(Role::Viewer),
            _ => Err(()),
        }
    }
}
impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Role::MasterAdmin => "MASTER_ADMIN",
            Role::MerchantAdmin => "MERCHANT_ADMIN",
            Role::Owner => "OWNER",
            Role::Admin => "ADMIN",
            Role::Developer => "DEVELOPER",
            Role::Finance => "FINANCE",
            Role::Viewer => "VIEWER",
        };
        write!(f, "{}", str)
    }
}

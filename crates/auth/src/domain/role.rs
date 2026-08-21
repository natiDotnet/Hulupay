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

#[cfg(test)]
mod tests {
    use super::*;

    const ALL_ROLES: [Role; 7] = [
        Role::MasterAdmin,
        Role::MerchantAdmin,
        Role::Owner,
        Role::Admin,
        Role::Developer,
        Role::Finance,
        Role::Viewer,
    ];

    #[test]
    fn parses_snake_case_discriminants() {
        assert_eq!(Role::from_string("master_admin"), Some(Role::MasterAdmin));
        assert_eq!(
            Role::from_string("merchant_admin"),
            Some(Role::MerchantAdmin)
        );
        assert_eq!(Role::from_string("owner"), Some(Role::Owner));
        assert_eq!(Role::from_string("admin"), Some(Role::Admin));
        assert_eq!(Role::from_string("developer"), Some(Role::Developer));
        assert_eq!(Role::from_string("finance"), Some(Role::Finance));
        assert_eq!(Role::from_string("viewer"), Some(Role::Viewer));
    }

    #[test]
    fn parses_legacy_uppercase_forms() {
        assert_eq!(Role::from_string("MASTER_ADMIN"), Some(Role::MasterAdmin));
        assert_eq!(Role::from_string("OWNER"), Some(Role::Owner));
        assert_eq!(Role::from_string("VIEWER"), Some(Role::Viewer));
    }

    #[test]
    fn rejects_unknown_roles() {
        assert_eq!(Role::from_string("superadmin"), None);
        assert_eq!(Role::from_string(""), None);
        assert_eq!(Role::from_string("Master Admin"), None);
    }

    #[test]
    fn display_roundtrips_through_from_string() {
        for role in ALL_ROLES {
            let displayed = role.to_string();
            assert_eq!(Role::from_string(&displayed), Some(role));
        }
    }
}

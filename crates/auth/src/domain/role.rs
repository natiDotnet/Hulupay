use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Role {
    MasterAdmin,
    MerchantAdmin,
}

impl Role {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "MASTER_ADMIN" => Some(Role::MasterAdmin),
            "MERCHANT_ADMIN" => Some(Role::MerchantAdmin),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Role::MasterAdmin => "MASTER_ADMIN",
            Role::MerchantAdmin => "MERCHANT_ADMIN",
        }
    }
}

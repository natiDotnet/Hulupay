use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Role {
    MasterAdmin,
    MerchantAdmin,
}

impl Role {
    pub fn from_string(s: &str) -> Option<Self> {
        Self::from_str(s).ok()
    }
}

impl FromStr for Role {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "MASTER_ADMIN" => Ok(Role::MasterAdmin),
            "MERCHANT_ADMIN" => Ok(Role::MerchantAdmin),
            _ => Err(()),
        }
    }
}
impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Role::MasterAdmin => "MASTER_ADMIN".to_string(),
            Role::MerchantAdmin => "MERCHANT_ADMIN".to_string(),
        };
        write!(f, "{}", str)
    }
}

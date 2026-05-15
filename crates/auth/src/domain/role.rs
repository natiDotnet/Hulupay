use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
use sea_orm::sea_query::StringLen;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
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

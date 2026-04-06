use std::str::FromStr;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, PartialEq, Display, EnumString, Default)]
#[strum(serialize_all = "snake_case")]
pub enum PaymentMethod {
    #[default]
    None,
    Telebirr,
    Cbebirr,
    Yaya,
}

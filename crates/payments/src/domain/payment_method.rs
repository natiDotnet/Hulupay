use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, PartialEq, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum PaymentMethod {
    Telebirr,
    Cbebirr,
    Yaya,
}

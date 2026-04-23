use serde::{Deserialize, Serialize};
use sqlx::Type;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Display, EnumString, Default, Deserialize, Serialize, Type)]
#[sqlx(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PaymentMethod {
    #[default]
    None,
    Telebirr,
    Cbebirr,
    Yaya,
}

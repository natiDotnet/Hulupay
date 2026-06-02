use sea_orm::sea_query::StringLen;
use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use strum_macros::{Display, EnumString};

#[derive(
    Debug,
    Clone,
    Display,
    EnumString,
    Default,
    Deserialize,
    Serialize,
    Type,
    EnumIter,
    DeriveActiveEnum,
    Eq,
    PartialEq,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "UPPERCASE"
)]
// #[strum(serialize_all = "snake_case")]
pub enum PaymentMethod {
    #[default]
    None,
    Telebirr,
    Mpesa,
    CbeBirr,
    AwashBirr,
    Yaya,
    CoopayEbirr,
    ZamZam,
    Binget,
    Kacha,
}

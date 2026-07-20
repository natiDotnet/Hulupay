use sea_orm::sea_query::StringLen;
use sea_orm::{DeriveActiveEnum, EnumIter};
use strum_macros::{Display, EnumString};
#[derive(EnumIter, DeriveActiveEnum, Clone, Eq, PartialEq, Debug, Display, EnumString)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "UPPERCASE"
)]
#[strum(serialize_all = "UPPERCASE")]
pub enum AccountStatus {
    Pending,
    Active,
    Suspended,
    Deleted,
}

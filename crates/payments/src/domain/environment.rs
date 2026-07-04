use sea_orm::sea_query::StringLen;
use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use strum_macros::{Display, EnumString};
use utoipa::ToSchema;

#[derive(
    Debug,
    Clone,
    Display,
    EnumString,
    Deserialize,
    Serialize,
    Type,
    EnumIter,
    DeriveActiveEnum,
    Eq,
    PartialEq,
    ToSchema,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "UPPERCASE"
)]
pub enum Environment {
    Production,
    Sandbox,
}

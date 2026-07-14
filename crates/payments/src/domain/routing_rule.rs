use sea_orm::sea_query::StringLen;
use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use strum_macros::{Display, EnumString};
use utoipa::ToSchema;

/// What a routing rule compares against the incoming payment.
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
    ToSchema,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "UPPERCASE"
)]
#[strum(serialize_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum ConditionType {
    #[default]
    PaymentMethod,
    Currency,
    AmountGreaterThan,
    AmountLessThan,
}

/// How a routing rule compares the condition value to the payment.
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
    ToSchema,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "UPPERCASE"
)]
#[strum(serialize_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum ConditionOperator {
    #[default]
    Equals,
    GreaterThan,
    LessThan,
}

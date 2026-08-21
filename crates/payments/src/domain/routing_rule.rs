use serde::{Deserialize, Serialize};
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
    Eq,
    PartialEq,
    ToSchema,
    toasty::Embed,
)]
#[column(rename_all = "UPPERCASE")]
#[strum(serialize_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum ConditionType {
    #[default]
    PaymentMethod,
    Currency,
    Country,
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
    Eq,
    PartialEq,
    ToSchema,
    toasty::Embed,
)]
#[column(rename_all = "UPPERCASE")]
#[strum(serialize_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum ConditionOperator {
    #[default]
    Equals,
    GreaterThan,
    LessThan,
}

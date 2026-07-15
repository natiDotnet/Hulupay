use sea_orm::sea_query::StringLen;
use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use strum_macros::{Display, EnumString};
use utoipa::ToSchema;

/// The high-level strategy a merchant uses to pick a provider.
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
pub enum RoutingStrategy {
    /// Use the merchant's default-configured provider.
    #[default]
    Default,
    /// Try providers in ascending `merchant_configs.priority` order.
    Priority,
    /// Route by the requested payment method.
    PaymentMethod,
    /// Route by the requested currency.
    Currency,
    /// Route by the payment amount thresholds.
    Amount,
    /// Route by the customer's country.
    Country,
    /// Route to the provider with the lowest configured fee.
    LowestCost,
    /// Evaluate the merchant's explicit ordered routing rules first.
    RuleBased,
}

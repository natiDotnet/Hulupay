use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use utoipa::ToSchema;

/// The live health state of a provider. `Offline` providers are never routed to.
#[derive(
    Debug, Clone, Display, EnumString, Default, Deserialize, Serialize, Eq, PartialEq, ToSchema, toasty::Embed,
)]
#[column(rename_all = "UPPERCASE")]
#[strum(serialize_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum ProviderStatus {
    #[default]
    Healthy,
    Degraded,
    Offline,
}

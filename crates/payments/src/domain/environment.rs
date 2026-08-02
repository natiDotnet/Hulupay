use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Display, EnumString, Deserialize, Serialize, Eq, PartialEq, ToSchema, toasty::Embed,
)]
#[column(rename_all = "UPPERCASE")]
pub enum Environment {
    Production,
    Sandbox,
}

use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct ArifPayConfigRequest {
    pub api_key: String,
    pub is_test_key: bool,
}
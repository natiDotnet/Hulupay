use uuid::Uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateMerchantRequest {
    pub name: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MerchantResponse {
    pub id: Uuid,
    pub name: String,
    pub is_active: bool,
}
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateMerchantRequest {
    pub name: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateMerchantRequest {
    pub id: Uuid,
    pub name: String,
    pub is_active: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MerchantResponse {
    pub id: Uuid,
    pub name: String,
    pub is_active: bool,
}

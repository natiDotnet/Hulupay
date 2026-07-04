use crate::domain::merchant_status::MerchantStatus;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateMerchantRequest {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub website: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateMerchantRequest {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub website: String,
    pub status: MerchantStatus,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MerchantResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub website: String,
    pub is_active: bool,
    pub status: MerchantStatus,
}

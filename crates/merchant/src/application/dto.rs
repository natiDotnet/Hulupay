use crate::domain::merchant_status::MerchantStatus;
use sea_orm::prelude::DateTimeUtc;
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

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub scopes: Vec<u8>,
    #[schema(value_type = String, format = DateTime)]
    pub expires_at: DateTimeUtc,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateApiKeyRequest {
    pub name: Option<String>,
    pub scopes: Option<Vec<u8>>,
    pub is_active: Option<bool>,
    #[schema(value_type = String, format = DateTime)]
    pub expires_at: Option<DateTimeUtc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiKeyResponse {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub name: String,
    pub prefix: String,
    pub scopes: Vec<u8>,
    pub is_active: bool,
    #[schema(value_type = String, format = DateTime)]
    pub expires_at: DateTimeUtc,
    #[schema(value_type = String, format = DateTime)]
    pub last_used_at: Option<DateTimeUtc>,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTimeUtc,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: Option<DateTimeUtc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateApiKeyResponse {
    #[serde(flatten)]
    pub apikey: ApiKeyResponse,
    pub key: String,
}

impl From<crate::domain::apikey::Model> for ApiKeyResponse {
    fn from(value: crate::domain::apikey::Model) -> Self {
        Self {
            id: value.id,
            merchant_id: value.merchant_id,
            name: value.name,
            prefix: value.prefix,
            scopes: value.scopes,
            is_active: value.is_active,
            expires_at: value.expires_at,
            last_used_at: value.last_used_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

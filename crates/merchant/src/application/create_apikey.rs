use crate::application::dto::{ApiKeyResponse, CreateApiKeyRequest, CreateApiKeyResponse};
use crate::application::error::ApplicationError;
use crate::domain::merchant;
use anyhow::anyhow;
use auth::domain::apikey;
use auth::application::password::hash_password;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use sqlx::types::chrono::Utc;
use uuid::Uuid;

#[derive(Clone)]
pub struct CreateApiKey {
    db: DatabaseConnection,
}

impl CreateApiKey {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        merchant_id: Uuid,
        request: CreateApiKeyRequest,
    ) -> Result<CreateApiKeyResponse, ApplicationError> {
        merchant::Entity::find_by_id(merchant_id)
            .one(&self.db)
            .await
            .map_err(|e| ApplicationError::Internal(anyhow!(e)))?
            .ok_or_else(|| {
                ApplicationError::NotFound("Merchant not found with given id".to_string())
            })?;

        let key = generate_key();
        let prefix = key.chars().take(12).collect::<String>();
        let hash = hash_password(&key).map_err(ApplicationError::Internal)?;

        let apikey = apikey::ActiveModel {
            merchant_id: Set(merchant_id),
            name: Set(request.name),
            prefix: Set(prefix),
            hash: Set(hash),
            scopes: Set(request.scopes),
            expires_at: Set(request.expires_at),
            created_at: Set(Utc::now()),
            updated_at: Set(None),
            last_used_at: Set(None),
            ..Default::default()
        }
        .insert(&self.db)
        .await
        .map_err(|e| ApplicationError::Internal(anyhow!(e)))?;

        Ok(CreateApiKeyResponse {
            apikey: ApiKeyResponse::from(apikey),
            key,
        })
    }
}

fn generate_key() -> String {
    format!("hp_{}_{}", Uuid::now_v7().simple(), Uuid::now_v7().simple())
}

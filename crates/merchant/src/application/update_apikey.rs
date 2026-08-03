use crate::application::dto::UpdateApiKeyRequest;
use crate::application::error::ApplicationError;
use anyhow::anyhow;
use auth::domain::apikey;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use sqlx::types::chrono::Utc;
use uuid::Uuid;

#[derive(Clone)]
pub struct UpdateApiKey {
    db: DatabaseConnection,
}

impl UpdateApiKey {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        id: Uuid,
        request: UpdateApiKeyRequest,
    ) -> Result<(), ApplicationError> {
        let apikey = apikey::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| ApplicationError::Internal(anyhow!(e)))?
            .ok_or_else(|| ApplicationError::NotFound("Api key not found".to_string()))?;

        let mut apikey: apikey::ActiveModel = apikey.into();

        if let Some(name) = request.name {
            apikey.name = Set(name);
        }
        if let Some(scopes) = request.scopes {
            apikey.scopes = Set(scopes);
        }
        if let Some(is_active) = request.is_active {
            apikey.is_active = Set(is_active);
        }
        if let Some(expires_at) = request.expires_at {
            apikey.expires_at = Set(expires_at);
        }

        apikey.updated_at = Set(Some(Utc::now()));
        apikey
            .update(&self.db)
            .await
            .map_err(|e| ApplicationError::Internal(anyhow!(e)))?;

        Ok(())
    }
}

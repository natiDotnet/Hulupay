use crate::application::dto::ApiKeyResponse;
use crate::application::error::ApplicationError;
use anyhow::anyhow;
use auth::domain::apikey::ApiKey;
use toasty::Db;
use uuid::Uuid;

#[derive(Clone)]
pub struct ListApiKeys {
    db: Db,
}

impl ListApiKeys {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        merchant_id: Uuid,
    ) -> Result<Vec<ApiKeyResponse>, ApplicationError> {
        let mut db = self.db.clone();
        let apikeys = ApiKey::filter(ApiKey::fields().merchant_id().eq(merchant_id))
            .order_by(ApiKey::fields().created_at().desc())
            .exec(&mut db)
            .await
            .map_err(|e| ApplicationError::Internal(anyhow!(e)))?;

        Ok(apikeys.into_iter().map(ApiKeyResponse::from).collect())
    }
}

use crate::application::dto::UpdateApiKeyRequest;
use crate::application::error::ApplicationError;
use crate::util;
use anyhow::anyhow;
use auth::domain::apikey::ApiKey;
use toasty::Db;
use uuid::Uuid;

#[derive(Clone)]
pub struct UpdateApiKey {
    db: Db,
}

impl UpdateApiKey {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        id: Uuid,
        request: UpdateApiKeyRequest,
    ) -> Result<(), ApplicationError> {
        let mut db = self.db.clone();
        let mut apikey = match ApiKey::filter_by_id(id).first().exec(&mut db).await {
            Ok(Some(k)) => k,
            Ok(None) => return Err(ApplicationError::NotFound("Api key not found".to_string())),
            Err(e) => return Err(ApplicationError::Internal(anyhow!(e))),
        };

        // Build conditional update — only apply fields that are Some
        if let Some(name) = request.name {
            toasty::update!(apikey {
                name,
                updated_at: util::now_jiff()
            })
            .exec(&mut db)
            .await
            .map_err(|e| ApplicationError::Internal(anyhow!(e)))?;
        }
        if let Some(scopes) = request.scopes {
            toasty::update!(apikey {
                scopes,
                updated_at: util::now_jiff()
            })
            .exec(&mut db)
            .await
            .map_err(|e| ApplicationError::Internal(anyhow!(e)))?;
        }
        if let Some(is_active) = request.is_active {
            toasty::update!(apikey {
                is_active,
                updated_at: util::now_jiff()
            })
            .exec(&mut db)
            .await
            .map_err(|e| ApplicationError::Internal(anyhow!(e)))?;
        }
        if let Some(expires_at) = request.expires_at {
            toasty::update!(apikey {
                expires_at: util::to_jiff(expires_at),
                updated_at: util::now_jiff()
            })
            .exec(&mut db)
            .await
            .map_err(|e| ApplicationError::Internal(anyhow!(e)))?;
        }

        Ok(())
    }
}

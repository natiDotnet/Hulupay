use crate::application::dto::{ApiKeyResponse, CreateApiKeyRequest, CreateApiKeyResponse};
use crate::application::error::ApplicationError;
use crate::domain::merchant::Merchant;
use crate::util;
use anyhow::anyhow;
use auth::application::password::hash_password;
use auth::domain::apikey::ApiKey;
use toasty::Db;
use uuid::Uuid;

#[derive(Clone)]
pub struct CreateApiKey {
    db: Db,
}

impl CreateApiKey {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        merchant_id: Uuid,
        request: CreateApiKeyRequest,
    ) -> Result<CreateApiKeyResponse, ApplicationError> {
        let mut db = self.db.clone();

        // Verify merchant exists
        match Merchant::filter_by_id(merchant_id)
            .first()
            .exec(&mut db)
            .await
        {
            Ok(Some(_)) => {}
            Ok(None) => {
                return Err(ApplicationError::NotFound(
                    "Merchant not found with given id".to_string(),
                ));
            }
            Err(e) => return Err(ApplicationError::Internal(anyhow!(e))),
        };

        let key = generate_key();
        let prefix = key.chars().take(12).collect::<String>();
        let hash = hash_password(&key).map_err(ApplicationError::Internal)?;

        let apikey = toasty::create!(ApiKey {
            merchant_id,
            name: request.name,
            prefix,
            hash,
            scopes: request.scopes,
            is_active: true,
            expires_at: request.expires_at,
            created_at: util::now_jiff(),
        })
        .exec(&mut db)
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

use crate::application::error::ApplicationError;
use anyhow::anyhow;
use auth::domain::apikey::ApiKey;
use toasty::Db;
use uuid::Uuid;

#[derive(Clone)]
pub struct DeleteApiKey {
    db: Db,
}

impl DeleteApiKey {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), ApplicationError> {
        let mut db = self.db.clone();

        // Verify existence first
        match ApiKey::filter_by_id(id).first().exec(&mut db).await {
            Ok(Some(_)) => {}
            Ok(None) => return Err(ApplicationError::NotFound("Api key not found".to_string())),
            Err(e) => return Err(ApplicationError::Internal(anyhow!(e))),
        };

        ApiKey::filter_by_id(id).delete().exec(&mut db).await?;

        Ok(())
    }
}

use crate::application::error::ApplicationError;
use anyhow::anyhow;
use auth::domain::apikey;
use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;

#[derive(Clone)]
pub struct DeleteApiKey {
    db: DatabaseConnection,
}

impl DeleteApiKey {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), ApplicationError> {
        let result = apikey::Entity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| ApplicationError::Internal(anyhow!(e)))?;

        if result.rows_affected == 0 {
            return Err(ApplicationError::NotFound("Api key not found".to_string()));
        }

        Ok(())
    }
}

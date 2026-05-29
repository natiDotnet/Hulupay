use crate::application::ApplicationError;
use crate::domain::merchant;
use crate::domain::merchant::ActiveModel;
use anyhow::anyhow;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use sqlx::types::chrono::Utc;
use uuid::Uuid;

#[derive(Clone)]
pub struct UpdateMerchant {
    db: DatabaseConnection,
}

impl UpdateMerchant {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        id: Uuid,
        name: String,
        is_active: bool,
    ) -> Result<(), ApplicationError> {
        let merchant = merchant::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| ApplicationError::Internal(anyhow!(e)))?
            .ok_or_else(|| {
                ApplicationError::NotFound("Merchant not found with given id".to_string())
            })?;
        // .ok_or_else(|| anyhow!("Merchant not found with given id"))?;
        let mut merchant: ActiveModel = merchant.into();
        merchant.name = Set(name);
        merchant.is_active = Set(is_active);
        merchant.updated_at = Set(Some(Utc::now()));
        merchant
            .update(&self.db)
            .await
            .map_err(|e| ApplicationError::Internal(anyhow!(e)))?;
        Ok(())
    }
}

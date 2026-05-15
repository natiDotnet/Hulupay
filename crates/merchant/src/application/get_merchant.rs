use crate::application::dto::MerchantResponse;
use crate::application::error::ApplicationError;
use crate::domain::merchant;
use anyhow::anyhow;
use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;

#[derive(Clone)]
pub struct GetMerchant {
    db: DatabaseConnection,
}

impl GetMerchant {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn execute(&self, id: Uuid) -> Result<MerchantResponse, ApplicationError> {
        // let merchant = self.repository.get_by_id(id).await?;
        let merchant = merchant::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| ApplicationError::Internal(anyhow!(e)))?
            .ok_or_else(|| anyhow!("Merchant not found with given id"))?;

        Ok(MerchantResponse {
            id: merchant.id,
            name: merchant.name,
            is_active: merchant.is_active,
        })
    }
}

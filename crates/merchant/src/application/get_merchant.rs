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
            .ok_or_else(|| {
                ApplicationError::NotFound("Merchant not found with given id".to_string())
            })?;

        Ok(MerchantResponse {
            id: merchant.id,
            name: merchant.name,
            email: merchant.email,
            phone: merchant.phone,
            website: merchant.website,
            status: merchant.status,
            is_active: merchant.is_active,
        })
    }
}

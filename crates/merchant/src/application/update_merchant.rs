use crate::application::ApplicationError;
use crate::domain::merchant;
use crate::domain::merchant::ActiveModel;
use crate::domain::merchant_status::MerchantStatus;
use crate::UpdateMerchantRequest;
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
        request: UpdateMerchantRequest,
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
        merchant.name = Set(request.name);
        merchant.email = Set(request.email);
        merchant.phone = Set(request.phone);
        merchant.website = Set(request.website);
        merchant.status = Set(request.status);
        merchant.is_active = Set(request.status == MerchantStatus::Active);
        merchant.updated_at = Set(Some(Utc::now()));
        merchant
            .update(&self.db)
            .await
            .map_err(|e| ApplicationError::Internal(anyhow!(e)))?;
        Ok(())
    }
}

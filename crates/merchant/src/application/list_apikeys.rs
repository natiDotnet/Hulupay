use crate::application::dto::ApiKeyResponse;
use crate::application::error::ApplicationError;
use anyhow::anyhow;
use auth::domain::apikey;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, Order, QueryFilter, QueryOrder};
use uuid::Uuid;

#[derive(Clone)]
pub struct ListApiKeys {
    db: DatabaseConnection,
}

impl ListApiKeys {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        merchant_id: Uuid,
    ) -> Result<Vec<ApiKeyResponse>, ApplicationError> {
        let apikeys = apikey::Entity::find()
            .filter(apikey::Column::MerchantId.eq(merchant_id))
            .order_by(apikey::Column::CreatedAt, Order::Desc)
            .all(&self.db)
            .await
            .map_err(|e| ApplicationError::Internal(anyhow!(e)))?;

        Ok(apikeys.into_iter().map(ApiKeyResponse::from).collect())
    }
}

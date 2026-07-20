use crate::domain::payments::merchant_webhook;
use anyhow::anyhow;
use merchant::application::ApplicationError;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct ListMerchantWebhooks {
    db: DatabaseConnection,
}

impl ListMerchantWebhooks {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        merchant_id: Uuid,
        page: u64,
        page_size: u64,
    ) -> anyhow::Result<PaginatedResponse<merchant_webhook::Model>> {
        let paginator = merchant_webhook::Entity::find()
            .filter(merchant_webhook::Column::MerchantId.eq(merchant_id))
            .order_by_desc(merchant_webhook::Column::CreatedAt)
            .paginate(&self.db, page_size);

        let total = paginator
            .num_items()
            .await
            .map_err(|e| ApplicationError::Internal(anyhow!(e)))?;
        let items = paginator.fetch_page(page.saturating_sub(1)).await?;

        Ok(PaginatedResponse {
            items,
            total,
            page,
            page_size,
        })
    }
}

#[derive(Debug, Clone)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}

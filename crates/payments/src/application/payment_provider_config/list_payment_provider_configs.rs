use crate::domain::{merchant_config, PaymentProviderConfig};
use anyhow::anyhow;
use merchant::application::ApplicationError;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder};
use uuid::Uuid;

#[derive(Clone)]
pub struct ListPaymentProviderConfigs {
    db: DatabaseConnection,
}

impl ListPaymentProviderConfigs {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        merchant_id: Uuid,
        page: u64,
        page_size: u64,
    ) -> anyhow::Result<PaginatedResponse<PaymentProviderConfig>> {
        let paginator = merchant_config::Entity::find()
            .filter(merchant_config::Column::MerchantId.eq(merchant_id))
            .order_by_desc(merchant_config::Column::CreatedAt)
            .paginate(&self.db, page_size);

        let total = paginator
            .num_items()
            .await
            .map_err(|e| ApplicationError::Internal(anyhow!(e)))?;
        let items = paginator
            .fetch_page(page - 1)
            .await?
            // .map_err(|e| ApplicationError::Internal(anyhow!(e)))?
            .into_iter()
            .map(|p| PaymentProviderConfig {
                id: p.id,
                config: p.config,
                provider_id: p.provider_id,
                merchant_id: p.merchant_id,
                is_active: p.is_active,
                is_test_mode: p.is_test_mode,
                created_at: p.created_at,
                updated_at: p.updated_at,
            })
            .collect();
        // let offset = (page - 1) * page_size;
        // let (configs, total) = self
        //     .repository
        //     .list_by_merchant(merchant_id, offset, page_size)
        //     .await?;

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

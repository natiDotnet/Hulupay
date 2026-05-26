use crate::domain;
use crate::domain::Provider;
use anyhow::anyhow;
use merchant::application::ApplicationError;
use sea_orm::{DatabaseConnection, EntityTrait, Order, PaginatorTrait};

#[derive(Clone)]
pub struct ListPaymentProviders {
    db: DatabaseConnection,
    // repository: Arc<dyn PaymentProviderRepository>,
}

impl ListPaymentProviders {
    pub fn new(
        db: DatabaseConnection,
        // repository: Arc<dyn PaymentProviderRepository>,
    ) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        page: u64,
        page_size: u64,
    ) -> anyhow::Result<PaginatedResponse<Provider>> {
        let paginator = domain::payment_provider::Entity::find()
            .order_by_id(Order::Desc)
            .paginate(&self.db, page_size);

        let total = paginator
            .num_items()
            .await
            .map_err(|e| ApplicationError::Internal(anyhow!(e)))?;
        let items = paginator
            .fetch_page(page)
            .await?
            // .map_err(|e| ApplicationError::Internal(anyhow!(e)))?
            .into_iter()
            .map(|p| Provider {
                id: p.id,
                code: p.code,
                name: p.name,
                is_active: p.is_active,
                created_at: p.created_at,
            })
            .collect();
        // let (providers, total) = self.repository.list(offset, page_size).await?;

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

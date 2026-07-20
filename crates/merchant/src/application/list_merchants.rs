use crate::application::dto::MerchantResponse;
use crate::application::error::ApplicationError;
use crate::domain::merchant;
use anyhow::anyhow;
use sea_orm::{DatabaseConnection, EntityTrait, Order, PaginatorTrait};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub page: u64,
    pub page_size: u64,
    pub total: u64,
}

#[derive(Clone)]
pub struct ListMerchants {
    db: DatabaseConnection,
}

impl ListMerchants {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        page: u64,
        page_size: u64,
    ) -> Result<PaginatedResponse<MerchantResponse>, ApplicationError> {
        let paginator = merchant::Entity::find()
            .order_by_id(Order::Desc)
            .paginate(&self.db, page_size);

        let total = paginator
            .num_items()
            .await
            .map_err(|e| ApplicationError::Internal(anyhow!(e)))?;

        let items = paginator
            .fetch_page(page - 1)
            .await
            .map_err(|e| ApplicationError::Internal(anyhow!(e)))?
            .into_iter()
            .map(|m| MerchantResponse {
                id: m.id,
                name: m.name,
                email: m.email,
                phone: m.phone,
                website: m.website,
                status: m.status,
                is_active: m.is_active,
            })
            .collect();

        Ok(PaginatedResponse {
            items,
            page,
            page_size,
            total,
        })
    }
}

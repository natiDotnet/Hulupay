use crate::application::dto::MerchantResponse;
use crate::application::error::ApplicationError;
use crate::application::repository::MerchantRepository;
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub page: i64,
    pub page_size: i64,
    pub total: i64,
}

#[derive(Clone)]
pub struct ListMerchants {
    repository: Arc<dyn MerchantRepository>,
}

impl ListMerchants {
    pub fn new(repository: Arc<dyn MerchantRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, page: i64, page_size: i64) -> Result<PaginatedResponse<MerchantResponse>, ApplicationError> {
        let offset = (page - 1) * page_size;
        let (merchants, total) = self.repository.list(offset, page_size).await?;
        
        let items = merchants.into_iter()
            .map(|m| MerchantResponse {
                id: m.id,
                name: m.name,
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

use std::sync::Arc;
use crate::merchant::dto::MerchantResponse;
use crate::merchant::repository::MerchantRepository;
use crate::merchant_error::MerchantError;
use crate::pagination::PaginatedResponse;
#[derive(Clone)]
pub struct ListMerchants {
    repository: Arc<dyn MerchantRepository>,
}

impl ListMerchants {
    pub fn new(repository: Arc<dyn MerchantRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, page: i64, page_size: i64) -> Result<PaginatedResponse<MerchantResponse>, MerchantError> {
        let offset = (page - 1) * page_size;
        let (merchants, total) = self.repository
            .list(offset, page_size).await
            .map_err(|err| MerchantError::Internal(err))?;
        
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
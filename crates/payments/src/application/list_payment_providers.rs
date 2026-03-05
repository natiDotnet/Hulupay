use crate::application::repository::PaymentProviderRepository;
use crate::domain::Provider;
use std::sync::Arc;

#[derive(Clone)]
pub struct ListPaymentProviders {
    repository: Arc<dyn PaymentProviderRepository>,
}

impl ListPaymentProviders {
    pub fn new(repository: Arc<dyn PaymentProviderRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, page: i64, page_size: i64) -> anyhow::Result<PaginatedResponse<Provider>> {
        let offset = (page - 1) * page_size;
        let (providers, total) = self.repository.list(offset, page_size).await?;

        Ok(PaginatedResponse {
            items: providers,
            total,
            page,
            page_size,
        })
    }
}

#[derive(Debug, Clone)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}

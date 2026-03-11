use crate::application::repository::PaymentProviderConfigRepository;
use crate::domain::PaymentProviderConfig;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct ListPaymentProviderConfigs {
    repository: Arc<dyn PaymentProviderConfigRepository>,
}

impl ListPaymentProviderConfigs {
    pub fn new(repository: Arc<dyn PaymentProviderConfigRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        merchant_id: Uuid,
        page: i64,
        page_size: i64,
    ) -> anyhow::Result<PaginatedResponse<PaymentProviderConfig>> {
        let offset = (page - 1) * page_size;
        let (configs, total) = self
            .repository
            .list_by_merchant(merchant_id, offset, page_size)
            .await?;

        Ok(PaginatedResponse {
            items: configs,
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

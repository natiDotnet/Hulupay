use crate::application::repository::PaymentProviderConfigRepository;
use crate::domain::PaymentProviderConfig;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct GetPaymentProviderConfig {
    repository: Arc<dyn PaymentProviderConfigRepository>,
}

impl GetPaymentProviderConfig {
    pub fn new(repository: Arc<dyn PaymentProviderConfigRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: Uuid) -> anyhow::Result<Option<PaymentProviderConfig>> {
        self.repository.get_by_id(id).await
    }
}

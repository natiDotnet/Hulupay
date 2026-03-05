use crate::application::repository::PaymentProviderConfigRepository;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct DeletePaymentProviderConfig {
    repository: Arc<dyn PaymentProviderConfigRepository>,
}

impl DeletePaymentProviderConfig {
    pub fn new(repository: Arc<dyn PaymentProviderConfigRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: Uuid) -> anyhow::Result<()> {
        self.repository.delete(id).await
    }
}

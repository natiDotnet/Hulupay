use crate::application::repository::PaymentProviderRepository;
use crate::domain::Provider;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct DeletePaymentProvider {
    repository: Arc<dyn PaymentProviderRepository>,
}

impl DeletePaymentProvider {
    pub fn new(repository: Arc<dyn PaymentProviderRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: Uuid) -> anyhow::Result<()> {
        self.repository.delete(id).await
    }
}

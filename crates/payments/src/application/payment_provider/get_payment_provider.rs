use crate::application::repository::PaymentProviderRepository;
use crate::domain::Provider;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct GetPaymentProvider {
    repository: Arc<dyn PaymentProviderRepository>,
}

impl GetPaymentProvider {
    pub fn new(repository: Arc<dyn PaymentProviderRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: Uuid) -> anyhow::Result<Option<Provider>> {
        self.repository.get_by_id(id).await
    }
}

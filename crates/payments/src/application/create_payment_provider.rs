use crate::application::repository::PaymentProviderRepository;
use crate::domain::Provider;
use std::sync::Arc;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone)]
pub struct CreatePaymentProvider {
    repository: Arc<dyn PaymentProviderRepository>,
}

impl CreatePaymentProvider {
    pub fn new(repository: Arc<dyn PaymentProviderRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        code: String,
        name: String,
        is_active: bool,
    ) -> anyhow::Result<Provider> {
        let provider = Provider {
            id: Uuid::new_v4(),
            code,
            name,
            is_active,
            created_at: OffsetDateTime::now_utc(),
        };

        self.repository.create(&provider).await?;

        Ok(provider)
    }
}

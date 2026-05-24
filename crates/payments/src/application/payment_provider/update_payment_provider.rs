use crate::application::repository::PaymentProviderRepository;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct UpdatePaymentProvider {
    repository: Arc<dyn PaymentProviderRepository>,
}

impl UpdatePaymentProvider {
    pub fn new(repository: Arc<dyn PaymentProviderRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        id: Uuid,
        code: String,
        name: String,
        is_active: bool,
    ) -> anyhow::Result<()> {
        let mut provider = self
            .repository
            .get_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Payment provider not found"))?;

        provider.code = code;
        provider.name = name;
        provider.is_active = is_active;

        self.repository.update(&provider).await?;

        Ok(())
    }
}

use crate::application::repository::PaymentProviderConfigRepository;
use crate::domain::PaymentProviderConfig;
use std::sync::Arc;
use uuid::Uuid;
use serde_json::Value;
use time::OffsetDateTime;

#[derive(Clone)]
pub struct UpdatePaymentProviderConfig {
    repository: Arc<dyn PaymentProviderConfigRepository>,
}

impl UpdatePaymentProviderConfig {
    pub fn new(repository: Arc<dyn PaymentProviderConfigRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        id: Uuid,
        merchant_id: Uuid,
        provider_id: Uuid,
        is_test_mode: bool,
        config: Value,
        is_active: bool,
    ) -> anyhow::Result<()> {
        let mut config_entity = self.repository.get_by_id(id).await?
            .ok_or_else(|| anyhow::anyhow!("Payment provider config not found"))?;

        config_entity.merchant_id = merchant_id;
        config_entity.provider_id = provider_id;
        config_entity.is_test_mode = is_test_mode;
        config_entity.config = config;
        config_entity.is_active = is_active;
        config_entity.updated_at = OffsetDateTime::now_utc();

        self.repository.update(&config_entity).await?;

        Ok(())
    }
}

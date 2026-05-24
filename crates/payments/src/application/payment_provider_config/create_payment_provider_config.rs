use crate::application::repository::{PaymentProviderConfigRepository, PaymentProviderRepository};
use crate::domain::PaymentProviderConfig;
use serde_json::Value;
use std::sync::Arc;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone)]
pub struct CreatePaymentProviderConfig {
    config_repository: Arc<dyn PaymentProviderConfigRepository>,
    provider_repository: Arc<dyn PaymentProviderRepository>,
}

impl CreatePaymentProviderConfig {
    pub fn new(
        config_repository: Arc<dyn PaymentProviderConfigRepository>,
        provider_repository: Arc<dyn PaymentProviderRepository>,
    ) -> Self {
        Self {
            config_repository,
            provider_repository,
        }
    }

    pub async fn execute(
        &self,
        merchant_id: Uuid,
        provider_id: Uuid,
        is_test_mode: bool,
        config: Value,
        is_active: bool,
    ) -> anyhow::Result<PaymentProviderConfig> {
        // Verify that the provider exists
        self.provider_repository
            .get_by_id(provider_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Payment provider not found"))?;

        let config_entity = PaymentProviderConfig {
            id: Uuid::new_v4(),
            merchant_id,
            provider_id,
            is_test_mode,
            config,
            is_active,
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        };

        self.config_repository.create(&config_entity).await?;

        Ok(config_entity)
    }
}

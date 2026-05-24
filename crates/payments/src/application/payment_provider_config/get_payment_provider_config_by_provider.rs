use crate::application::repository::PaymentProviderConfigRepository;
use crate::application::PaymentGatewayError::ProviderNotFound;
use crate::application::PaymentProviderRepository;
use crate::domain::PaymentProviderConfig;
use crate::ArifPayConfig;
use anyhow::anyhow;
use auth::UserContext;
use std::sync::Arc;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone)]
pub struct GetPaymentProviderConfigByProvider {
    repository: Arc<dyn PaymentProviderConfigRepository>,
    provider_repo: Arc<dyn PaymentProviderRepository>,
}

impl GetPaymentProviderConfigByProvider {
    pub fn new(
        repository: Arc<dyn PaymentProviderConfigRepository>,
        provider_repo: Arc<dyn PaymentProviderRepository>,
    ) -> Self {
        Self {
            repository,
            provider_repo,
        }
    }

    pub async fn execute(
        &self,
        user_context: UserContext,
        provider_code: &str,
    ) -> anyhow::Result<PaymentProviderConfig> {
        let merchant_id = user_context
            .merchant_id
            .ok_or_else(|| anyhow!("Merchant ID not found"))?;
        let provider = self.provider_repo.get_by_code(provider_code).await?;

        let provider = provider.ok_or_else(|| anyhow!("Provider not found"))?;

        let result = self
            .repository
            .list_active_by_provider_code(merchant_id, provider_code)
            .await?;
        if !result.is_empty() {
            return result
                .into_iter()
                .next()
                .ok_or_else(|| anyhow!(ProviderNotFound));
        }

        if provider_code == "arifpay" {
            return Ok(PaymentProviderConfig {
                id: Uuid::nil(),
                merchant_id,
                provider_id: provider.id,
                config: serde_json::json!(ArifPayConfig::new(String::new(), true)),
                is_test_mode: true,
                is_active: false,
                created_at: OffsetDateTime::now_utc(),
                updated_at: OffsetDateTime::now_utc(),
            });
        }
        // Return the first matching config
        result
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!(ProviderNotFound))
    }
}

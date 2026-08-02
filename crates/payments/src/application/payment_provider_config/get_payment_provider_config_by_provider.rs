use crate::domain::environment::Environment;
use crate::domain::merchant_config::MerchantConfig;
use crate::domain::payment_provider::PaymentProvider;
use crate::domain::{PaymentProviderConfig, provider};
use crate::ProviderEngine;
use anyhow::anyhow;
use auth::UserContext;
use chrono::Utc;
use toasty::Db;
use uuid::Uuid;

#[derive(Clone)]
pub struct GetPaymentProviderConfigByProvider {
    db: Db,
    provider_engine: ProviderEngine,
}

impl GetPaymentProviderConfigByProvider {
    pub fn new(db: Db, provider_engine: ProviderEngine) -> Self {
        Self {
            db,
            provider_engine,
        }
    }

    pub async fn execute(
        &self,
        user_context: UserContext,
        provider_code: provider::Provider,
    ) -> anyhow::Result<PaymentProviderConfig> {
        let mut db = self.db.clone();
        let merchant_id = user_context.merchant_id;

        let provider_row = PaymentProvider::filter(
            PaymentProvider::fields().code().eq(provider_code.to_string()),
        )
        .first()
        .exec(&mut db)
        .await?
        .ok_or_else(|| anyhow!("provider not found"))?;

        let result = MerchantConfig::filter(
            MerchantConfig::fields()
                .merchant_id()
                .eq(merchant_id)
                .and(MerchantConfig::fields().provider_id().eq(provider_row.id))
                .and(MerchantConfig::fields().is_active().eq(true)),
        )
        .first()
        .exec(&mut db)
        .await?
        .map(|p| PaymentProviderConfig {
            id: p.id,
            merchant_id: p.merchant_id,
            provider_id: p.provider_id,
            priority: p.priority,
            environment: p.environment,
            config: p.config,
            is_active: p.is_active,
            is_default: p.is_default,
            created_at: crate::util::to_chrono(p.created_at),
            updated_at: crate::util::to_chrono(p.updated_at),
        });

        let gateway = self
            .provider_engine
            .get_provider(None, Some(&provider_code))
            .await
            .ok_or(anyhow!("provider not found"))?;

        let now = Utc::now();
        match result {
            None => Ok(PaymentProviderConfig {
                id: Uuid::nil(),
                merchant_id,
                provider_id: provider_row.id,
                config: gateway.get_config(),
                priority: 1,
                environment: Environment::Sandbox,
                is_active: false,
                is_default: false,
                created_at: now,
                updated_at: now,
            }),
            Some(config) => Ok(config),
        }
    }
}

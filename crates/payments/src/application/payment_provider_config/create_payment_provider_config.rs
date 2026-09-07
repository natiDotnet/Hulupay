use crate::domain;
use crate::domain::PaymentProviderConfig;
use crate::domain::environment::Environment;
use anyhow::anyhow;
use serde::Deserialize;
use toasty::Db;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct CreatePaymentProviderConfigRequest {
    pub merchant_id: Uuid,
    pub provider_id: Uuid,
    pub environment: Environment,
    pub priority: i32,
    pub config: serde_json::Value,
    #[serde(default = "default_is_active")]
    pub is_active: bool,
    pub is_default: bool,
}

fn default_is_active() -> bool {
    true
}

#[derive(Clone)]
pub struct CreatePaymentProviderConfig {
    db: Db,
}

impl CreatePaymentProviderConfig {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        request: CreatePaymentProviderConfigRequest,
    ) -> anyhow::Result<PaymentProviderConfig> {
        let mut db = self.db.clone();

        domain::payment_provider::PaymentProvider::filter_by_id(request.provider_id)
            .first()
            .exec(&mut db)
            .await?
            .ok_or_else(|| anyhow!("Payment provider does not exist"))?;

        let is_test_mode = request.environment == Environment::Sandbox;
        let now = crate::util::now_jiff();
        let config_entity = toasty::create!(domain::merchant_config::MerchantConfig {
            merchant_id: request.merchant_id,
            provider_id: request.provider_id,
            is_test_mode,
            environment: request.environment,
            priority: request.priority,
            config: request.config,
            is_active: request.is_active,
            is_default: request.is_default,
            created_at: now,
            updated_at: now,
        })
        .exec(&mut db)
        .await?;

        Ok(PaymentProviderConfig {
            id: config_entity.id,
            merchant_id: config_entity.merchant_id,
            provider_id: config_entity.provider_id,
            priority: config_entity.priority,
            environment: config_entity.environment,
            config: config_entity.config,
            is_active: config_entity.is_active,
            is_default: config_entity.is_default,
            created_at: config_entity.created_at,
            updated_at: config_entity.updated_at,
        })
    }
}

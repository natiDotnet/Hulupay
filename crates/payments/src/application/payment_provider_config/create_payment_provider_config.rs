use crate::domain;
use crate::domain::PaymentProviderConfig;
use anyhow::anyhow;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use serde::Deserialize;
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;
use crate::domain::environment::Environment;

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
    db: DatabaseConnection,
}

impl CreatePaymentProviderConfig {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        request: CreatePaymentProviderConfigRequest,
    ) -> anyhow::Result<PaymentProviderConfig> {
        domain::payment_provider::Entity::find_by_id(request.provider_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| anyhow!("Payment provider does not exist"))?;

        let config_entity = domain::merchant_config::ActiveModel {
            id: Set(Uuid::now_v7()),
            merchant_id: Set(request.merchant_id),
            provider_id: Set(request.provider_id),
            is_test_mode: Set(request.environment == Environment::Sandbox),
            environment: Set(request.environment),
            priority: Set(request.priority),
            config: Set(request.config),
            is_active: Set(request.is_active),
            is_default: Set(request.is_default),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
        }
        .insert(&self.db)
        .await?;

        Ok(PaymentProviderConfig {
            id: config_entity.id,
            merchant_id: config_entity.merchant_id,
            provider_id: config_entity.provider_id,
            is_test_mode: config_entity.is_test_mode,
            config: config_entity.config,
            is_active: config_entity.is_active,
            is_default: config_entity.is_default,
            created_at: config_entity.created_at,
            updated_at: config_entity.updated_at,
        })
    }
}

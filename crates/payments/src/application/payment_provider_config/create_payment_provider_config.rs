use crate::application::repository::{PaymentProviderConfigRepository, PaymentProviderRepository};
use crate::domain::PaymentProviderConfig;
use serde_json::Value;
use std::sync::Arc;
use anyhow::anyhow;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use time::OffsetDateTime;
use uuid::Uuid;
use crate::domain;

#[derive(Clone)]
pub struct CreatePaymentProviderConfig {
    db: DatabaseConnection,
}

impl CreatePaymentProviderConfig {
    pub fn new(
        db: DatabaseConnection,
    ) -> Self {
        Self {
            db,
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
        domain::payment_provider::Entity::find_by_id(provider_id).one(&self.db).await?
            .ok_or_else(|| anyhow!("Payment provider does not exist"))?;
        
        let config_entity = domain::merchant_config::ActiveModel {
            id: Set(Uuid::now_v7()),
            merchant_id: Set(merchant_id),
            provider_id: Set(provider_id),
            is_test_mode: Set(is_test_mode),
            config: Set(config),
            is_active: Set(is_active),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
        }.insert(&self.db).await?;
        
        Ok(PaymentProviderConfig {
            id: config_entity.id,
            merchant_id: config_entity.merchant_id,
            provider_id: config_entity.provider_id,
            is_test_mode: config_entity.is_test_mode,
            config: config_entity.config,
            is_active: config_entity.is_active,
            created_at: config_entity.created_at,
            updated_at: config_entity.updated_at,
        })
    }
}

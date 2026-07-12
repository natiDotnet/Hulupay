use crate::domain;
use crate::domain::PaymentProviderConfig;
use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;

#[derive(Clone)]
pub struct GetPaymentProviderConfig {
    db: DatabaseConnection,
}

impl GetPaymentProviderConfig {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn execute(&self, id: Uuid) -> anyhow::Result<Option<PaymentProviderConfig>> {
        Ok(domain::merchant_config::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .map(|p| PaymentProviderConfig {
                id: p.id,
                merchant_id: p.merchant_id,
                provider_id: p.provider_id,
                updated_at: p.updated_at,
                priority: p.priority,
                environment: p.environment,
                // is_test_mode: p.is_test_mode,
                config: p.config,
                is_active: p.is_active,
                is_default: p.is_default,
                created_at: p.created_at,
            }))
    }
}

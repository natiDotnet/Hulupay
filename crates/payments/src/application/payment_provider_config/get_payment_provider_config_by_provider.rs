use crate::domain::environment::Environment;
use crate::domain::{PaymentProviderConfig, merchant_config};
use crate::{ProviderEngine, domain};
use anyhow::anyhow;
use auth::UserContext;
use chrono::Utc;
use domain::payment_provider;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{DatabaseConnection, EntityTrait, JoinType, QuerySelect, RelationTrait};
use uuid::Uuid;

#[derive(Clone)]
pub struct GetPaymentProviderConfigByProvider {
    db: DatabaseConnection,
    provider_engine: ProviderEngine,
}

impl GetPaymentProviderConfigByProvider {
    pub fn new(db: DatabaseConnection, provider_engine: ProviderEngine) -> Self {
        Self {
            db,
            provider_engine,
        }
    }

    pub async fn execute(
        &self,
        user_context: UserContext,
        provider_code: domain::provider::Provider,
    ) -> anyhow::Result<PaymentProviderConfig> {
        let merchant_id = user_context.merchant_id;

        let provider = payment_provider::Entity::find_by_code(provider_code.to_string())
            .one(&self.db)
            .await?
            .ok_or_else(|| anyhow!("provider not found"))?;

        let result = merchant_config::Entity::find()
            .join(
                JoinType::InnerJoin,
                merchant_config::Relation::PaymentProvider.def(),
            )
            .filter(merchant_config::Column::MerchantId.eq(merchant_id))
            .filter(domain::payment_provider::Column::Code.eq(provider_code.clone()))
            .filter(merchant_config::Column::IsActive.eq(true))
            .one(&self.db)
            .await?
            .map(|p| PaymentProviderConfig {
                id: p.id,
                merchant_id: p.merchant_id,
                provider_id: p.provider_id,
                priority: p.priority,
                environment: p.environment,
                // is_test_mode: p.is_test_mode,
                config: p.config,
                is_active: p.is_active,
                is_default: p.is_default,
                created_at: p.created_at,
                updated_at: p.updated_at,
            });
        let gateway = self
            .provider_engine
            .get_provider(None, Some(&provider_code))
            .await
            .ok_or(anyhow!("provider not found"))?;
        match result {
            None => Ok(PaymentProviderConfig {
                id: Uuid::nil(),
                merchant_id,
                provider_id: provider.id,
                config: gateway.get_config(),
                priority: 1,
                environment: Environment::Sandbox,
                // is_test_mode: true,
                is_active: false,
                is_default: false,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            }),
            Some(config) => Ok(config),
        }
    }
}

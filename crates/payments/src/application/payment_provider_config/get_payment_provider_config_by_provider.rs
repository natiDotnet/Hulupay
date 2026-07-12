use crate::domain::provider::Provider;
use crate::domain::{merchant_config, PaymentProviderConfig};
use crate::{domain, ArifPayConfig};
use anyhow::anyhow;
use auth::UserContext;
use chrono::Utc;
use domain::payment_provider;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{DatabaseConnection, EntityTrait, JoinType, QuerySelect, RelationTrait};
use uuid::Uuid;
use crate::domain::environment::Environment;

#[derive(Clone)]
pub struct GetPaymentProviderConfigByProvider {
    db: DatabaseConnection,
}

impl GetPaymentProviderConfigByProvider {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
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

        match result {
            None => match &provider_code {
                Provider::Hulu => Err(anyhow!("provider not found")),
                Provider::Stripe => Err(anyhow!("provider not found")),
                Provider::Chapa => Err(anyhow!("provider not found")),
                Provider::ArifPay => Ok(PaymentProviderConfig {
                    id: Uuid::nil(),
                    merchant_id,
                    provider_id: provider.id,
                    config: serde_json::json!(ArifPayConfig::new(String::new(), true)),
                    priority: 1,
                    environment: Environment::Sandbox,
                    // is_test_mode: true,
                    is_active: false,
                    is_default: false,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                }),
            },
            Some(config) => Ok(config),
        }
    }
}

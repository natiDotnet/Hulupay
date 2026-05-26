use crate::application::payment_gateway::PaymentGateway;
use crate::domain::merchant_config;
use crate::{domain, ArifPayConfig, ArifPayProvider};
use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, RelationTrait};
use sea_orm::{DatabaseConnection, EntityTrait, JoinType, QuerySelect};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

/// A routing engine that selects the appropriate payment provider based on provider name
#[derive(Clone)]
pub struct ProviderEngine {
    providers: HashMap<&'static str, Arc<dyn PaymentGateway>>,
    db: DatabaseConnection, // config: Arc<dyn PaymentProviderConfigRepository>,
                            // transaction_repository: Arc<dyn TransactionRepository>,
}

impl ProviderEngine {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            providers: HashMap::new(),
            db,
        }
    }

    /// Register a payment provider with a specific name
    pub fn register_provider(&mut self, name: &'static str, provider: Arc<dyn PaymentGateway>) {
        self.providers.insert(name, provider);
    }

    /// Get a provider by name
    pub async fn get_provider(
        &self,
        merchant_id: Uuid,
        name: domain::provider::Provider,
    ) -> anyhow::Result<Arc<dyn PaymentGateway>> {
        let my_config = merchant_config::Entity::find()
            .join(
                JoinType::InnerJoin,
                merchant_config::Relation::PaymentProvider.def(),
            )
            .filter(merchant_config::Column::MerchantId.eq(merchant_id))
            .filter(domain::payment_provider::Column::Code.eq(name.clone().to_string()))
            .filter(merchant_config::Column::IsActive.eq(true))
            .one(&self.db)
            .await?;
        if my_config.is_none() {
            return Err(anyhow::anyhow!("Provider config not found"));
        }
        match my_config {
            None => Err(anyhow::anyhow!("Provider config not found")),
            Some(config) => {
                match name {
                    domain::provider::Provider::ArifPay => {
                        let arif_config =
                            serde_json::from_value::<ArifPayConfig>(config.config.clone())?;
                        let payment_gateway: Arc<dyn PaymentGateway> = Arc::new(
                            ArifPayProvider::new(arif_config, config.provider_id, self.db.clone()),
                        );
                        Ok(payment_gateway)
                    }
                    _ => Err(anyhow::anyhow!("Provider not found")),
                }
            }
        }
    }

    /// Check if a provider exists
    pub fn has_provider(&self, name: &str) -> bool {
        self.providers.contains_key(name)
    }

    /// Get all registered provider names
    pub fn provider_names(&self) -> Vec<&'static str> {
        self.providers.keys().copied().collect()
    }
}

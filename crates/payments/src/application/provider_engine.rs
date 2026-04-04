use crate::application::payment_gateway::PaymentGateway;
use crate::application::{PaymentProviderConfigRepository, TransactionRepository};
use crate::{ArifPayConfig, ArifPayProvider};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

/// A routing engine that selects the appropriate payment provider based on provider name
#[derive(Clone)]
pub struct ProviderEngine {
    providers: HashMap<&'static str, Arc<dyn PaymentGateway>>,
    config: Arc<dyn PaymentProviderConfigRepository>,
    transaction_repository: Arc<dyn TransactionRepository>,
}

impl ProviderEngine {
    pub fn new(config_repo: Arc<dyn PaymentProviderConfigRepository>, transaction_repository: Arc<dyn TransactionRepository>) -> Self {
        Self {
            providers: HashMap::new(),
            config: config_repo,
            transaction_repository,
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
        name: &str,
    ) -> anyhow::Result<Arc<dyn PaymentGateway>> {
        let my_config = self
            .config
            .list_active_by_provider_code(merchant_id, name)
            .await?;
        if my_config.is_empty() {
            return Err(anyhow::anyhow!("Provider config not found"));
        }
        let my_config = my_config.first().ok_or(anyhow::anyhow!("Provider config not found"))?;
        
        match name {
            "arifpay" => {
                let arif_config = serde_json::from_value::<ArifPayConfig>(my_config.config.clone())?;
                let payment_gateway: Arc<dyn PaymentGateway> =
                    Arc::new(ArifPayProvider::new(arif_config, my_config.provider_id, self.transaction_repository.clone()));
                Ok(payment_gateway)
            },
            _ => Err(anyhow::anyhow!("Provider not found")),
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

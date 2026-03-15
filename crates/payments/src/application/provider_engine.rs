use crate::application::payment_gateway::PaymentGateway;
use crate::application::PaymentProviderConfigRepository;
use crate::ArifPayProvider;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

/// A routing engine that selects the appropriate payment provider based on provider name
#[derive(Clone)]
pub struct ProviderEngine {
    providers: HashMap<&'static str, Arc<dyn PaymentGateway>>,
    config: Arc<dyn PaymentProviderConfigRepository>,
}

impl ProviderEngine {
    pub fn new(config_repo: Arc<dyn PaymentProviderConfigRepository>) -> Self {
        Self {
            providers: HashMap::new(),
            config: config_repo,
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
    ) -> Option<Arc<dyn PaymentGateway>> {
        let my_config = self
            .config
            .list_active_by_provider_code(merchant_id, name)
            .await
            .unwrap();
        if name == "arifpay" {
            let arif_config = serde::Deserialize::deserialize(&my_config.first()?.config).unwrap();
            let payment_gateway: Arc<dyn PaymentGateway> =
                Arc::new(ArifPayProvider::new(arif_config));
            return Some(payment_gateway);
        }
        self.providers.get(name).cloned()
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

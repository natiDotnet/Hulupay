use crate::application::payment_gateway::PaymentGateway;
use std::collections::HashMap;
use std::sync::Arc;

/// A routing engine that selects the appropriate payment provider based on provider name
#[derive(Clone)]
pub struct ProviderEngine {
    providers: HashMap<&'static str, Arc<dyn PaymentGateway>>,
}

impl ProviderEngine {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    /// Register a payment provider with a specific name
    pub fn register_provider(&mut self, name: &'static str, provider: Arc<dyn PaymentGateway>) {
        self.providers.insert(name, provider);
    }

    /// Get a provider by name
    pub fn get_provider(&self, name: &str) -> Option<&Arc<dyn PaymentGateway>> {
        self.providers.get(name)
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

impl Default for ProviderEngine {
    fn default() -> Self {
        Self::new()
    }
}

use crate::domain;
use crate::domain::merchant_config::MerchantConfig;
use crate::domain::payment_provider::PaymentProvider;
use hulu_core::payment_gateway::PaymentGateway;
use std::collections::HashMap;
use std::sync::Arc;
use toasty::Db;
use uuid::Uuid;
use crate::application::gateways::simulator::{SimulationMode, SimulationProvider};
use crate::domain::provider::Provider::Simulator;

/// A routing engine that selects the appropriate payment provider based on provider name
#[derive(Clone)]
pub struct ProviderEngine {
    providers: HashMap<String, Arc<dyn PaymentGateway>>,
    db: Db,
}

impl ProviderEngine {
    pub fn new(
        providers: HashMap<String, Arc<dyn PaymentGateway>>,
        db: Db,
    ) -> Self {
        Self { providers, db }
    }

    /// Get a provider by name
    pub async fn get_provider(
        &self,
        merchant_id: Option<Uuid>,
        name: Option<&domain::provider::Provider>,
    ) -> Option<Arc<dyn PaymentGateway>> {
        let mut db = self.db.clone();
        let provider = match name {
            None => {
                let merchant_id = merchant_id?;
                // Find the merchant's default active config, then resolve the provider's name.
                let config = MerchantConfig::filter(
                    MerchantConfig::fields()
                        .merchant_id()
                        .eq(merchant_id)
                        .and(MerchantConfig::fields().is_active().eq(true))
                        .and(MerchantConfig::fields().is_default().eq(true)),
                )
                .first()
                .exec(&mut db)
                .await
                .ok()??;

                let provider = PaymentProvider::filter_by_id(config.provider_id)
                    .first()
                    .exec(&mut db)
                    .await
                    .ok()??;

                if provider.is_active {
                    provider.name
                } else {
                    String::new()
                }
            }
            Some(provider) => provider.to_string(),
        };

        self.providers.get(&provider).cloned().or_else(|| {
            Some(Arc::new(SimulationProvider::new(SimulationMode::Success)))
        })
    }

    /// Get a gateway implementation by the provider's UUID. Looks up the
    /// `payment_providers` row, then resolves the gateway by its name.
    pub async fn get_provider_by_id(
        &self,
        provider_id: Uuid,
    ) -> Option<Arc<dyn PaymentGateway>> {
        let mut db = self.db.clone();
        let row = PaymentProvider::filter_by_id(provider_id)
            .first()
            .exec(&mut db)
            .await
            .ok()
            .flatten()?;
        self.providers.get(&row.name).cloned().or_else(|| {
            Some(Arc::new(SimulationProvider::new(SimulationMode::Success)))
        })
    }

    /// Register (or replace) a gateway implementation under the given name.
    ///
    /// This is useful for injecting the `SimulationProvider` at startup
    /// without requiring a database row:
    /// ```ignore
    /// engine.register("Simulator", Arc::new(SimulationProvider::new(SimulationMode::Success)));
    /// ```
    pub fn register(&mut self, name: String, gateway: Arc<dyn PaymentGateway>) {
        self.providers.insert(name, gateway);
    }
}

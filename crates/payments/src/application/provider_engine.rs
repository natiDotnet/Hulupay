use crate::domain;
use crate::domain::{MerchantConfigs, PaymentProviders, merchant_config};
use hulu_core::payment_gateway::PaymentGateway;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

/// A routing engine that selects the appropriate payment provider based on provider name
#[derive(Clone)]
pub struct ProviderEngine {
    providers: HashMap<String, Arc<dyn PaymentGateway>>,
    db: DatabaseConnection,
}

impl ProviderEngine {
    pub fn new(
        providers: HashMap<String, Arc<dyn PaymentGateway>>,
        db: DatabaseConnection,
    ) -> Self {
        Self { providers, db }
    }

    /// Get a provider by name
    pub async fn get_provider(
        &self,
        merchant_id: Option<Uuid>,
        name: Option<&domain::provider::Provider>,
    ) -> Option<&Arc<dyn PaymentGateway>> {
        let provider = match name {
            None => {
                let merchant_id = merchant_id?;
                let (_, provider) = MerchantConfigs::find()
                    .filter(merchant_config::Column::MerchantId.eq(merchant_id))
                    .filter(merchant_config::Column::IsActive.eq(true))
                    .filter(merchant_config::Column::IsDefault.eq(true))
                    .filter(domain::payment_provider::Column::IsActive.eq(true))
                    .find_also_related(PaymentProviders)
                    // .select_only()
                    // .column(domain::payment_provider::Column::Name)
                    .one(&self.db)
                    .await
                    .ok()??;
                provider.map(|m| m.name).unwrap_or_default()
            }
            Some(provider) => provider.to_string(),
        };

        self.providers.get(&provider)
    }

    /// Get a gateway implementation by the provider's UUID. Looks up the
    /// `payment_providers` row, then resolves the gateway by its name.
    pub async fn get_provider_by_id(
        &self,
        provider_id: Uuid,
    ) -> Option<Arc<dyn PaymentGateway>> {
        let row = PaymentProviders::find_by_id(provider_id)
            .one(&self.db)
            .await
            .ok()
            .flatten()?;
        self.providers.get(&row.name).cloned()
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

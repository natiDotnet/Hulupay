use crate::domain;
use crate::domain::{merchant_config, MerchantConfigs, PaymentProviders};
use hulu_core::payment_gateway::PaymentGateway;
use reqwest::Client;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

/// A routing engine that selects the appropriate payment provider based on provider name
#[derive(Clone)]
pub struct ProviderEngine {
    providers: HashMap<String, Arc<dyn PaymentGateway>>,
    db: DatabaseConnection,
    client: Client,
}

impl ProviderEngine {
    pub fn new(
        providers: HashMap<String, Arc<dyn PaymentGateway>>,
        db: DatabaseConnection,
        client: Client,
    ) -> Self {
        Self {
            providers,
            db,
            client,
        }
    }

    /// Get a provider by name
    pub async fn get_provider(
        &self,
        merchant_id: Option<Uuid>,
        name: Option<domain::provider::Provider>,
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
                    .select_only()
                    .column(domain::payment_provider::Column::Name)
                    .one(&self.db)
                    .await
                    .ok()??;
                provider.map(|m| m.name).unwrap_or_default()
            }
            Some(provider) => provider.to_string(),
        };

        self.providers.get(&provider)
    }
}

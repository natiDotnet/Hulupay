use crate::application::payment_gateway::PaymentGateway;
use crate::domain;
use reqwest::Client;
use sea_orm::DatabaseConnection;
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
    pub fn get_provider(
        &self,
        merchant_id: Uuid,
        name: domain::provider::Provider,
    ) -> Option<&Arc<dyn PaymentGateway>> {
        return self.providers.get(&name.to_string());
        // let my_config = merchant_config::Entity::find()
        //     .join(
        //         JoinType::InnerJoin,
        //         merchant_config::Relation::PaymentProvider.def(),
        //     )
        //     .filter(merchant_config::Column::MerchantId.eq(merchant_id))
        //     .filter(domain::payment_provider::Column::Code.eq(name.clone().to_string()))
        //     .filter(merchant_config::Column::IsActive.eq(true))
        //     .one(&self.db)
        //     .await?;
        // if my_config.is_none() {
        //     return Err(anyhow::anyhow!("Provider config not found"));
        // }
        // match my_config {
        //     None => Err(anyhow::anyhow!("Provider config not found")),
        //     Some(config) => match name {
        //         domain::provider::Provider::ArifPay => {
        //             let arif_config =
        //                 serde_json::from_value::<ArifPayConfig>(config.config.clone())?;
        //             let payment_gateway = self.providers.get(&name.to_string()).ok_or_else();
        //             // let payment_gateway: Arc<dyn PaymentGateway> =
        //             //     Arc::new(ArifPayProvider::new(self.client.clone(), self.db.clone()));
        //             Ok(payment_gateway)
        //         }
        //         _ => Err(anyhow::anyhow!("Provider not found")),
        //     },
        // }
    }
}

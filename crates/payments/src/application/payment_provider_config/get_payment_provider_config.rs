use crate::domain;
use crate::domain::PaymentProviderConfig;
use toasty::Db;
use uuid::Uuid;

#[derive(Clone)]
pub struct GetPaymentProviderConfig {
    db: Db,
}

impl GetPaymentProviderConfig {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn execute(&self, id: Uuid) -> anyhow::Result<Option<PaymentProviderConfig>> {
        let mut db = self.db.clone();
        Ok(domain::merchant_config::MerchantConfig::filter_by_id(id)
            .first()
            .exec(&mut db)
            .await?
            .map(|p| PaymentProviderConfig {
                id: p.id,
                merchant_id: p.merchant_id,
                provider_id: p.provider_id,
                updated_at: p.updated_at,
                priority: p.priority,
                environment: p.environment,
                config: p.config,
                is_active: p.is_active,
                is_default: p.is_default,
                created_at: p.created_at,
            }))
    }
}

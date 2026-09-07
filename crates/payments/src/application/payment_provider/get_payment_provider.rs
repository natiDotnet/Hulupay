use crate::domain::Provider;
use crate::domain::payment_provider::PaymentProvider;
use toasty::Db;
use uuid::Uuid;

#[derive(Clone)]
pub struct GetPaymentProvider {
    db: Db,
}

impl GetPaymentProvider {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn execute(&self, id: Uuid) -> anyhow::Result<Option<Provider>> {
        let mut db = self.db.clone();

        let provider = PaymentProvider::filter_by_id(id)
            .first()
            .exec(&mut db)
            .await?
            .map(|provider| Provider {
                id: provider.id,
                code: provider.code,
                name: provider.name,
                logo: provider.logo,
                is_active: provider.is_active,
                created_at: provider.created_at,
            });

        Ok(provider)
    }
}

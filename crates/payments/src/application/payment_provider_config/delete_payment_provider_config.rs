use crate::domain;
use toasty::Db;
use uuid::Uuid;

#[derive(Clone)]
pub struct DeletePaymentProviderConfig {
    db: Db,
}

impl DeletePaymentProviderConfig {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn execute(&self, id: Uuid) -> anyhow::Result<()> {
        let mut db = self.db.clone();
        domain::merchant_config::MerchantConfig::filter_by_id(id)
            .delete()
            .exec(&mut db)
            .await?;
        Ok(())
    }
}

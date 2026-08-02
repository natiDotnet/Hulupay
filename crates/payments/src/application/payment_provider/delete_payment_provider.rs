use crate::domain;
use toasty::Db;
use uuid::Uuid;

#[derive(Clone)]
pub struct DeletePaymentProvider {
    db: Db,
}

impl DeletePaymentProvider {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn execute(&self, id: Uuid) -> anyhow::Result<()> {
        let mut db = self.db.clone();

        match domain::payment_provider::PaymentProvider::filter_by_id(id)
            .first()
            .exec(&mut db)
            .await?
        {
            Some(_) => {}
            None => return Err(anyhow::anyhow!("Payment provider not found")),
        }

        domain::payment_provider::PaymentProvider::filter_by_id(id)
            .delete()
            .exec(&mut db)
            .await?;
        Ok(())
    }
}

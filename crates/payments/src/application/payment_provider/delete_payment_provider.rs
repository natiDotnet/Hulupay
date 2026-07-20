use crate::domain;
use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;

#[derive(Clone)]
pub struct DeletePaymentProvider {
    db: DatabaseConnection,
    // repository: Arc<dyn PaymentProviderRepository>,
}

impl DeletePaymentProvider {
    pub fn new(
        db: DatabaseConnection,
        // repository: Arc<dyn PaymentProviderRepository>
    ) -> Self {
        Self { db }
    }

    pub async fn execute(&self, id: Uuid) -> anyhow::Result<()> {
        domain::payment_provider::Entity::delete_by_id(id)
            .exec(&self.db)
            .await?;
        Ok(())
    }
}

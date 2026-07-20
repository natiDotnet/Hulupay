use crate::domain;
use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;

#[derive(Clone)]
pub struct DeletePaymentProviderConfig {
    db: DatabaseConnection,
}

impl DeletePaymentProviderConfig {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn execute(&self, id: Uuid) -> anyhow::Result<()> {
        domain::merchant_config::Entity::delete_by_id(id)
            .exec(&self.db)
            .await?;
        Ok(())
    }
}

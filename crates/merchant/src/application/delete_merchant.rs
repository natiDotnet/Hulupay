use crate::domain::merchant;
use anyhow::anyhow;
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};
use uuid::Uuid;

#[derive(Clone)]
pub struct DeleteMerchant {
    db: DatabaseConnection,
}

impl DeleteMerchant {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn execute(&self, id: Uuid) -> anyhow::Result<()> {
        let merchant = merchant::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| anyhow!("Merchant not found"))?;

        merchant.delete(&self.db).await?;
        Ok(())
    }
}

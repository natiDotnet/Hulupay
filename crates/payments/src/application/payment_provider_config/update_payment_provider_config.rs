use crate::domain;
use anyhow::anyhow;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, Set};
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone)]
pub struct UpdatePaymentProviderConfig {
    db: DatabaseConnection,
}

impl UpdatePaymentProviderConfig {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        id: Uuid,
        is_test_mode: bool,
        config: Value,
        is_active: bool,
    ) -> anyhow::Result<()> {
        let config_entity = domain::merchant_config::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| anyhow!("merchant provider config not found"))?;

        let mut config_entity = config_entity.into_active_model();
        config_entity.is_test_mode = Set(is_test_mode);
        config_entity.config = Set(config);
        config_entity.is_active = Set(is_active);
        config_entity.updated_at = Set(Utc::now());

        config_entity.save(&self.db).await?;
        Ok(())
    }
}

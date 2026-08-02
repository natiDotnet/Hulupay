use crate::domain;
use crate::domain::environment::Environment;
use anyhow::anyhow;
use serde_json::Value;
use toasty::Db;
use uuid::Uuid;

#[derive(Clone)]
pub struct UpdatePaymentProviderConfig {
    db: Db,
}

impl UpdatePaymentProviderConfig {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        id: Uuid,
        environment: Environment,
        config: Value,
        is_active: bool,
    ) -> anyhow::Result<()> {
        let mut db = self.db.clone();

        let mut config_entity = domain::merchant_config::MerchantConfig::filter_by_id(id)
            .first()
            .exec(&mut db)
            .await?
            .ok_or_else(|| anyhow!("merchant provider config not found"))?;

        let is_test_mode = environment == Environment::Sandbox;
        toasty::update!(config_entity {
            environment: environment.clone(),
            is_test_mode,
            config,
            is_active,
            updated_at: crate::util::now_jiff(),
        })
        .exec(&mut db)
        .await?;
        Ok(())
    }
}

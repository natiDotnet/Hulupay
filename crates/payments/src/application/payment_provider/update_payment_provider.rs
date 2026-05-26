use crate::domain;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, Set};
use uuid::Uuid;

#[derive(Clone)]
pub struct UpdatePaymentProvider {
    db: DatabaseConnection,
    // repository: Arc<dyn PaymentProviderRepository>,
}

impl UpdatePaymentProvider {
    pub fn new(
        db: DatabaseConnection,
        // repository: Arc<dyn PaymentProviderRepository>,
    ) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        id: Uuid,
        code: String,
        name: String,
        is_active: bool,
    ) -> anyhow::Result<()> {
        let mut provider = domain::payment_provider::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Payment provider not found"))?
            .into_active_model();

        provider.code = Set(code);
        provider.name = Set(name);
        provider.is_active = Set(is_active);

        provider.save(&self.db).await?;
        Ok(())
    }
}

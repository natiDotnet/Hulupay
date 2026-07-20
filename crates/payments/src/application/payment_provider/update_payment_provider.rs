use crate::domain;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, Set};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdatePaymentProviderRequest {
    pub code: String,
    pub name: String,
    pub logo: String,
    pub is_active: bool,
}
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
        request: UpdatePaymentProviderRequest,
    ) -> anyhow::Result<()> {
        let mut provider = domain::payment_provider::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Payment provider not found"))?
            .into_active_model();

        provider.code = Set(request.code);
        provider.name = Set(request.name);
        provider.logo = Set(request.logo);
        provider.is_active = Set(request.is_active);

        provider.save(&self.db).await?;
        Ok(())
    }
}

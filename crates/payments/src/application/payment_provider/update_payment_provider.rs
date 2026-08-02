use crate::domain;
use serde::{Deserialize, Serialize};
use toasty::Db;
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
    db: Db,
}

impl UpdatePaymentProvider {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        id: Uuid,
        request: UpdatePaymentProviderRequest,
    ) -> anyhow::Result<()> {
        let mut db = self.db.clone();

        let mut provider = domain::payment_provider::PaymentProvider::filter_by_id(id)
            .first()
            .exec(&mut db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Payment provider not found"))?;

        toasty::update!(provider {
            code: request.code,
            name: request.name,
            logo: request.logo,
            is_active: request.is_active,
        })
        .exec(&mut db)
        .await?;

        Ok(())
    }
}

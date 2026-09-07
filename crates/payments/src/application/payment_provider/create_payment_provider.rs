use crate::domain;
use crate::domain::Provider;
use serde::Deserialize;
use toasty::Db;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CreatePaymentProviderRequest {
    pub code: String,
    pub name: String,
    pub logo: String,
    #[serde(default = "default_is_active")]
    pub is_active: bool,
}
fn default_is_active() -> bool {
    true
}
#[derive(Clone)]
pub struct CreatePaymentProvider {
    db: Db,
}

impl CreatePaymentProvider {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn execute(&self, request: CreatePaymentProviderRequest) -> anyhow::Result<Provider> {
        let mut db = self.db.clone();

        let provider = toasty::create!(domain::payment_provider::PaymentProvider {
            code: request.code,
            name: request.name,
            logo: request.logo,
            is_active: request.is_active,
            created_at: crate::util::now_jiff(),
        })
        .exec(&mut db)
        .await?;

        Ok(Provider {
            id: provider.id,
            code: provider.code,
            name: provider.name,
            logo: provider.logo,
            is_active: provider.is_active,
            created_at: provider.created_at,
        })
    }
}

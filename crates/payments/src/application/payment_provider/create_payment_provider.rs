use crate::domain;
use crate::domain::Provider;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, NotSet, Set};
use serde::Deserialize;
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
    db: DatabaseConnection,
}

impl CreatePaymentProvider {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn execute(&self, request: CreatePaymentProviderRequest) -> anyhow::Result<Provider> {
        let provider = domain::payment_provider::ActiveModel {
            id: NotSet,
            code: Set(request.code),
            name: Set(request.name),
            logo: Set(request.logo),
            is_active: Set(request.is_active),
            created_at: Set(Utc::now()),
        }
        .insert(&self.db)
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

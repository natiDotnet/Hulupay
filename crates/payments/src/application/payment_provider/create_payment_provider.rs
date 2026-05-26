use crate::domain;
use crate::domain::Provider;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, NotSet, Set};

#[derive(Clone)]
pub struct CreatePaymentProvider {
    db: DatabaseConnection,
}

impl CreatePaymentProvider {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        code: String,
        name: String,
        is_active: bool,
    ) -> anyhow::Result<Provider> {
        let provider = domain::payment_provider::ActiveModel {
            id: NotSet,
            code: Set(code),
            name: Set(name),
            is_active: Set(is_active),
            created_at: Set(Utc::now()),
        }
        .insert(&self.db)
        .await?;

        Ok(Provider {
            id: provider.id,
            code: provider.code,
            name: provider.name,
            is_active: provider.is_active,
            created_at: provider.created_at,
        })
    }
}

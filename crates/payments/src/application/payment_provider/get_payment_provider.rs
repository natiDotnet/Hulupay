use crate::domain;
use crate::domain::Provider;
use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;

#[derive(Clone)]
pub struct GetPaymentProvider {
    db: DatabaseConnection,
}

impl GetPaymentProvider {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn execute(&self, id: Uuid) -> anyhow::Result<Option<Provider>> {
        let provider = domain::payment_provider::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .map(|provider| Provider {
                id: provider.id,
                code: provider.code,
                name: provider.name,
                logo: provider.logo,
                is_active: provider.is_active,
                created_at: provider.created_at,
            });

        Ok(provider)
    }
}

use crate::application::dto::{CreateMerchantRequest, MerchantResponse};
use crate::domain::merchant;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

#[derive(Clone)]
pub struct CreateMerchant {
    db: DatabaseConnection,
}

impl CreateMerchant {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        request: CreateMerchantRequest,
    ) -> anyhow::Result<MerchantResponse> {
        let merchant = merchant::ActiveModel {
            name: Set(request.name),
            ..Default::default()
        }
        .insert(&self.db)
        .await?;

        Ok(MerchantResponse {
            id: merchant.id,
            name: merchant.name,
            is_active: merchant.is_active,
        })
    }
}

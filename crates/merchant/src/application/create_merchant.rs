use crate::application::dto::{CreateMerchantRequest, MerchantResponse};
use crate::domain::merchant;
use hulucore::create_slug;
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
            slug: Set(create_slug(&request.name)),
            name: Set(request.name),
            email: Set(request.email),
            phone: Set(request.phone),
            website: Set(request.website),
            ..Default::default()
        }
        .insert(&self.db)
        .await?;

        Ok(MerchantResponse {
            id: merchant.id,
            name: merchant.name,
            email: merchant.email,
            phone: merchant.phone,
            website: merchant.website,
            status: merchant.status,
            is_active: merchant.is_active,
        })
    }
}

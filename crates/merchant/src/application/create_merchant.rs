use crate::application::dto::{CreateMerchantRequest, MerchantResponse};
use crate::domain::merchant::Merchant;
use hulucore::create_slug;
use toasty::Db;

#[derive(Clone)]
pub struct CreateMerchant {
    db: Db,
}

impl CreateMerchant {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        request: CreateMerchantRequest,
    ) -> anyhow::Result<MerchantResponse> {
        let mut db = self.db.clone();
        let merchant = toasty::create!(Merchant {
            slug: create_slug(&request.name),
            name: request.name,
            email: request.email,
            phone: request.phone,
            website: request.website,
        })
        .exec(&mut db)
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

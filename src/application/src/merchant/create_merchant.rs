use crate::merchant::dto::{CreateMerchantRequest, MerchantResponse};
use crate::merchant::repository::MerchantRepository;
use domain::merchant::Merchant;
use std::sync::Arc;
#[derive(Clone)]
pub struct CreateMerchant {
    repository: Arc<dyn MerchantRepository>,
}

impl CreateMerchant {
    pub fn new(repository: Arc<dyn MerchantRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, request: CreateMerchantRequest)
                         -> anyhow::Result<MerchantResponse> {
        let merchant = Merchant::new(request.name, true);

        self.repository.create(&merchant).await?;

        Ok(MerchantResponse {
            id: merchant.id,
            name: merchant.name,
            is_active: merchant.is_active,
        })
    }
}
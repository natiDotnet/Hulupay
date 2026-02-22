use uuid::Uuid;
use domain::merchant::Merchant;
use crate::merchant::dto::MerchantResponse;
use crate::merchant::repository::MerchantRepository;

pub struct GetMerchant<R: MerchantRepository> {
    repository: R,
}

impl<R: MerchantRepository> GetMerchant<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: Uuid) -> anyhow::Result<Option<MerchantResponse>> {
        let merchant = self.repository.get_by_id(id).await?;
        
        Ok(merchant.map(|m| MerchantResponse {
            id: m.id,
            name: m.name,
            is_active: m.is_active,
        }))
    }
}
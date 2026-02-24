use crate::merchant::dto::MerchantResponse;
use crate::merchant::repository::MerchantRepository;
use std::sync::Arc;
use anyhow::anyhow;
use uuid::Uuid;

#[derive(Clone)]
pub struct GetMerchant {
    repository: Arc<dyn MerchantRepository>,
}
enum  AppError {
    NotFount(String)
}
impl GetMerchant {
    pub fn new(repository: Arc<dyn MerchantRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: Uuid) -> anyhow::Result<Option<MerchantResponse>> {
        let merchant = self.repository.get_by_id(id).await?;

        // return anyhow!();
        
        Ok(merchant.map(|m| MerchantResponse {
            id: m.id,
            name: m.name,
            is_active: m.is_active,
        }))
    }
}
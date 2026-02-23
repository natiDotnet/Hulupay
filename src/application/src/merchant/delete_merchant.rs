use std::sync::Arc;
use anyhow::anyhow;
use uuid::Uuid;
use crate::merchant::repository::MerchantRepository;
#[derive(Clone)]
pub struct DeleteMerchant{
    repository: Arc<dyn MerchantRepository>,
}
impl DeleteMerchant {
    pub fn new(repository: Arc<dyn MerchantRepository>) -> Self {
        Self { repository }
    }
    
    pub async fn execute(&self, id: Uuid) -> anyhow::Result<()> {
        self.repository.delete(id).await?;
        Ok(())
    }
}
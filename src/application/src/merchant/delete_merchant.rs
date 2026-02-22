use anyhow::anyhow;
use uuid::Uuid;
use crate::merchant::repository::MerchantRepository;

pub struct DeleteMerchant<R: MerchantRepository> {
    repository: R,
}
impl<R: MerchantRepository> DeleteMerchant<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    
    pub async fn execute(&self, id: Uuid) -> anyhow::Result<()> {
        self.repository.delete(id).await?;
        Ok(())
    }
}
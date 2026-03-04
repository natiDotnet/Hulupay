use crate::domain::Merchant;
use uuid::Uuid;
use async_trait::async_trait;

#[async_trait]
pub trait MerchantRepository: Sync + Send {
    async fn create(&self, merchant: &Merchant) -> anyhow::Result<()>;
    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<Merchant>>;
    async fn update(&self, merchant: &Merchant) -> anyhow::Result<()>;
    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
    async fn list(
        &self,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<(Vec<Merchant>, i64)>;
}

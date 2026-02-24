use domain::merchant::Merchant;
use uuid::Uuid;
use async_trait::async_trait;

#[async_trait]
pub trait MerchantRepository: Sync + Send {
    async fn create(&self, merchant: &Merchant) -> anyhow::Result<()>;
    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<Merchant>>;
    async fn update(&self, merchant: &Merchant) -> anyhow::Result<()>;
    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
}
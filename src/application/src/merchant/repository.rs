use domain::merchant::Merchant;
use uuid::Uuid;
use async_trait::async_trait;
use domain::transaction::Transaction;

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

#[async_trait]
pub trait TransactionRepository: Sync + Send {
    async fn save(&self, transaction: &Transaction) -> anyhow::Result<()>;

    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Transaction>>;

    async fn find_by_provider_session(
        &self,
        session_id: &str,
    ) -> anyhow::Result<Option<Transaction>>;
}
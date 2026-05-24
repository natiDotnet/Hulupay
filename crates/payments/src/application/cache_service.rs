use anyhow::Result;

#[async_trait::async_trait]
pub trait CacheService: Send + Sync {
    async fn set(
        &self,
        key: &str,
        value: String,
        ttl_seconds: u64,
    ) -> Result<()>;

    async fn get(
        &self,
        key: &str,
    ) -> Result<Option<String>>;

    async fn delete(
        &self,
        key: &str,
    ) -> Result<()>;
}
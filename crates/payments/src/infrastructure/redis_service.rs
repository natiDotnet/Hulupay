use crate::application::cache_service::CacheService;
use anyhow::Result;
use deadpool_redis::{redis::AsyncCommands, Pool};

pub struct RedisCacheService {
    pool: Pool,
}

impl RedisCacheService {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl CacheService for RedisCacheService {
    async fn set(&self, key: &str, value: String, ttl_seconds: u64) -> Result<()> {
        let mut conn = self.pool.get().await?;

        conn.set_ex::<_, _, ()>(key, value, ttl_seconds).await?;

        Ok(())
    }

    async fn get(&self, key: &str) -> Result<Option<String>> {
        let mut conn = self.pool.get().await?;

        let value = conn.get(key).await?;

        Ok(value)
    }

    async fn delete(&self, key: &str) -> Result<()> {
        let mut conn = self.pool.get().await?;

        conn.del::<_, ()>(key).await?;

        Ok(())
    }
}

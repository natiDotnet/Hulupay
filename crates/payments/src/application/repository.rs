use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{PaymentProviderConfig, Provider};

#[async_trait]
pub trait PaymentProviderRepository: Send + Sync {
    async fn create(&self, provider: &Provider) -> anyhow::Result<()>;
    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<Provider>>;
    async fn get_by_code(&self, code: &str) -> anyhow::Result<Option<Provider>>;
    async fn update(&self, provider: &Provider) -> anyhow::Result<()>;
    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
    async fn list(
        &self,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<(Vec<Provider>, i64)>;
    async fn list_active(&self) -> anyhow::Result<Vec<Provider>>;
}

#[async_trait]
pub trait PaymentProviderConfigRepository: Send + Sync {
    async fn create(&self, config: &PaymentProviderConfig) -> anyhow::Result<()>;
    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<PaymentProviderConfig>>;
    async fn get_by_merchant_and_provider(
        &self,
        merchant_id: Uuid,
        provider_id: Uuid,
        is_test_mode: bool,
    ) -> anyhow::Result<Option<PaymentProviderConfig>>;
    async fn update(&self, config: &PaymentProviderConfig) -> anyhow::Result<()>;
    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
    async fn list_by_merchant(
        &self,
        merchant_id: Uuid,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<(Vec<PaymentProviderConfig>, i64)>;
    async fn list_active_by_merchant(
        &self,
        merchant_id: Uuid,
    ) -> anyhow::Result<Vec<PaymentProviderConfig>>;
}

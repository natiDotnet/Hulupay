use crate::domain::DomainError;
use crate::Transaction;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait TransactionRepository: Send + Sync {
    async fn create(&self, transaction: &Transaction) -> Result<(), DomainError>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Transaction>, DomainError>;
    async fn get_by_nonce(&self, nonce: &str) -> Result<Option<Transaction>, DomainError>;
    async fn update(&self, transaction: &Transaction) -> Result<(), DomainError>;
}

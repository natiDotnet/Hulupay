use crate::domain::user::User;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn exists_by_email(&self, email: &str) -> anyhow::Result<bool>;
    async fn insert(&self, user: &User) -> anyhow::Result<()>;
    async fn find_by_email(&self, email: &str) -> anyhow::Result<Option<User>>;
}

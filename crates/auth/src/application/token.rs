use crate::application::UserContext;
use crate::domain::AuthError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait TokenService: Send + Sync {
    fn generate(
        &self,
        user_id: Uuid,
        email: &str,
        role: &str,
        merchant_id: Uuid,
    ) -> Result<String, AuthError>;
    fn validate(&self, token: &str) -> Result<UserContext, AuthError>;
}

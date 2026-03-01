use crate::auth::claims::Claims;
use crate::auth::error::AuthError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait TokenService: Send + Sync {
    fn generate(&self, user_id: Uuid, email: &str, role: &str, merchant_id: Option<Uuid>) -> Result<String, AuthError>;
    fn validate(&self, token: &str) -> Result<Claims, AuthError>;
}
use crate::application::UserContext;
use crate::domain::AuthError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait TokenService: Send + Sync {
    /// Mint a short-lived access token.
    fn generate_access(
        &self,
        user_id: Uuid,
        email: &str,
        role: &str,
        merchant_id: Uuid,
        permissions: Vec<String>,
    ) -> Result<String, AuthError>;

    /// Mint a long-lived refresh token. `jti` is the caller-chosen id
    /// (typically the refresh_tokens row id) so the persisted row and the
    /// JWT share an identifier.
    fn generate_refresh(
        &self,
        user_id: Uuid,
        email: &str,
        role: &str,
        merchant_id: Uuid,
        jti: Uuid,
    ) -> Result<String, AuthError>;

    /// Validate an access token and return its claims. Refresh tokens are
    /// rejected here.
    fn validate_access(&self, token: &str) -> Result<UserContext, AuthError>;

    /// Validate a refresh token and return its claims. Access tokens are
    /// rejected here.
    fn validate_refresh(&self, token: &str) -> Result<UserContext, AuthError>;
}

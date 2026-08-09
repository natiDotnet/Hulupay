use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// How the user was authenticated — JWT or API key.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AuthenticationType {
    Jwt,
    ApiKey,
}

/// Whether a JWT is a short-lived access token or a long-lived refresh.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum TokenType {
    #[default]
    Access,
    Refresh,
}

/// JWT claims carried in every authenticated request.
///
/// Embedded into the `AuthUser` extractor so handlers can read
/// `user.0.permissions` to make fine-grained authorization decisions.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserContext {
    /// User id (for JWT) or API key id (for API key auth).
    pub sub: Uuid,
    /// User email (JWT) or empty string (API key).
    pub email: String,
    /// Merchant this identity belongs to.
    pub merchant_id: Uuid,
    /// Role name (JWT) or `"apikey"` (API key).
    pub role: String,
    /// Dot-notation permission strings, e.g. `["payment.read", "payment.create"]`.
    pub permissions: Vec<String>,
    /// JWT expiry (unix seconds). Always 0 for API key auth.
    pub exp: usize,
    /// Whether this context came from a JWT or an API key.
    pub auth_type: AuthenticationType,
    /// auth value token or API key
    pub auth_value: Option<String>,
    /// JWT id — unique per token, used for logout revocation.
    #[serde(default)]
    pub jti: Uuid,
    /// Access vs refresh.
    #[serde(default)]
    pub typ: TokenType,
}

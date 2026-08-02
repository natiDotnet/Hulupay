use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Merchant API keys used for `Authorization: Bearer hp_...` auth.
///
/// `scopes` stores dot-notation permission strings (native PostgreSQL
/// `TEXT[]` array column) such as `["payment.create", "payment.read"]`.
///
/// Table name is explicitly set to `"apikeys"` to match the existing schema
/// (Toasty would auto-pluralize `ApiKey` to `api_keys`).
#[derive(Debug, Clone, Serialize, Deserialize, toasty::Model)]
#[table = "apikeys"]
pub struct ApiKey {
    #[key]
    #[auto]
    pub id: Uuid,
    pub merchant_id: Uuid,
    #[unique]
    pub name: String,
    /// First 12 chars of the generated key — used for O(1) lookup.
    pub prefix: String,
    /// Argon2 hash of the full key.
    pub hash: String,
    /// Dot-notation permission strings granted to this key.
    pub scopes: Vec<String>,
    pub is_active: bool,
    pub expires_at: jiff::Timestamp,
    pub last_used_at: Option<jiff::Timestamp>,
    pub created_at: jiff::Timestamp,
    pub updated_at: Option<jiff::Timestamp>,
}

use uuid::Uuid;

/// Email-verification tokens. Created on registration and consumed
/// (single-use) by the verify-email endpoint. The raw token is delivered
/// by email; only its SHA-256 hash is stored here.
#[derive(Debug, Clone, toasty::Model)]
pub struct EmailVerification {
    #[key]
    #[auto]
    pub id: Uuid,
    pub user_id: Uuid,
    /// SHA-256 hex of the raw token.
    pub token_hash: String,
    pub expires_at: jiff::Timestamp,
    /// `Some` once the email has been verified.
    pub verified_at: Option<jiff::Timestamp>,
    pub created_at: jiff::Timestamp,
}

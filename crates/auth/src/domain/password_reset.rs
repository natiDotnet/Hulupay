use uuid::Uuid;

/// Password-reset tokens. Created by the forgot-password flow and consumed
/// (single-use) by reset-password. The raw token is delivered by email;
/// only its SHA-256 hash is stored here.
#[derive(Debug, Clone, toasty::Model)]
pub struct PasswordReset {
    #[key]
    #[auto]
    pub id: Uuid,
    pub user_id: Uuid,
    /// SHA-256 hex of the raw token.
    pub token_hash: String,
    pub expires_at: jiff::Timestamp,
    /// `Some` once the token has been used to reset a password.
    pub used_at: Option<jiff::Timestamp>,
    pub created_at: jiff::Timestamp,
}

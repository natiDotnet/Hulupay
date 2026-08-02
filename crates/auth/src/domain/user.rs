use super::Role;
use crate::domain::status::AccountStatus;
use uuid::Uuid;

/// User account entity.
///
/// Primary key is UUID v7 (time-ordered, auto-generated).
/// Email is unique, generating `User::get_by_email()` / `User::filter_by_email()`.
#[derive(Debug, Clone, toasty::Model)]
pub struct User {
    #[key]
    #[auto]
    pub id: Uuid,
    #[unique]
    pub email: String,
    pub name: String,
    pub password_hash: String,
    pub merchant_id: Uuid,
    pub role: Role,
    pub is_active: bool,
    pub status: AccountStatus,
    /// `Some` once the user has clicked the verification link. `None`
    /// before that. Login is allowed but callers may surface a warning.
    pub email_verified_at: Option<jiff::Timestamp>,
    /// Updated whenever the password changes; can be used to invalidate
    /// older tokens / detect stale sessions.
    pub password_changed_at: Option<jiff::Timestamp>,
    pub created_at: jiff::Timestamp,
    pub updated_at: Option<jiff::Timestamp>,
}

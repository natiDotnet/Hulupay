use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("User not found")]
    UserNotFound,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Token expired")]
    TokenExpired,

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    // ── Email verification ──────────────────────────────────────
    #[error("Email is not verified")]
    EmailNotVerified,

    #[error("Email is already verified")]
    EmailAlreadyVerified,

    #[error("Invalid or unknown verification token")]
    InvalidVerificationToken,

    #[error("Verification token has expired")]
    VerificationTokenExpired,

    // ── Refresh tokens ──────────────────────────────────────────
    #[error("Invalid refresh token")]
    InvalidRefreshToken,

    #[error("Refresh token has been revoked")]
    TokenRevoked,

    // ── Password reset ──────────────────────────────────────────
    #[error("Invalid or unknown reset token")]
    InvalidResetToken,

    #[error("Reset token has expired")]
    ResetTokenExpired,

    #[error("The new password must be different from the current password")]
    SamePassword,
}

impl From<sqlx::Error> for AuthError {
    fn from(err: sqlx::Error) -> Self {
        AuthError::DatabaseError(err.to_string())
    }
}

impl From<sea_orm::DbErr> for AuthError {
    fn from(err: sea_orm::DbErr) -> Self {
        AuthError::DatabaseError(err.to_string())
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Database error")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Invalid state transition")]
    InvalidStateTransition,

    #[error("Provider unavailable")]
    ProviderUnavailable,

    #[error("Provider request failed")]
    RequestFailed,

    #[error("Invalid response from provider")]
    InvalidResponse,
}

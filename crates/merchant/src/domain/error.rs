use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Provider unavailable")]
    ProviderUnavailable,
}

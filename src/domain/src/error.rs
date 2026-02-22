// use thiserror::Error;
#[derive(Debug)]
pub enum DomainError {
    ProviderUnavailable,
    InvalidStateTransition
}
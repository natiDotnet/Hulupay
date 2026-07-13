use crate::domain::Provider;
use crate::domain::payment_status::TransitionError;
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("merchant {0} is inactive")]
    MerchantInactive(Uuid),

    #[error("provider {provider:?} is not enabled for merchant {merchant_id}")]
    ProviderNotEnabled {
        merchant_id: Uuid,
        provider: Provider,
    },

    #[error("invalid amount: {0}")]
    InvalidAmount(Decimal),

    #[error("refund amount {requested} exceeds original {original}")]
    RefundExceedsOriginal {
        requested: Decimal,
        original: Decimal,
    },

    #[error("state transition error: {0}")]
    Transition(#[from] TransitionError),

    #[error("payment not found")]
    NotFound,

    #[error("merchant not found")]
    MerchantNotFound,
}

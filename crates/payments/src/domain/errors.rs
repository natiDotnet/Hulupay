use crate::domain::payment_status::PaymentStatus;
use crate::domain::Provider;
use rust_decimal::Decimal;
use thiserror::Error;
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

#[derive(Debug, Error)]
pub enum TransitionError {
    #[error("illegal transition: {from:?} -> {to:?}")]
    Illegal {
        from: PaymentStatus,
        to: PaymentStatus,
    },
    #[error("payment is already in terminal state: {0:?}")]
    Terminal(PaymentStatus),
}

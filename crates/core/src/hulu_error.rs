use crate::payment_gateway_error::PaymentGatewayError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HuluError {
    #[error("provider not found")]
    ProviderNotFound,
    #[error("provider responded with an error!")]
    ProviderError {
        message: String,
        status_code: u16,
        errors: Option<serde_json::Value>,
    },
    #[error("internal server error!")]
    InternalServerError,

    #[error("unsupported payment method!")]
    UnsupportedPaymentMethod,

    #[error("unable to parse the response!")]
    ResponseParseError,

    #[error("unable to send the request!")]
    ConnectionError,

    #[error("payment already completed")]
    PaymentAlreadyCompleted,
}

impl From<PaymentGatewayError> for HuluError {
    fn from(value: PaymentGatewayError) -> Self {
        match value {
            PaymentGatewayError::RequestFailed => HuluError::ConnectionError,
            PaymentGatewayError::InvalidResponse => HuluError::ResponseParseError,
            PaymentGatewayError::ProviderNotFound => HuluError::ProviderNotFound,
            PaymentGatewayError::ProviderError {
                message,
                status_code,
                errors,
            } => HuluError::ProviderError {
                message,
                status_code,
                errors,
            },
            PaymentGatewayError::InternalServerError => HuluError::InternalServerError,
            PaymentGatewayError::MerchantNotFound => HuluError::ProviderNotFound,
            PaymentGatewayError::UnsupportedPaymentMethod => HuluError::UnsupportedPaymentMethod,
            PaymentGatewayError::TransactionNotFound => HuluError::ProviderNotFound,
        }
    }
}

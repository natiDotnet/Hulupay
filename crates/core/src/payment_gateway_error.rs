use thiserror::Error;

#[derive(Debug, Error)]
pub enum PaymentGatewayError {
    #[error("merchant not found!")]
    MerchantNotFound,

    #[error("request failed!")]
    RequestFailed,
    #[error("invalid response")]
    InvalidResponse,
    #[error("provider was not found!")]
    ProviderNotFound,
    #[error("transaction was not found!")]
    TransactionNotFound,
    #[error("provider responded with an error!")]
    ProviderError {
        status_code: u16,
        message: String,
        errors: Option<serde_json::Value>,
    },
    #[error("internal server error!")]
    InternalServerError,
    #[error("payment method is not supported!")]
    UnsupportedPaymentMethod,
}

impl PaymentGatewayError {
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            PaymentGatewayError::RequestFailed | PaymentGatewayError::InvalidResponse
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_failed_is_retryable() {
        assert!(PaymentGatewayError::RequestFailed.is_retryable());
    }

    #[test]
    fn invalid_response_is_retryable() {
        assert!(PaymentGatewayError::InvalidResponse.is_retryable());
    }

    #[test]
    fn provider_errors_are_not_retryable() {
        assert!(!PaymentGatewayError::MerchantNotFound.is_retryable());
        assert!(!PaymentGatewayError::ProviderNotFound.is_retryable());
        assert!(!PaymentGatewayError::TransactionNotFound.is_retryable());
        assert!(!PaymentGatewayError::InternalServerError.is_retryable());
        assert!(!PaymentGatewayError::UnsupportedPaymentMethod.is_retryable());
    }

    #[test]
    fn provider_error_payload_is_not_retryable() {
        let err = PaymentGatewayError::ProviderError {
            status_code: 502,
            message: "bad gateway".into(),
            errors: None,
        };
        assert!(!err.is_retryable());
    }
}

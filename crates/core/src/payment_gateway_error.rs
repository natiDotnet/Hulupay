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

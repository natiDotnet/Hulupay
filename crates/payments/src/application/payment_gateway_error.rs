use thiserror::Error;

#[derive(Debug, Error)]
pub enum PaymentGatewayError {
    #[error("request failed!")]
    RequestFailed,
    #[error("invalid response")]
    InvalidResponse,
    #[error("provider was not found!")]
    ProviderNotFound,
}

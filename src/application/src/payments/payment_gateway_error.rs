#[derive(Debug)]
pub enum PaymentGatewayError {
    // #[error("Provider request failed")]
    RequestFailed,

    // #[error("Invalid response from provider")]
    InvalidResponse,
}
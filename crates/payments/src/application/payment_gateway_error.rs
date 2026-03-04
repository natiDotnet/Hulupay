#[derive(Debug)]
pub enum PaymentGatewayError {
    RequestFailed,
    InvalidResponse,
}

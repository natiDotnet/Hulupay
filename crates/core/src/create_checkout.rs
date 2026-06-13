use crate::gateway_response::CheckoutResponse;
use crate::hulu_error::HuluError;
use async_trait::async_trait;
#[async_trait]
pub trait CreateCheckout: Send + Sync + 'static {
    async fn execute(
        &self,
        merchant: &str,
        payload: crate::payment_request::PaymentRequest,
    ) -> Result<CheckoutResponse, HuluError>;
}

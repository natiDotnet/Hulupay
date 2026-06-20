use crate::gateway_response::{CheckoutResponse, VerifyResponse};
use crate::hulu_error::HuluError;
use crate::request_context::RequestContext;
use async_trait::async_trait;

#[async_trait]
pub trait CreateCheckout: Send + Sync + 'static {
    async fn execute(
        &self,
        context: &RequestContext,
        payload: crate::payment_request::PaymentRequest,
    ) -> Result<CheckoutResponse, HuluError>;
}

#[async_trait]
pub trait VerifyPayment: Send + Sync + 'static {
    async fn execute(&self, merchant: &str, reference: &str) -> Result<VerifyResponse, HuluError>;
}
#[async_trait]
pub trait CancelPayment: Send + Sync + 'static {
    async fn execute(&self, merchant: &str, reference: &str) -> Result<(), HuluError>;
}

use crate::claims::UserContext;
use crate::gateway_response::{CheckoutResponse, VerifyResponse};
use crate::hulu_error::HuluError;
use crate::payment_method::GatewayProvider;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait CreateCheckout: Send + Sync + 'static {
    async fn execute(
        &self,
        provider: GatewayProvider,
        context: &UserContext,
        payload: crate::payment_request::PaymentRequest,
    ) -> Result<CheckoutResponse, HuluError>;
}

#[async_trait]
pub trait VerifyPayment: Send + Sync + 'static {
    async fn execute(&self, merchant: Uuid, reference: &str) -> Result<VerifyResponse, HuluError>;
}
#[async_trait]
pub trait CancelPayment: Send + Sync + 'static {
    async fn execute(&self, merchant: Uuid, reference: &str) -> Result<(), HuluError>;
}

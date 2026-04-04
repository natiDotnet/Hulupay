use crate::application::dto::InitializePaymentCommand;
use crate::application::payment_gateway_error::PaymentGatewayError;

#[async_trait::async_trait]
pub trait PaymentGateway: Send + Sync {
    async fn initialize_payment(
        &self,
        cmd: InitializePaymentCommand,
    ) -> Result<PaymentInitResult, PaymentGatewayError>;

    async fn handle_webhook(
        &self,
        webhook: serde_json::Value,
    ) -> Result<PaymentInitResult, PaymentGatewayError>;

    async fn verify_payment(
        &self,
        reference: &str,
    ) -> Result<PaymentVerificationResult, PaymentGatewayError>;
}

pub struct PaymentInitResult {
    pub checkout_url: String,
    pub provider_reference: String,
}

pub struct PaymentVerificationResult {
    pub success: bool,
    pub provider_reference: String,
}

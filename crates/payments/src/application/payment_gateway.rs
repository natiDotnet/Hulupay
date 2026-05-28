use crate::application::dto::InitializePaymentCommand;
use crate::application::payment_gateway_error::PaymentGatewayError;
use crate::domain::payment_status::TxStatus;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait::async_trait]
pub trait PaymentGateway: Send + Sync {
    async fn initialize_payment(
        &self,
        cmd: InitializePaymentCommand,
    ) -> Result<PaymentInitResult, PaymentGatewayError>;

    // async fn handle_webhook(
    //     &self,
    //     webhook: serde_json::Value,
    // ) -> Result<PaymentInitResult, PaymentGatewayError>;

    async fn verify_payment(
        &self,
        reference: &str,
    ) -> Result<PaymentVerificationResult, PaymentGatewayError>;
}

#[async_trait]
pub trait WebhookHandler: Send + Sync {
    async fn handle_webhook(&self, webhook: serde_json::Value) -> Result<(), PaymentGatewayError>;

    async fn success_handler(&self, request: serde_json::Value) -> Result<(), PaymentGatewayError>;
    async fn failure_handler(&self, request: serde_json::Value) -> Result<(), PaymentGatewayError>;

    async fn change_status(
        &self,
        request: serde_json::Value,
        status: TxStatus,
    ) -> Result<(), PaymentGatewayError>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentInitResult {
    pub checkout_url: String,
    pub provider_reference: String,
    pub row_response: String,
}

#[derive(Serialize, Deserialize)]
pub struct PaymentVerificationResult {
    pub success: bool,
    pub provider_reference: String,
    pub row_response: String,
}

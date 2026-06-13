use crate::domain::payment_status::TxStatus;
use async_trait::async_trait;
use hulu_core::payment_gateway_error::PaymentGatewayError;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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

#[derive(Serialize, Deserialize, Debug, ToSchema, PartialEq, Eq)]
pub enum ApiStatus {
    Success,
    Failure,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentChargeResult {
    pub provider_reference: String,
    pub row_response: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct PaymentVerificationResult {
    pub success: bool,
    pub provider_reference: String,
    pub row_response: serde_json::Value,
}

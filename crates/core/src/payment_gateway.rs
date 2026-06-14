use crate::gateway_response::VerifyResponse;
use crate::payment_gateway_error::PaymentGatewayError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait PaymentGateway: Send + Sync {
    fn get_name(&self) -> &'static str;
    fn get_apikey_name(&self) -> &'static str;
    async fn checkout(
        &self,
        request: &crate::payment_request::PaymentRequest,
        config: serde_json::Value,
    ) -> Result<GatewayResponse, PaymentGatewayError>;

    // async fn charge(
    //     &self,
    //     request: DirectPaymentRequest,
    // ) -> Result<PaymentChargeResult, PaymentGatewayError>;

    // async fn handle_webhook(
    //     &self,
    //     webhook: serde_json::Value,
    // ) -> Result<PaymentInitResult, PaymentGatewayError>;

    async fn verify(
        &self,
        reference: &str,
        config: serde_json::Value,
    ) -> Result<VerifyResponse, PaymentGatewayError>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayResponse {
    pub checkout_url: String,
    pub reference: String,
    pub row_response: Option<serde_json::Value>,
}

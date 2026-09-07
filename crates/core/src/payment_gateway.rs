use crate::claims::UserContext;
use crate::gateway_response::VerifyResponse;
use crate::payment_gateway_error::PaymentGatewayError;
use crate::payment_method::{GatewayProvider, PaymentMethod};
use async_trait::async_trait;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[async_trait]
pub trait PaymentGateway: Send + Sync {
    fn get_config(&self) -> serde_json::Value;
    fn change_callback_urls(
        &self,
        _host: &str,
        request: &crate::payment_request::PaymentRequest,
    ) -> crate::payment_request::PaymentRequest {
        let mut request = request.clone();

        // let mut notify = Url::parse(&request.callbacks.notify_url)?;
        // let mut cancel = Url::parse(&request.callbacks.cancel_url)?;
        // let mut success = Url::parse(&request.callbacks.success_url)?;
        // let mut error = Url::parse(&request.callbacks.error_url)?;

        // notify.set_host(Some(host))?;
        // cancel.set_host(Some(host))?;
        // success.set_host(Some(host))?;
        // error.set_host(Some(host))?;

        request.callbacks.notify_url =
            "https://69f8a62bf7044aa0103e3ba6.mockapi.io/api/callback".into();
        // request.callbacks.cancel_url = cancel.into();
        // request.callbacks.success_url = success.into();
        // request.callbacks.error_url = error.into();

        request
    }
    fn get_name(&self) -> GatewayProvider;
    fn get_apikey_name(&self) -> &'static str;
    async fn checkout(
        &self,
        context: &UserContext,
        request: &crate::payment_request::PaymentRequest,
        apikey_header: &str,
        config: serde_json::Value,
    ) -> Result<GatewayResponse, PaymentGatewayError>;

    // async fn charge(
    //     &self,
    //     request: DirectPaymentRequest,
    // ) -> Result<PaymentChargeResult, PaymentGatewayError>;

    async fn webhook(&self, webhook: &WebhookInfo) -> Result<(), PaymentGatewayError>;
    fn webhook_info(&self, webhook: serde_json::Value) -> Result<WebhookInfo, PaymentGatewayError>;

    async fn verify(
        &self,
        reference: &str,
        config: serde_json::Value,
    ) -> Result<VerifyResponse, PaymentGatewayError>;
    async fn cancel(
        &self,
        reference: &str,
        config: serde_json::Value,
    ) -> Result<(), PaymentGatewayError>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayResponse {
    pub checkout_url: String,
    pub reference: String,
    pub row_response: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Display, Serialize, Deserialize)]
#[strum(serialize_all = "UPPERCASE")]
pub enum PaymentStatus {
    Success,
    Failed,
    Pending,
    Cancelled,
    Refunding,
    Refunded,
    Reversed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebhookInfo {
    pub status: PaymentStatus,
    pub provider_reference: String,
    pub payment_method: PaymentMethod,
    pub amount: Decimal,
    pub charge: Decimal,
    pub client_reference: String,
    pub txn_reference: String,
    pub received_at: jiff::Timestamp,
}

#[derive(Display)]
#[strum(serialize_all = "UPPERCASE")]
pub enum WebhookStatus {
    Pending,
    Processing,
    Forwarded,
    Failed,
}

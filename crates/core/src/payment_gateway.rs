use crate::gateway_response::VerifyResponse;
use crate::payment_gateway_error::PaymentGatewayError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use url::{ParseError, Url};

#[async_trait]
pub trait PaymentGateway: Send + Sync {
    fn change_callback_urls(
        &self,
        host: &str,
        request: &crate::payment_request::PaymentRequest,
    ) -> Result<crate::payment_request::PaymentRequest, ParseError> {
        let mut request = request.clone();

        let mut notify = Url::parse(&request.callbacks.notify_url)?;
        let mut cancel = Url::parse(&request.callbacks.cancel_url)?;
        let mut success = Url::parse(&request.callbacks.success_url)?;
        let mut error = Url::parse(&request.callbacks.error_url)?;

        notify.set_host(Some(host))?;
        cancel.set_host(Some(host))?;
        success.set_host(Some(host))?;
        error.set_host(Some(host))?;

        request.callbacks.notify_url = notify.into();
        request.callbacks.cancel_url = cancel.into();
        request.callbacks.success_url = success.into();
        request.callbacks.error_url = error.into();

        Ok(request)
    }
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

    async fn webhook(&self, webhook: serde_json::Value) -> Result<(), PaymentGatewayError>;

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

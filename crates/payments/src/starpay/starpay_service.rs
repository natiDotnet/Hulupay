use std::collections::HashMap;
use serde_json::json;
use tracing::debug;
use hulu_core::payment_gateway::WebhookInfo;
use hulu_core::payment_gateway_error::PaymentGatewayError;
use crate::arifpay::arif_webhook::ArifWebhook;
use crate::starpay::checkout_request::StarCheckoutRequest;
use crate::starpay::checkout_response::StarCheckoutResponse;
use crate::starpay::start_webhook::StarWebhook;

#[derive(Clone)]
pub struct StarPayService {
    client: reqwest::Client,
    _urls: HashMap<String, String>,
}

impl StarPayService {
    fn get_apikey_name(&self) -> &'static str {
        "X-API-Key"
    }
    pub async fn create_session(
        &self,
        base_url: &str,
        apikey: &str,
        request: &hulu_core::payment_request::PaymentRequest,
    ) -> Result<StarCheckoutResponse, PaymentGatewayError> {

        let request: StarCheckoutRequest = request.into();
        debug!(?base_url, ?apikey, ?request, "the request");
        let response = self
            .client
            .post(format!("{}/api/v2/payment/checkout", base_url))
            .header(self.get_apikey_name(), apikey)
            .json(&request)
            .send()
            .await
            .map_err(|_| PaymentGatewayError::RequestFailed)?;
        let status = response.status();
        debug!(?status, "the response status");

        let body: StarCheckoutResponse = response.json().await.map_err(|e| {
            debug!(?e, "response body parse error");
            PaymentGatewayError::InvalidResponse
        })?;
        debug!(?body, "the response body");

        if !status.is_success() || body.status == "error" {
            return Err(PaymentGatewayError::ProviderError {
                message: body.error.clone().unwrap().message,
                errors: Some(json!(body.error)),
                status_code: status.as_u16(),
            });
        }

        Ok(body)
    }

    pub fn map_webhook(
        &self,
        webhook: serde_json::Value,
    ) -> Result<StarWebhook, PaymentGatewayError> {
        let webhook: StarWebhook = serde_json::from_value(webhook).map_err(|e| {
            debug!(?e, "webhook parse error");
            PaymentGatewayError::RequestFailed
        })?;
        Ok(webhook)
    }

    pub async fn send_webhook(
        &self,
        url: &str,
        webhook: ArifWebhook,
    ) -> Result<(), PaymentGatewayError> {
        let response = self
            .client
            .post(url)
            .json(&webhook)
            .send()
            .await
            .map_err(|_| PaymentGatewayError::RequestFailed)?;
        let status = response.status();
        if !status.is_success() {
            return Err(PaymentGatewayError::RequestFailed);
        }
        Ok(())
    }

}
use std::collections::HashMap;
use tracing::debug;
use hulu_core::payment_gateway_error::PaymentGatewayError;
use crate::lakipay::checkout_request::LakiCheckoutRequest;
use crate::lakipay::checkout_response::LakiCheckoutResponse;

#[derive(Clone)]
pub struct LakiPayService {
    client: reqwest::Client,
    _urls: HashMap<String, String>,
}

impl LakiPayService {
    fn get_apikey_name(&self) -> &'static str {
        "X-API-Key"
    }
    pub async fn create_session(
        &self,
        base_url: &str,
        apikey: &str,
        request: &hulu_core::payment_request::PaymentRequest,
    ) -> Result<LakiCheckoutResponse, PaymentGatewayError> {

        let request: LakiCheckoutRequest = request.into();
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

        let body: LakiCheckoutResponse = response.json().await.map_err(|e| {
            debug!(?e, "response body parse error");
            PaymentGatewayError::InvalidResponse
        })?;
        debug!(?body, "the response body");

        if !status.is_success() || !body.success {
            return Err(PaymentGatewayError::ProviderError {
                message: body.message,
                errors: None,
                status_code: status.as_u16(),
            });
        }

        Ok(body)
    }

}
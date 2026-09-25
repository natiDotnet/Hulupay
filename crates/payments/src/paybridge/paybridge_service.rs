//! HTTP client for the PayBridge microservice's merchant API.
//!
//! All calls are authenticated with the merchant's `hp_…` platform key,
//! forwarded in `Authorization: Bearer …` exactly as the merchant sent it to
//! the platform — PayBridge resolves the key back to the merchant via the
//! platform's key-introspection endpoint.

use crate::paybridge::checkout_request::PayBridgeCreateCheckout;
use crate::paybridge::checkout_response::PayBridgeCheckoutDto;
use hulu_core::payment_gateway_error::PaymentGatewayError;

pub struct PayBridgeService {
    client: reqwest::Client,
}

impl PayBridgeService {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }
}

impl PayBridgeService {
    pub async fn create_checkout(
        &self,
        base_url: &str,
        api_key: &str,
        request: &PayBridgeCreateCheckout,
    ) -> Result<PayBridgeCheckoutDto, PaymentGatewayError> {
        let response = self
            .client
            .post(format!("{base_url}/api/v1/checkouts"))
            .bearer_auth(api_key)
            // Same merchant reference replays the original checkout instead of
            // creating a duplicate.
            .header("idempotency-key", &request.reference)
            .json(request)
            .send()
            .await
            .map_err(|_| PaymentGatewayError::RequestFailed)?;

        self.parse(response).await
    }

    pub async fn get_checkout(
        &self,
        base_url: &str,
        api_key: &str,
        checkout_id: &str,
    ) -> Result<PayBridgeCheckoutDto, PaymentGatewayError> {
        let response = self
            .client
            .get(format!("{base_url}/api/v1/checkouts/{checkout_id}"))
            .bearer_auth(api_key)
            .send()
            .await
            .map_err(|_| PaymentGatewayError::RequestFailed)?;

        self.parse(response).await
    }

    /// Map a PayBridge error response onto the platform's gateway error.
    async fn parse(
        &self,
        response: reqwest::Response,
    ) -> Result<PayBridgeCheckoutDto, PaymentGatewayError> {
        let status = response.status();
        let body: serde_json::Value = response
            .json()
            .await
            .map_err(|_| PaymentGatewayError::InvalidResponse)?;

        if !status.is_success() {
            let message = body
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("paybridge rejected the request")
                .to_string();
            return Err(PaymentGatewayError::ProviderError {
                status_code: status.as_u16(),
                message,
                errors: body.get("errors").cloned(),
            });
        }

        serde_json::from_value(body).map_err(|_| PaymentGatewayError::InvalidResponse)
    }

    /// Deliver a webhook event to a merchant callback URL (the platform acts
    /// as the webhook fan-out for merchants that subscribed via PayBridge).
    pub async fn forward_webhook(
        &self,
        url: &str,
        event: &crate::paybridge::webhook::PayBridgeWebhookEvent,
    ) -> Result<(), PaymentGatewayError> {
        let response = self
            .client
            .post(url)
            .json(event)
            .send()
            .await
            .map_err(|_| PaymentGatewayError::RequestFailed)?;
        if !response.status().is_success() {
            return Err(PaymentGatewayError::RequestFailed);
        }
        Ok(())
    }
}

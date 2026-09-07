use std::str::FromStr;
use hulu_core::claims::UserContext;
use hulu_core::gateway_response::VerifyResponse;
use hulu_core::payment_gateway::{GatewayResponse, PaymentGateway, PaymentStatus, WebhookInfo};
use hulu_core::payment_gateway_error::PaymentGatewayError;
use hulu_core::payment_method::{GatewayProvider, PaymentMethod};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Determines how the simulator responds to payment operations.
///
/// Developers set this to exercise different edge-case paths
/// (happy path, timeouts, failures, malformed webhooks) without
/// calling any real provider APIs.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimulationMode {
    /// Checkout, verify and webhook all succeed.
    #[default]
    Success,

    /// Every call immediately returns `PaymentGatewayError::RequestFailed`,
    /// simulating a network timeout or unreachable provider.
    Timeout,

    /// Checkout succeeds but verify and webhook return a `Failed` status,
    /// simulating insufficient funds on the customer's account.
    InsufficientFunds,

    /// Checkout succeeds but `webhook_info` returns a parse error,
    /// simulating a provider that sends malformed / unexpected callback
    /// payloads.
    InvalidCallback,

    /// Checkout and verify succeed but `webhook` returns an error,
    /// simulating a provider that sends the same notification twice.
    DuplicateWebhook,
}

/// A purely in-memory `PaymentGateway` implementation that simulates
/// provider behaviour based on the chosen `SimulationMode`.
///
/// No HTTP calls, no database queries — everything is deterministic
/// and runs locally, making it ideal for development, integration
/// tests and CI pipelines.
#[derive(Debug, Clone)]
pub struct SimulationProvider {
    mode: SimulationMode,
}

impl SimulationProvider {
    pub fn new(mode: SimulationMode) -> Self {
        Self { mode }
    }
}

#[async_trait::async_trait]
impl PaymentGateway for SimulationProvider {
    fn get_config(&self) -> Value {
        Value::Object(serde_json::Map::new())
    }

    fn get_name(&self) -> GatewayProvider {
        GatewayProvider::Simulator
    }

    fn get_apikey_name(&self) -> &'static str {
        "x-simulation-key"
    }

    async fn checkout(
        &self,
        _context: &UserContext,
        request: &hulu_core::payment_request::PaymentRequest,
        _apikey_header: &str,
        _config: Value,
    ) -> Result<GatewayResponse, PaymentGatewayError> {
        // schedule call the request.callbacks.notify_url after two minutes with WebhookInfo body
        let webhook = WebhookInfo {
            status: PaymentStatus::Success,
            provider_reference: request.payment.reference.clone(),
            payment_method: PaymentMethod::from_str(request.payment.payment_methods.first().unwrap().as_str()).unwrap_or(
                PaymentMethod::Telebirr),
            amount: request.payment.amount,
            charge: request.payment.amount,
            client_reference: request.payment.reference.clone(),
            txn_reference: request.payment.reference.clone(),
            received_at: jiff::Timestamp::now(),
        };
        let url = request.callbacks.notify_url.clone();
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(120)).await;
            let client = reqwest::Client::new();
            let _ = client
                .post(&url)
                .json(&webhook)
                .send()
                .await;
        });
        match self.mode {
            SimulationMode::Timeout => Err(PaymentGatewayError::RequestFailed),

            SimulationMode::Success
            | SimulationMode::InsufficientFunds
            | SimulationMode::InvalidCallback
            | SimulationMode::DuplicateWebhook => {
                let reference = request.payment.reference.clone();
                let ref_clone = reference.clone();
                Ok(GatewayResponse {
                    checkout_url: format!("https://simulation.example.com/checkout/{reference}"),
                    reference,
                    row_response: Some(serde_json::json!({
                        "simulation_mode": self.mode,
                        "reference": ref_clone,
                    })),
                })
            }
        }
    }

    async fn webhook(&self, _webhook: &WebhookInfo) -> Result<(), PaymentGatewayError> {
        match self.mode {
            SimulationMode::Timeout => Err(PaymentGatewayError::RequestFailed),
            SimulationMode::DuplicateWebhook => Err(PaymentGatewayError::ProviderError {
                status_code: 409,
                message: "duplicate webhook received".into(),
                errors: None,
            }),
            SimulationMode::Success
            | SimulationMode::InsufficientFunds
            | SimulationMode::InvalidCallback => Ok(()),
        }
    }

    fn webhook_info(&self, webhook: Value) -> Result<WebhookInfo, PaymentGatewayError> {
        let webhook_info = serde_json::from_value::<WebhookInfo>(webhook.clone())
            .map_err(|_| PaymentGatewayError::InvalidResponse)?;
        match self.mode {
            SimulationMode::Timeout => Err(PaymentGatewayError::RequestFailed),

            SimulationMode::InvalidCallback => {
                // Return a parse error to simulate a malformed callback payload.
                Err(PaymentGatewayError::InvalidResponse)
            }

            SimulationMode::Success | SimulationMode::DuplicateWebhook => {
                let status = PaymentStatus::Success;
                Ok(webhook_info)
                // Ok(WebhookInfo {
                //     status,
                //     provider_reference: webhook
                //         .get("provider_reference")
                //         .and_then(|v| v.as_str())
                //         .unwrap_or("sim_ref_success")
                //         .to_string(),
                //     payment_method: PaymentMethod::Telebirr,
                //     amount: webhook
                //         .get("amount")
                //         .and_then(|v| {
                //             Decimal::from_str_exact(&v.to_string().trim_matches('"')).ok()
                //         })
                //         .unwrap_or(Decimal::ZERO),
                //     charge: webhook
                //         .get("charge")
                //         .and_then(|v| {
                //             Decimal::from_str_exact(&v.to_string().trim_matches('"')).ok()
                //         })
                //         .unwrap_or(Decimal::ZERO),
                //     client_reference: webhook
                //         .get("client_reference")
                //         .and_then(|v| v.as_str())
                //         .unwrap_or("sim_client_ref")
                //         .to_string(),
                //     txn_reference: webhook
                //         .get("txn_reference")
                //         .and_then(|v| v.as_str())
                //         .unwrap_or("sim_txn_ref")
                //         .to_string(),
                //     received_at: jiff::Timestamp::now(),
                // })
            }

            SimulationMode::InsufficientFunds => {
                let status = PaymentStatus::Failed;
                Ok(WebhookInfo {
                    status,
                    provider_reference: webhook
                        .get("provider_reference")
                        .and_then(|v| v.as_str())
                        .unwrap_or("sim_ref_failed")
                        .to_string(),
                    payment_method: PaymentMethod::Telebirr,
                    amount: webhook
                        .get("amount")
                        .and_then(|v| {
                            Decimal::from_str_exact(&v.to_string().trim_matches('"')).ok()
                        })
                        .unwrap_or(Decimal::ZERO),
                    charge: webhook
                        .get("charge")
                        .and_then(|v| {
                            Decimal::from_str_exact(&v.to_string().trim_matches('"')).ok()
                        })
                        .unwrap_or(Decimal::ZERO),
                    client_reference: webhook
                        .get("client_reference")
                        .and_then(|v| v.as_str())
                        .unwrap_or("sim_client_ref")
                        .to_string(),
                    txn_reference: webhook
                        .get("txn_reference")
                        .and_then(|v| v.as_str())
                        .unwrap_or("sim_txn_ref")
                        .to_string(),
                    received_at: jiff::Timestamp::now(),
                })
            }
        }
    }

    async fn verify(
        &self,
        reference: &str,
        _config: Value,
    ) -> Result<VerifyResponse, PaymentGatewayError> {
        match self.mode {
            SimulationMode::Timeout => Err(PaymentGatewayError::RequestFailed),

            SimulationMode::InsufficientFunds => {
                let now = jiff::Timestamp::now();
                Ok(VerifyResponse {
                    id: Some(reference.to_string()),
                    reference: reference.to_string(),
                    status: PaymentStatus::Failed.to_string(),
                    payment_method: PaymentMethod::Telebirr,
                    charge: Decimal::ZERO,
                    amount: Decimal::ZERO,
                    created_at: now,
                    updated_at: now,
                })
            }

            SimulationMode::Success
            | SimulationMode::InvalidCallback
            | SimulationMode::DuplicateWebhook => {
                let now = jiff::Timestamp::now();
                Ok(VerifyResponse {
                    id: Some(reference.to_string()),
                    reference: reference.to_string(),
                    status: PaymentStatus::Success.to_string(),
                    payment_method: PaymentMethod::Telebirr,
                    charge: Decimal::ZERO,
                    amount: Decimal::ZERO,
                    created_at: now,
                    updated_at: now,
                })
            }
        }
    }

    async fn cancel(&self, _reference: &str, _config: Value) -> Result<(), PaymentGatewayError> {
        match self.mode {
            SimulationMode::Timeout => Err(PaymentGatewayError::RequestFailed),
            _ => Ok(()),
        }
    }
}

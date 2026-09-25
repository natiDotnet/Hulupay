//! PayBridge gateway — forwards checkout requests to the PayBridge microservice
//! using the merchant's `hp_…` platform key (PayBridge resolves it via the
//! platform's key-introspection endpoint). Webhook signature verification is
//! handled at the platform's webhook endpoint before routing here.

use crate::domain::merchant_config::MerchantConfig;
use crate::domain::payment_order::PaymentOrder;
use crate::domain::payments::payment_callback::PaymentCallback;
use crate::paybridge::{
    PayBridgeCreateCheckout, PayBridgeService, PayBridgeWebhookEvent,
};
use hulu_core::claims::UserContext;
use hulu_core::gateway_response::VerifyResponse;
use hulu_core::payment_gateway::{GatewayResponse, PaymentGateway, WebhookInfo};
use hulu_core::payment_gateway_error::PaymentGatewayError;
use hulu_core::payment_method::{GatewayProvider, PaymentMethod};
use hulu_core::request_context::RequestContext;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use toasty::Db;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayBridgeConfig {
    /// PayBridge service base URL, e.g. `http://localhost:4000`.
    pub base_url: String,
    /// The merchant's `hp_…` platform key. PayBridge resolves this itself via
    /// the platform's key-introspection endpoint — no local key storage needed.
    pub api_key: String,
    /// Webhook secret returned when the platform registered the webhook
    /// endpoint via PayBridge's `POST /internal/merchants/{id}/webhook`.
    /// Used to verify incoming `X-PayBridge-Signature` headers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook_secret: Option<String>,
}

#[derive(Clone)]
pub struct PayBridgeProvider {
    service: Arc<PayBridgeService>,
    db: Db,
}

impl PayBridgeProvider {
    pub fn new(service: Arc<PayBridgeService>, db: Db) -> Self {
        Self { service, db }
    }
}

#[async_trait::async_trait]
impl PaymentGateway for PayBridgeProvider {
    fn get_config(&self) -> Value {
        serde_json::json!(PayBridgeConfig {
            base_url: String::new(),
            api_key: String::new(),
            webhook_secret: None,
        })
    }

    fn get_name(&self) -> GatewayProvider {
        GatewayProvider::PayBridge
    }

    fn get_apikey_name(&self) -> &'static str {
        "authorization"
    }

    async fn checkout(
        &self,
        _context: &UserContext,
        request: &hulu_core::payment_request::PaymentRequest,
        _apikey_header: &str,
        config: Value,
    ) -> Result<GatewayResponse, PaymentGatewayError> {
        let config: PayBridgeConfig =
            serde_json::from_value(config).map_err(|_| PaymentGatewayError::ProviderNotFound)?;

        let body: PayBridgeCreateCheckout =
            PayBridgeCreateCheckout::try_from(request).map_err(|_| PaymentGatewayError::InvalidResponse)?;

        let dto = self
            .service
            .create_checkout(&config.base_url, &config.api_key, &body)
            .await?;

        Ok(GatewayResponse {
            checkout_url: dto.payment_url.clone(),
            reference: dto.checkout_id.clone(),
            row_response: serde_json::to_value(&dto).ok(),
        })
    }

    async fn webhook(&self, request: &WebhookInfo) -> Result<(), PaymentGatewayError> {
        // Forward the platform's internal webhook to the merchant's callback URL.
        let mut db = self.db.clone();

        let order = PaymentOrder::filter(
            PaymentOrder::fields().order_ref().eq(request.client_reference.clone()),
        )
        .first()
        .exec(&mut db)
        .await
        .map_err(|_| PaymentGatewayError::ProviderNotFound)?
        .ok_or(PaymentGatewayError::ProviderNotFound)?;

        let callback = PaymentCallback::filter(
            PaymentCallback::fields().payment_order_id().eq(order.id),
        )
        .first()
        .exec(&mut db)
        .await
        .map_err(|_| PaymentGatewayError::ProviderNotFound)?
        .ok_or(PaymentGatewayError::ProviderNotFound)?;

        let event = PayBridgeWebhookEvent {
            event_id: request.provider_reference.clone(),
            event: "checkout.payment_succeeded".to_string(),
            checkout_id: request.client_reference.clone(),
            reference: request.client_reference.clone(),
            amount: format!("{:.2}", request.amount),
            amount_minor: (request.amount * Decimal::from(100))
                .round()
                .trunc()
                .to_string()
                .parse::<i64>()
                .unwrap_or(0),
            currency: order.currency.clone(),
            payment_method: request.payment_method.to_string(),
            transaction_reference: request.txn_reference.clone(),
            status: "succeeded".to_string(),
            occurred_at: chrono::Utc::now().to_rfc3339(),
        };

        self.service.forward_webhook(&callback.notify_url, &event).await
    }

    fn webhook_info(&self, json: Value) -> Result<WebhookInfo, PaymentGatewayError> {
        let event: PayBridgeWebhookEvent =
            serde_json::from_value(json).map_err(|_| PaymentGatewayError::InvalidResponse)?;

        let status = match event.status.as_str() {
            "succeeded" => hulu_core::payment_gateway::PaymentStatus::Success,
            "failed" => hulu_core::payment_gateway::PaymentStatus::Failed,
            _ => hulu_core::payment_gateway::PaymentStatus::Pending,
        };

        let payment_method = match event.payment_method.as_str() {
            "telebirr" => PaymentMethod::Telebirr,
            "cbebirr" => PaymentMethod::CbeBirr,
            "mpesa" => PaymentMethod::Mpesa,
            "awash" => PaymentMethod::Awash,
            other => PaymentMethod::Unknown(other.to_string()),
        };

        Ok(WebhookInfo {
            status,
            provider_reference: event.event_id.clone(),
            payment_method,
            amount: event.amount(),
            charge: Decimal::ZERO,
            client_reference: event.reference,
            txn_reference: event.transaction_reference,
            received_at: jiff::Timestamp::now(),
        })
    }

    async fn verify(
        &self,
        reference: &str,
        config: Value,
    ) -> Result<VerifyResponse, PaymentGatewayError> {
        let config: PayBridgeConfig =
            serde_json::from_value(config).map_err(|_| PaymentGatewayError::ProviderNotFound)?;

        let dto = self
            .service
            .get_checkout(&config.base_url, &config.api_key, reference)
            .await?;

        let status = match dto.status.as_str() {
            "succeeded" => "SUCCESS".to_string(),
            "pending" => "PENDING".to_string(),
            "expired" => "EXPIRED".to_string(),
            _ => "FAILED".to_string(),
        };

        Ok(VerifyResponse {
            id: Some(dto.checkout_id),
            reference: dto.reference,
            status,
            amount: Decimal::new(dto.amount_minor, 2),
            payment_method: PaymentMethod::None,
            charge: Decimal::ZERO,
            created_at: jiff::Timestamp::now(),
            updated_at: jiff::Timestamp::now(),
        })
    }

    async fn cancel(&self, _reference: &str, _config: Value) -> Result<(), PaymentGatewayError> {
        // PayBridge checkouts are immutable once the customer begins payment;
        // cancellation means waiting for expiry (handled by PayBridge).
        Ok(())
    }
}
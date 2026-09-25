//! PayBridge webhook event (`checkout.payment_succeeded`).
//!
//! Delivered by PayBridge's outbox dispatcher with
//! `X-PayBridge-Signature: sha256=<hex(HMAC_SHA256(endpoint_secret, raw_body))>`,
//! `X-PayBridge-Event-Id` and `X-PayBridge-Timestamp` headers.

use hulu_core::payment_gateway::PaymentStatus;
use hulu_core::payment_method::PaymentMethod;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayBridgeWebhookEvent {
    /// `evt_…` — unique per event; used for delivery dedupe.
    pub event_id: String,
    /// `checkout.payment_succeeded` (only success events are emitted today).
    pub event: String,
    pub checkout_id: String,
    /// The merchant-side order reference passed at creation.
    pub reference: String,
    /// Decimal string, e.g. "500.00".
    pub amount: String,
    pub amount_minor: i64,
    pub currency: String,
    /// `telebirr`, `cbebirr`, `mpesa`, or `awash`.
    pub payment_method: String,
    /// The wallet transaction reference the customer entered.
    pub transaction_reference: String,
    /// `succeeded` | `failed`.
    pub status: String,
    /// ISO-8601.
    pub occurred_at: String,
}

impl PayBridgeWebhookEvent {
    /// Map PayBridge's checkout status onto the platform's payment status.
    pub fn status(&self) -> hulu_core::payment_gateway::PaymentStatus {
        match self.status.as_str() {
            "succeeded" => hulu_core::payment_gateway::PaymentStatus::Success,
            "failed" => hulu_core::payment_gateway::PaymentStatus::Failed,
            "expired" | "cancelled" => hulu_core::payment_gateway::PaymentStatus::Cancelled,
            _ => hulu_core::payment_gateway::PaymentStatus::Pending,
        }
    }

    pub fn payment_method(&self) -> PaymentMethod {
        match self.payment_method.as_str() {
            "telebirr" => PaymentMethod::Telebirr,
            "cbebirr" => PaymentMethod::CbeBirr,
            "mpesa" => PaymentMethod::Mpesa,
            "awash" => PaymentMethod::Awash,
            other => PaymentMethod::Unknown(other.to_string()),
        }
    }

    /// Amount in major units derived from the exact minor-unit integer.
    pub fn amount(&self) -> rust_decimal::Decimal {
        rust_decimal::Decimal::new(self.amount_minor, 2)
    }
}

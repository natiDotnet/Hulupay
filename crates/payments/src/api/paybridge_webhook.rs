//! PayBridge webhook receiver.
//!
//! PayBridge delivers `checkout.payment_succeeded` events to
//! `POST /api/payments/webhook/paybridge/{merchant_id}` (the URL registered via
//! its `/internal/merchants/{id}/webhook` provisioning endpoint), signed with
//! `X-PayBridge-Signature: sha256=hex(HMAC_SHA256(endpoint_secret, raw_body))`.
//!
//! The merchant's endpoint secret lives in their PayBridge provider config
//! (`webhook_secret`), so this endpoint resolves it by the merchant id in the
//! path, verifies the signature over the raw body *before* parsing, enforces a
//! ±5 minute freshness window, and dedupes on the `eventId` before handing the
//! event to the shared payment-webhook pipeline.

use axum::body::Bytes;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use toasty::Db;
use uuid::Uuid;

use crate::application::checkout::payment_webhook::PaymentWebhookHandler;
use crate::application::gateways::paybridge::PayBridgeConfig;
use crate::api::request_context::RequestCtx;
use crate::domain::merchant_config::MerchantConfig;
use crate::domain::payment_provider::PaymentProvider;
use crate::domain::provider::Provider;

/// Signature freshness window (PayBridge stamps `X-PayBridge-Timestamp` in
/// unix seconds).
const MAX_CLOCK_SKEW_SECS: i64 = 300;

/// Process-wide set of already-processed PayBridge event ids. PayBridge
/// retries on transient failures and at-least-once delivery can duplicate the
/// immediate attempt, so each `evt_…` id is remembered for a day.
fn first_time_event(event_id: &str) -> bool {
    static SEEN: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<String, std::time::Instant>>> =
        std::sync::OnceLock::new();
    let mut seen = SEEN
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let now = std::time::Instant::now();
    seen.retain(|_, at| at.elapsed() < std::time::Duration::from_secs(24 * 3600));
    seen.insert(event_id.to_string(), std::time::Instant::now())
        .is_none()
}

/// Constant-time byte equality.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter()
        .zip(b.iter())
        .fold(0u8, |acc, (x, y)| acc | (x ^ y))
        == 0
}

#[utoipa::path(
    post,
    tag = "payments",
    path = "/payments/webhook/paybridge/{merchant_id}",
    request_body = serde_json::Value,
    responses((status = OK, body = ()))
)]
pub async fn paybridge_webhook_handler(
    State(db): State<Db>,
    State(handler): State<PaymentWebhookHandler>,
    ctx: RequestCtx,
    Path(merchant_id): Path<Uuid>,
    headers: HeaderMap,
    body: Bytes,
) -> StatusCode {
    // The webhook secret is per merchant, from their PayBridge provider config.
    let secret = match load_webhook_secret(db.clone(), merchant_id).await {
        Ok(secret) => secret,
        Err(status) => {
            tracing::warn!(%merchant_id, status = status.as_u16(), "paybridge webhook: no usable provider config");
            return status;
        }
    };

    if !verify_signature(&secret, &headers, &body) {
        tracing::warn!(%merchant_id, "paybridge webhook: signature verification failed");
        return StatusCode::UNAUTHORIZED;
    }

    let payload: serde_json::Value = match serde_json::from_slice(&body) {
        Ok(v) => v,
        Err(_) => return StatusCode::BAD_REQUEST,
    };

    let event_id = payload
        .get("eventId")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    if event_id.is_empty() {
        return StatusCode::BAD_REQUEST;
    }
    if !first_time_event(&event_id) {
        tracing::info!(%merchant_id, %event_id, "paybridge webhook: duplicate event ignored");
        return StatusCode::OK;
    }

    match handler
        .execute(Provider::PayBridge, &ctx.0, payload)
        .await
    {
        Ok(()) => StatusCode::OK,
        Err(e) => {
            tracing::warn!(%merchant_id, %event_id, ?e, "paybridge webhook: processing failed");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

/// Resolve the merchant's active PayBridge provider config and extract the
/// webhook secret. 404 when the merchant has no usable PayBridge config.
async fn load_webhook_secret(
    db: Db,
    merchant_id: Uuid,
) -> Result<String, StatusCode> {
    let mut db = db;
    // Find the PayBridge provider by code.
    let provider = PaymentProvider::filter(
        PaymentProvider::fields().code().eq(Provider::PayBridge.as_str()),
    )
    .first()
    .exec(&mut db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Find the merchant's active config for this provider.
    let row = MerchantConfig::filter(
        MerchantConfig::fields()
            .merchant_id()
            .eq(merchant_id)
            .and(MerchantConfig::fields().provider_id().eq(provider.id))
            .and(MerchantConfig::fields().is_active().eq(true)),
    )
    .first()
    .exec(&mut db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    let config: PayBridgeConfig =
        serde_json::from_value(row.config).map_err(|_| StatusCode::NOT_FOUND)?;
    config.webhook_secret.ok_or(StatusCode::NOT_FOUND)
}

/// `X-PayBridge-Signature: sha256=<hex>` over the raw body, constant-time
/// compared; `X-PayBridge-Timestamp` must be within ±5 minutes.
fn verify_signature(secret: &str, headers: &HeaderMap, body: &[u8]) -> bool {
    let timestamp = headers
        .get("x-paybridge-timestamp")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.trim().parse::<i64>().ok());
    let now = chrono::Utc::now().timestamp();
    match timestamp {
        Some(ts) if (now - ts).abs() <= MAX_CLOCK_SKEW_SECS => {}
        _ => return false,
    }

    let Some(signature) = headers
        .get("x-paybridge-signature")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("sha256="))
    else {
        return false;
    };

    let expected = hmac_sha256_hex(secret, body);
    constant_time_eq(signature.as_bytes(), expected.as_bytes())
}

fn hmac_sha256_hex(secret: &str, data: &[u8]) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
        .expect("hmac accepts any key length");
    mac.update(data);
    hex::encode(mac.finalize().into_bytes())
}
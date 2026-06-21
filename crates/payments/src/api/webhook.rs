use crate::api::request_context::RequestCtx;
use crate::application::checkout::payment_webhook::PaymentWebhookHandler;
use crate::domain;
use axum::extract::Path;
use axum::{extract::State, http::StatusCode, Json};
use tracing::debug;

#[utoipa::path(
    post,
    tag = "payments",
    path = "/payments/webhook/{provider_name}",
    request_body = serde_json::Value,
    responses((status = OK, body = ()))
)]
pub async fn webhook_payment_handler(
    ctx: RequestCtx,
    Path(provider_name): Path<domain::provider::Provider>,
    State(handler): State<PaymentWebhookHandler>,
    Json(payload): Json<serde_json::Value>,
) -> Result<StatusCode, StatusCode> {
    handler
        .execute(provider_name, &ctx.0, payload)
        .await
        .map_err(|e| {
            debug!(?e, "webhook error");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::OK)
}

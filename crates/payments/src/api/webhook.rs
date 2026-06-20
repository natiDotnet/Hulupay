use crate::{domain, ProviderEngine};
use axum::extract::Path;
use axum::{extract::State, http::StatusCode, Json};
#[utoipa::path(
    post,
    tag = "payments",
    path = "/payments/webhook/{provider_name}",
    request_body = serde_json::Value,
    responses((status = OK, body = ()))
)]
pub async fn webhook_payment_handler(
    Path(provider_name): Path<domain::provider::Provider>,
    State(engine): State<ProviderEngine>,
    Json(payload): Json<serde_json::Value>,
) -> Result<StatusCode, StatusCode> {
    let provider = engine
        .get_provider(None, Some(provider_name))
        .await
        .ok_or(StatusCode::NOT_FOUND)?
        .clone();
    tokio::spawn(async move {
        if let Err(e) = provider.webhook(payload.clone()).await {
            tracing::error!("Webhook error: {}", e);
        }
    });

    Ok(StatusCode::OK)
}

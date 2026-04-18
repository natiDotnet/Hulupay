use crate::application::HandleProviderWebhook;
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
    Path(provider_name): Path<String>,
    State(use_case): State<HandleProviderWebhook>,
    Json(payload): Json<serde_json::Value>,
) -> Result<StatusCode, StatusCode> {
    use_case
        .execute(payload, provider_name)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

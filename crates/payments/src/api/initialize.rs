use crate::application::{InitializePaymentCommand, ProviderEngine};
use axum::extract::Path;
use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct InitializePaymentRequest {
    pub merchant_id: Uuid,
    pub phone: String,
    pub email: String,
    pub amount: i64,
    pub currency: String,
}

#[derive(serde::Serialize, ToSchema)]
pub struct InitializePaymentResponse {
    pub checkout_url: String,
    pub provider_reference: String,
}

#[utoipa::path(
    post,
    tag = "payments",
    security(("bearer_auth" = [])),
    path = "/payments/{provider_name}/initialize",
    request_body = InitializePaymentRequest,
    responses((status = OK, body = InitializePaymentResponse))
)]
pub async fn initialize_payment_handler(
    Path(provider_name): Path<String>,
    State(provider_engine): State<ProviderEngine>,
    Json(payload): Json<InitializePaymentRequest>,
) -> Result<Json<InitializePaymentResponse>, StatusCode> {
    let cmd = InitializePaymentCommand {
        merchant_id: payload.merchant_id,
        phone: payload.phone,
        email: payload.email,
        amount: payload.amount,
        currency: payload.currency,
    };

    // Get the provider from the engine based on provider_name
    let provider = provider_engine
        .get_provider(payload.merchant_id, &provider_name)
        .await
        .map_err(|_| {
            eprintln!("Provider '{}' not found", provider_name);
            StatusCode::NOT_FOUND
        })?;

    let result = provider.initialize_payment(cmd).await.map_err(|e| {
        eprintln!("Error initializing payment: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(InitializePaymentResponse {
        checkout_url: result.checkout_url,
        provider_reference: result.provider_reference,
    }))
}

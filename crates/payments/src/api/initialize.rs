use crate::application::initiate_payment::{InitializePaymentRequest, InitializePaymentResponse};
use crate::application::{InitializePaymentCommand, ProviderEngine};
use crate::domain;
use axum::extract::Path;
use axum::{extract::State, http::StatusCode, Json};
use rust_decimal::prelude::ToPrimitive;

#[utoipa::path(
    post,
    tag = "payments",
    security(("bearer_auth" = [])),
    path = "/payments/{provider_name}/initialize",
    request_body = InitializePaymentRequest,
    responses((status = OK, body = InitializePaymentResponse))
)]
pub async fn initialize_payment_handler(
    Path(provider): Path<domain::provider::Provider>,
    State(provider_engine): State<ProviderEngine>,
    Json(payload): Json<InitializePaymentRequest>,
) -> Result<Json<InitializePaymentResponse>, StatusCode> {
    let cmd = InitializePaymentCommand {
        merchant_id: payload.merchant_id,
        phone: payload.phone,
        email: payload.email,
        amount: payload.amount.to_i64().unwrap(),
        currency: payload.currency,
    };

    // Get the provider from the engine based on provider_name
    let provider = provider_engine
        .get_provider(payload.merchant_id, provider.clone())
        .await
        .map_err(|_| {
            eprintln!("Provider '{}' not found", provider);
            StatusCode::NOT_FOUND
        })?;

    let result = provider.initialize_payment(cmd).await.map_err(|e| {
        eprintln!("Error initializing payment: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(InitializePaymentResponse {
        checkout_url: Some(result.checkout_url),
        provider_reference: result.provider_reference,
    }))
}

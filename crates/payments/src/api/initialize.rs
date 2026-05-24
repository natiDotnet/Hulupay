use crate::application::initiate_payment::{InitializePaymentRequest, InitializePaymentResponse};
use crate::application::{InitializePaymentCommand, ProviderEngine};
use axum::extract::Path;
use axum::{extract::State, http::StatusCode, Json};
use utoipa::ToSchema;

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

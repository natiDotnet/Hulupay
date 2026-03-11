use crate::application::{ArifPayProvider, InitializePaymentCommand, PaymentGateway};
use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct InitializePaymentRequest {
    pub merchant_id: Uuid,
    pub phone: String,
    pub email: String,
    pub amount: f64,
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
    path = "/payments/initialize",
    request_body = InitializePaymentRequest,
    responses((status = OK, body = InitializePaymentResponse))
)]
pub async fn initialize_payment_handler(
    State(provider): State<ArifPayProvider>,
    Json(payload): Json<InitializePaymentRequest>,
) -> Result<Json<InitializePaymentResponse>, StatusCode> {
    let cmd = InitializePaymentCommand {
        merchant_id: payload.merchant_id,
        phone: payload.phone,
        email: payload.email,
        amount: payload.amount,
        currency: payload.currency,
    };

    let result = provider.initialize_payment(cmd).await.map_err(|e| {
        eprintln!("Error initializing payment: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(InitializePaymentResponse {
        checkout_url: result.checkout_url,
        provider_reference: result.provider_reference,
    }))
}

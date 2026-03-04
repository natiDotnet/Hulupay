use crate::application::{ArifPayProvider, PaymentGateway};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct VerifyPaymentResponse {
    pub success: bool,
    pub provider_reference: String,
}

#[utoipa::path(
    get,
    tag = "payments",
    path = "/payments/verify/{reference}",
    responses((status = OK, body = VerifyPaymentResponse))
)]
pub async fn verify_payment_handler(
    State(provider): State<ArifPayProvider>,
    Path(reference): Path<String>,
) -> Result<Json<VerifyPaymentResponse>, StatusCode> {
    let result = provider.verify_payment(&reference).await
        .map_err(|e| {
            eprintln!("Error verifying payment: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(VerifyPaymentResponse {
        success: result.success,
        provider_reference: result.provider_reference,
    }))
}

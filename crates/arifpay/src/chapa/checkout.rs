use crate::api::checkout::ArifpayApiErr;
use crate::chapa::checkout_request::ChapaInitializeRequest;
use crate::chapa::checkout_response::{ChapaCheckoutResponse, ChapaResponse};
use axum::extract::State;
use axum::http::header::AUTHORIZATION;
use axum::http::HeaderMap;
use axum::Json;
use hulu_core::create_checkout::CreateCheckout;
use hulu_core::payment_request::PaymentRequest;
use std::sync::Arc;

// #[debug_handler]
#[utoipa::path(
    post,
    tag = "chapa",
    path = "/v1/transaction/initialize",
    responses((status = OK, body = ChapaInitializeRequest))
)]
pub async fn chapa_checkout_handler(
    State(checkout): State<Arc<dyn CreateCheckout>>,
    headers: HeaderMap,
    Json(payload): Json<ChapaInitializeRequest>,
) -> Result<Json<ChapaResponse<ChapaCheckoutResponse>>, ArifpayApiErr> {
    let token = headers
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .map(str::trim);
    let request: PaymentRequest = payload.into();
    let response = checkout.execute("master", request).await?;

    Ok(Json(response.into()))
}

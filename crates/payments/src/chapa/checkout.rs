use crate::chapa::chapa_api_error::ChapaApiErr;
use crate::chapa::checkout_request::ChapaInitializeRequest;
use crate::chapa::checkout_response::{ChapaCheckoutResponse, ChapaResponse};
use auth::api::AuthUser;
use axum::Json;
use axum::extract::State;
use hulu_core::create_checkout::CreateCheckout;
use hulu_core::payment_method::GatewayProvider;
use hulu_core::payment_request::PaymentRequest;
use std::sync::Arc;

// #[debug_handler]
#[utoipa::path(
    post,
    tag = "chapa",
    path = "/v1/transaction/initialize",
    request_body = ChapaInitializeRequest,
    responses((status = OK, body = ChapaResponse<ChapaCheckoutResponse>),
        (status = BAD_REQUEST, body = ChapaResponse<serde_json::Value>))
)]
pub async fn chapa_checkout_handler(
    AuthUser(ctx): AuthUser,
    State(checkout): State<Arc<dyn CreateCheckout>>,
    Json(payload): Json<ChapaInitializeRequest>,
) -> Result<Json<ChapaResponse<ChapaCheckoutResponse>>, ChapaApiErr> {
    // ctx.0.set_provider(GatewayProvider::Chapa);
    // Chapa-compatible validation: single-error responses that mirror the
    // upstream API exactly (field-keyed or plain-string message shapes).
    let request: PaymentRequest = payload.validate()?.into();
    let response = checkout.execute(GatewayProvider::Chapa, &ctx, request).await?;

    Ok(Json(response.into()))
}

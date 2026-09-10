use crate::lakipay::checkout_request::LakiInitializeRequest;
use crate::lakipay::checkout_response::LakiCheckoutResponse;
use crate::lakipay::lakipay_api_error::LakiApiErr;
use auth::api::AuthUser;
use axum::Json;
use axum::extract::State;
use hulu_core::create_checkout::CreateCheckout;
use hulu_core::payment_method::GatewayProvider;
use hulu_core::payment_request::PaymentRequest;
use std::sync::Arc;

#[utoipa::path(
    post,
    tag = "lakipay",
    path = "/v2/payment/checkout",
    request_body = LakiInitializeRequest,
    responses((status = OK, body = LakiCheckoutResponse),
        (status = BAD_REQUEST, body = LakiCheckoutResponse))
)]
pub async fn laki_checkout_handler(
    AuthUser(ctx): AuthUser,
    State(checkout): State<Arc<dyn CreateCheckout>>,
    Json(payload): Json<LakiInitializeRequest>,
) -> Result<Json<LakiCheckoutResponse>, LakiApiErr> {
    // LakiPay-compatible validation: multi-error "; "-joined responses that
    // mirror the upstream API exactly (field order per the validation study).
    let request: PaymentRequest = payload.validate()?.into();
    let response = checkout
        .execute(GatewayProvider::LakiPay, &ctx, request)
        .await?;

    Ok(Json(LakiCheckoutResponse::success(
        response.checkout_url,
        response.reference,
    )))
}

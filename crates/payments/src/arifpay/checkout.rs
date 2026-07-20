use crate::api::request_context::RequestCtx;
use crate::arifpay::arif_api_error::ArifpayApiErr;
use crate::arifpay::payment_request::ArifpayPaymentRequest;
use crate::arifpay::payment_response::{ArifInitializeData, ArifResponse};
use axum::{Json, extract::State};
use hulu_core::create_checkout::CreateCheckout;
use hulu_core::payment_method::GatewayProvider;
use hulu_core::payment_request::PaymentRequest;
use std::sync::Arc;

#[utoipa::path(
    post,
    tag = "arifpay",
    path = "/checkout/session",
    request_body = ArifpayPaymentRequest,
    responses((status = CREATED, body = ArifResponse<ArifInitializeData>),
        (status = BAD_REQUEST, body = ArifResponse<serde_json::Value>))
)]
pub async fn arifpay_checkout_handler(
    mut ctx: RequestCtx,
    State(checkout): State<Arc<dyn CreateCheckout>>,
    Json(payload): Json<ArifpayPaymentRequest>,
) -> Result<Json<ArifResponse<ArifInitializeData>>, ArifpayApiErr> {
    let request: PaymentRequest = payload.try_into()?;
    ctx.0.set_provider(GatewayProvider::Arifpay);
    let provider = checkout.execute(&ctx.0, request).await?;

    // let provider = ArifPayInitializeResponse {
    //     error: false,
    //     msg: "".to_string(),
    //     data: Some(ArifPayInitializeData {
    //         session_id: "qwertyuiop".to_string(),
    //         payment_url: "qwertyuiop".to_string(),
    //         cancel_url: "QWERTYUIOP".to_string(),
    //         total_amount: Decimal::from_f64_retain(11.1).unwrap(),
    //     }),
    // };

    Ok(Json(provider.into()))
}

// use axum::{extract::State, Json};
// use hulu_core::create_checkout::CreateCheckout;
// use hulu_core::gateway_response::CheckoutResponse;
// use hulu_core::hulu_error::HuluError;
// use hulu_core::payment_request::PaymentRequest;
// use std::sync::Arc;
//
// fn default_is_active() -> bool {
//     true
// }
//
// #[utoipa::path(
//     post,
//     tag = "arifpay",
//     path = "/arifpay/api/checkout/session",
//     request_body = PaymentRequest,
//     responses((status = CREATED, body = CheckoutResponse))
// )]
// pub async fn create_checkout_session_handler(
//     State(usecase): State<Arc<dyn CreateCheckout>>,
//     Json(payload): Json<PaymentRequest>,
// ) -> Result<Json<CheckoutResponse>, HuluError> {
//     // let request: PaymentRequest = payload.try_into()?;
//     let provider = usecase.execute("master", payload).await?;
//
//     // let provider = ArifPayInitializeResponse {
//     //     error: false,
//     //     msg: "".to_string(),
//     //     data: Some(ArifPayInitializeData {
//     //         session_id: "qwertyuiop".to_string(),
//     //         payment_url: "qwertyuiop".to_string(),
//     //         cancel_url: "QWERTYUIOP".to_string(),
//     //         total_amount: Decimal::from_f64_retain(11.1).unwrap(),
//     //     }),
//     // };
//
//     Ok(Json(provider))
// }

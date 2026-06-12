// use crate::application::ProviderEngine;
// use crate::{domain, PaymentVerificationResult};
// use axum::{
//     extract::{Path, State},
//     http::StatusCode,
//     Json,
// };
// use serde::Serialize;
// use utoipa::ToSchema;
// use uuid::Uuid;
//
// #[derive(Serialize, ToSchema)]
// pub struct VerifyPaymentResponse {
//     pub success: bool,
//     pub provider_reference: String,
// }
//
// #[utoipa::path(
//     get,
//     tag = "payments",
//     path = "/payments/{provider_name}/verify/{reference}",
//     responses((status = OK, body = VerifyPaymentResponse))
// )]
// pub async fn verify_payment_handler(
//     State(provider_engine): State<ProviderEngine>,
//     Path((provider_name, reference)): Path<(domain::provider::Provider, String)>,
// ) -> Result<Json<VerifyPaymentResponse>, StatusCode> {
//     // Get the provider from the engine based on provider_name
//     let provider = provider_engine
//         .get_provider(Uuid::nil(), provider_name.clone())
//         // .await
//         .ok_or(StatusCode::NOT_FOUND)?;
//     // .ok_or(|_| {
//     //     eprintln!("Provider '{}' not found", provider_name);
//     //     StatusCode::NOT_FOUND
//     // })?;
//
//     let result: PaymentVerificationResult = provider.verify(&reference).await.map_err(|e| {
//         eprintln!("Error verifying payment: {:?}", e);
//         StatusCode::INTERNAL_SERVER_ERROR
//     })?;
//
//     Ok(Json(VerifyPaymentResponse {
//         success: result.success,
//         provider_reference: result.provider_reference,
//     }))
// }

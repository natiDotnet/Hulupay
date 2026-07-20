// use crate::application::initiate_payment::{
//     InitializePaymentRequest, InitializePaymentResponse, InitiatePayment,
// };
// use crate::application::payment_gateway::ApiStatus;
// use crate::domain;
// use auth::api::AuthUser;
// use axum::extract::Path;
// use axum::{extract::State, http::StatusCode, Json};
// use tracing::log::error;
//
// #[utoipa::path(
//     post,
//     tag = "payments",
//     security(("bearer_auth" = [])),
//     path = "/payments/{provider_name}/initialize",
//     request_body = InitializePaymentRequest,
//     responses((status = OK, body = InitializePaymentResponse))
// )]
// pub async fn initialize_payment_handler(
//     Path(provider): Path<domain::provider::Provider>,
//     State(init_handler): State<InitiatePayment>,
//     Json(payload): Json<InitializePaymentRequest>,
//     AuthUser(user): AuthUser,
// ) -> Result<Json<InitializePaymentResponse>, StatusCode> {
//     let result = init_handler.execute(payload).await.map_err(|e| {
//         error!("errrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr: {:?}", e);
//         StatusCode::NOT_FOUND
//     })?;
//     Ok(Json(InitializePaymentResponse {
//         message: "success".to_string(),
//         status: ApiStatus::Success,
//         status_code: result.status_code,
//         checkout_url: result.checkout_url,
//         provider_reference: result.provider_reference,
//     }))
// }

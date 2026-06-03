use crate::application::initiate_payment::{InitializePaymentRequest, InitiatePayment};
use crate::application::payment_gateway::ApiStatus;
use crate::domain::provider::Provider;
use crate::infrastructure::arifpay::payment_request::PaymentRequest;
use crate::infrastructure::arifpay::payment_response::ArifPayInitializeResponse;
use axum::{extract::State, http::StatusCode, Json};
use tracing::debug;

fn default_is_active() -> bool {
    true
}

#[utoipa::path(
    post,
    tag = "arifpay",
    path = "/arifpay/api/checkout/session",
    request_body = PaymentRequest,
    responses((status = CREATED, body = ArifPayInitializeResponse))
)]
pub async fn create_checkout_session_handler(
    State(usecase): State<InitiatePayment>,
    Json(payload): Json<PaymentRequest>,
) -> Result<Json<ArifPayInitializeResponse>, StatusCode> {
    let request = InitializePaymentRequest {
        merchant_id: Default::default(),
        phone: payload.phone,
        email: payload.email,
        amount: payload.beneficiaries.get(0).unwrap().amount,
        currency: payload.currency.unwrap_or_else(|| "ETB".to_string()),
        provider: Provider::ArifPay,
        nonce: payload.nonce,
    };
    let provider = usecase.execute(request).await.map_err(|e| {
        debug!(?e, "Error creating payment provider");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(ArifPayInitializeResponse {
        msg: provider.message,
        error: provider.status != ApiStatus::Success,
        data: None,
    }))
}

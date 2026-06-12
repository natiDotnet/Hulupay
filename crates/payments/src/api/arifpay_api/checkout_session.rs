use crate::application::checkout::create_checkout::CreateCheckout;
use arif::arifpay::payment_request::ArifpayPaymentRequest;
use arif::arifpay::payment_response::ArifPayInitializeResponse;
use auth::HuluResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{extract::State, Json};
use hulu_core::hulu_error::HuluError;
use hulu_core::payment_request::PaymentRequest;

fn default_is_active() -> bool {
    true
}

#[utoipa::path(
    post,
    tag = "arifpay",
    path = "/arifpay/api/checkout/session",
    request_body = ArifpayPaymentRequest,
    responses((status = CREATED, body = ArifPayInitializeResponse))
)]
pub async fn create_checkout_session_handler(
    State(usecase): State<CreateCheckout>,
    Json(payload): Json<ArifpayPaymentRequest>,
) -> Result<Json<ArifPayInitializeResponse>, ArifpayApiErr> {
    let request: PaymentRequest = payload.try_into()?;
    let provider = usecase.execute("master", request).await?;

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

pub struct ArifpayApiErr(pub HuluError);
impl From<HuluError> for ArifpayApiErr {
    fn from(value: HuluError) -> Self {
        Self(value)
    }
}
pub struct ApiError(pub HuluError);

impl From<HuluError> for ApiError {
    fn from(value: HuluError) -> Self {
        Self(value)
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message, errors) = match self.0 {
            HuluError::ProviderNotFound => (
                StatusCode::NOT_FOUND,
                "provider not found".to_string(),
                None,
            ),

            HuluError::UnsupportedPaymentMethod => (
                StatusCode::BAD_REQUEST,
                "unsupported payment method".to_string(),
                None,
            ),

            HuluError::ResponseParseError => (
                StatusCode::BAD_GATEWAY,
                "unable to parse provider response".to_string(),
                None,
            ),

            HuluError::ConnectionError => (
                StatusCode::BAD_GATEWAY,
                "connection error".to_string(),
                None,
            ),

            HuluError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error".to_string(),
                None,
            ),

            HuluError::ProviderError {
                message,
                status_code,
                errors,
            } => (
                StatusCode::from_u16(status_code).unwrap_or(StatusCode::BAD_GATEWAY),
                message,
                errors,
            ),
        };

        (
            status,
            Json(HuluResponse::<serde_json::Value> {
                // status_code: status.as_u16(),
                success: false,
                message,
                data: errors,
            }),
        )
            .into_response()
    }
}

impl IntoResponse for ArifpayApiErr {
    fn into_response(self) -> Response {
        let (status, message, errors) = match self.0 {
            HuluError::ProviderNotFound => (
                StatusCode::NOT_FOUND,
                "provider not found".to_string(),
                None,
            ),

            HuluError::UnsupportedPaymentMethod => (
                StatusCode::BAD_REQUEST,
                "unsupported payment method".to_string(),
                None,
            ),

            HuluError::ResponseParseError => (
                StatusCode::BAD_GATEWAY,
                "unable to parse provider response".to_string(),
                None,
            ),

            HuluError::ConnectionError => (
                StatusCode::BAD_GATEWAY,
                "connection error".to_string(),
                None,
            ),

            HuluError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error".to_string(),
                None,
            ),

            HuluError::ProviderError {
                message,
                status_code,
                errors,
            } => (
                StatusCode::from_u16(status_code).unwrap_or(StatusCode::BAD_GATEWAY),
                message,
                errors,
            ),
        };

        (
            status,
            Json(HuluResponse::<serde_json::Value> {
                // status_code: status.as_u16(),
                success: false,
                message,
                data: errors,
            }),
        )
            .into_response()
    }
}

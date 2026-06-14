use crate::arifpay::payment_response::ArifResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use hulu_core::hulu_error::HuluError;

pub struct ArifpayApiErr(pub HuluError);
impl From<HuluError> for ArifpayApiErr {
    fn from(value: HuluError) -> Self {
        Self(value)
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
            Json(ArifResponse::<serde_json::Value> {
                // status_code: status.as_u16(),
                error: true,
                msg: message,
                data: errors,
            }),
        )
            .into_response()
    }
}

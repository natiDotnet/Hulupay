use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use hulu_core::hulu_error::HuluError;
use hulu_core::hulu_response::HuluResponse;
use serde_json;

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
            HuluError::PaymentAlreadyCompleted => (
                StatusCode::BAD_REQUEST,
                "payment already completed".to_string(),
                None,
            ),
            HuluError::PaymentOrderAlreadyExists => (
                StatusCode::BAD_REQUEST,
                "payment order already exists".to_string(),
                None,
                )
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

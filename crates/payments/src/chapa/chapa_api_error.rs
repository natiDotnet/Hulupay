use crate::chapa::checkout_response::ChapaResponse;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use hulu_core::hulu_error::HuluError;

pub struct ChapaApiErr(pub HuluError);

impl From<HuluError> for ChapaApiErr {
    fn from(value: HuluError) -> Self {
        Self(value)
    }
}
impl IntoResponse for ChapaApiErr {
    fn into_response(self) -> Response {
        let (status, message) = match self.0 {
            HuluError::ProviderNotFound => {
                (StatusCode::NOT_FOUND, "provider not found".to_string())
            }

            HuluError::UnsupportedPaymentMethod => (
                StatusCode::BAD_REQUEST,
                "unsupported payment method".to_string(),
            ),

            HuluError::ResponseParseError => (
                StatusCode::BAD_GATEWAY,
                "unable to parse provider response".to_string(),
            ),

            HuluError::ConnectionError => (StatusCode::BAD_GATEWAY, "connection error".to_string()),

            HuluError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error".to_string(),
            ),

            HuluError::ProviderError {
                message,
                status_code,
                errors: _,
            } => (
                StatusCode::from_u16(status_code).unwrap_or(StatusCode::BAD_GATEWAY),
                message,
            ),
            HuluError::PaymentAlreadyCompleted => (
                StatusCode::BAD_REQUEST,
                "payment already completed".to_string(),
            ),
        };

        (
            status,
            Json(ChapaResponse::<serde_json::Value> {
                // status_code: status.as_u16(),
                status: "failed".to_string(),
                message,
                data: None,
            }),
        )
            .into_response()
    }
}

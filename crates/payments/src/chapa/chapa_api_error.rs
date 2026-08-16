use crate::chapa::checkout_request::ChapaRequestError;
use crate::chapa::checkout_response::ChapaResponse;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use hulu_core::hulu_error::HuluError;
use serde_json::{Value, json};

pub enum ChapaApiErr {
    Hulu(HuluError),
    /// Request failed Chapa-compatible field validation; the response body
    /// mirrors the upstream API exactly (chapa-validation/report.md §12).
    Validation(ChapaRequestError),
}

impl From<HuluError> for ChapaApiErr {
    fn from(value: HuluError) -> Self {
        Self::Hulu(value)
    }
}
impl From<ChapaRequestError> for ChapaApiErr {
    fn from(value: ChapaRequestError) -> Self {
        Self::Validation(value)
    }
}

impl IntoResponse for ChapaApiErr {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ChapaApiErr::Validation(err) => {
                // Upstream error shapes, verbatim:
                //
                // field-keyed (amount, currency, email, tx_ref, urls, names):
                //   {"message":{"amount":["validation.min.numeric"]},
                //    "status":"failed","data":null}
                //
                // plain string (phone only):
                //   {"message":"Invalid Phone number, please use a proper
                //    phone number or use business shortcode.",
                //    "status":"failed","data":null}
                let message = match err {
                    ChapaRequestError::Field(field, messages) => json!({ field: messages }),
                    ChapaRequestError::Plain(message) => Value::String(message.to_string()),
                };
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "message": message,
                        "status": "failed",
                        "data": Value::Null,
                    })),
                )
                    .into_response();
            }

            ChapaApiErr::Hulu(error) => match error {
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

                HuluError::ConnectionError => {
                    (StatusCode::BAD_GATEWAY, "connection error".to_string())
                }

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
            },
        };

        (
            status,
            Json(ChapaResponse::<serde_json::Value> {
                status: "failed".to_string(),
                message,
                data: None,
            }),
        )
            .into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chapa::checkout_request::PHONE_INVALID_MSG;
    use axum::body::to_bytes;

    async fn body_of(err: ChapaApiErr) -> (axum::http::StatusCode, serde_json::Value) {
        let response = err.into_response();
        let status = response.status();
        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        (status, serde_json::from_slice(&bytes).unwrap())
    }

    #[tokio::test]
    async fn field_error_matches_upstream_shape_exactly() {
        // verbatim upstream response from T001 (amount -10)
        let (status, body) = body_of(ChapaApiErr::Validation(
            ChapaRequestError::Field("amount", vec!["validation.min.numeric".into()]),
        ))
        .await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(
            body,
            json!({
                "message": { "amount": ["validation.min.numeric"] },
                "status": "failed",
                "data": null
            })
        );
    }

    #[tokio::test]
    async fn plain_error_matches_upstream_shape_exactly() {
        // verbatim upstream response from PH10 (invalid phone)
        let (status, body) = body_of(ChapaApiErr::Validation(
            ChapaRequestError::Plain(PHONE_INVALID_MSG),
        ))
        .await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(
            body,
            json!({
                "message": "Invalid Phone number, please use a proper phone number or use business shortcode.",
                "status": "failed",
                "data": null
            })
        );
    }

    #[tokio::test]
    async fn hulu_errors_keep_envelope() {
        let (status, body) = body_of(ChapaApiErr::Hulu(HuluError::ConnectionError)).await;
        assert_eq!(status, StatusCode::BAD_GATEWAY);
        assert_eq!(body["status"], json!("failed"));
        assert_eq!(body["message"], json!("connection error"));
    }
}

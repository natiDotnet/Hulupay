use crate::lakipay::checkout_request::LakiRequestError;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use hulu_core::hulu_error::HuluError;
use serde_json::{Value, json};

pub enum LakiApiErr {
    Hulu(HuluError),
    /// Request failed LakiPay-compatible validation; the response body
    /// mirrors the upstream flat envelope exactly
    /// (lakipay-validation/report.md §19).
    Validation(LakiRequestError),
}

impl From<HuluError> for LakiApiErr {
    fn from(value: HuluError) -> Self {
        Self::Hulu(value)
    }
}
impl From<LakiRequestError> for LakiApiErr {
    fn from(value: LakiRequestError) -> Self {
        Self::Validation(value)
    }
}

impl LakiApiErr {
    fn message(&self) -> String {
        match self {
            LakiApiErr::Validation(err) => match err {
                // Multi-field: "amount: cannot be blank; currency: cannot be blank."
                LakiRequestError::Fields(fields) => fields
                    .iter()
                    .map(|(_, message)| message.as_str())
                    .collect::<Vec<_>>()
                    .join("; "),
                // Post-schema business messages, verbatim ("unsupported medium: X").
                LakiRequestError::Plain(message) => message.clone(),
                // Go unmarshal leak, verbatim.
                LakiRequestError::Unmarshal(message) => message.clone(),
            },
            LakiApiErr::Hulu(error) => hulu_message(error),
        }
    }

    fn status(&self) -> StatusCode {
        match self {
            LakiApiErr::Validation(_) => StatusCode::BAD_REQUEST,
            LakiApiErr::Hulu(error) => hulu_status(error),
        }
    }
}

fn hulu_status(error: &HuluError) -> StatusCode {
    match error {
        HuluError::ProviderNotFound => StatusCode::NOT_FOUND,
        HuluError::UnsupportedPaymentMethod
        | HuluError::PaymentAlreadyCompleted
        | HuluError::PaymentOrderAlreadyExists => StatusCode::BAD_REQUEST,
        HuluError::ResponseParseError | HuluError::ConnectionError => StatusCode::BAD_GATEWAY,
        HuluError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        HuluError::ProviderError { status_code, .. } => {
            StatusCode::from_u16(*status_code).unwrap_or(StatusCode::BAD_GATEWAY)
        }
    }
}

fn hulu_message(error: &HuluError) -> String {
    match error {
        HuluError::ProviderNotFound => "provider not found".to_string(),
        HuluError::UnsupportedPaymentMethod => "unsupported payment method".to_string(),
        HuluError::ResponseParseError => "unable to parse provider response".to_string(),
        HuluError::ConnectionError => "connection error".to_string(),
        HuluError::InternalServerError => "internal server error".to_string(),
        HuluError::PaymentAlreadyCompleted => "payment already completed".to_string(),
        HuluError::PaymentOrderAlreadyExists => "payment order already exists".to_string(),
        HuluError::ProviderError { message, .. } => message.clone(),
    }
}

impl IntoResponse for LakiApiErr {
    fn into_response(self) -> Response {
        // Upstream uses ONE flat envelope for every non-auth error
        // (validation, business logic, duplicates — report.md §19):
        //   {"success":false,"message":"<text>"}
        // (only its 401 auth errors nest under "error"; HuluPay's own auth
        // middleware is responsible for that layer, so we never emit it.)
        (
            self.status(),
            Json(json!({
                "success": false,
                "message": self.message(),
            })),
        )
            .into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;

    async fn body_of(err: LakiApiErr) -> (StatusCode, Value) {
        let response = err.into_response();
        let status = response.status();
        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        (status, serde_json::from_slice(&bytes).unwrap())
    }

    #[tokio::test]
    async fn multi_field_error_joined_with_semicolons() {
        // verbatim upstream shape from XB02
        let (status, body) = body_of(LakiApiErr::Validation(LakiRequestError::Fields(vec![
            ("amount", "amount: cannot be blank.".to_string()),
            ("currency", "currency: cannot be blank.".to_string()),
        ])))
        .await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(
            body,
            json!({
                "success": false,
                "message": "amount: cannot be blank; currency: cannot be blank."
            })
        );
    }

    #[tokio::test]
    async fn plain_medium_error_verbatim() {
        // verbatim upstream shape from SM07
        let (status, body) = body_of(LakiApiErr::Validation(LakiRequestError::Plain(
            "unsupported medium: UNKNOWN_MEDIUM".to_string(),
        )))
        .await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(
            body,
            json!({ "success": false, "message": "unsupported medium: UNKNOWN_MEDIUM" })
        );
    }

    #[tokio::test]
    async fn unmarshal_error_verbatim() {
        // verbatim upstream shape from AM18
        let (status, body) = body_of(LakiApiErr::Validation(LakiRequestError::Unmarshal(
            "json: cannot unmarshal string into Go struct field HostedCheckoutRequest.amount of type float64"
                .to_string(),
        )))
        .await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(
            body,
            json!({
                "success": false,
                "message": "json: cannot unmarshal string into Go struct field HostedCheckoutRequest.amount of type float64"
            })
        );
    }

    #[tokio::test]
    async fn hulu_errors_use_the_same_flat_envelope() {
        let (status, body) = body_of(LakiApiErr::Hulu(HuluError::ConnectionError)).await;
        assert_eq!(status, StatusCode::BAD_GATEWAY);
        assert_eq!(body, json!({ "success": false, "message": "connection error" }));
    }
}

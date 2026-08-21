use crate::payment_gateway_error::PaymentGatewayError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HuluError {
    #[error("provider not found")]
    ProviderNotFound,
    #[error("provider responded with an error!")]
    ProviderError {
        message: String,
        status_code: u16,
        errors: Option<serde_json::Value>,
    },
    #[error("internal server error!")]
    InternalServerError,

    #[error("unsupported payment method!")]
    UnsupportedPaymentMethod,

    #[error("unable to parse the response!")]
    ResponseParseError,

    #[error("unable to send the request!")]
    ConnectionError,

    #[error("payment already completed")]
    PaymentAlreadyCompleted,
}

impl From<PaymentGatewayError> for HuluError {
    fn from(value: PaymentGatewayError) -> Self {
        match value {
            PaymentGatewayError::RequestFailed => HuluError::ConnectionError,
            PaymentGatewayError::InvalidResponse => HuluError::ResponseParseError,
            PaymentGatewayError::ProviderNotFound => HuluError::ProviderNotFound,
            PaymentGatewayError::ProviderError {
                message,
                status_code,
                errors,
            } => HuluError::ProviderError {
                message,
                status_code,
                errors,
            },
            PaymentGatewayError::InternalServerError => HuluError::InternalServerError,
            PaymentGatewayError::MerchantNotFound => HuluError::ProviderNotFound,
            PaymentGatewayError::UnsupportedPaymentMethod => HuluError::UnsupportedPaymentMethod,
            PaymentGatewayError::TransactionNotFound => HuluError::ProviderNotFound,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_failed_maps_to_connection_error() {
        let err: HuluError = PaymentGatewayError::RequestFailed.into();
        assert!(matches!(err, HuluError::ConnectionError));
    }

    #[test]
    fn invalid_response_maps_to_response_parse_error() {
        let err: HuluError = PaymentGatewayError::InvalidResponse.into();
        assert!(matches!(err, HuluError::ResponseParseError));
    }

    #[test]
    fn not_found_errors_map_to_provider_not_found() {
        let err: HuluError = PaymentGatewayError::ProviderNotFound.into();
        assert!(matches!(err, HuluError::ProviderNotFound));

        let err: HuluError = PaymentGatewayError::MerchantNotFound.into();
        assert!(matches!(err, HuluError::ProviderNotFound));

        let err: HuluError = PaymentGatewayError::TransactionNotFound.into();
        assert!(matches!(err, HuluError::ProviderNotFound));
    }

    #[test]
    fn provider_error_payload_is_preserved() {
        let err: HuluError = PaymentGatewayError::ProviderError {
            message: "insufficient funds".into(),
            status_code: 402,
            errors: Some(serde_json::json!({"code": "E01"})),
        }
        .into();

        match err {
            HuluError::ProviderError {
                message,
                status_code,
                errors,
            } => {
                assert_eq!(message, "insufficient funds");
                assert_eq!(status_code, 402);
                assert!(errors.is_some());
            }
            other => panic!("expected ProviderError, got {other:?}"),
        }
    }

    #[test]
    fn internal_error_passes_through() {
        let err: HuluError = PaymentGatewayError::InternalServerError.into();
        assert!(matches!(err, HuluError::InternalServerError));
    }
}

use crate::arifpay::payment_request::{ArifpayPaymentRequest, OtpRequest, VerifyOtpRequest};
use crate::arifpay::payment_response::{ArifInitializeData, ArifResponse, ArifVerifyResponse};
use hulu_core::payment_gateway_error::PaymentGatewayError;
use hulu_core::payment_method::PaymentMethod;
use std::collections::HashMap;
use tracing::debug;

#[derive(Clone)]
pub struct ArifpayService {
    client: reqwest::Client,
    urls: HashMap<String, String>,
}

impl ArifpayService {
    pub fn new(client: reqwest::Client) -> Self {
        let mut urls = HashMap::new();

        urls.insert(
            PaymentMethod::Telebirr.to_string(),
            "/api/checkout/telebirr-ussd/transfer/direct".to_string(),
        );
        urls.insert(
            PaymentMethod::CbeBirr.to_string(),
            "/api/checkout/v2/cbe/direct/transfer".to_string(),
        );
        urls.insert(
            PaymentMethod::Mpesa.to_string(),
            "/api/checkout/mpesa/transfer/direct".to_string(),
        );
        urls.insert(
            PaymentMethod::AwashBirr.to_string(),
            "/api/checkout/awash/direct/transfer".to_string(),
        );
        urls.insert(
            PaymentMethod::Kacha.to_string(),
            "/api/checkout/kacha/direct/transfer".to_string(),
        );
        urls.insert(
            PaymentMethod::ZamZam.to_string(),
            "/api/checkout/zamzam/direct/transfer".to_string(),
        );

        Self { client, urls }
    }
}

impl ArifpayService {
    fn get_apikey_name(&self) -> &'static str {
        "x-arifpay-key"
    }
    pub async fn create_session(
        &self,
        base_url: String,
        apikey: String,
        request: &hulu_core::payment_request::PaymentRequest,
    ) -> Result<ArifInitializeData, PaymentGatewayError> {
        let request: ArifpayPaymentRequest = request.into();
        let response = self
            .client
            .post(format!("{}/api/checkout/session", base_url))
            .header(self.get_apikey_name(), apikey)
            .json(&request)
            .send()
            .await
            .map_err(|_| PaymentGatewayError::RequestFailed)?;
        let status = response.status();

        let body: ArifResponse<ArifInitializeData> = response.json().await.map_err(|e| {
            debug!(?e, "response body parse error");
            PaymentGatewayError::InvalidResponse
        })?;
        debug!(?body, "the response body");

        if !status.is_success() || body.error {
            return Err(PaymentGatewayError::ProviderError {
                message: body.msg,
                errors: serde_json::to_value(&body.data).ok(),
                status_code: status.as_u16(),
            });
        }

        body.data.ok_or_else(|| PaymentGatewayError::ProviderError {
            message: "Provider returned success but data was empty".to_string(),
            errors: None,
            status_code: status.as_u16(),
        })
    }

    pub async fn verify_session(
        &self,
        base_url: String,
        apikey: String,
        session_id: &str,
    ) -> Result<ArifVerifyResponse, PaymentGatewayError> {
        let response = self
            .client
            .get(format!(
                "{}/api/ms/transaction/status/{}",
                base_url, session_id
            ))
            .header(self.get_apikey_name(), apikey)
            // .json(&request)
            .send()
            .await
            .map_err(|_| PaymentGatewayError::RequestFailed)?;

        let status = response.status();

        let body: ArifResponse<ArifVerifyResponse> = response.json().await.map_err(|e| {
            debug!(?e, "response body parse error");
            PaymentGatewayError::InvalidResponse
        })?;
        debug!(?body, "the response body");

        if !status.is_success() || body.error {
            return Err(PaymentGatewayError::ProviderError {
                message: body.msg,
                errors: serde_json::to_value(&body.data).ok(),
                status_code: status.as_u16(),
            });
        }

        body.data.ok_or_else(|| PaymentGatewayError::ProviderError {
            message: "Provider returned success but data was empty".to_string(),
            errors: None,
            status_code: status.as_u16(),
        })
    }

    async fn direct_pay(
        &self,
        base_url: String,
        apikey: String,
        payment_method: PaymentMethod,
        request: ArifpayPaymentRequest,
    ) -> Result<ArifResponse<serde_json::Value>, PaymentGatewayError> {
        let path_segment = self
            .urls
            .get(&payment_method.to_string())
            .ok_or(PaymentGatewayError::UnsupportedPaymentMethod)?;
        let response = self
            .client
            .post(format!("{}{}", base_url, path_segment))
            .header(self.get_apikey_name(), apikey)
            .json(&request)
            .send()
            .await
            .map_err(|_| PaymentGatewayError::RequestFailed)?;

        let body: ArifResponse<serde_json::Value> = response.json().await.map_err(|e| {
            debug!(?e, "response body parse error");
            PaymentGatewayError::InvalidResponse
        })?;
        debug!(?body, "the response body");

        Ok(body)
    }
    pub async fn cancel_session(
        &self,
        base_url: &str,
        apikey: &str,
        session_id: &str,
    ) -> Result<serde_json::Value, PaymentGatewayError> {
        let response = self
            .client
            .delete(format!(
                "{}/v0/checkout/session/cancel/{}",
                base_url, session_id
            ))
            .header(self.get_apikey_name(), apikey)
            .send()
            .await
            .map_err(|_| PaymentGatewayError::RequestFailed)?;
        let status = response.status();

        let body: ArifResponse<serde_json::Value> = response.json().await.map_err(|e| {
            debug!(?e, "response body parse error");
            PaymentGatewayError::InvalidResponse
        })?;
        debug!(?body, "the response body");

        if !status.is_success() || body.error {
            return Err(PaymentGatewayError::ProviderError {
                message: body.msg,
                errors: serde_json::to_value(&body.data).ok(),
                status_code: status.as_u16(),
            });
        }

        body.data.ok_or_else(|| PaymentGatewayError::ProviderError {
            message: "Provider returned success but data was empty".to_string(),
            errors: None,
            status_code: status.as_u16(),
        })
    }

    async fn verify_otp(
        &self,
        base_url: String,
        apikey: String,
        payment_method: String,
        request: VerifyOtpRequest,
    ) -> Result<serde_json::Value, PaymentGatewayError> {
        if payment_method != "kacha" || payment_method != "zamzam" {
            return Err(PaymentGatewayError::UnsupportedPaymentMethod);
        }
        let response = self
            .client
            .post(format!(
                "{}/api/checkout/{}/transfer",
                base_url, payment_method
            ))
            .header(self.get_apikey_name(), apikey)
            .json(&request)
            .send()
            .await
            .map_err(|_| PaymentGatewayError::RequestFailed)?;

        let body: serde_json::Value = response.json().await.map_err(|e| {
            debug!(?e, "response body parse error");
            PaymentGatewayError::InvalidResponse
        })?;
        debug!(?body, "the response body");

        Ok(body)
    }

    async fn send_otp(
        &self,
        base_url: String,
        apikey: String,
        payment_method: String,
        request: OtpRequest,
    ) -> Result<serde_json::Value, PaymentGatewayError> {
        if payment_method != "kacha" || payment_method != "zamzam" {
            return Err(PaymentGatewayError::UnsupportedPaymentMethod);
        }
        let response = self
            .client
            .post(format!(
                "{}/api/checkout/{}/transfer",
                base_url, payment_method
            ))
            .header(self.get_apikey_name(), apikey)
            .json(&request)
            .send()
            .await
            .map_err(|_| PaymentGatewayError::RequestFailed)?;

        let body: serde_json::Value = response.json().await.map_err(|e| {
            debug!(?e, "response body parse error");
            PaymentGatewayError::InvalidResponse
        })?;
        debug!(?body, "the response body");

        Ok(body)
    }
}

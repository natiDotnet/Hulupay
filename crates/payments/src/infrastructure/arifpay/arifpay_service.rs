use crate::application::PaymentGatewayError;
use crate::infrastructure::arifpay::payment_request::{
    OtpRequest, PaymentRequest, VerifyOtpRequest,
};
use crate::infrastructure::arifpay::payment_response::ArifPayInitializeResponse;
use crate::PaymentMethod;
use std::collections::HashMap;
use tracing::debug;

pub struct ArifpayService {
    client: reqwest::Client,
    urls: HashMap<String, String>,
}

impl ArifpayService {
    fn new(client: reqwest::Client) -> Self {
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
        request: PaymentRequest,
    ) -> Result<ArifPayInitializeResponse, PaymentGatewayError> {
        let response = self
            .client
            .post(format!("{}/api/checkout/session", base_url))
            .header(self.get_apikey_name(), apikey)
            .json(&request)
            .send()
            .await
            .map_err(|_| PaymentGatewayError::RequestFailed)?;

        let body: ArifPayInitializeResponse = response.json().await.map_err(|e| {
            debug!(?e, "response body parse error");
            PaymentGatewayError::InvalidResponse
        })?;
        debug!(?body, "the response body");

        Ok(body)
    }

    pub async fn verify_session(
        &self,
        base_url: String,
        apikey: String,
        session_id: String,
    ) -> Result<serde_json::Value, PaymentGatewayError> {
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

        let body: serde_json::Value = response.json().await.map_err(|e| {
            debug!(?e, "response body parse error");
            PaymentGatewayError::InvalidResponse
        })?;
        debug!(?body, "the response body");

        Ok(body)
    }

    async fn direct_pay(
        &self,
        base_url: String,
        apikey: String,
        payment_method: PaymentMethod,
        request: PaymentRequest,
    ) -> Result<ArifPayInitializeResponse, PaymentGatewayError> {
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

        let body: ArifPayInitializeResponse = response.json().await.map_err(|e| {
            debug!(?e, "response body parse error");
            PaymentGatewayError::InvalidResponse
        })?;
        debug!(?body, "the response body");

        Ok(body)
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

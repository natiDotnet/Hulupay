use crate::application::dto::{
    ArifPayBeneficiary, ArifPayInitializeData, ArifPayInitializeRequest, ArifPayInitializeResponse,
    ArifPayItem, InitializePaymentCommand,
};
use crate::application::payment_gateway::{
    PaymentGateway, PaymentInitResult, PaymentVerificationResult,
};
use crate::application::payment_gateway_error::PaymentGatewayError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ArifPayConfig {
    pub api_key: String,
    pub base_url: String,
    pub cancel_url: Option<String>,
    pub success_url: Option<String>,
    pub error_url: Option<String>,
    pub notify_url: String,
    pub account_number: String,
    pub bank: String,
    pub is_test_key: bool,
}

impl ArifPayConfig {
    pub fn new(api_key: String, is_test_key: bool) -> Self {
        let base_url = if is_test_key {
            "https://gateway.sandbox.arifpay.org".to_string()
        } else {
            "https://gateway.arifpay.org".to_string()
        };

        Self {
            api_key,
            is_test_key,
            base_url,
            notify_url: "".to_string(),
            cancel_url: None,
            success_url: None,
            error_url: None,
            account_number: "01320811436100".to_string(),
            bank: "AWINETAA".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct ArifPayProvider {
    client: reqwest::Client,
    config: ArifPayConfig,
}

impl ArifPayProvider {
    pub fn new(config: ArifPayConfig) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("x-arifpay-key", config.api_key.parse().unwrap());

        headers.insert("Content-Type", "application/json".parse().unwrap());

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self { client, config }
    }

    fn build_request(
        &self,
        cmd: &InitializePaymentCommand,
        reference: &str,
    ) -> ArifPayInitializeRequest {
        ArifPayInitializeRequest {
            cancel_url: self
                .config
                .cancel_url
                .clone()
                .map_or_else(|| "".to_string(), |u| u.clone()),
            success_url: self
                .config
                .success_url
                .clone()
                .map_or_else(|| "".to_string(), |u| u.clone()),
            error_url: self
                .config
                .error_url
                .clone()
                .map_or_else(|| "".to_string(), |u| u.clone()),
            notify_url: self.config.notify_url.clone(),
            nonce: reference.to_string(),
            phone: cmd.phone.clone(),
            email: cmd.email.clone(),
            payment_methods: vec!["TELEBIRR".into(), "AWAASH".into()],
            expire_date: chrono::Utc::now()
                .checked_add_signed(chrono::Duration::minutes(30))
                .unwrap()
                .to_rfc3339(),
            items: vec![ArifPayItem {
                name: "Payment".into(),
                quantity: 1,
                price: cmd.amount,
                description: "Merchant payment".into(),
            }],
            beneficiaries: vec![ArifPayBeneficiary {
                account_number: self.config.account_number.clone(),
                bank: self.config.bank.clone(),
                amount: cmd.amount,
            }],
            lang: "EN".into(),
        }
    }
}

#[async_trait::async_trait]
impl PaymentGateway for ArifPayProvider {
    async fn initialize_payment(
        &self,
        cmd: InitializePaymentCommand,
    ) -> Result<PaymentInitResult, PaymentGatewayError> {
        let reference = uuid::Uuid::new_v4().to_string();

        let request = self.build_request(&cmd, &reference);

        let response = self
            .client
            .post(format!("{}/checkout/session", self.config.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|_| PaymentGatewayError::RequestFailed)?;

        let body: ArifPayInitializeResponse = response
            .json()
            .await
            .map_err(|_| PaymentGatewayError::InvalidResponse)?;

        if body.error {
            return Err(PaymentGatewayError::RequestFailed);
        }

        let data = body.data.ok_or(PaymentGatewayError::InvalidResponse)?;

        Ok(PaymentInitResult {
            checkout_url: data.payment_url,
            provider_reference: data.session_id,
        })
    }

    async fn verify_payment(
        &self,
        reference: &str,
    ) -> Result<PaymentVerificationResult, PaymentGatewayError> {
        let response = self
            .client
            .get(format!("{}/verify/{}", self.config.base_url, reference))
            .bearer_auth(&self.config.api_key)
            .send()
            .await
            .map_err(|_| PaymentGatewayError::RequestFailed)?;

        let body: serde_json::Value = response
            .json()
            .await
            .map_err(|_| PaymentGatewayError::InvalidResponse)?;

        Ok(PaymentVerificationResult {
            success: body["status"] == "success",
            provider_reference: reference.to_string(),
        })
    }
}

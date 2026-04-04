use crate::application::dto::{
    ArifPayBeneficiary, ArifPayInitializeData, ArifPayInitializeRequest, ArifPayInitializeResponse,
    ArifPayItem, InitializePaymentCommand,
};
use crate::application::payment_gateway::{
    PaymentGateway, PaymentInitResult, PaymentVerificationResult,
};
use crate::application::payment_gateway_error::PaymentGatewayError;
use crate::application::TransactionRepository;
use crate::Transaction;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    provider_id: Uuid,
    transaction_repository: Arc<dyn TransactionRepository>,
}

impl ArifPayProvider {
    pub fn new(
        config: ArifPayConfig,
        provider_id: Uuid,
        transaction_repository: Arc<dyn TransactionRepository>,
    ) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("x-arifpay-key", config.api_key.parse().unwrap());
        headers.insert("Content-Type", "application/json".parse().unwrap());
        dbg!(&headers.values());

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self {
            client,
            config,
            provider_id,
            transaction_repository,
        }
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
                .checked_add_signed(chrono::Duration::days(1))
                .unwrap()
                .format("%Y-%m-%dT%H:%M:%S")
                .to_string(),
            items: vec![ArifPayItem {
                name: "Payment".into(),
                quantity: 1,
                price: cmd.amount as f64,
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
        let reference = Uuid::new_v4().to_string();

        let request = self.build_request(&cmd, &reference);

        // Serialize and log the request body
        let request_json = serde_json::to_string_pretty(&request).unwrap_or_else(|e| {
            eprintln!("Failed to serialize request: {:?}", e);
            "{}".to_string()
        });
        println!("REQUEST BODY:\n{}", request_json);
        dbg!(&self.config);

        let response = self
            .client
            .post(format!("{}/checkout/session", self.config.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|_| PaymentGatewayError::RequestFailed)?;

        let body: ArifPayInitializeResponse = response.json().await.map_err(|e| {
            dbg!(e);
            PaymentGatewayError::InvalidResponse
        })?;

        if body.error {
            return Err(PaymentGatewayError::RequestFailed);
        }

        let data: ArifPayInitializeData = body
            .data
            .clone()
            .ok_or(PaymentGatewayError::InvalidResponse)?;

        let mut transaction = Transaction::new(
            cmd.merchant_id,
            (data.total_amount * 100_f64) as i64,
            cmd.currency,
            self.provider_id,
            reference,
            serde_json::to_value(body).map_err(|_| PaymentGatewayError::InvalidResponse)?,
        );
        transaction
            .initialize(data.session_id)
            .map_err(|_| PaymentGatewayError::InvalidResponse)?;

        self.transaction_repository
            .create(&transaction)
            .await
            .map_err(|_| PaymentGatewayError::InvalidResponse)?;

        Ok(PaymentInitResult {
            checkout_url: data.payment_url,
            provider_reference: transaction.external_reference.unwrap(),
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

        // Log the response body as string
        let response_text = response.text().await.unwrap_or_else(|e| {
            eprintln!("Failed to read response text: {:?}", e);
            "".to_string()
        });

        let body: serde_json::Value = serde_json::from_str(&response_text)
            .map_err(|_| PaymentGatewayError::InvalidResponse)?;

        Ok(PaymentVerificationResult {
            success: body["status"] == "success",
            provider_reference: reference.to_string(),
        })
    }
}

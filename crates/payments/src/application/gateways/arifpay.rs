use crate::domain;
use crate::domain::{PaymentOrders, PaymentTransactions};
use arif::arifpay::arifpay_service::ArifpayService;
use hulu_core::gateway_response::{Transaction, VerifyResponse};
use hulu_core::payment_gateway::{GatewayResponse, PaymentGateway};
use hulu_core::payment_gateway_error::PaymentGatewayError;
use sea_orm::prelude::async_trait;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

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
    // client: reqwest::Client,
    service: Arc<ArifpayService>,
    // config: ArifPayConfig,
    // provider_id: Uuid,
    db: DatabaseConnection,
}

impl ArifPayProvider {
    pub fn new(service: Arc<ArifpayService>, db: DatabaseConnection) -> Self {
        Self { service, db }
    }

    // fn build_request(
    //     &self,
    //     config: &ArifPayConfig,
    //     cmd: &hulu_core::payment_request::PaymentRequest,
    //     reference: &str,
    //     payment_methods: Vec<String>,
    // ) -> ArifPayInitializeRequest {
    //     ArifPayInitializeRequest {
    //         cancel_url: config.cancel_url.clone().unwrap_or_else(|| "".to_string()),
    //         success_url: config.success_url.clone().unwrap_or_else(|| "".to_string()),
    //         error_url: config.error_url.clone().unwrap_or_else(|| "".to_string()),
    //         notify_url: config.notify_url.clone(),
    //         nonce: cmd.payment.reference.clone(),
    //         phone: cmd.customer.phone.clone(),
    //         email: cmd.customer.email.clone(),
    //         payment_methods,
    //         expire_date: cmd.payment.expire_date.unwrap_or(
    //             chrono::Utc::now()
    //                 .checked_add_signed(chrono::Duration::days(1))
    //                 .unwrap(),
    //         ),
    //         items: vec![ArifPayItem {
    //             name: "Payment".into(),
    //             quantity: 1,
    //             price: cmd.payment.amount,
    //             description: "Merchant payment".into(),
    //         }],
    //         beneficiaries: vec![ArifPayBeneficiary {
    //             account_number: config.account_number.clone(),
    //             bank: config.bank.clone(),
    //             amount: cmd.payment.amount,
    //         }],
    //         lang: "EN".into(),
    //     }
    // }
}

#[async_trait::async_trait]
impl PaymentGateway for ArifPayProvider {
    fn get_name(&self) -> &'static str {
        "ARIFPAY"
    }

    fn get_apikey_name(&self) -> &'static str {
        "x-arifpay-key"
    }

    async fn checkout(
        &self,
        request: &hulu_core::payment_request::PaymentRequest,
        config: serde_json::Value,
    ) -> Result<GatewayResponse, PaymentGatewayError> {
        let request = self
            .change_callback_urls("test", request)
            .map_err(|e| PaymentGatewayError::InternalServerError)?;
        // let my_config = MerchantConfigs::find()
        //     .inner_join(payment_provider::Entity)
        //     .filter(merchant_config::Column::MerchantId.eq(request.merchant_id))
        //     .filter(payment_provider::Column::Code.eq(provider::Provider::ArifPay))
        //     .filter(merchant_config::Column::IsActive.eq(true))
        //     .one(&self.db)
        //     .await
        //     .map_err(|_| PaymentGatewayError::ProviderNotFound)?
        //     .ok_or(PaymentGatewayError::ProviderNotFound)?;

        let arif_config: ArifPayConfig =
            serde_json::from_value(config).map_err(|_| PaymentGatewayError::ProviderNotFound)?;

        self.service
            .create_session(arif_config.base_url, arif_config.api_key, &request)
            .await
            .map(|r| GatewayResponse {
                reference: r.session_id.clone(),
                checkout_url: r.payment_url.clone(),
                row_response: serde_json::to_value(r).ok(),
            })
    }

    async fn webhook(&self, request: Value) -> Result<(), PaymentGatewayError> {
        todo!("handle arifpay webhook")
    }

    async fn verify(
        &self,
        reference: &str,
        config: serde_json::Value,
    ) -> Result<VerifyResponse, PaymentGatewayError> {
        let arif_config: ArifPayConfig =
            serde_json::from_value(config).map_err(|_| PaymentGatewayError::ProviderNotFound)?;

        // let order = PaymentOrders::find()
        //     .filter(domain::payment_order::Column::OrderRef.eq(reference))
        //     .one(&self.db)
        //     .await;

        let (transaction, order) = PaymentTransactions::find()
            .filter(domain::payment_transaction::Column::ProviderTxId.eq(reference))
            .find_also_related(PaymentOrders)
            .one(&self.db)
            .await
            .map_err(|_| PaymentGatewayError::InternalServerError)?
            .ok_or_else(|| PaymentGatewayError::ProviderNotFound)?;

        let order = order.ok_or_else(|| PaymentGatewayError::ProviderNotFound)?;

        self.service
            .verify_session(arif_config.base_url, arif_config.api_key, reference)
            .await
            .map(|v| VerifyResponse {
                callbacks: None,
                metadata: HashMap::new(),
                items: vec![],
                customer: None,
                payment: None,
                beneficiaries: vec![],
                transaction: Transaction {
                    id: v.transaction_id,
                    reference: reference.to_string(),
                    status: transaction.status.to_string(),
                    charge: None,
                    updated_at: transaction.updated_at,
                    created_at: transaction.created_at,
                },
            })
    }

    async fn cancel(&self, reference: &str, config: Value) -> Result<(), PaymentGatewayError> {
        let arif_config: ArifPayConfig =
            serde_json::from_value(config).map_err(|_| PaymentGatewayError::ProviderNotFound)?;
        self.service
            .cancel_session(&arif_config.base_url, &arif_config.api_key, reference)
            .await?;
        Ok(())
    }

    // async fn charge(
    //     &self,
    //     request: DirectPaymentRequest,
    // ) -> Result<PaymentChargeResult, PaymentGatewayError> {
    //     let my_config = MerchantConfigs::find()
    //         .inner_join(payment_provider::Entity)
    //         .filter(merchant_config::Column::MerchantId.eq(request.merchant_id))
    //         .filter(payment_provider::Column::Code.eq(provider::Provider::ArifPay))
    //         .filter(merchant_config::Column::IsActive.eq(true))
    //         .one(&self.db)
    //         .await
    //         .map_err(|_| PaymentGatewayError::ProviderNotFound)?;
    //
    //     let my_config = my_config.ok_or(PaymentGatewayError::ProviderNotFound)?;
    //
    //     let arif_config: ArifPayConfig = serde_json::from_value(my_config.config)
    //         .map_err(|_| PaymentGatewayError::ProviderNotFound)?;
    //     let reference = Uuid::now_v7().to_string();
    //     let cmd = InitializePaymentCommand {
    //         merchant_id: request.merchant_id,
    //         phone: request.phone_number,
    //         email: request.email.unwrap_or("".to_string()),
    //         amount: request.amount,
    //         currency: request.currency,
    //     };
    //
    //     let payment_method = ProviderPaymentMethods::find()
    //         .inner_join(payment_provider::Entity)
    //         .filter(
    //             payment_provider::COLUMN
    //                 .name
    //                 .eq(provider::Provider::ArifPay.to_string()),
    //         )
    //         .filter(
    //             provider_payment_method::COLUMN
    //                 .payment_method_code
    //                 .eq(request.payment_method),
    //         )
    //         .filter(provider_payment_method::COLUMN.is_active.eq(true))
    //         .one(&self.db)
    //         .await
    //         .map_err(|_| PaymentGatewayError::ProviderNotFound)?
    //         .ok_or(PaymentGatewayError::ProviderNotFound)?;
    //
    //     let payload = self.build_request(
    //         &arif_config,
    //         &cmd,
    //         &reference,
    //         vec![payment_method.provider_method_code],
    //     );
    //
    //     debug!(?payload, "the request body");
    //
    //     let response = self
    //         .client
    //         .post(format!(
    //             "{}/api/checkout/{}/transfer/direct",
    //             arif_config.base_url,
    //             payment_method
    //                 .provider_path_segment
    //                 .unwrap_or_else(|| "".to_string()),
    //         ))
    //         .header(self.get_apikey_name(), arif_config.api_key.clone())
    //         .json(&payload)
    //         .send()
    //         .await
    //         .map_err(|_| PaymentGatewayError::RequestFailed)?;
    //
    //     let body: ArifPayInitializeResponse = response.json().await.map_err(|e| {
    //         debug!(?e, "response body parse error");
    //         PaymentGatewayError::InvalidResponse
    //     })?;
    //
    //     if body.error {
    //         debug!(?body, "the response body error is true");
    //         return Err(PaymentGatewayError::RequestFailed);
    //     }
    //
    //     let data: ArifPayInitializeData = serde_json::from_value(
    //         body.data
    //             .clone()
    //             .ok_or(PaymentGatewayError::InvalidResponse)?,
    //     )
    //     .map_err(|e| {
    //         debug!(?e, "response body data field parse error");
    //         PaymentGatewayError::InvalidResponse
    //     })?;
    //
    //     debug!(?body, "the response body");
    //     let row = serde_json::to_value(&body).map_err(|e| {
    //         debug!(?e, "parse error");
    //         PaymentGatewayError::InvalidResponse
    //     })?;
    //
    //     Ok(PaymentChargeResult {
    //         provider_reference: data.session_id,
    //         row_response: row,
    //     })
    // }

    // async fn verify(
    //     &self,
    //     reference: &str,
    // ) -> Result<PaymentVerificationResult, PaymentGatewayError> {
    //     let my_config = MerchantConfigs::find()
    //         .join(
    //             JoinType::InnerJoin,
    //             merchant_config::Relation::PaymentProvider.def(),
    //         )
    //         // .filter(merchant_config::Column::MerchantId.eq(cmd.merchant_id))
    //         .filter(payment_provider::Column::Code.eq(provider::Provider::ArifPay))
    //         .filter(merchant_config::Column::IsActive.eq(true))
    //         .one(&self.db)
    //         .await
    //         .map_err(|_| PaymentGatewayError::ProviderNotFound)?
    //         .ok_or(PaymentGatewayError::ProviderNotFound)?;
    //
    //     let arif_config = serde_json::from_value::<ArifPayConfig>(my_config.config)
    //         .map_err(|_| PaymentGatewayError::ProviderNotFound)?;
    //
    //     let response = self
    //         .client
    //         .get(format!("{}/api/verify/{}", arif_config.base_url, reference))
    //         .bearer_auth(&arif_config.api_key)
    //         .send()
    //         .await
    //         .map_err(|_| PaymentGatewayError::RequestFailed)?;
    //
    //     // Log the response body as string
    //     let body: Value = response.json().await.map_err(|e| {
    //         debug!(?e, "response parse error");
    //         PaymentGatewayError::InvalidResponse
    //     })?;
    //
    //     Ok(PaymentVerificationResult {
    //         success: body["status"] == "success",
    //         provider_reference: reference.to_string(),
    //         row_response: body,
    //     })
    // }
}

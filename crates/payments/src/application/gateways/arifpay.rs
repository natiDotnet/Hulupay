use crate::arifpay::arif_webhook::{ArifTransaction, ArifWebhook};
use crate::arifpay::arifpay_service::ArifpayService;
use crate::domain::payment_order::PaymentOrder;
use crate::domain::payment_provider::PaymentProvider;
use crate::domain::payment_transaction::PaymentTransaction;
use crate::domain::payments::payment_callback::PaymentCallback;
use crate::domain::payments::payment_customer::PaymentCustomer;
use async_trait::async_trait;
use auth::domain::apikey;
use auth::domain::apikey::ApiKey;
use axum::http::StatusCode;
use hulu_core::claims::UserContext;
use hulu_core::gateway_response::VerifyResponse;
use hulu_core::payment_gateway::{GatewayResponse, PaymentGateway, WebhookInfo};
use hulu_core::payment_gateway_error::PaymentGatewayError;
use hulu_core::payment_method::{GatewayProvider, PaymentMethod};
use hulu_core::request_context::RequestContext;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use toasty::Db;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArifPayConfig {
    pub api_key: String,
    pub base_url: String,
    pub sandbox_url: String,
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
        Self {
            api_key,
            is_test_key,
            base_url: "https://gateway.arifpay.net".to_string(),
            sandbox_url: "https://gateway.arifpay.org".to_string(),
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
    service: Arc<ArifpayService>,
    db: Db,
}

impl ArifPayProvider {
    pub fn new(service: Arc<ArifpayService>, db: Db) -> Self {
        Self { service, db }
    }
}

#[async_trait::async_trait]
impl PaymentGateway for ArifPayProvider {
    fn get_config(&self) -> Value {
        serde_json::json!(ArifPayConfig::new(String::new(), true))
    }

    fn get_name(&self) -> GatewayProvider {
        GatewayProvider::Arifpay
    }

    fn get_apikey_name(&self) -> &'static str {
        "x-arifpay-key"
    }

    async fn checkout(
        &self,
        context: &UserContext,
        request: &hulu_core::payment_request::PaymentRequest,
        apikey_header: &str,
        config: serde_json::Value,
    ) -> Result<GatewayResponse, PaymentGatewayError> {
        let _request = self.change_callback_urls("test", request);

        let arif_config: ArifPayConfig =
            serde_json::from_value(config).map_err(|_| PaymentGatewayError::ProviderNotFound)?;

        let apikey = context
            .auth_value
            .as_ref()
            .and_then(|value| value.strip_prefix("Bearer "))
            .unwrap_or(&arif_config.api_key);

        self.service
            .create_session(&arif_config.base_url, apikey, &request)
            .await
            .map(|r| GatewayResponse {
                reference: r.session_id.clone(),
                checkout_url: r.payment_url.clone(),
                row_response: serde_json::to_value(r).ok(),
            })
    }

    async fn webhook(&self, request: &WebhookInfo) -> Result<(), PaymentGatewayError> {
        let mut db = self.db.clone();

        let order = PaymentOrder::filter(
            PaymentOrder::fields()
                .order_ref()
                .eq(request.client_reference.clone()),
        )
        .first()
        .exec(&mut db)
        .await
        .map_err(|_| PaymentGatewayError::ProviderNotFound)?
        .ok_or(PaymentGatewayError::ProviderNotFound)?;

        let callback =
            PaymentCallback::filter(PaymentCallback::fields().payment_order_id().eq(order.id))
                .first()
                .exec(&mut db)
                .await
                .map_err(|_| PaymentGatewayError::ProviderNotFound)?
                .ok_or(PaymentGatewayError::ProviderNotFound)?;

        let customer =
            PaymentCustomer::filter(PaymentCustomer::fields().payment_order_id().eq(order.id))
                .first()
                .exec(&mut db)
                .await
                .map_err(|_| PaymentGatewayError::ProviderNotFound)?
                .ok_or(PaymentGatewayError::ProviderNotFound)?;

        let webhook = ArifWebhook {
            uuid: request.provider_reference.clone(),
            nonce: request.client_reference.clone(),
            session_id: request.provider_reference.clone(),
            transaction_status: request.status.clone().into(),
            transaction: ArifTransaction {
                transaction_id: request.txn_reference.clone(),
                transaction_status: request.status.clone().into(),
            },
            total_amount: request.amount,
            payment_method: request.payment_method.clone().into(),
            notification_url: callback.notify_url.clone(),
            phone: customer.phone,
        };
        self.service
            .send_webhook(&callback.notify_url, webhook)
            .await
    }

    fn webhook_info(&self, webhook: Value) -> Result<WebhookInfo, PaymentGatewayError> {
        self.service.map_webhook(webhook).map(|w| WebhookInfo {
            status: w.transaction_status.into(),
            amount: w.total_amount,
            provider_reference: w.session_id,
            txn_reference: w.transaction.transaction_id,
            client_reference: w.nonce,
            charge: w.total_amount * dec!(2.875) / dec!(100),
            payment_method: w.payment_method.into(),
            received_at: chrono::Utc::now(),
        })
    }

    async fn verify(
        &self,
        reference: &str,
        config: serde_json::Value,
    ) -> Result<VerifyResponse, PaymentGatewayError> {
        let arif_config: ArifPayConfig =
            serde_json::from_value(config).map_err(|_| PaymentGatewayError::ProviderNotFound)?;

        let mut db = self.db.clone();

        let transaction = PaymentTransaction::filter(
            PaymentTransaction::fields()
                .provider_tx_id()
                .eq(reference.to_string()),
        )
        .first()
        .exec(&mut db)
        .await
        .map_err(|_| PaymentGatewayError::InternalServerError)?
        .ok_or(PaymentGatewayError::ProviderNotFound)?;

        let order = PaymentOrder::filter_by_id(transaction.payment_order_id)
            .first()
            .exec(&mut db)
            .await
            .map_err(|_| PaymentGatewayError::InternalServerError)?
            .ok_or(PaymentGatewayError::ProviderNotFound)?;

        self.service
            .verify_session(arif_config.base_url, arif_config.api_key, reference)
            .await
            .map(|v| VerifyResponse {
                id: v.transaction_id,
                reference: reference.to_string(),
                status: transaction.status.to_string(),
                payment_method: PaymentMethod::None,
                charge: order.amount * dec!(2.875) / dec!(100),
                amount: order.amount,
                updated_at: crate::util::to_chrono(transaction.updated_at),
                created_at: crate::util::to_chrono(transaction.created_at),
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
}

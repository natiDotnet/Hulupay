use crate::chapa::chapa_service::ChapaService;
use crate::chapa::chapa_webhook::{self, ChapaPaymentStatus, Customization};
use crate::domain::payment_order::PaymentOrder;
use crate::domain::payments::payment_callback::PaymentCallback;
use crate::domain::payments::payment_customer::PaymentCustomer;
use async_trait::async_trait;
use hulu_core::gateway_response::VerifyResponse;
use hulu_core::payment_gateway::{GatewayResponse, PaymentGateway, WebhookInfo};
use hulu_core::payment_gateway_error::PaymentGatewayError;
use hulu_core::payment_request::PaymentRequest;
use hulu_core::request_context::RequestContext;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use toasty::Db;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapaConfig {
    pub api_key: String,
    pub base_url: String,
    pub sandbox_url: String,
    pub return_url: Option<String>,
    pub notify_url: String,
    pub account_number: String,
    pub bank: String,
    pub is_test_key: bool,
}

impl ChapaConfig {
    pub fn new(api_key: String, is_test_key: bool) -> Self {
        Self {
            api_key,
            is_test_key,
            base_url: "https://gateway.arifpay.net".to_string(),
            sandbox_url: "https://gateway.arifpay.org".to_string(),
            return_url: None,
            notify_url: "".to_string(),
            account_number: "01320811436100".to_string(),
            bank: "AWINETAA".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct ChapaProvider {
    service: Arc<ChapaService>,
    db: Db,
}

impl ChapaProvider {
    pub fn new(service: Arc<ChapaService>, db: Db) -> Self {
        Self { service, db }
    }
}

#[async_trait::async_trait]
impl PaymentGateway for ChapaProvider {
    fn get_config(&self) -> Value {
        serde_json::json!(ChapaConfig::new(String::new(), true))
    }

    fn get_name(&self) -> hulu_core::payment_method::GatewayProvider {
        hulu_core::payment_method::GatewayProvider::Chapa
    }

    fn get_apikey_name(&self) -> &'static str {
        "authorization"
    }

    async fn checkout(
        &self,
        _context: &RequestContext,
        _request: &PaymentRequest,
        _apikey_header: &str,
        _config: Value,
    ) -> Result<GatewayResponse, PaymentGatewayError> {
        todo!()
    }

    async fn webhook(&self, webhook: &WebhookInfo) -> Result<(), PaymentGatewayError> {
        let mut db = self.db.clone();

        let order = PaymentOrder::filter(
            PaymentOrder::fields().order_ref().eq(webhook.client_reference.clone()),
        )
        .first()
        .exec(&mut db)
        .await
        .map_err(|_| PaymentGatewayError::ProviderNotFound)?
        .ok_or(PaymentGatewayError::ProviderNotFound)?;

        let customer = PaymentCustomer::filter(
            PaymentCustomer::fields().payment_order_id().eq(order.id),
        )
        .first()
        .exec(&mut db)
        .await
        .map_err(|_| PaymentGatewayError::ProviderNotFound)?
        .ok_or(PaymentGatewayError::ProviderNotFound)?;

        let callbacks = PaymentCallback::filter(
            PaymentCallback::fields().payment_order_id().eq(order.id),
        )
        .first()
        .exec(&mut db)
        .await
        .map_err(|_| PaymentGatewayError::ProviderNotFound)?
        .ok_or(PaymentGatewayError::ProviderNotFound)?;

        let request = chapa_webhook::ChapaWebhook {
            event: format!(
                "charge.{}",
                ChapaPaymentStatus::from(webhook.status.clone())
            ),
            first_name: customer.name.clone(),
            last_name: customer.name.clone(),
            email: Some(customer.email),
            mobile: customer.phone,
            currency: order.currency,
            amount: order.amount,
            charge: webhook.charge,
            status: webhook.status.clone().into(),
            mode: "live".into(),
            reference: webhook.txn_reference.clone(),
            created_at: crate::util::to_chrono(order.created_at),
            updated_at: crate::util::to_chrono(order.updated_at),
            r#type: "API".into(),
            tx_ref: webhook.client_reference.clone(),
            payment_method: webhook.payment_method.clone().into(),
            customization: Customization {
                title: None,
                description: None,
                logo: None,
            },
            meta: None,
        };
        self.service
            .send_webhook(&callbacks.notify_url, request)
            .await
    }

    fn webhook_info(&self, _webhook: Value) -> Result<WebhookInfo, PaymentGatewayError> {
        todo!()
    }

    async fn verify(
        &self,
        _reference: &str,
        _config: Value,
    ) -> Result<VerifyResponse, PaymentGatewayError> {
        todo!()
    }

    async fn cancel(&self, _reference: &str, _config: Value) -> Result<(), PaymentGatewayError> {
        todo!()
    }
}

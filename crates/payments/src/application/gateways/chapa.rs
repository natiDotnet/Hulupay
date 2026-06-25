use crate::chapa::chapa_service::ChapaService;
use crate::chapa::chapa_webhook::{self, Customization};
use crate::domain::payments::payment_callback;
use crate::domain::{PaymentOrders, payments};
use hulu_core::gateway_response::VerifyResponse;
use hulu_core::payment_gateway::{GatewayResponse, PaymentGateway, WebhookInfo};
use hulu_core::payment_gateway_error::PaymentGatewayError;
use hulu_core::payment_request::PaymentRequest;
use hulu_core::request_context::RequestContext;
use sea_orm::DatabaseConnection;
use sea_orm::sea_query::ValueTuple::One;
use serde_json::Value;
use std::fmt::format;
use std::sync::Arc;

#[derive(Clone)]
pub struct ChapaProvider {
    // client: reqwest::Client,
    service: Arc<ChapaService>,
    // config: ArifPayConfig,
    // provider_id: Uuid,
    db: DatabaseConnection,
}

impl ChapaProvider {
    pub fn new(service: Arc<ChapaService>, db: DatabaseConnection) -> Self {
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
impl PaymentGateway for ChapaProvider {
    fn get_name(&self) -> hulu_core::payment_method::GatewayProvider {
        hulu_core::payment_method::GatewayProvider::Chapa
    }

    fn get_apikey_name(&self) -> &'static str {
        "authorization"
    }

    async fn checkout(
        &self,
        context: &RequestContext,
        request: &PaymentRequest,
        apikey_header: &str,
        config: Value,
    ) -> Result<GatewayResponse, PaymentGatewayError> {
        todo!()
    }

    async fn webhook(&self, webhook: &WebhookInfo) -> Result<(), PaymentGatewayError> {
        let (order, customer, callbacks) =
            PaymentOrders::find_by_order_ref(&webhook.client_reference)
                .find_also_related(payments::payment_customer::Entity)
                .find_also_related(payment_callback::Entity)
                .one(&self.db)
                .await
                .map_err(|_| PaymentGatewayError::ProviderNotFound)?
                .ok_or(PaymentGatewayError::ProviderNotFound)?;
        let customer = customer.ok_or(PaymentGatewayError::ProviderNotFound)?;
        let callbacks = callbacks.ok_or(PaymentGatewayError::ProviderNotFound)?;

        let request = chapa_webhook::ChapaWebhook {
            event: format!("charge.{}", webhook.status),
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
            created_at: order.created_at,
            updated_at: order.updated_at,
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

    fn webhook_info(&self, webhook: Value) -> Result<WebhookInfo, PaymentGatewayError> {
        todo!()
    }

    async fn verify(
        &self,
        reference: &str,
        config: Value,
    ) -> Result<VerifyResponse, PaymentGatewayError> {
        todo!()
    }

    async fn cancel(&self, reference: &str, config: Value) -> Result<(), PaymentGatewayError> {
        todo!()
    }
}

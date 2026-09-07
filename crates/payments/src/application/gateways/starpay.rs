use crate::ArifPayConfig;
use crate::arifpay::arif_webhook::{ArifTransaction, ArifWebhook};
use crate::domain::payment_order::PaymentOrder;
use crate::domain::payments::payment_callback::PaymentCallback;
use crate::domain::payments::payment_customer::PaymentCustomer;
use crate::starpay::starpay_service::StarPayService;
use hulu_core::claims::UserContext;
use hulu_core::gateway_response::VerifyResponse;
use hulu_core::payment_gateway::{GatewayResponse, PaymentGateway, WebhookInfo};
use hulu_core::payment_gateway_error::PaymentGatewayError;
use hulu_core::payment_method::{GatewayProvider, PaymentMethod};
use serde_json::Value;
use std::sync::Arc;
use rust_decimal_macros::dec;
use toasty::Db;

#[derive(Clone)]
pub struct StarPayProvider {
    service: Arc<StarPayService>,
    db: Db,
}

impl StarPayProvider {
    pub fn new(service: Arc<StarPayService>, db: Db) -> Self {
        Self { service, db }
    }
}
#[async_trait::async_trait]
impl PaymentGateway for StarPayProvider {
    fn get_config(&self) -> Value {
        serde_json::json!(ArifPayConfig::new(String::new(), true))
    }

    fn get_name(&self) -> GatewayProvider {
        GatewayProvider::ArifPay
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
            .map(|r| {
                let data = r.data.clone().unwrap();
                GatewayResponse {
                    reference: data.order_id.clone(),
                    checkout_url: data.payment_url.clone(),
                    row_response: serde_json::to_value(r).ok(),
                }
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
            status: w.status.into(),
            amount: w.amount.unwrap_or_else(|| dec!(0)),
            client_reference: w.external_reference_id.clone().unwrap(),
            provider_reference: w.bill_ref_no,
            charge: dec!(0),
            payment_method: PaymentMethod::None,
            txn_reference: w.external_reference_id.clone().unwrap(),
            received_at: w.timestamp.unwrap_or_else(|| jiff::Timestamp::now())
        })
    }

    async fn verify(&self, reference: &str, config: Value) -> Result<VerifyResponse, PaymentGatewayError> {
        todo!()
    }

    async fn cancel(&self, reference: &str, config: Value) -> Result<(), PaymentGatewayError> {
        todo!()
    }

    // async fn verify(
    //     &self,
    //     reference: &str,
    //     config: serde_json::Value,
    // ) -> Result<VerifyResponse, PaymentGatewayError> {
    //     let arif_config: ArifPayConfig =
    //         serde_json::from_value(config).map_err(|_| PaymentGatewayError::ProviderNotFound)?;
    //
    //     let mut db = self.db.clone();
    //
    //     let transaction = PaymentTransaction::filter(
    //         PaymentTransaction::fields()
    //             .provider_tx_id()
    //             .eq(reference.to_string()),
    //     )
    //     .first()
    //     .exec(&mut db)
    //     .await
    //     .map_err(|_| PaymentGatewayError::InternalServerError)?
    //     .ok_or(PaymentGatewayError::ProviderNotFound)?;
    //
    //     let order = PaymentOrder::filter_by_id(transaction.payment_order_id)
    //         .first()
    //         .exec(&mut db)
    //         .await
    //         .map_err(|_| PaymentGatewayError::InternalServerError)?
    //         .ok_or(PaymentGatewayError::ProviderNotFound)?;
    //
    //     self.service
    //         .verify_session(arif_config.base_url, arif_config.api_key, reference)
    //         .await
    //         .map(|v| VerifyResponse {
    //             id: v.transaction_id,
    //             reference: reference.to_string(),
    //             status: transaction.status.to_string(),
    //             payment_method: PaymentMethod::None,
    //             charge: order.amount * dec!(2.875) / dec!(100),
    //             amount: order.amount,
    //             updated_at: transaction.updated_at,
    //             created_at: transaction.created_at,
    //         })
    // }

    // async fn cancel(&self, reference: &str, config: Value) -> Result<(), PaymentGatewayError> {
    //     let arif_config: ArifPayConfig =
    //         serde_json::from_value(config).map_err(|_| PaymentGatewayError::ProviderNotFound)?;
    //     self.service
    //         .cancel_session(&arif_config.base_url, &arif_config.api_key, reference)
    //         .await?;
    //     Ok(())
    // }
}

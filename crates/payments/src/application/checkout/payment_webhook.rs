use crate::ProviderEngine;
use crate::application::checkout::process_webhook::process_webhook;
use crate::domain::payment_order::PaymentOrder;
use crate::domain::payments::merchant_webhook::MerchantWebhook;
use crate::domain::payments::payment_webhook::PaymentWebhook;
use crate::domain::provider::Provider;
use hulu_core::hulu_error::HuluError;
use hulu_core::payment_gateway::{WebhookInfo, WebhookStatus};
use hulu_core::request_context::RequestContext;
use tracing::debug;
use toasty::Db;

#[derive(Clone)]
pub struct PaymentWebhookHandler {
    db: Db,
    payment_engine: ProviderEngine,
}

impl PaymentWebhookHandler {
    pub fn new(db: Db, payment_engine: ProviderEngine) -> Self {
        Self { db, payment_engine }
    }
}

impl PaymentWebhookHandler {
    pub async fn execute(
        &self,
        provider_name: Provider,
        context: &RequestContext,
        payload: serde_json::Value,
    ) -> Result<(), HuluError> {
        let mut db = self.db.clone();
        let provider = self
            .payment_engine
            .get_provider(None, Some(&provider_name))
            .await
            .ok_or(HuluError::ProviderNotFound)?;
        let webhook_info = provider.webhook_info(payload.clone()).map_err(|e| {
            debug!(?e, "webhook info error");
            HuluError::ResponseParseError
        })?;
        debug!(?webhook_info, "webhook info");

        let mut order = PaymentOrder::filter(
            PaymentOrder::fields().order_ref().eq(webhook_info.client_reference.clone()),
        )
        .first()
        .exec(&mut db)
        .await
        .map_err(|_e| HuluError::ConnectionError)?
        .ok_or(HuluError::ResponseParseError)?;

        // Record the incoming payment webhook
        let now = crate::util::now_jiff();
        toasty::create!(PaymentWebhook {
            provider: crate::domain::provider::Provider::from(provider.get_name()),
            status: WebhookStatus::Pending.to_string(),
            payment_order_id: order.id,
            body: payload.clone(),
            headers: serde_json::to_value(&context.headers).unwrap(),
            created_at: now,
            updated_at: now,
        })
        .exec(&mut db)
        .await
        .map_err(|e| {
            debug!(?e, "database error");
            HuluError::ConnectionError
        })?;

        // Record the merchant webhook
        let merchant_status: crate::domain::payment_status::PaymentStatus =
            webhook_info.status.clone().into();
        let merchant_payment_method: crate::domain::payment_method::PaymentMethod =
            webhook_info.payment_method.clone().into();
        toasty::create!(MerchantWebhook {
            status: merchant_status,
            provider_reference: webhook_info.provider_reference.clone(),
            payment_method: merchant_payment_method,
            amount: webhook_info.amount,
            charge: webhook_info.charge,
            client_reference: webhook_info.client_reference.clone(),
            txn_reference: webhook_info.txn_reference.clone(),
            merchant_id: order.merchant_id,
            payment_order_id: order.id,
            created_at: now,
        })
        .exec(&mut db)
        .await
        .map_err(|e| {
            debug!(?e, "database error");
            HuluError::ConnectionError
        })?;

        self.callback(order.id, &order.request_provider, &webhook_info)
            .await?;

        let order_status: crate::domain::payment_status::PaymentStatus =
            webhook_info.status.into();
        toasty::update!(order {
            status: order_status,
        })
        .exec(&mut db)
        .await
        .map_err(|e| {
            debug!(?e, "database error");
            HuluError::ConnectionError
        })?;

        Ok(())
    }

    pub async fn callback(
        &self,
        order_id: uuid::Uuid,
        callback_provider: &Provider,
        webhook: &WebhookInfo,
    ) -> Result<(), HuluError> {
        let provider = self
            .payment_engine
            .get_provider(None, Some(callback_provider))
            .await
            .ok_or(HuluError::ProviderNotFound)?
            .clone();

        let db = self.db.clone();
        let webhook_payload = webhook.clone();

        tokio::spawn(async move {
            process_webhook(db, provider, order_id, webhook_payload).await;
        });

        Ok(())
    }
}

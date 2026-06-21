use crate::application::cache_service::CacheService;
use crate::application::checkout::process_webhook::process_webhook;
use crate::domain::provider::Provider;
use crate::domain::{payments, provider, PaymentOrders};
use crate::{domain, ProviderEngine};
use hulu_core::hulu_error::HuluError;
use hulu_core::payment_gateway::{WebhookInfo, WebhookStatus};
use hulu_core::request_context::RequestContext;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};
use std::sync::Arc;
use tracing::debug;
use uuid::Uuid;
#[derive(Clone)]
pub struct PaymentWebhookHandler {
    db: DatabaseConnection,
    cache: Arc<dyn CacheService>,
    payment_engine: ProviderEngine,
}

impl PaymentWebhookHandler {
    pub fn new(
        db: DatabaseConnection,
        cache: Arc<dyn CacheService>,
        payment_engine: ProviderEngine,
    ) -> Self {
        Self {
            db,
            cache,
            payment_engine,
        }
    }
}

impl PaymentWebhookHandler {
    pub async fn execute(
        &self,
        provider_name: provider::Provider,
        context: &RequestContext,
        payload: serde_json::Value,
    ) -> Result<(), HuluError> {
        let provider = self
            .payment_engine
            .get_provider(None, Some(&provider_name))
            .await
            .ok_or(HuluError::ProviderNotFound)?;
        let webhook_info = provider.webhook_info(payload.clone()).map_err(|e| {
            debug!(?e, "webhook info error");
            HuluError::ResponseParseError
        })?;
        let order = PaymentOrders::find_by_order_ref(&webhook_info.client_reference)
            .one(&self.db)
            .await
            .map_err(|e| HuluError::ConnectionError)?
            .ok_or(HuluError::ResponseParseError)?;

        payments::payment_webhook::ActiveModel {
            status: Set(WebhookStatus::Pending.to_string()),
            payment_order_id: Set(order.id),
            body: Set(payload.clone()),
            headers: Set(serde_json::to_value(&context.headers).unwrap()),
            ..Default::default()
        }
        .insert(&self.db)
        .await
        .map_err(|e| {
            debug!(?e, "database error");
            HuluError::ConnectionError
        })?;

        self.callback(order.id, &order.request_provider, &webhook_info)
            .await?;
        let mut order = order.into_active_model();
        order.status = Set(webhook_info.status.into());
        order.save(&self.db).await.map_err(|e| {
            debug!(?e, "database error");
            HuluError::ConnectionError
        })?;

        Ok(())
    }

    pub async fn callback(
        &self,
        order_id: Uuid,
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

    // async fn callback(
    //     &self,
    //     order_id: Uuid,
    //     callback_provider: &Provider,
    //     webhook: &WebhookInfo,
    // ) -> Result<(), HuluError> {
    //     let provider = self
    //         .payment_engine
    //         .get_provider(None, Some(callback_provider))
    //         .await
    //         .ok_or(HuluError::ProviderNotFound)?
    //         .clone();
    //     let db = self.db.clone();
    //     let webhook = webhook.clone();
    //     tokio::spawn(async move {
    //         let webhook_result = provider.webhook(&webhook).await;
    //         let result = domain::payments::payment_webhook::Entity::find()
    //             .filter(payments::payment_webhook::Column::PaymentOrderId.eq(order_id))
    //             .one(&db)
    //             .await;
    //         match result {
    //             Ok(Some(webhook)) => {
    //                 let mut webhook = webhook.into_active_model();
    //                 match webhook_result {
    //                     Ok(_) => {
    //                         webhook.status = Set(WebhookStatus::Forwarded.to_string());
    //                     }
    //                     Err(e) => {
    //                         webhook.status = Set(WebhookStatus::Failed.to_string());
    //                         webhook.last_error = Set(Some(e.to_string()));
    //                     }
    //                 }
    //                 if let Err(e) = webhook.save(&db).await {
    //                     tracing::error!(?e, "failed to update webhook status");
    //                 }
    //             }
    //             Err(e) => {
    //                 debug!(?e, "webhook error");
    //             }
    //             _ => {
    //                 debug!("webhook not found");
    //             }
    //         }
    //     });
    //     Ok(())
    // }
}

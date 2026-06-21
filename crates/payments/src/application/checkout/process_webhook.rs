use crate::domain::payments::payment_webhook;
use chrono::{Duration, Utc};
use hulu_core::payment_gateway::{PaymentGateway, WebhookInfo, WebhookStatus};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};
use std::sync::Arc;
use uuid::Uuid;

const MAX_RETRIES: u32 = 5;

pub async fn process_webhook(
    db: DatabaseConnection,
    provider: Arc<dyn PaymentGateway>,
    order_id: Uuid,
    payload: WebhookInfo,
) {
    let webhook = match payment_webhook::Entity::find()
        .filter(payment_webhook::Column::PaymentOrderId.eq(order_id))
        .one(&db)
        .await
    {
        Ok(Some(webhook)) => webhook,
        Ok(None) => {
            tracing::warn!(
                %order_id,
                "webhook record not found"
            );
            return;
        }
        Err(err) => {
            tracing::error!(
                ?err,
                %order_id,
                "failed to load webhook"
            );
            return;
        }
    };

    let mut webhook_am = webhook.into_active_model();

    webhook_am.status = Set(WebhookStatus::Processing.to_string());

    // if let Err(err) = webhook_am.save(&db).await {
    //     tracing::error!(?err, "failed to update webhook status");
    //     return;
    // }

    match provider.webhook(&payload).await {
        Ok(_) => {
            webhook_am.status = Set(WebhookStatus::Forwarded.to_string());

            webhook_am.last_error = Set(None);

            // if let Err(err) = webhook_am.save(&db).await {
            //     tracing::error!(?err, "failed to save forwarded webhook");
            // }
        }

        Err(err) => {
            let retries = webhook_am.retry_count.clone().unwrap() + 1;

            webhook_am.retry_count = Set(retries);

            webhook_am.last_error = Set(Some(err.to_string()));

            if retries >= MAX_RETRIES {
                webhook_am.status = Set(WebhookStatus::Failed.to_string());

                tracing::error!(
                    retries,
                    error = %err,
                    "webhook permanently failed"
                );
            } else {
                webhook_am.status = Set(WebhookStatus::Pending.to_string());

                let delay_minutes = 2_i64.pow(retries);

                webhook_am.next_retry_at = Set(Some(Utc::now() + Duration::minutes(delay_minutes)));

                tracing::warn!(
                    retries,
                    error = %err,
                    "webhook scheduled for retry"
                );
            }

            // if let Err(save_err) = webhook_am.save(&db).await {
            //     tracing::error!(?save_err, "failed to save retry state");
            // }
        }
    }
    if let Err(err) = webhook_am.save(&db).await {
        tracing::error!(?err, "failed to update webhook status");
        return;
    }
}

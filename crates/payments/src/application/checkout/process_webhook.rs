use crate::domain::payments::payment_webhook::PaymentWebhook;
use hulu_core::payment_gateway::{PaymentGateway, WebhookInfo, WebhookStatus};
use std::sync::Arc;
use toasty::Db;

const MAX_RETRIES: u32 = 5;

pub async fn process_webhook(
    db: Db,
    provider: Arc<dyn PaymentGateway>,
    order_id: uuid::Uuid,
    payload: WebhookInfo,
) {
    let mut db_mut = db.clone();
    let mut webhook = match PaymentWebhook::filter(
        PaymentWebhook::fields().payment_order_id().eq(order_id),
    )
    .first()
    .exec(&mut db_mut)
    .await
    {
        Ok(Some(w)) => w,
        Ok(None) => {
            tracing::warn!(%order_id, "webhook record not found");
            return;
        }
        Err(err) => {
            tracing::error!(?err, %order_id, "failed to load webhook");
            return;
        }
    };

    let retries = webhook.retry_count;

    // Set processing status
    let _ = toasty::update!(webhook {
        status: WebhookStatus::Processing.to_string(),
    })
    .exec(&mut db_mut)
    .await;

    match provider.webhook(&payload).await {
        Ok(_) => {
            let _ = toasty::update!(webhook {
                status: WebhookStatus::Forwarded.to_string(),
                last_error: None,
            })
            .exec(&mut db_mut)
            .await;
        }
        Err(err) => {
            let new_retries = retries + 1;
            if retries >= MAX_RETRIES {
                let _ = toasty::update!(webhook {
                    status: WebhookStatus::Failed.to_string(),
                    retry_count: new_retries,
                    last_error: Some(err.to_string()),
                })
                .exec(&mut db_mut)
                .await;
                tracing::error!(retries, error = %err, "webhook permanently failed");
            } else {
                let delay_minutes = 2_i64.pow(retries);
                let next_retry = chrono::Utc::now() + chrono::Duration::minutes(delay_minutes);
                let _ = toasty::update!(webhook {
                    status: WebhookStatus::Pending.to_string(),
                    retry_count: new_retries,
                    last_error: Some(err.to_string()),
                    next_retry_at: Some(crate::util::to_jiff(next_retry)),
                })
                .exec(&mut db_mut)
                .await;
                tracing::warn!(retries, error = %err, "webhook scheduled for retry");
            }
        }
    }
}

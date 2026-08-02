use crate::application::payment_gateway::WebhookHandler;
use crate::domain::payment_status::{PaymentStatus, TxStatus};
use crate::domain::{ArifPayment, ArifTransactionStatus};
use crate::domain::payment_order::PaymentOrder;
use crate::domain::payment_transaction::PaymentTransaction;
use async_trait::async_trait;
use hulu_core::payment_gateway_error::PaymentGatewayError;
use serde_json::Value;
use toasty::Db;
use tracing::debug;

#[derive(Clone)]
pub struct ArifWebhook {
    db: Db,
}
impl ArifWebhook {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}
#[async_trait]
impl WebhookHandler for ArifWebhook {
    async fn handle_webhook(&self, webhook: Value) -> Result<(), PaymentGatewayError> {
        let request: ArifPayment = serde_json::from_value(webhook.clone()).map_err(|e| {
            println!("Error parsing webhook: {:?}", e);
            PaymentGatewayError::InvalidResponse
        })?;

        let result = match request.transaction_status {
            ArifTransactionStatus::Success => self.success_handler(webhook).await,
            ArifTransactionStatus::Pending => self.failure_handler(webhook).await,
            ArifTransactionStatus::Failed => self.failure_handler(webhook).await,
        };

        println!("notify the users via rabbitmq ...{:?}", result);
        Ok(())
    }

    async fn success_handler(&self, request: Value) -> Result<(), PaymentGatewayError> {
        self.change_status(request, TxStatus::Success).await
    }

    async fn failure_handler(&self, request: Value) -> Result<(), PaymentGatewayError> {
        self.change_status(request, TxStatus::Failed).await
    }

    async fn change_status(
        &self,
        request: Value,
        status: TxStatus,
    ) -> Result<(), PaymentGatewayError> {
        let webhook: ArifPayment = serde_json::from_value(request.clone())
            .map_err(|_| PaymentGatewayError::InvalidResponse)?;

        let mut db = self.db.clone();

        let mut order = PaymentOrder::filter(
            PaymentOrder::fields().idempotency_key().eq(webhook.nonce.clone()),
        )
        .first()
        .exec(&mut db)
        .await
        .map_err(|err| {
            println!("Error creating order: {:?}", err);
            PaymentGatewayError::InvalidResponse
        })?
        .ok_or(PaymentGatewayError::TransactionNotFound)?;

        let mut txn = db
            .transaction()
            .await
            .map_err(|_| PaymentGatewayError::InvalidResponse)?;

        let now = crate::util::now_jiff();
        let _ = toasty::create!(PaymentTransaction {
            status: status.clone(),
            amount: rust_decimal::Decimal::try_from(webhook.total_amount).unwrap(),
            payment_order_id: order.id,
            provider_tx_id: None,
            direction: crate::domain::payment_status::TxDirection::Charge,
            currency: webhook.payment_method.clone(),
            provider: crate::domain::provider::Provider::ArifPay,
            provider_response: Some(request.clone()),
            updated_at: now,
            created_at: now,
        })
        .exec(&mut txn)
        .await
        .map_err(|_| PaymentGatewayError::InvalidResponse)?;

        let order_status = match status {
            TxStatus::Pending => PaymentStatus::Processing,
            TxStatus::Success => PaymentStatus::Completed,
            TxStatus::Failed => PaymentStatus::Failed,
        };

        toasty::update!(order {
            status: order_status,
            currency: webhook.payment_method.clone(),
        })
        .exec(&mut txn)
        .await
        .map_err(|_| PaymentGatewayError::InvalidResponse)?;

        txn.commit()
            .await
            .map_err(|_| PaymentGatewayError::InvalidResponse)?;

        debug!(?webhook, "Payment successful");
        Ok(())
    }
}

use crate::application::payment_gateway::WebhookHandler;
// use crate::application::PaymentGatewayError;
use crate::domain;
use crate::domain::payment_status::{PaymentStatus, TxStatus};
use crate::domain::{ArifPayment, ArifTransactionStatus, payment_order, payment_transaction};
use async_trait::async_trait;
use hulu_core::payment_gateway_error::PaymentGatewayError;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TransactionTrait};
use serde_json::Value;
use tracing::debug;

#[derive(Clone)]
pub struct ArifWebhook {
    db: DatabaseConnection,
}
impl ArifWebhook {
    pub fn new(db: DatabaseConnection) -> Self {
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
        // Ok(())
        let order = domain::payment_order::Entity::find_by_idempotency_key(&webhook.nonce)
            .one(&self.db)
            .await
            .map_err(|err| {
                println!("Error creating order: {:?}", err);
                PaymentGatewayError::InvalidResponse
            })?;

        match order {
            Some(order) => {
                let txn = self
                    .db
                    .begin()
                    .await
                    .map_err(|_| PaymentGatewayError::InvalidResponse)?;
                payment_transaction::ActiveModel::new_state(
                    &order,
                    &status,
                    rust_decimal::Decimal::try_from(webhook.total_amount).unwrap(),
                    request.clone(),
                )
                .save(&self.db)
                .await
                .map_err(|_| PaymentGatewayError::InvalidResponse)?;

                let mut order: payment_order::ActiveModel = order.into();
                order.status = Set(match status {
                    TxStatus::Pending => PaymentStatus::Processing,
                    TxStatus::Success => PaymentStatus::Completed,
                    TxStatus::Failed => PaymentStatus::Failed,
                });
                order.currency = Set(webhook.payment_method.clone());

                order
                    .save(&self.db)
                    .await
                    .map_err(|_| PaymentGatewayError::InvalidResponse)?;

                txn.commit()
                    .await
                    .map_err(|_| PaymentGatewayError::InvalidResponse)?;
                debug!(?webhook, "Payment successful");
                Ok(())
            }
            None => {
                // Handle case where transaction is not found
                debug!(?webhook.nonce, "Transaction not found");
                Err(PaymentGatewayError::TransactionNotFound)
            }
        }
    }
}

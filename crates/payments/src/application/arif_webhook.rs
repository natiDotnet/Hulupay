use crate::application::payment_gateway::WebhookHandler;
use crate::application::{PaymentGatewayError, TransactionRepository};
use crate::domain::{ArifPayment, ArifTransactionStatus};
use crate::{PaymentMethod, TransactionStatus};
use async_trait::async_trait;
use serde_json::Value;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Clone)]
pub struct ArifWebhook {
    transaction_repository: Arc<dyn TransactionRepository>,
}
impl ArifWebhook {
    pub fn new(transaction_repository: Arc<dyn TransactionRepository>) -> Self {
        Self {
            transaction_repository,
        }
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
        self.change_status(request, TransactionStatus::Completed)
            .await
    }

    async fn failure_handler(&self, request: Value) -> Result<(), PaymentGatewayError> {
        self.change_status(request, TransactionStatus::Failed).await
    }

    async fn change_status(
        &self,
        request: Value,
        status: TransactionStatus,
    ) -> Result<(), PaymentGatewayError> {
        let webhook: ArifPayment = serde_json::from_value(request.clone())
            .map_err(|_| PaymentGatewayError::InvalidResponse)?;

        match self
            .transaction_repository
            .get_by_nonce(&webhook.nonce)
            .await
        {
            Ok(Some(mut transaction)) => {
                // Update transaction status and payment method
                transaction.status = status;
                transaction.webhook_body = Some(request);
                transaction.payment_method =
                    Some(PaymentMethod::from_str(&webhook.payment_method).unwrap_or_default());

                // Save updated transaction to database
                let _ = self.transaction_repository.update(&transaction).await;

                println!("Payment successful: {:?}", webhook);
                Ok(())
            }
            Ok(None) => {
                // Handle case where transaction is not found
                println!("Transaction not found: {:?}", webhook.nonce);
                Err(PaymentGatewayError::InvalidResponse)
            }
            Err(e) => {
                // Handle repository error
                println!("Error getting transaction: {:?}", e);
                Err(PaymentGatewayError::InvalidResponse)
            }
        }
    }
}

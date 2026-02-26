use crate::merchant::repository::TransactionRepository;
use crate::payments::arifpay::dto::InitializePaymentCommand;
use crate::payments::payment_gateway::{PaymentGateway, PaymentInitResult};
use crate::transactions::error::Error;
use domain::transaction::Transaction;
use std::sync::Arc;
use uuid::Uuid;

pub struct CreateTransaction {
    pub repository: Arc<dyn TransactionRepository>,
    pub gateway: Arc<dyn PaymentGateway>,
}

impl CreateTransaction {
    pub async fn execute(
        &self,
        merchant_id: Uuid,
        amount: f64,
        currency: String,
        phone: String,
        email: String,
    ) -> Result<PaymentInitResult, Error> {

        let mut transaction =
            Transaction::new(merchant_id, amount, currency);

        self.repository
            .save(&transaction)
            .await
            .map_err(|_| Error::PersistenceFailed)?;

        let result = self.gateway
            .initialize_payment(
                InitializePaymentCommand {
                    merchant_id,
                    amount,
                    currency: transaction.currency.clone(),
                    phone,
                    email,
                }
            )
            .await
            .map_err(|_| Error::PaymentFailed)?;

        transaction.initialize(
            result.provider_reference.clone(),
        ).map_err(|_| Error::PaymentFailed)?;

        self.repository
            .save(&transaction)
            .await
            .map_err(|_| Error::PersistenceFailed)?;

        Ok(result)
    }
}
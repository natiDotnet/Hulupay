use crate::PaymentMethod;
use crate::domain::error::DomainError;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Initialized,
    AwaitingConfirmation,
    Completed,
    Failed,
    Refunded,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub amount: f64,
    pub currency: String,
    pub payment_method: Option<PaymentMethod>,
    pub status: TransactionStatus,
    pub external_reference: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

impl Transaction {
    pub fn new(merchant_id: Uuid, amount: f64, currency: String) -> Self {
        let now = OffsetDateTime::now_utc();

        Self {
            id: Uuid::new_v4(),
            merchant_id,
            amount,
            currency,
            payment_method: None,
            external_reference: None,
            status: TransactionStatus::Pending,
            created_at: now,
            updated_at: Some(now),
        }
    }
}

impl Transaction {
    pub fn initialize(&mut self, external_reference: String) -> Result<(), DomainError> {
        if self.status != TransactionStatus::Pending {
            return Err(DomainError::InvalidStateTransition);
        }
        self.external_reference = Some(external_reference);
        self.status = TransactionStatus::Initialized;
        self.updated_at = Some(OffsetDateTime::now_utc());
        Ok(())
    }

    pub fn mark_awaiting_confirmation(&mut self) -> Result<(), DomainError> {
        if self.status != TransactionStatus::Initialized {
            return Err(DomainError::InvalidStateTransition);
        }

        self.status = TransactionStatus::AwaitingConfirmation;
        self.updated_at = Some(OffsetDateTime::now_utc());
        Ok(())
    }

    pub fn complete(&mut self) -> Result<(), DomainError> {
        if self.status != TransactionStatus::Initialized {
            return Err(DomainError::InvalidStateTransition);
        }

        self.status = TransactionStatus::Completed;
        self.updated_at = Some(OffsetDateTime::now_utc());
        Ok(())
    }

    pub fn fail(&mut self) -> Result<(), DomainError> {
        if matches!(
            self.status,
            TransactionStatus::Completed | TransactionStatus::Refunded
        ) {
            return Err(DomainError::InvalidStateTransition);
        }

        self.status = TransactionStatus::Failed;
        self.updated_at = Some(OffsetDateTime::now_utc());
        Ok(())
    }
}

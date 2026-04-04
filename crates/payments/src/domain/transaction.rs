use crate::domain::error::DomainError;
use crate::PaymentMethod;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::str::FromStr;
use strum_macros::{Display, EnumString};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display, EnumString)]
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
    pub amount: i64,
    pub currency: String,
    pub payment_method: Option<PaymentMethod>,
    pub provider_id: Uuid,
    pub response: serde_json::Value,
    pub status: TransactionStatus,
    pub nonce: String,
    pub external_reference: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

impl Transaction {
    pub fn new(
        merchant_id: Uuid,
        amount: i64,
        currency: String,
        provider_id: Uuid,
        nonce: String,
        response: serde_json::Value,
    ) -> Self {
        let now = OffsetDateTime::now_utc();

        Self {
            id: Uuid::new_v4(),
            merchant_id,
            amount,
            currency,
            payment_method: None,
            provider_id,
            response,
            nonce,
            external_reference: None,
            status: TransactionStatus::Pending,
            created_at: now,
            updated_at: Some(now),
        }
    }
}

#[derive(FromRow)]
pub struct TransactionRow {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub amount: i64,
    pub currency: String,
    pub payment_method: Option<String>,
    pub provider_id: Uuid,
    pub response: serde_json::Value,
    pub status: String,
    pub nonce: String,
    pub external_reference: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}
impl TryFrom<TransactionRow> for Transaction {
    type Error = DomainError;

    fn try_from(row: TransactionRow) -> Result<Self, Self::Error> {
        Ok(Transaction {
            id: row.id,
            merchant_id: row.merchant_id,
            amount: row.amount,
            currency: row.currency,
            payment_method: row.payment_method.and_then(|s| s.parse().ok()), // 👈 clean conversion
            provider_id: row.provider_id,
            response: row.response,
            status: TransactionStatus::from_str(&row.status)
                .unwrap_or_else(|x| TransactionStatus::Pending),
            nonce: row.nonce,
            external_reference: row.external_reference,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
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

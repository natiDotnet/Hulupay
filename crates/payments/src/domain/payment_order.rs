// domain/src/entities.rs — updated PaymentOrder

use crate::domain::errors::DomainError;
use crate::domain::payment_status::{PaymentStatus, TransitionError};
use crate::domain::provider::Provider;
use chrono::{DateTime, Utc};
use merchant::Merchant;
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PaymentOrder {
    pub id: Uuid,
    pub merchant_id: Uuid, // ← new
    pub customer_id: Uuid,
    pub order_ref: String,
    pub amount: Decimal,
    pub currency: String,
    pub status: PaymentStatus,
    pub provider: Provider,
    pub idempotency_key: String,
    pub retry_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PaymentOrder {
    pub fn new(
        merchant: &Merchant, // takes the full merchant now
        customer_id: Uuid,
        order_ref: String,
        amount: Decimal,
        currency: String,
        provider: Provider,
        idempotency_key: String,
    ) -> Result<Self, DomainError> {
        // merchant.allows_provider(&provider)?; // guard at construction time
        Ok(Self {
            id: Uuid::new_v4(),
            merchant_id: merchant.id,
            customer_id,
            order_ref,
            amount,
            currency,
            status: PaymentStatus::Initiated,
            provider,
            idempotency_key,
            retry_count: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
}

impl PaymentOrder {
    // The only way to change status — enforces state machine
    pub fn transition_to(&mut self, new_status: PaymentStatus) -> Result<(), TransitionError> {
        self.status.transition(&new_status)?;
        self.status = new_status;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn can_retry(&self) -> bool {
        self.status == PaymentStatus::Failed && self.retry_count < self.provider.max_retries()
    }

    pub fn increment_retry(&mut self) {
        self.retry_count += 1;
        self.updated_at = Utc::now();
    }
}

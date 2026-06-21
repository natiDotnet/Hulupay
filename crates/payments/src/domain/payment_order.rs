// domain/src/entities.rs — updated PaymentOrder
use crate::domain;
use crate::domain::payment_status::{PaymentStatus, TransitionError};
use crate::domain::provider::Provider;
use chrono::Utc;
use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;
use sea_orm::prelude::DateTimeUtc;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, Set};
use uuid::Uuid;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "payment_orders")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub customer_id: Uuid,
    #[sea_orm(unique)]
    pub order_ref: String,
    pub amount: Decimal,
    pub currency: String,
    pub status: PaymentStatus,
    pub request_provider: Provider,
    pub provider: Provider,
    #[sea_orm(unique)]
    pub idempotency_key: String,
    pub retry_count: i32,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    #[sea_orm(has_many)]
    pub payment_transaction: HasMany<domain::payment_transaction::Entity>,
    #[sea_orm(has_one)]
    pub payment_callback: HasOne<domain::payments::payment_callback::Entity>,
    #[sea_orm(has_one)]
    pub payment_customer: HasOne<domain::payments::payment_customer::Entity>,
}
impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(Uuid::now_v7()),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..ActiveModelTrait::default()
        }
    }
}

impl ActiveModel {
    pub fn transition_to(&mut self, new_status: PaymentStatus) -> Result<(), TransitionError> {
        self.status.clone().unwrap().transition(&new_status)?;
        self.status = Set(new_status);
        self.updated_at = Set(Utc::now());
        Ok(())
    }
}
// impl PaymentOrder {
//     pub fn new(
//         merchant: &Merchant, // takes the full merchant now
//         customer_id: Uuid,
//         order_ref: String,
//         amount: Decimal,
//         currency: String,
//         provider: Provider,
//         idempotency_key: String,
//     ) -> Result<Self, DomainError> {
//         // merchant.allows_provider(&provider)?; // guard at construction time
//         Ok(Self {
//             id: Uuid::now_v7(),
//             merchant_id: merchant.id,
//             customer_id,
//             order_ref,
//             amount,
//             currency,
//             status: PaymentStatus::Initiated,
//             provider,
//             idempotency_key,
//             retry_count: 0,
//             created_at: Utc::now(),
//             updated_at: Utc::now(),
//         })
//     }
// }

// impl PaymentOrder {
// The only way to change status — enforces state machine
// pub fn transition_to(&mut self, new_status: PaymentStatus) -> Result<(), TransitionError> {
//     self.status.transition(&new_status)?;
//     self.status = new_status;
//     self.updated_at = Utc::now();
//     Ok(())
// }
//
//     pub fn can_retry(&self) -> bool {
//         self.status == PaymentStatus::Failed && self.retry_count < self.provider.max_retries()
//     }
//
//     pub fn increment_retry(&mut self) {
//         self.retry_count += 1;
//         self.updated_at = Utc::now();
//     }
// }

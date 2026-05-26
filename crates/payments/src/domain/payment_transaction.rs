// ---------------------------------------------------------------------------
// PaymentTransaction — what actually happened on the wire, per attempt
// ---------------------------------------------------------------------------
use crate::domain;
use crate::domain::payment_status::{TxDirection, TxStatus};
use crate::domain::provider::Provider;
use chrono::Utc;
use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;
use sea_orm::{DeriveEntityModel, Order, Set};
use uuid::Uuid;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "payment_transactions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub payment_order_id: Uuid,
    pub provider: Provider,
    /// None when the provider call never reached the provider (local timeout)
    pub provider_tx_id: Option<String>,
    pub direction: TxDirection,
    pub amount: Decimal,
    pub currency: String,
    pub status: TxStatus,
    /// Full raw JSON response from the provider — never discard this
    pub provider_response: serde_json::Value,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    #[sea_orm(belongs_to, from = "payment_order_id", to = "id")]
    pub payment_order: HasOne<domain::payment_order::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
impl ActiveModel {
    pub fn new_state(
        order: &domain::payment_order::Model,
        status: &TxStatus,
        amount: Decimal,
        response: serde_json::Value,
    ) -> Self {
        Self {
            id: Set(Uuid::now_v7()),
            updated_at: Set(Utc::now()),
            created_at: Set(Utc::now()),
            payment_order_id: Set(order.id),
            provider: Set(order.provider.clone()),
            currency: Set(order.currency.clone()),
            status: Set(status.clone()),
            amount: Set(amount),
            direction: Set(TxDirection::Charge),
            provider_tx_id: Set(Some(order.order_ref.clone())),
            provider_response: Set(response),
        }
    }
}

// impl PaymentTransaction {
//     pub fn new_charge(order: &PaymentOrder) -> Self {
//         Self {
//             id: Uuid::new_v4(),
//             payment_order_id: order.id,
//             provider: order.provider.clone(),
//             provider_tx_id: None,
//             direction: TxDirection::Charge,
//             amount: order.amount,
//             currency: order.currency.clone(),
//             status: TxStatus::Pending,
//             provider_response: serde_json::Value::Null,
//             created_at: Utc::now(),
//             updated_at: Utc::now(),
//         }
//     }
//
//     pub fn new_refund(order: &PaymentOrder, amount: Decimal) -> Self {
//         Self {
//             id: Uuid::new_v4(),
//             payment_order_id: order.id,
//             provider: order.provider.clone(),
//             provider_tx_id: None,
//             direction: TxDirection::Refund,
//             amount,
//             currency: order.currency.clone(),
//             status: TxStatus::Pending,
//             provider_response: serde_json::Value::Null,
//             created_at: Utc::now(),
//             updated_at: Utc::now(),
//         }
//     }
//
//     pub fn mark_success(&mut self, provider_tx_id: String, response: serde_json::Value) {
//         self.provider_tx_id = Some(provider_tx_id);
//         self.status = TxStatus::Success;
//         self.provider_response = response;
//         self.updated_at = Utc::now();
//     }
//
//     pub fn mark_failed(&mut self, response: serde_json::Value) {
//         self.status = TxStatus::Failed;
//         self.provider_response = response;
//         self.updated_at = Utc::now();
//     }
// }

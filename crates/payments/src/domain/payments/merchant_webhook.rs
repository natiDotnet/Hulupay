use crate::domain::payment_status::PaymentStatus;
use crate::{PaymentMethod, domain};
use chrono::Utc;
use hulu_core::payment_gateway::WebhookInfo;
use sea_orm::entity::prelude::*;
use sea_orm::{DeriveEntityModel, Set};
use uuid::Uuid;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "merchant_webhooks")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub payment_order_id: Uuid,
    pub merchant_id: Uuid,
    pub status: PaymentStatus,
    pub provider_reference: String,
    pub payment_method: PaymentMethod,
    pub amount: Decimal,
    pub charge: Decimal,
    pub client_reference: String,
    pub txn_reference: String,
    pub created_at: DateTimeUtc,

    #[sea_orm(belongs_to, from = "payment_order_id", to = "id")]
    pub payment_order: HasOne<domain::payment_order::Entity>,
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(Uuid::now_v7()),
            ..ActiveModelTrait::default()
        }
    }
}

impl ActiveModel {
    pub fn from_webhook(value: WebhookInfo, merchant_id: Uuid, order_id: Uuid) -> Self {
        Self {
            id: Set(Uuid::now_v7()),
            status: Set(value.status.into()),
            payment_method: Set(value.payment_method.into()),
            amount: Set(value.amount),
            charge: Set(value.charge),
            txn_reference: Set(value.txn_reference),
            provider_reference: Set(value.provider_reference),
            client_reference: Set(value.client_reference),
            merchant_id: Set(merchant_id),
            payment_order_id: Set(order_id),
            created_at: Set(value.received_at),
        }
    }
}

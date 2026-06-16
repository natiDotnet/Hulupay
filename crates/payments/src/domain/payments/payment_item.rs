use crate::domain;
use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;
use sea_orm::DeriveEntityModel;
use uuid::Uuid;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "payment_items")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub payment_order_id: Uuid,
    pub name: String,
    pub description: String,
    pub quantity: u32,
    pub image: Option<String>,
    pub unit_price: Decimal,
    pub total_price: Decimal,
    #[sea_orm(belongs_to, from = "payment_order_id", to = "id")]
    pub payment_order: HasOne<domain::payment_order::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

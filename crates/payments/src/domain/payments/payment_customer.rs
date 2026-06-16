use crate::domain;
use sea_orm::entity::prelude::*;
use sea_orm::Set;
use uuid::Uuid;
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "payment_customers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub payment_order_id: Uuid,

    pub name: String,
    pub email: String,
    pub phone: String,
    pub account_number: Option<String>,
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

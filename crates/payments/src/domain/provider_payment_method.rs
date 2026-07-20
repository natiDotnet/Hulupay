use crate::{PaymentMethod, domain};
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel};
use uuid::Uuid;
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "provider_payment_methods")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub provider_id: Uuid,
    #[sea_orm(indexed)]
    pub payment_method_code: PaymentMethod,
    #[sea_orm(indexed)]
    pub provider_method_code: String,
    // providers url segment
    pub provider_path_segment: Option<String>,
    pub is_active: bool,
    #[sea_orm(belongs_to, from = "provider_id", to = "id")]
    pub provider: HasOne<domain::payment_provider::Entity>,
}
impl ActiveModelBehavior for ActiveModel {}

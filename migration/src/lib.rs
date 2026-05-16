pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20260515_183622_create_merchant;
mod m20260515_191849_seed_default_merchant_and_user;
mod m20260516_123521_create_payment_order_table;
mod m20260516_124726_create_payment_transaction_table;
mod m20260516_140952_payment_enum_create;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20260515_183622_create_merchant::Migration),
            Box::new(m20260515_191849_seed_default_merchant_and_user::Migration),
            Box::new(m20260516_123521_create_payment_order_table::Migration),
            Box::new(m20260516_124726_create_payment_transaction_table::Migration),
            Box::new(m20260516_140952_payment_enum_create::Migration),
        ]
    }
}

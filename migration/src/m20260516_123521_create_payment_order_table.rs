use crate::m20220101_000001_create_table::Users;
use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::sea_orm::DatabaseBackend;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // payment_orders table
        manager
            .create_table(
                Table::create()
                    .table(PaymentOrders::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PaymentOrders::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PaymentOrders::MerchantId).uuid().not_null())
                    .col(ColumnDef::new(PaymentOrders::CustomerId).uuid().not_null())
                    .col(ColumnDef::new(PaymentOrders::OrderRef).string().not_null())
                    .col(
                        ColumnDef::new(PaymentOrders::Amount)
                            .decimal_len(18, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentOrders::Currency)
                            .string_len(3)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentOrders::Status)
                            .string()
                            // .custom(PaymentStatusEnum::Table)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentOrders::Provider)
                            .string()
                            // .custom(ProviderEnum::Table)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentOrders::IdempotencyKey)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentOrders::RetryCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(PaymentOrders::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(PaymentOrders::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-payment-orders-merchant-id")
                    .table(PaymentOrders::Table)
                    .col(PaymentOrders::MerchantId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-payment-orders-customer-id")
                    .table(PaymentOrders::Table)
                    .col(PaymentOrders::CustomerId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-payment-orders-status")
                    .table(PaymentOrders::Table)
                    .col(PaymentOrders::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("uq-payment-orders-idempotency-key")
                    .table(PaymentOrders::Table)
                    .col(PaymentOrders::IdempotencyKey)
                    .unique()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PaymentOrders::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum PaymentOrders {
    Table,
    Id,
    MerchantId,
    CustomerId,
    OrderRef,
    Amount,
    Currency,
    Status,
    Provider,
    IdempotencyKey,
    RetryCount,
    CreatedAt,
    UpdatedAt,
}

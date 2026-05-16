use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // payment_transactions table
        manager
            .create_table(
                Table::create()
                    .table(PaymentTransactions::Table)
                    // .if_not_exists()
                    .col(
                        ColumnDef::new(PaymentTransactions::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PaymentTransactions::PaymentOrderId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentTransactions::Provider)
                            .string()
                            // .custom(ProviderEnum::Table)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentTransactions::ProviderTxId)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(PaymentTransactions::Direction)
                            .string()
                            // .custom(TxDirectionEnum::Table)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentTransactions::Amount)
                            .decimal_len(18, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentTransactions::Currency)
                            .string_len(3)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentTransactions::Status)
                            .string()
                            // .custom(TxStatusEnum::Table)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentTransactions::ProviderResponse)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentTransactions::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(PaymentTransactions::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-payment-transactions-payment-order")
                            .from(
                                PaymentTransactions::Table,
                                PaymentTransactions::PaymentOrderId,
                            )
                            .to(PaymentOrders::Table, PaymentOrders::Id)
                            .on_delete(ForeignKeyAction::NoAction)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-payment-transactions-payment-order-id")
                    .table(PaymentTransactions::Table)
                    .col(PaymentTransactions::PaymentOrderId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-payment-transactions-provider")
                    .table(PaymentTransactions::Table)
                    .col(PaymentTransactions::Provider)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-payment-transactions-status")
                    .table(PaymentTransactions::Table)
                    .col(PaymentTransactions::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-payment-transactions-provider-tx-id")
                    .table(PaymentTransactions::Table)
                    .col(PaymentTransactions::ProviderTxId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("uq-provider-provider-tx-id")
                    .table(PaymentTransactions::Table)
                    .col(PaymentTransactions::Provider)
                    .col(PaymentTransactions::ProviderTxId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PaymentTransactions::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum PaymentTransactions {
    Table,
    Id,
    PaymentOrderId,
    Provider,
    ProviderTxId,
    Direction,
    Amount,
    Currency,
    Status,
    ProviderResponse,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum PaymentOrders {
    Table,
    Id,
}

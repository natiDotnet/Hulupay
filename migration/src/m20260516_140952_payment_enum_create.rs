use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Provider enum
        manager
            .create_type(
                Type::create()
                    .as_enum(ProviderEnum::Table)
                    .values([
                        ProviderEnum::Stripe,
                        ProviderEnum::Chapa,
                        ProviderEnum::ArifPay,
                    ])
                    .to_owned(),
            )
            .await?;

        // Payment status enum
        manager
            .create_type(
                Type::create()
                    .as_enum(PaymentStatusEnum::Table)
                    .values([
                        PaymentStatusEnum::Initiated,
                        PaymentStatusEnum::Pending,
                        PaymentStatusEnum::Processing,
                        PaymentStatusEnum::Completed,
                        PaymentStatusEnum::Failed,
                        PaymentStatusEnum::Cancelled,
                        PaymentStatusEnum::RefundPending,
                        PaymentStatusEnum::Refunded,
                    ])
                    .to_owned(),
            )
            .await?;

        // Provider enum

        // Transaction direction enum
        manager
            .create_type(
                Type::create()
                    .as_enum(TxDirectionEnum::Table)
                    .values([TxDirectionEnum::Charge, TxDirectionEnum::Refund])
                    .to_owned(),
            )
            .await?;

        // Transaction status enum
        manager
            .create_type(
                Type::create()
                    .as_enum(TxStatusEnum::Table)
                    .values([
                        TxStatusEnum::Pending,
                        TxStatusEnum::Success,
                        TxStatusEnum::Failed,
                    ])
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_type(Type::drop().name(PaymentStatusEnum::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(ProviderEnum::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(TxStatusEnum::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(TxDirectionEnum::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum ProviderEnum {
    Table,
    Stripe,
    Chapa,
    ArifPay,
}

#[derive(DeriveIden)]
enum PaymentStatusEnum {
    Table,
    Initiated,
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
    RefundPending,
    Refunded,
}

#[derive(DeriveIden)]
enum TxDirectionEnum {
    Table,
    Charge,
    Refund,
}

#[derive(DeriveIden)]
enum TxStatusEnum {
    Table,
    Pending,
    Success,
    Failed,
}

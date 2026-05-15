use crate::m20220101_000001_create_table::Users;
use crate::m20260515_183622_create_merchant::Merchants;
use argon2::password_hash::phc::SaltString;
use argon2::{Argon2, PasswordHash};
use password_hash::PasswordHasher;
use sea_orm_migration::sea_orm::sqlx::types::chrono::Utc;
use sea_orm_migration::{prelude::*, schema::*};
use uuid::Uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let merchant_id = Uuid::now_v7();
        let admin_id = Uuid::now_v7();
        let stmt = Query::insert()
            .into_table(Merchants::Table)
            .columns([
                Merchants::Id,
                Merchants::Name,
                Merchants::IsActive,
                Merchants::CreatedAt,
                Merchants::UpdatedAt,
            ])
            .values_panic([
                merchant_id.into(),
                "master".into(),
                true.into(),
                Utc::now().into(),
                Some(Utc::now()).into(),
            ])
            .to_owned();
        manager.exec_stmt(stmt).await?;

        let stmt = Query::insert()
            .into_table(Users::Table)
            .columns([
                Users::Id,
                Users::Email,
                Users::PasswordHash,
                Users::MerchantId,
                Users::Role,
                Users::IsActive,
                Users::CreatedAt,
                Users::UpdatedAt,
            ])
            .values_panic([
                admin_id.into(),
                "admin".into(),
                hash_password("admin").into(),
                merchant_id.into(),
                "master_admin".into(),
                true.into(),
                Utc::now().into(),
                Some(Utc::now()).into(),
            ])
            .to_owned();
        manager.exec_stmt(stmt).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // delete admin user
        manager
            .exec_stmt(
                Query::delete()
                    .from_table(Users::Table)
                    .and_where(Expr::col(Users::Email).eq("admin"))
                    .to_owned(),
            )
            .await?;

        // delete master merchant
        manager
            .exec_stmt(
                Query::delete()
                    .from_table(Merchants::Table)
                    .and_where(Expr::col(Merchants::Name).eq("master"))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

fn hash_password(password: &str) -> String {
    let salt = SaltString::generate();
    let argon2 = Argon2::default();

    argon2.hash_password_with_salt(password.as_bytes(), salt.as_bytes())
        .map(|h| h.to_string())
        .unwrap_or_default()
}

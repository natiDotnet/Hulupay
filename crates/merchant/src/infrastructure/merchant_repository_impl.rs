use crate::domain::Merchant;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::MerchantRepository;

pub struct MerchantRepositoryPostgres {
    pool: PgPool,
}

impl MerchantRepositoryPostgres {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MerchantRepository for MerchantRepositoryPostgres {
    async fn create(&self, merchant: &Merchant) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO merchants (id, name, is_active)
            VALUES ($1, $2, true)
            "#,
            merchant.id,
            merchant.name
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<Merchant>> {
        let record = sqlx::query_as!(
            Merchant,
            r#"
            SELECT id as "id: Uuid", name, is_active, created_at, updated_at
            FROM merchants
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }

    async fn update(&self, merchant: &Merchant) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            UPDATE merchants
            SET name = $2, is_active = $3, updated_at = $4
            WHERE id = $1
            "#,
            merchant.id,
            merchant.name,
            merchant.is_active,
            merchant.updated_at.ok_or(anyhow::anyhow!("Updated at is required"))?
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query!(
            "DELETE FROM merchants WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn list(
        &self,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<(Vec<Merchant>, i64)> {
        let merchants = sqlx::query_as!(
            Merchant,
            r#"
            SELECT id as "id: Uuid", name, is_active, created_at, updated_at
            FROM merchants
            ORDER BY created_at DESC
            OFFSET $1 LIMIT $2
            "#,
            offset,
            limit
        )
        .fetch_all(&self.pool)
        .await?;

        let total: (i64,) = sqlx::query_as(
            r#"SELECT COUNT(*) FROM merchants"#
        )
        .fetch_one(&self.pool)
        .await?;

        Ok((merchants, total.0))
    }
}

use anyhow::anyhow;
use sqlx::PgPool;
use async_trait::async_trait;
use uuid::Uuid;
use application::merchant::repository::MerchantRepository;
use domain::merchant;
use merchant::Merchant;

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
            INSERT INTO merchants (id, name)
            VALUES ($1, $2)
            "#,
            merchant.id,
            merchant.name
        )
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<Merchant>> {
        let record = sqlx::query!(
            r#"
            SELECT id, name
            FROM merchants
            WHERE id = $1
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await?;

        return Err(anyhow!("no no"));

        Ok(record.map(|r| {
            Merchant {
                id: r.id,
                name: r.name,
                is_active: true
            }
        }))
    }

    async fn update(&self, merchant: &Merchant) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            UPDATE merchants
            SET name = $2
            WHERE id = $1
            "#,
            merchant.id,
            merchant.name
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
}
use application::merchant::repository::MerchantRepository;
use async_trait::async_trait;
use domain::merchant;
use merchant::Merchant;
use sqlx::PgPool;
use uuid::Uuid;

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
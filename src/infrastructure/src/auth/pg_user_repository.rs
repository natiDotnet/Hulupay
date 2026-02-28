use application::auth::user_repository::UserRepository;
use async_trait::async_trait;
use domain::user::User;
use sqlx::PgPool;

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {

    async fn exists_by_email(&self, email: &str) -> anyhow::Result<bool> {
        let exists: (bool,) = sqlx::query_as(
            "SELECT EXISTS (SELECT 1 FROM users WHERE email = $1)"
        )
            .bind(email)
            .fetch_one(&self.pool)
            .await?;
            // .map_err(|_| AppError::Internal)?;

        Ok(exists.0)
    }

    async fn insert(&self, user: &User) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO users (id, email, password_hash, role, merchant_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#
        )
            .bind(user.id)
            .bind(&user.email)
            .bind(&user.password_hash)
            .bind(format!("{:?}", user.role))
            .bind(user.merchant_id)
            .bind(user.created_at)
            .bind(user.updated_at)
            .execute(&self.pool)
            .await?;
            // .map_err(|_| AppError::Internal)?;

        Ok(())
    }

    async fn find_by_email(&self, email: &str) -> anyhow::Result<Option<User>> {
        let row = sqlx::query!(
            r#"select id, email, password_hash, role, merchant_id, created_at, updated_at
                from users where email = $1
                "#, email)
            .fetch_optional(&self.pool)
            .await?;
            // .map_err(|_| AppError::Internal)?;
        Ok(row.map(|r| {
            User {
                id: r.id,
                email: r.email,
                password_hash: r.password_hash,
                role: r.role,
                merchant_id: r.merchant_id.unwrap_or_default(),
                created_at: r.created_at,
                updated_at: None
            }
        }))
    }
}
use crate::application::{PaymentProviderConfigRepository, PaymentProviderRepository};
use crate::domain::{nati, PaymentProviderConfig, Provider};
use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sqlx::PgPool;
use uuid::Uuid;

pub struct PgPaymentProviderRepository {
    pool: PgPool,
    db: DatabaseConnection,
}

impl PgPaymentProviderRepository {
    pub fn new(pool: PgPool, db: DatabaseConnection) -> Self {
        Self { pool, db }
    }
}

#[async_trait]
impl PaymentProviderRepository for PgPaymentProviderRepository {
    async fn create(&self, provider: &Provider) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO payment_providers (id, code, name, is_active, created_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            provider.id,
            provider.code,
            provider.name,
            provider.is_active,
            provider.created_at
        )
        .execute(&self.pool)
        .await?;
        let id = Uuid::new_v4();

        let nati: Option<nati::Model> = nati::Entity::find_by_id(id).one(&self.db).await?;

        Ok(())
    }

    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<Provider>> {
        let record = sqlx::query_as!(
            Provider,
            r#"
            SELECT 
                id as "id: Uuid",
                code,
                name,
                is_active,
                created_at
            FROM payment_providers
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }

    async fn get_by_code(&self, code: &str) -> anyhow::Result<Option<Provider>> {
        let record = sqlx::query_as!(
            Provider,
            r#"
            SELECT 
                id as "id: Uuid",
                code,
                name,
                is_active,
                created_at
            FROM payment_providers
            WHERE code = $1
            "#,
            code
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }

    async fn update(&self, provider: &Provider) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            UPDATE payment_providers
            SET code = $2, name = $3, is_active = $4
            WHERE id = $1
            "#,
            provider.id,
            provider.code,
            provider.name,
            provider.is_active
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query!("DELETE FROM payment_providers WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn list(&self, offset: i64, limit: i64) -> anyhow::Result<(Vec<Provider>, i64)> {
        let providers = sqlx::query_as!(
            Provider,
            r#"
            SELECT 
                id as "id: Uuid",
                code,
                name,
                is_active,
                created_at
            FROM payment_providers
            ORDER BY created_at DESC
            OFFSET $1 LIMIT $2
            "#,
            offset,
            limit
        )
        .fetch_all(&self.pool)
        .await?;

        let total: (i64,) = sqlx::query_as(r#"SELECT COUNT(*) FROM payment_providers"#)
            .fetch_one(&self.pool)
            .await?;

        Ok((providers, total.0))
    }

    async fn list_active(&self) -> anyhow::Result<Vec<Provider>> {
        let providers = sqlx::query_as!(
            Provider,
            r#"
            SELECT 
                id as "id: Uuid",
                code,
                name,
                is_active,
                created_at
            FROM payment_providers
            WHERE is_active = true
            ORDER BY created_at desc
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(providers)
    }
}

pub struct PgPaymentProviderConfigRepository {
    pool: PgPool,
}

impl PgPaymentProviderConfigRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PaymentProviderConfigRepository for PgPaymentProviderConfigRepository {
    async fn create(&self, config: &PaymentProviderConfig) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO payment_provider_configs 
                (id, merchant_id, provider_id, is_test_mode, config, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            config.id,
            config.merchant_id,
            config.provider_id,
            config.is_test_mode,
            config.config,
            config.is_active,
            config.created_at,
            config.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<PaymentProviderConfig>> {
        let record = sqlx::query_as!(
            PaymentProviderConfig,
            r#"
            SELECT 
                id as "id: Uuid",
                merchant_id as "merchant_id: Uuid",
                provider_id as "provider_id: Uuid",
                is_test_mode,
                config as "config: serde_json::Value",
                is_active,
                created_at,
                updated_at
            FROM payment_provider_configs
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }

    async fn get_by_merchant_and_provider(
        &self,
        merchant_id: Uuid,
        provider_id: Uuid,
        is_test_mode: bool,
    ) -> anyhow::Result<Option<PaymentProviderConfig>> {
        let record = sqlx::query_as!(
            PaymentProviderConfig,
            r#"
            SELECT 
                id as "id: Uuid",
                merchant_id as "merchant_id: Uuid",
                provider_id as "provider_id: Uuid",
                is_test_mode,
                config as "config: serde_json::Value",
                is_active,
                created_at,
                updated_at
            FROM payment_provider_configs
            WHERE merchant_id = $1 
              AND provider_id = $2 
              AND is_test_mode = $3
            "#,
            merchant_id,
            provider_id,
            is_test_mode
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }

    async fn update(&self, config: &PaymentProviderConfig) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            UPDATE payment_provider_configs
            SET merchant_id = $2, 
                provider_id = $3, 
                is_test_mode = $4, 
                config = $5, 
                is_active = $6,
                updated_at = $7
            WHERE id = $1
            "#,
            config.id,
            config.merchant_id,
            config.provider_id,
            config.is_test_mode,
            config.config,
            config.is_active,
            config.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query!("DELETE FROM payment_provider_configs WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn list_by_merchant(
        &self,
        merchant_id: Uuid,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<(Vec<PaymentProviderConfig>, i64)> {
        let configs = sqlx::query_as!(
            PaymentProviderConfig,
            r#"
            SELECT 
                id as "id: Uuid",
                merchant_id as "merchant_id: Uuid",
                provider_id as "provider_id: Uuid",
                is_test_mode,
                config as "config: serde_json::Value",
                is_active,
                created_at,
                updated_at
            FROM payment_provider_configs
            WHERE merchant_id = $1
            ORDER BY created_at DESC
            OFFSET $2 LIMIT $3
            "#,
            merchant_id,
            offset,
            limit
        )
        .fetch_all(&self.pool)
        .await?;

        let total: (i64,) = sqlx::query_as(
            r#"SELECT COUNT(*) FROM payment_provider_configs WHERE merchant_id = $1"#,
        )
        .bind(merchant_id)
        .fetch_one(&self.pool)
        .await?;

        Ok((configs, total.0))
    }

    async fn list_active_by_merchant(
        &self,
        merchant_id: Uuid,
    ) -> anyhow::Result<Vec<PaymentProviderConfig>> {
        let configs = sqlx::query_as!(
            PaymentProviderConfig,
            r#"
            SELECT 
                id as "id: Uuid",
                merchant_id as "merchant_id: Uuid",
                provider_id as "provider_id: Uuid",
                is_test_mode,
                config as "config: serde_json::Value",
                is_active,
                created_at,
                updated_at
            FROM payment_provider_configs
            WHERE merchant_id = $1 AND is_active = true
            ORDER BY created_at DESC
            "#,
            merchant_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(configs)
    }

    async fn list_active_by_provider_code(
        &self,
        merchant_id: Uuid,
        provider_code: &str,
    ) -> anyhow::Result<Vec<PaymentProviderConfig>> {
        let configs = sqlx::query_as!(
            PaymentProviderConfig,
            r#"
            SELECT 
                cfg.id as "id: Uuid",
                cfg.merchant_id as "merchant_id: Uuid",
                cfg.provider_id as "provider_id: Uuid",
                cfg.is_test_mode,
                cfg.config,
                cfg.is_active,
                cfg.created_at,
                cfg.updated_at
            FROM payment_provider_configs cfg
            INNER JOIN payment_providers prv ON cfg.provider_id = prv.id
            WHERE prv.code = $1 AND merchant_id = $2 AND cfg.is_active = true
            ORDER BY cfg.created_at DESC
            "#,
            provider_code,
            merchant_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(configs)
    }
}

use anyhow::Result;
use application::encryption::EncryptionService;
use application::payments::arifpay::config_request_dto::ArifPayConfigRequest;
use domain::arifpay::config::ArifPayConfig;
use sqlx::types::time::OffsetDateTime;
use sqlx::PgPool;
use uuid::Uuid;

pub struct ProviderConfigService<E: EncryptionService> {
    pool: PgPool,
    encryption: E,
}

impl<E: EncryptionService> ProviderConfigService<E> {
    pub fn new(pool: PgPool, encryption: E) -> Self {
        Self { pool, encryption }
    }

    pub async fn set_arifpay_config(
        &self,
        request: ArifPayConfigRequest,
    ) -> Result<()> {

        let provider_id: Uuid = sqlx::query_scalar(
            "SELECT id FROM payment_providers WHERE code = 'ARIFPAY'"
        )
            .fetch_one(&self.pool)
            .await?;

        let config = ArifPayConfig::new(request.api_key, request.is_test_key);

        let json = serde_json::to_string(&config)?;
        let encrypted = self.encryption.encrypt(&json)?;

        sqlx::query(
            r#"
                INSERT INTO public.payment_provider_configs
                (id, merchant_id, provider_id, is_test_mode, config, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                ON CONFLICT (merchant_id, provider_id)
                DO UPDATE SET
                    is_test_mode = $4,
                    config = $5,
                    updated_at = $7
                "#
        )
        .bind(Uuid::new_v4())          // id
        .bind("merchant_id")            // merchant_id
        .bind(provider_id)            // provider_id
        .bind(request.is_test_key)           // $4
        .bind(encrypted)
        .bind(OffsetDateTime::now_utc())
        .bind(OffsetDateTime::now_utc())
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
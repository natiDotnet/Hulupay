use crate::application::TransactionRepository;
use crate::domain::DomainError;
use crate::PaymentMethod;
use crate::Transaction;
use crate::TransactionStatus;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

pub struct PgTransactionRepository {
    pool: PgPool,
}

impl PgTransactionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TransactionRepository for PgTransactionRepository {
    async fn create(&self, transaction: &Transaction) -> Result<(), DomainError> {
        sqlx::query!(
            r#"
            INSERT INTO transactions (
                id, merchant_id, amount, currency, payment_method, provider_id, response, status, external_reference, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
           "#,
            transaction.id,
            transaction.merchant_id,
            transaction.amount,
            transaction.currency,
            transaction.payment_method.as_ref().map(|pm| pm.to_string()),
            transaction.provider_id,
            transaction.response,
            transaction.status.to_string(),
            transaction.external_reference,
            transaction.created_at,
            transaction.updated_at
)
        .execute(&self.pool)
        .await?
        .rows_affected();

        Ok(())
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Transaction>, DomainError> {
        let record = sqlx::query_as!(
            Transaction,
            r#"
SELECT
                id,
                merchant_id,
                amount,
                currency,
                payment_method as "payment_method: PaymentMethod",
                provider_id,
                response,
                status as "status: TransactionStatus",
                nonce,
                external_reference,
                created_at,
                updated_at
            FROM transactions
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(record)
    }

    async fn get_by_nonce(&self, nonce: &str) -> Result<Option<Transaction>, DomainError> {
        let record = sqlx::query_as!(
            Transaction,
            r#"
                SELECT
                    id,
                    merchant_id,
                    amount,
                    currency,
                    payment_method as "payment_method: PaymentMethod",
                    provider_id,
                    response,
                    status as "status: TransactionStatus",
                    nonce,
                    external_reference,
                    created_at,
                    updated_at
                FROM transactions
                WHERE id = $1
            "#,
            Uuid::parse_str(nonce).unwrap()
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(record)
    }

    async fn update(&self, transaction: &Transaction) -> Result<(), DomainError> {
        sqlx::query!(
            r#"
            UPDATE transactions
            SET merchant_id = $2, amount = $3, currency = $4, payment_method = $5, provider_id = $6, response =$7, status = $8, external_reference = $9, updated_at = $10
            WHERE id = $1
            "#,
            transaction.id,
            transaction.merchant_id,
            transaction.amount,
            transaction.currency,
            transaction.payment_method.as_ref().map(|pm| pm.to_string()),
           transaction.provider_id,
            transaction.response,
            transaction.status.to_string(),
            transaction.external_reference,
            transaction.updated_at
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        Ok(())
    }
}

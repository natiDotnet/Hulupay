use crate::domain::payments::merchant_webhook::MerchantWebhook;
use anyhow::anyhow;
use merchant::application::ApplicationError;
use toasty::Db;
use uuid::Uuid;

#[derive(Clone)]
pub struct ListMerchantWebhooks {
    db: Db,
}

impl ListMerchantWebhooks {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        merchant_id: Uuid,
        page: u64,
        page_size: u64,
    ) -> anyhow::Result<PaginatedResponse<MerchantWebhook>> {
        let mut db = self.db.clone();

        let all = MerchantWebhook::filter(MerchantWebhook::fields().merchant_id().eq(merchant_id))
            .exec(&mut db)
            .await
            .map_err(|e| ApplicationError::Internal(anyhow!(e)))?;
        let total = all.len() as u64;

        let offset = if page > 1 { (page - 1) * page_size } else { 0 };
        let items =
            MerchantWebhook::filter(MerchantWebhook::fields().merchant_id().eq(merchant_id))
                .order_by(MerchantWebhook::fields().created_at().desc())
                .limit(page_size as usize)
                .offset(offset as usize)
                .exec(&mut db)
                .await
                .map_err(|e| ApplicationError::Internal(anyhow!(e)))?;

        Ok(PaginatedResponse {
            items,
            total,
            page,
            page_size,
        })
    }
}

#[derive(Debug, Clone)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}

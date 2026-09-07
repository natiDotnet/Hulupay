use crate::domain::PaymentProviderConfig;
use crate::domain::merchant_config::MerchantConfig;
use toasty::Db;
use uuid::Uuid;

#[derive(Clone)]
pub struct ListPaymentProviderConfigs {
    db: Db,
}

impl ListPaymentProviderConfigs {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        merchant_id: Uuid,
        page: u64,
        page_size: u64,
    ) -> anyhow::Result<PaginatedResponse<PaymentProviderConfig>> {
        let mut db = self.db.clone();

        let all = MerchantConfig::filter(MerchantConfig::fields().merchant_id().eq(merchant_id))
            .exec(&mut db)
            .await?;
        let total = all.len() as u64;

        let offset = if page > 1 { (page - 1) * page_size } else { 0 };
        let items = MerchantConfig::filter(MerchantConfig::fields().merchant_id().eq(merchant_id))
            .order_by(MerchantConfig::fields().created_at().desc())
            .limit(page_size as usize)
            .offset(offset as usize)
            .exec(&mut db)
            .await?
            .into_iter()
            .map(|p| PaymentProviderConfig {
                id: p.id,
                config: p.config,
                provider_id: p.provider_id,
                merchant_id: p.merchant_id,
                is_active: p.is_active,
                priority: p.priority,
                environment: p.environment,
                created_at: p.created_at,
                updated_at: p.updated_at,
                is_default: p.is_default,
            })
            .collect();

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

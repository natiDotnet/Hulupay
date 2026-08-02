use crate::application::dto::MerchantResponse;
use crate::application::error::ApplicationError;
use crate::domain::merchant::Merchant;
use anyhow::anyhow;
use serde::Serialize;
use toasty::Db;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub page: u64,
    pub page_size: u64,
    pub total: u64,
}

#[derive(Clone)]
pub struct ListMerchants {
    db: Db,
}

impl ListMerchants {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        page: u64,
        page_size: u64,
    ) -> Result<PaginatedResponse<MerchantResponse>, ApplicationError> {
        let mut db = self.db.clone();

        // Count total merchants
        let total = Merchant::all().exec(&mut db).await.map_err(|e| ApplicationError::Internal(anyhow!(e)))?.len() as u64;

        // Fetch page with offset pagination (keeps ?page= API contract)
        let offset = if page > 1 { (page - 1) * page_size } else { 0 };
        let items = Merchant::all()
            .order_by(Merchant::fields().id().desc())
            .limit(page_size as usize)
            .offset(offset as usize)
            .exec(&mut db)
            .await
            .map_err(|e| ApplicationError::Internal(anyhow!(e)))?
            .into_iter()
            .map(|m| MerchantResponse {
                id: m.id,
                name: m.name,
                email: m.email,
                phone: m.phone,
                website: m.website,
                status: m.status,
                is_active: m.is_active,
            })
            .collect();

        Ok(PaginatedResponse {
            items,
            page,
            page_size,
            total,
        })
    }
}

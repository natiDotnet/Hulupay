use crate::domain;
use crate::domain::Provider;
use toasty::Db;

#[derive(Clone)]
pub struct ListPaymentProviders {
    db: Db,
}

impl ListPaymentProviders {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        page: u64,
        page_size: u64,
    ) -> anyhow::Result<PaginatedResponse<Provider>> {
        let mut db = self.db.clone();

        let all = domain::payment_provider::PaymentProvider::all()
            .exec(&mut db)
            .await?;
        let total = all.len() as u64;

        let offset = if page > 1 { (page - 1) * page_size } else { 0 };
        let items = domain::payment_provider::PaymentProvider::all()
            .order_by(
                domain::payment_provider::PaymentProvider::fields()
                    .id()
                    .desc(),
            )
            .limit(page_size as usize)
            .offset(offset as usize)
            .exec(&mut db)
            .await?
            .into_iter()
            .map(|p| Provider {
                id: p.id,
                code: p.code,
                name: p.name,
                logo: p.logo,
                is_active: p.is_active,
                created_at: crate::util::to_chrono(p.created_at),
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

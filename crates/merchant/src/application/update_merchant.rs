use crate::application::repository::MerchantRepository;
use std::sync::Arc;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone)]
pub struct UpdateMerchant {
    pub repository: Arc<dyn MerchantRepository>,
}

impl UpdateMerchant {
    pub fn new(repository: Arc<dyn MerchantRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: Uuid, name: String, is_active: bool) -> anyhow::Result<()> {
        let mut merchant = self.repository.get_by_id(id).await?
            .ok_or_else(|| anyhow::anyhow!("Merchant not found"))?;

        merchant.name = name;
        merchant.is_active = is_active;
        merchant.updated_at = Some(OffsetDateTime::now_utc());

        self.repository.update(&merchant).await?;
        Ok(())
    }
}

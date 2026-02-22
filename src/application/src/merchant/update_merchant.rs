use anyhow::anyhow;
use uuid::Uuid;
use domain::merchant::Merchant;
use crate::merchant::dto::CreateMerchantRequest;
use crate::merchant::repository::MerchantRepository;

pub struct UpdateMerchant<R: MerchantRepository> {
    pub repository: R,
}

impl<R: MerchantRepository> UpdateMerchant<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(& mut self, id: Uuid, name: String, is_active: bool)
    -> anyhow::Result<()> {
        let mut merchant = self.repository.get_by_id(id).await?
            .ok_or_else(|| anyhow!("Merchant not found"))?;

        merchant.name = name;
        merchant.is_active = is_active;

        self.repository.update(merchant).await?;
        Ok(())
    }
}
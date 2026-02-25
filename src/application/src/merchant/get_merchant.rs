use crate::merchant::dto::MerchantResponse;
use crate::merchant::repository::MerchantRepository;
use std::sync::Arc;
use anyhow::anyhow;
use uuid::Uuid;
use domain::merchant::Merchant;
use crate::merchant_error::MerchantError;

#[derive(Clone)]
pub struct GetMerchant {
    repository: Arc<dyn MerchantRepository>,
}
enum  AppError {
    NotFount(String)
}
impl GetMerchant {
    pub fn new(repository: Arc<dyn MerchantRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: Uuid) -> Result<MerchantResponse, MerchantError> {
        let merchant = self.repository.get_by_id(id).await
            .map_err(|err| MerchantError::Internal(err))?;

        match merchant {
            None => Err(MerchantError::NotFound(format!("merchant not found with id {}", id))),
            Some(mer) =>
                Ok(MerchantResponse {
                    id: mer.id,
                    name: mer.name,
                    is_active: mer.is_active,
                })
        }
    }
}
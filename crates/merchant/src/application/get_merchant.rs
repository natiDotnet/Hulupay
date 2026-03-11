use crate::application::dto::MerchantResponse;
use crate::application::error::ApplicationError;
use crate::application::repository::MerchantRepository;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct GetMerchant {
    repository: Arc<dyn MerchantRepository>,
}

impl GetMerchant {
    pub fn new(repository: Arc<dyn MerchantRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: Uuid) -> Result<MerchantResponse, ApplicationError> {
        let merchant = self.repository.get_by_id(id).await?;

        match merchant {
            None => Err(ApplicationError::NotFound(format!(
                "merchant not found with id {}",
                id
            ))),
            Some(mer) => Ok(MerchantResponse {
                id: mer.id,
                name: mer.name,
                is_active: mer.is_active,
            }),
        }
    }
}

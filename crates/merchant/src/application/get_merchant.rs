use crate::application::dto::MerchantResponse;
use crate::application::error::ApplicationError;
use crate::domain::merchant::Merchant;
use anyhow::anyhow;
use toasty::Db;
use uuid::Uuid;

#[derive(Clone)]
pub struct GetMerchant {
    db: Db,
}

impl GetMerchant {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn execute(&self, id: Uuid) -> Result<MerchantResponse, ApplicationError> {
        let mut db = self.db.clone();
        let merchant = match Merchant::filter_by_id(id).first().exec(&mut db).await {
            Ok(Some(m)) => m,
            Ok(None) => {
                return Err(ApplicationError::NotFound(
                    "Merchant not found with given id".to_string(),
                ))
            }
            Err(e) => return Err(ApplicationError::Internal(anyhow!(e))),
        };

        Ok(MerchantResponse {
            id: merchant.id,
            name: merchant.name,
            email: merchant.email,
            phone: merchant.phone,
            website: merchant.website,
            status: merchant.status,
            is_active: merchant.is_active,
        })
    }
}

use crate::application::ApplicationError;
use crate::application::dto::UpdateMerchantRequest;
use crate::domain::merchant::Merchant;
use crate::domain::merchant_status::MerchantStatus;
use crate::util;
use anyhow::anyhow;
use toasty::Db;
use uuid::Uuid;

#[derive(Clone)]
pub struct UpdateMerchant {
    db: Db,
}

impl UpdateMerchant {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        id: Uuid,
        request: UpdateMerchantRequest,
    ) -> Result<(), ApplicationError> {
        let mut db = self.db.clone();
        let mut merchant = match Merchant::filter_by_id(id).first().exec(&mut db).await {
            Ok(Some(m)) => m,
            Ok(None) => {
                return Err(ApplicationError::NotFound(
                    "Merchant not found with given id".to_string(),
                ));
            }
            Err(e) => return Err(ApplicationError::Internal(anyhow!(e))),
        };

        let is_active = request.status == MerchantStatus::Active;
        toasty::update!(merchant {
            name: request.name,
            email: request.email,
            phone: request.phone,
            website: request.website,
            status: request.status,
            is_active,
            updated_at: util::now_jiff(),
        })
        .exec(&mut db)
        .await
        .map_err(|e| ApplicationError::Internal(anyhow!(e)))?;

        Ok(())
    }
}

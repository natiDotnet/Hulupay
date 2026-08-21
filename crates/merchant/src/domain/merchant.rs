use crate::domain::error::DomainError;
use crate::domain::merchant_status::MerchantStatus;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Merchant entity.
///
/// Primary key is UUID v7 (time-ordered, auto-generated).
#[derive(Debug, Clone, Serialize, Deserialize, toasty::Model)]
pub struct Merchant {
    #[key]
    #[auto]
    pub id: Uuid,
    #[unique]
    pub name: String,
    #[unique]
    pub slug: String,
    #[unique]
    pub email: String,
    pub phone: String,
    pub website: String,
    pub is_active: bool,
    pub status: MerchantStatus,
    pub created_at: jiff::Timestamp,
    pub updated_at: Option<jiff::Timestamp>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MerchantView {
    pub id: Uuid,
    pub name: String,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl MerchantView {
    pub fn new(
        id: Uuid,
        name: String,
        is_active: bool,
        created_at: jiff::Timestamp,
        updated_at: Option<jiff::Timestamp>,
    ) -> Self {
        Self {
            id,
            name,
            is_active,
            created_at: crate::util::to_chrono(created_at),
            updated_at: updated_at.map(crate::util::to_chrono),
        }
    }

    pub fn ensure_active(&self) -> Result<(), DomainError> {
        if !self.is_active {
            return Err(DomainError::ProviderUnavailable);
        }
        Ok(())
    }
}

impl From<Merchant> for MerchantView {
    fn from(value: Merchant) -> Self {
        Self::new(
            value.id,
            value.name,
            value.is_active,
            value.created_at,
            value.updated_at,
        )
    }
}

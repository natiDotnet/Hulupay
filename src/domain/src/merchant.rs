use uuid::Uuid;
use crate::error::DomainError;
#[derive(Debug, Clone)]
pub struct Merchant {
    pub id: Uuid,
    pub name: String,
    pub is_active: bool,
}

impl Merchant {
    pub fn ensure_active(&self) -> Result<(), DomainError> {
        if !self.is_active {
            return Err(DomainError::ProviderUnavailable)
        }
        Ok(())
    }
}
use crate::domain::error::DomainError;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Merchant {
    pub id: Uuid,
    pub name: String,
    pub is_active: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

impl Merchant {
    pub fn new(name: String, is_active: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            is_active,
            created_at: OffsetDateTime::now_utc(),
            updated_at: Some(OffsetDateTime::now_utc()),
        }
    }

    pub fn ensure_active(&self) -> Result<(), DomainError> {
        if !self.is_active {
            return Err(DomainError::ProviderUnavailable);
        }
        Ok(())
    }
}

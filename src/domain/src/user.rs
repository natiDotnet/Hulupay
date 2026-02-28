use time::OffsetDateTime;
use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

impl User {
    pub fn new(
        merchant_id: Uuid,
        email: String,
        password_hash: String,
        role: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            merchant_id,
            email,
            password_hash,
            role,
            created_at: OffsetDateTime::now_utc(),
            updated_at: Some(OffsetDateTime::now_utc()),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    MasterAdmin,
    MerchantAdmin,
}
impl Role {

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "master_admin" => Some(Self::MasterAdmin),
            "merchant_admin" => Some(Self::MerchantAdmin),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            Role::MasterAdmin => "master_admin",
            Role::MerchantAdmin => "merchant_admin",
        }
    }
}
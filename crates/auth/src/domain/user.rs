use super::Role;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub merchant_id: Option<Uuid>,
    pub role: Role,
    pub is_active: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

impl User {
    pub fn new(email: String, password_hash: String, role: Role, merchant_id: Option<Uuid>) -> Self {
        let now = OffsetDateTime::now_utc();
        Self {
            id: Uuid::new_v4(),
            email,
            password_hash,
            role,
            merchant_id,
            is_active: true,
            created_at: now,
            updated_at: None,
        }
    }
}

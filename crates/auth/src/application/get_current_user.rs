use crate::application::login_request::CurrentUserResponse;
use crate::domain::user;
use crate::DomainAuthError;
use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;

#[derive(Clone)]
pub struct GetCurrentUser {
    db: DatabaseConnection,
}

impl GetCurrentUser {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn execute(&self, user_id: Uuid) -> anyhow::Result<CurrentUserResponse> {
        let user = user::Entity::find_by_id(user_id)
            .one(&self.db)
            .await?
            .ok_or(DomainAuthError::UserNotFound)?;

        Ok(CurrentUserResponse {
            id: user.id,
            email: user.email,
            name: user.name,
            role: user.role.to_string(),
            merchant_id: user.merchant_id,
            email_verified: user.email_verified_at.is_some(),
            created_at: user.created_at.to_rfc3339(),
        })
    }
}

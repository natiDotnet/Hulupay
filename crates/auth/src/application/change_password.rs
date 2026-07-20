use crate::application::login_request::ChangePasswordRequest;
use crate::application::password::{hash_password, verify_password};
use crate::domain::user;
use crate::DomainAuthError;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

#[derive(Clone)]
pub struct ChangePassword {
    db: DatabaseConnection,
}

impl ChangePassword {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn execute(&self, user_id: Uuid, request: ChangePasswordRequest) -> anyhow::Result<()> {
        let user = user::Entity::find_by_id(user_id)
            .one(&self.db)
            .await?
            .ok_or(DomainAuthError::UserNotFound)?;

        // Verify current password.
        if !verify_password(&user.password_hash, &request.current_password) {
            return Err(anyhow::anyhow!(DomainAuthError::InvalidCredentials));
        }

        // Prevent re-using the same password.
        if verify_password(&user.password_hash, &request.new_password) {
            return Err(anyhow::anyhow!(DomainAuthError::SamePassword));
        }

        let new_hash = hash_password(&request.new_password)?;

        let mut am: user::ActiveModel = user.into();
        am.password_hash = Set(new_hash);
        am.password_changed_at = Set(Some(Utc::now()));
        am.updated_at = Set(Some(Utc::now()));
        am.update(&self.db).await?;

        Ok(())
    }
}

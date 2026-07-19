use crate::application::login_request::ResetPasswordRequest;
use crate::application::password::{hash_password, verify_password};
use crate::domain::password_reset;
use crate::domain::user;
use crate::DomainAuthError;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

#[derive(Clone)]
pub struct ResetPassword {
    db: DatabaseConnection,
}

impl ResetPassword {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn execute(&self, request: ResetPasswordRequest) -> anyhow::Result<()> {
        let token_hash = crate::infrastructure::hash_token(&request.token);

        let row = password_reset::Entity::find()
            .filter(password_reset::Column::TokenHash.eq(&token_hash))
            .filter(password_reset::Column::UsedAt.is_null())
            .one(&self.db)
            .await?
            .ok_or(DomainAuthError::InvalidResetToken)?;

        if row.expires_at < Utc::now() {
            return Err(anyhow::anyhow!(DomainAuthError::ResetTokenExpired));
        }

        // Mark reset token as used (single-use).
        let mut reset_am: password_reset::ActiveModel = row.clone().into();
        reset_am.used_at = Set(Some(Utc::now()));
        reset_am.update(&self.db).await?;

        // Load the user.
        let user = user::Entity::find_by_id(row.user_id)
            .one(&self.db)
            .await?
            .ok_or(DomainAuthError::UserNotFound)?;

        // Prevent re-using the same password.
        if verify_password(&user.password_hash, &request.new_password) {
            return Err(anyhow::anyhow!(DomainAuthError::SamePassword));
        }

        // Update password.
        let new_hash = hash_password(&request.new_password)?;
        let mut user_am: user::ActiveModel = user.into();
        user_am.password_hash = Set(new_hash);
        user_am.password_changed_at = Set(Some(Utc::now()));
        user_am.updated_at = Set(Some(Utc::now()));
        user_am.update(&self.db).await?;

        Ok(())
    }
}

use crate::application::login_request::{VerifyEmailRequest, VerifyEmailResponse};
use crate::domain::email_verification;
use crate::domain::user;
use crate::DomainAuthError;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

#[derive(Clone)]
pub struct VerifyEmail {
    db: DatabaseConnection,
}

impl VerifyEmail {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn execute(&self, request: VerifyEmailRequest) -> anyhow::Result<VerifyEmailResponse> {
        let token_hash = crate::infrastructure::hash_token(&request.token);

        // Find the unexpired, unused verification row.
        let row = email_verification::Entity::find()
            .filter(email_verification::Column::TokenHash.eq(&token_hash))
            .filter(email_verification::Column::VerifiedAt.is_null())
            .one(&self.db)
            .await?
            .ok_or(DomainAuthError::InvalidVerificationToken)?;

        if row.expires_at < Utc::now() {
            return Err(anyhow::anyhow!(DomainAuthError::VerificationTokenExpired));
        }

        let user_id = row.user_id;

        // Mark verification record as used.
        let mut am: email_verification::ActiveModel = row.into();
        am.verified_at = Set(Some(Utc::now()));
        am.update(&self.db).await?;

        // Mark user's email_verified_at.
        let user = user::Entity::find_by_id(user_id)
            .one(&self.db)
            .await?
            .ok_or(DomainAuthError::UserNotFound)?;

        let mut user_am: user::ActiveModel = user.into();
        user_am.email_verified_at = Set(Some(Utc::now()));
        user_am.updated_at = Set(Some(Utc::now()));
        user_am.update(&self.db).await?;

        Ok(VerifyEmailResponse { verified: true })
    }
}

use crate::application::login_request::ForgotPasswordRequest;
use crate::domain::password_reset;
use crate::domain::user;
use crate::infrastructure::MailService;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use std::sync::Arc;

/// Generates a password-reset token, persists its hash, and emails the raw
/// token to the user. Always returns 200 (to avoid user enumeration).
#[derive(Clone)]
pub struct ForgotPassword {
    db: DatabaseConnection,
    mail_service: Option<Arc<dyn MailService>>,
}

impl ForgotPassword {
    pub fn new(db: DatabaseConnection, mail_service: Option<Arc<dyn MailService>>) -> Self {
        Self { db, mail_service }
    }

    pub async fn execute(&self, request: ForgotPasswordRequest) -> anyhow::Result<()> {
        let user = user::Entity::find_by_email(&request.email)
            .one(&self.db)
            .await?;

        // Whether or not the user exists, we return Ok to avoid enumeration.
        let Some(user) = user else {
            return Ok(());
        };

        // Invalidate any previous unused resets for this user.
        let old_rows = password_reset::Entity::find()
            .filter(password_reset::Column::UserId.eq(user.id))
            .filter(password_reset::Column::UsedAt.is_null())
            .all(&self.db)
            .await?;

        for row in old_rows {
            let mut am: password_reset::ActiveModel = row.into();
            am.used_at = Set(Some(Utc::now())); // soft-invalidate
            let _ = am.update(&self.db).await;
        }

        // Create a new reset token.
        let raw_token = uuid::Uuid::now_v7().to_string();
        let token_hash = crate::infrastructure::hash_token(&raw_token);
        let ttl_hours: i64 = std::env::var("PASSWORD_RESET_TTL_HOURS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1);

        let row = password_reset::ActiveModel {
            id: Set(uuid::Uuid::now_v7()),
            user_id: Set(user.id),
            token_hash: Set(token_hash),
            expires_at: Set(Utc::now() + chrono::Duration::hours(ttl_hours)),
            used_at: Set(None),
            created_at: Set(Utc::now()),
        };
        row.insert(&self.db).await?;

        // Email the raw token (link) to the user.
        if let Some(mail) = &self.mail_service {
            let _ = mail.send_password_reset(&user.email, &raw_token).await;
        }

        Ok(())
    }
}

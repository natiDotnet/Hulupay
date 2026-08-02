use crate::application::login_request::ForgotPasswordRequest;
use crate::domain::password_reset::PasswordReset;
use crate::domain::user::User;
use crate::infrastructure::MailService;
use crate::util;
use chrono::Utc;
use std::sync::Arc;

/// Generates a password-reset token, persists its hash, and emails the raw
/// token to the user. Always returns 200 (to avoid user enumeration).
#[derive(Clone)]
pub struct ForgotPassword {
    db: toasty::Db,
    mail_service: Option<Arc<dyn MailService>>,
}

impl ForgotPassword {
    pub fn new(db: toasty::Db, mail_service: Option<Arc<dyn MailService>>) -> Self {
        Self { db, mail_service }
    }

    pub async fn execute(&self, request: ForgotPasswordRequest) -> anyhow::Result<()> {
        let mut db = self.db.clone();

        let user = User::filter_by_email(&request.email)
            .first()
            .exec(&mut db)
            .await?;

        // Whether or not the user exists, we return Ok to avoid enumeration.
        let Some(user) = user else {
            return Ok(());
        };

        // Invalidate any previous unused resets for this user.
        let old_rows = PasswordReset::filter(PasswordReset::fields().user_id().eq(user.id))
            .filter(PasswordReset::fields().used_at().is_none())
            .exec(&mut db)
            .await
            .unwrap_or_default();

        let now = util::now_jiff();
        for mut row in old_rows {
            let _ = toasty::update!(row { used_at: now })
                .exec(&mut db)
                .await;
        }

        // Create a new reset token.
        let raw_token = uuid::Uuid::now_v7().to_string();
        let token_hash = crate::infrastructure::hash_token(&raw_token);
        let ttl_hours: i64 = std::env::var("PASSWORD_RESET_TTL_HOURS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1);

        let expires_at = util::to_jiff(Utc::now() + chrono::Duration::hours(ttl_hours));
        toasty::create!(PasswordReset {
            user_id: user.id,
            token_hash,
            expires_at,
        })
        .exec(&mut db)
        .await?;

        // Email the raw token (link) to the user.
        if let Some(mail) = &self.mail_service {
            let _ = mail.send_password_reset(&user.email, &raw_token).await;
        }

        Ok(())
    }
}

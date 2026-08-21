use crate::Role;
use crate::application::login_request::{RegisterUserRequest, RegisterUserResponse};
use crate::application::password::hash_password;
use crate::domain::email_verification::EmailVerification;
use crate::domain::user::User;
use crate::infrastructure::MailService;
use crate::util;
use chrono::Utc;
use std::sync::Arc;

#[derive(Clone)]
pub struct RegisterUser {
    db: toasty::Db,
    mail_service: Option<Arc<dyn MailService>>,
}

impl RegisterUser {
    pub fn new(db: toasty::Db, mail_service: Option<Arc<dyn MailService>>) -> Self {
        Self { db, mail_service }
    }

    pub async fn execute(
        &self,
        request: RegisterUserRequest,
    ) -> anyhow::Result<RegisterUserResponse> {
        let mut db = self.db.clone();

        let exists = User::filter_by_email(&request.email)
            .first()
            .exec(&mut db)
            .await?
            .is_some();
        if exists {
            return Err(anyhow::anyhow!("User already exists"));
        }

        let password_hash = hash_password(&request.password)?;
        let role =
            Role::from_string(&request.role).ok_or_else(|| anyhow::anyhow!("Invalid role"))?;

        let user = toasty::create!(User {
            email: request.email.clone(),
            password_hash,
            role,
            merchant_id: request.merchant_id,
            status: crate::domain::status::AccountStatus::Pending,
            name: String::new(),
            created_at: util::now_jiff(),
        })
        .exec(&mut db)
        .await?;

        // Create an email verification token and send it.
        let raw_token = uuid::Uuid::now_v7().to_string();
        let token_hash = crate::infrastructure::hash_token(&raw_token);
        let ttl_hours: i64 = std::env::var("EMAIL_VERIFICATION_TTL_HOURS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(24);

        let expires_at = util::to_jiff(Utc::now() + chrono::Duration::hours(ttl_hours));
        toasty::create!(EmailVerification {
            user_id: user.id,
            token_hash,
            expires_at,
        })
        .exec(&mut db)
        .await?;

        // Fire-and-forget email send.
        if let Some(mail) = &self.mail_service {
            let email = request.email.clone();
            let mail = mail.clone();
            tokio::spawn(async move {
                if let Err(e) = mail.send_verification(&email, &raw_token).await {
                    tracing::error!(error = %e, "failed to send verification email");
                }
            });
        }

        Ok(RegisterUserResponse {
            id: user.id,
            email: user.email,
            role: user.role.to_string(),
        })
    }
}

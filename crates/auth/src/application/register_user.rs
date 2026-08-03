use crate::application::login_request::{RegisterUserRequest, RegisterUserResponse};
use crate::application::password::hash_password;
use crate::domain::email_verification;
use crate::domain::user;
use crate::infrastructure::MailService;
use crate::Role;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, SelectExt, Set};
use std::sync::Arc;

#[derive(Clone)]
pub struct RegisterUser {
    db: DatabaseConnection,
    mail_service: Option<Arc<dyn MailService>>,
}

impl RegisterUser {
    pub fn new(db: DatabaseConnection, mail_service: Option<Arc<dyn MailService>>) -> Self {
        Self { db, mail_service }
    }

    pub async fn execute(
        &self,
        request: RegisterUserRequest,
    ) -> anyhow::Result<RegisterUserResponse> {
        let exists = user::Entity::find_by_email(&request.email)
            .exists(&self.db)
            .await?;
        if exists {
            return Err(anyhow::anyhow!("User already exists"));
        }

        let password_hash = hash_password(&request.password)?;
        let role =
            Role::from_string(&request.role).ok_or_else(|| anyhow::anyhow!("Invalid role"))?;

        let user = user::ActiveModel {
            email: Set(request.email.clone()),
            password_hash: Set(password_hash),
            role: Set(role),
            merchant_id: Set(request.merchant_id),
            email_verified_at: Set(None),
            password_changed_at: Set(None),
            ..ActiveModelTrait::default()
        };
        let user = user.insert(&self.db).await?;

        // Create an email verification token and send it.
        let raw_token = uuid::Uuid::now_v7().to_string();
        let token_hash = crate::infrastructure::hash_token(&raw_token);
        let ttl_hours: i64 = std::env::var("EMAIL_VERIFICATION_TTL_HOURS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(24);

        let verification_row = email_verification::ActiveModel {
            id: Set(uuid::Uuid::now_v7()),
            user_id: Set(user.id),
            token_hash: Set(token_hash),
            expires_at: Set(Utc::now() + chrono::Duration::hours(ttl_hours)),
            verified_at: Set(None),
            created_at: Set(Utc::now()),
        };
        verification_row.insert(&self.db).await?;

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

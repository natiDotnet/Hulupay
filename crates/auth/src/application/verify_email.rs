use crate::DomainAuthError;
use crate::application::login_request::{VerifyEmailRequest, VerifyEmailResponse};
use crate::domain::email_verification::EmailVerification;
use crate::domain::user::User;
use crate::util;

#[derive(Clone)]
pub struct VerifyEmail {
    db: toasty::Db,
}

impl VerifyEmail {
    pub fn new(db: toasty::Db) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        request: VerifyEmailRequest,
    ) -> anyhow::Result<VerifyEmailResponse> {
        let mut db = self.db.clone();

        let token_hash = crate::infrastructure::hash_token(&request.token);

        // Find the unexpired, unused verification row.
        let mut row =
            EmailVerification::filter(EmailVerification::fields().token_hash().eq(&token_hash))
                .filter(EmailVerification::fields().verified_at().is_none())
                .first()
                .exec(&mut db)
                .await?
                .ok_or(DomainAuthError::InvalidVerificationToken)?;

        if row.expires_at < util::now_jiff() {
            return Err(anyhow::anyhow!(DomainAuthError::VerificationTokenExpired));
        }

        let user_id = row.user_id;
        let now = util::now_jiff();

        // Mark verification record as used.
        toasty::update!(row { verified_at: now })
            .exec(&mut db)
            .await?;

        // Mark user's email_verified_at.
        let mut user = User::filter_by_id(user_id)
            .first()
            .exec(&mut db)
            .await?
            .ok_or(DomainAuthError::UserNotFound)?;

        toasty::update!(user {
            email_verified_at: now,
            updated_at: now,
        })
        .exec(&mut db)
        .await?;

        Ok(VerifyEmailResponse { verified: true })
    }
}

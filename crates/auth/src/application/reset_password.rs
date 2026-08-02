use crate::application::login_request::ResetPasswordRequest;
use crate::application::password::{hash_password, verify_password};
use crate::domain::password_reset::PasswordReset;
use crate::domain::user::User;
use crate::util;
use crate::DomainAuthError;

#[derive(Clone)]
pub struct ResetPassword {
    db: toasty::Db,
}

impl ResetPassword {
    pub fn new(db: toasty::Db) -> Self {
        Self { db }
    }

    pub async fn execute(&self, request: ResetPasswordRequest) -> anyhow::Result<()> {
        let mut db = self.db.clone();

        let token_hash = crate::infrastructure::hash_token(&request.token);

        let mut row = PasswordReset::filter(PasswordReset::fields().token_hash().eq(&token_hash))
            .filter(PasswordReset::fields().used_at().is_none())
            .first()
            .exec(&mut db)
            .await?
            .ok_or(DomainAuthError::InvalidResetToken)?;

        if row.expires_at < util::now_jiff() {
            return Err(anyhow::anyhow!(DomainAuthError::ResetTokenExpired));
        }

        let user_id = row.user_id;
        let now = util::now_jiff();

        // Mark reset token as used (single-use).
        toasty::update!(row { used_at: now })
            .exec(&mut db)
            .await?;

        // Load the user.
        let mut user = User::filter_by_id(user_id)
            .first()
            .exec(&mut db)
            .await?
            .ok_or(DomainAuthError::UserNotFound)?;

        // Prevent re-using the same password.
        if verify_password(&user.password_hash, &request.new_password) {
            return Err(anyhow::anyhow!(DomainAuthError::SamePassword));
        }

        // Update password.
        let new_hash = hash_password(&request.new_password)?;
        toasty::update!(user {
            password_hash: new_hash,
            password_changed_at: now,
            updated_at: now,
        })
        .exec(&mut db)
        .await?;

        Ok(())
    }
}

use crate::application::login_request::ChangePasswordRequest;
use crate::application::password::{hash_password, verify_password};
use crate::domain::user::User;
use crate::util;
use crate::DomainAuthError;
use uuid::Uuid;

#[derive(Clone)]
pub struct ChangePassword {
    db: toasty::Db,
}

impl ChangePassword {
    pub fn new(db: toasty::Db) -> Self {
        Self { db }
    }

    pub async fn execute(&self, user_id: Uuid, request: ChangePasswordRequest) -> anyhow::Result<()> {
        let mut db = self.db.clone();

        let mut user = User::filter_by_id(user_id)
            .first()
            .exec(&mut db)
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
        let now = util::now_jiff();

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

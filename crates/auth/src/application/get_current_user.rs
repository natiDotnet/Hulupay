use crate::DomainAuthError;
use crate::application::login_request::CurrentUserResponse;
use crate::domain::user::User;
use crate::util;
use uuid::Uuid;

#[derive(Clone)]
pub struct GetCurrentUser {
    db: toasty::Db,
}

impl GetCurrentUser {
    pub fn new(db: toasty::Db) -> Self {
        Self { db }
    }

    pub async fn execute(&self, user_id: Uuid) -> anyhow::Result<CurrentUserResponse> {
        let mut db = self.db.clone();

        let user = User::filter_by_id(user_id)
            .first()
            .exec(&mut db)
            .await?
            .ok_or(DomainAuthError::UserNotFound)?;

        Ok(CurrentUserResponse {
            id: user.id,
            email: user.email,
            name: user.name,
            role: user.role.to_string(),
            merchant_id: user.merchant_id,
            email_verified: user.email_verified_at.is_some(),
            created_at: util::to_chrono(user.created_at).to_rfc3339(),
        })
    }
}

use crate::Role;
use crate::application::login_request::{RegisterUserRequest, RegisterUserResponse};
use crate::application::password::hash_password;
use crate::application::user_repository::UserRepository;
use crate::domain::User;
use std::sync::Arc;

#[derive(Clone)]
pub struct RegisterUser {
    repo: Arc<dyn UserRepository>,
}

impl RegisterUser {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        request: RegisterUserRequest,
    ) -> anyhow::Result<RegisterUserResponse> {
        let exists = self.repo.exists_by_email(&request.email).await?;
        if exists {
            return Err(anyhow::anyhow!("User already exists"));
        }

        let password_hash = hash_password(&request.password)?;
        let role = Role::from_str(&request.role).ok_or_else(|| anyhow::anyhow!("Invalid role"))?;

        let user = User::new(request.email, password_hash, role, request.merchant_id);

        self.repo.insert(&user).await?;

        Ok(RegisterUserResponse {
            id: user.id,
            email: user.email,
            role: user.role.as_str().to_string(),
        })
    }
}

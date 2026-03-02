use crate::auth::login_request::{RegisterUserRequest, RegisterUserResponse};
use crate::auth::password::hash_password;
use crate::auth::user_repository::UserRepository;
use domain::user::{Role, User};
use std::sync::Arc;

#[derive(Clone)]
pub struct RegisterUser {
    repo: Arc<dyn UserRepository>
}

impl RegisterUser {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, request: RegisterUserRequest)
                         -> anyhow::Result<RegisterUserResponse> {
        
        self.repo.exists_by_email(&request.email).await?;

        let password_hash = hash_password(&request.password)?;
        let role = Role::from_str(&request.role).ok_or(anyhow::anyhow!("Invalid role"))?;

        let user = User::new(request.merchant_id.unwrap(), request.email, password_hash, role.as_str().into());

        self.repo.insert(&user).await?;

        Ok(RegisterUserResponse {
            id: user.id,
            email: user.email,
            role: user.role,
        })
    }
}
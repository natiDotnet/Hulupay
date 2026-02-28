use crate::auth::login_request::{RegisterUserRequest, RegisterUserResponse};
use crate::auth::password::hash_password;
use crate::auth::user_repository::UserRepository;
use domain::user::User;
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

        let user = User::new(request.merchant_id.unwrap(), request.email, password_hash, request.role);

        self.repo.insert(&user).await?;

        Ok(RegisterUserResponse {
            id: user.id,
            email: user.email,
            role: user.role,
        })
    }
}
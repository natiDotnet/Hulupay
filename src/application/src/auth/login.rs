use crate::auth::login_request::{LoginRequest, LoginResponse};
use crate::auth::password::verify_password;
use crate::auth::token::TokenService;
use crate::auth::user_repository::UserRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct LoginUser {
    repo: Arc<dyn UserRepository>,
    token_service: Arc<dyn TokenService>,
    
}

impl LoginUser {
    pub fn new(repo: Arc<dyn UserRepository>, token_service: Arc<dyn TokenService>) -> Self {
        Self { repo, token_service }
    }

    pub async fn execute(&self, request: LoginRequest) -> anyhow::Result<LoginResponse> {
        let user = self.repo
            .find_by_email(&request.email)
            .await?;

        let user = user.ok_or_else(|| anyhow::anyhow!("Invalid email or password"))?;

        let is_password_valid = verify_password(&user.password_hash, &request.password);

        if !is_password_valid {
            return Err(anyhow::anyhow!("Invalid email or password"));
        }

        let token = self.token_service.generate(user.id, &user.email, &user.role, Some(user.merchant_id))?;

        Ok(LoginResponse { access_token: token })
    }
}
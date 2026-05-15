use crate::application::login_request::{LoginRequest, LoginResponse};
use crate::application::password::verify_password;
use crate::application::token::TokenService;
use crate::domain::user;
use crate::domain::user::Model;
use crate::DomainAuthError;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct LoginUser {
    db: DatabaseConnection,
    token_service: Arc<dyn TokenService>,
}

impl LoginUser {
    pub fn new(db: DatabaseConnection, token_service: Arc<dyn TokenService>) -> Self {
        Self { db, token_service }
    }

    pub async fn execute(&self, request: LoginRequest) -> anyhow::Result<LoginResponse> {
        let user: Option<Model> = user::Entity::find_by_email(&request.email)
            .one(&self.db)
            .await?;
        let user = user.ok_or(DomainAuthError::InvalidCredentials)?;

        let is_password_valid = verify_password(&user.password_hash, &request.password);
        if !is_password_valid {
            return Err(anyhow::anyhow!(DomainAuthError::InvalidCredentials));
        }

        let token = self.token_service.generate(
            user.id,
            &user.email,
            &user.role.to_string(),
            user.merchant_id,
        )?;

        Ok(LoginResponse {
            access_token: token,
        })
    }
}

use crate::auth::claims::Claims;
use crate::auth::login_request::{LoginRequest, LoginResponse};
use crate::auth::password::verify_password;
use crate::auth::user_repository::UserRepository;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::sync::Arc;
#[derive(Clone)]
pub struct LoginUser {
    repo: Arc<dyn UserRepository>,
    jwt_secret: String,
    
}

impl LoginUser {
    pub fn new(repo: Arc<dyn UserRepository>, jwt_secret: String) -> Self {
        Self { repo, jwt_secret }
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

        // Generate JWT
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .unwrap()
            .timestamp() as usize;

        let claims = Claims {
            sub: user.id,
            merchant_id: Some(user.merchant_id),
            role: user.role,
            exp: expiration,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;
            // .map_err(|_| AppError::Internal)?;

        Ok(LoginResponse { access_token: token })
    }
}
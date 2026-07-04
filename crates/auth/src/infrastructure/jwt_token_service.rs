use crate::application::{TokenService, UserContext};
use crate::domain::AuthError;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Clone)]
pub struct JwtTokenService {
    secret: String,
}

impl JwtTokenService {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}

impl TokenService for JwtTokenService {
    fn generate(
        &self,
        user_id: Uuid,
        email: &str,
        role: &str,
        merchant_id: Uuid,
    ) -> Result<String, AuthError> {
        let expiration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 3600;

        let claims = UserContext {
            sub: user_id,
            email: email.to_string(),
            role: role.to_string(),
            merchant_id,
            exp: expiration as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|_| AuthError::InternalError("Failed to encode token".to_string()))
    }

    fn validate(&self, token: &str) -> Result<UserContext, AuthError> {
        decode::<UserContext>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|_| AuthError::InvalidToken)
    }
}

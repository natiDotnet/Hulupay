use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use crate::application::{Claims, TokenService};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::domain::AuthError;

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
    fn generate(&self, user_id: Uuid, email: &str, role: &str, merchant_id: Option<Uuid>) -> Result<String, AuthError> {
        let expiration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() + 3600;

        let claims = Claims {
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

    fn validate(&self, token: &str) -> Result<Claims, AuthError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|_| AuthError::InvalidToken)
    }
}

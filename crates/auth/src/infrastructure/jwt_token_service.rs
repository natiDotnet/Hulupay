use crate::application::TokenService;
use crate::domain::AuthError;
use hulu_core::claims::{AuthenticationType, TokenType, UserContext};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Clone)]
pub struct JwtTokenService {
    secret: String,
    /// Access-token TTL in seconds.
    access_ttl: u64,
    /// Refresh-token TTL in seconds.
    refresh_ttl: u64,
}

impl JwtTokenService {
    /// Build from `JWT_SECRET`. TTLs are read from
    /// `JWT_ACCESS_TTL_SECS` (default 3600 = 1h) and
    /// `JWT_REFRESH_TTL_SECS` (default 604800 = 7d).
    pub fn new(secret: String) -> Self {
        let access_ttl = std::env::var("JWT_ACCESS_TTL_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(3600);
        let refresh_ttl = std::env::var("JWT_REFRESH_TTL_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(604_800);
        Self {
            secret,
            access_ttl,
            refresh_ttl,
        }
    }

    fn now_secs() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }

    fn encode(&self, claims: UserContext) -> Result<String, AuthError> {
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|_| AuthError::InternalError("Failed to encode token".to_string()))
    }

    fn decode(&self, token: &str) -> Result<UserContext, AuthError> {
        decode::<UserContext>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|_| AuthError::InvalidToken)
    }
}

impl TokenService for JwtTokenService {
    fn generate_access(
        &self,
        user_id: Uuid,
        email: &str,
        role: &str,
        merchant_id: Uuid,
        permissions: Vec<String>,
    ) -> Result<String, AuthError> {
        let exp = (Self::now_secs() + self.access_ttl) as usize;
        let claims = UserContext {
            sub: user_id,
            email: email.to_string(),
            role: role.to_string(),
            merchant_id,
            permissions,
            exp,
            auth_type: AuthenticationType::Jwt,
            auth_value: None,
            jti: Uuid::now_v7(),
            typ: TokenType::Access,
        };
        self.encode(claims)
    }

    fn generate_refresh(
        &self,
        user_id: Uuid,
        email: &str,
        role: &str,
        merchant_id: Uuid,
        jti: Uuid,
    ) -> Result<String, AuthError> {
        let exp = (Self::now_secs() + self.refresh_ttl) as usize;
        let claims = UserContext {
            sub: user_id,
            email: email.to_string(),
            role: role.to_string(),
            merchant_id,
            permissions: Vec::new(),
            exp,
            auth_type: AuthenticationType::Jwt,
            auth_value: None,
            jti,
            typ: TokenType::Refresh,
        };
        self.encode(claims)
    }

    fn validate_access(&self, token: &str) -> Result<UserContext, AuthError> {
        let claims = self.decode(token)?;
        if claims.typ != TokenType::Access {
            return Err(AuthError::InvalidToken);
        }
        Ok(claims)
    }

    fn validate_refresh(&self, token: &str) -> Result<UserContext, AuthError> {
        let claims = self.decode(token)?;
        if claims.typ != TokenType::Refresh {
            return Err(AuthError::InvalidToken);
        }
        Ok(claims)
    }
}

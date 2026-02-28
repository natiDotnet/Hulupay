use application::auth::claims::Claims;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use domain::user::Role;
use jsonwebtoken::{decode, DecodingKey, Validation};
use uuid::Uuid;

pub struct AuthUser {
    pub user_id: Uuid,
    pub merchant_id: Option<Uuid>,
    pub role: Role,
}

// #[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {

        let auth_header = parts
            .headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let secret = std::env::var("JWT_SECRET")
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        let claims = token_data.claims;

        let role = match claims.role.as_str() {
            "MASTER_ADMIN" => Role::MasterAdmin,
            "MERCHANT_ADMIN" => Role::MerchantAdmin,
            _ => return Err(StatusCode::FORBIDDEN),
        };

        Ok(AuthUser {
            user_id: claims.sub,
            merchant_id: claims.merchant_id,
            role,
        })
    }
}
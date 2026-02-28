use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Utc, Duration};
use uuid::Uuid;
use crate::auth::claims::Claims;

pub fn generate_token(
    user_id: Uuid,
    merchant_id: Option<Uuid>,
    role: String,
    secret: &str,
) -> anyhow::Result<String> {

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        merchant_id,
        role,
        exp: expiration,
    };

    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?)
}
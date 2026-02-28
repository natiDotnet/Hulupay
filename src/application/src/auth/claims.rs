use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,                // user_id
    pub merchant_id: Option<Uuid>, // None for master
    pub role: String,
    pub exp: usize,
}
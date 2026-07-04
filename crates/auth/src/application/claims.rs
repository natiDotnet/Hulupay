use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserContext {
    pub sub: Uuid,
    pub email: String,
    pub merchant_id: Uuid,
    pub role: String,
    pub exp: usize,
}

use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserContext {
    pub sub: Uuid,
    pub email: String,
    pub merchant_id: Option<Uuid>,
    pub role: String,
    pub exp: usize,
}

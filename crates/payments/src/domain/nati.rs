use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone, toasty::Model)]
pub struct Nati {
    #[key]
    #[auto]
    pub id: Uuid,
    pub name: String,
    pub test: bool,
    pub created_at: jiff::Timestamp,
    pub updated_at: Option<jiff::Timestamp>,
}

use serde::{Deserialize, Serialize};

/// Account lifecycle status.
///
/// Stored as a native PostgreSQL enum type. Each variant uses `#[column(variant = "...")]`
/// to preserve UPPERCASE discriminant values matching the previous SeaORM convention.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, toasty::Embed)]
pub enum AccountStatus {
    #[column(variant = "PENDING")]
    Pending,
    #[column(variant = "ACTIVE")]
    Active,
    #[column(variant = "SUSPENDED")]
    Suspended,
    #[column(variant = "DELETED")]
    Deleted,
}

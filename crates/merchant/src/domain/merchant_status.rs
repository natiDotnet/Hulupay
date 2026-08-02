use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Merchant lifecycle status.
///
/// Stored as a native PostgreSQL enum type. Each variant uses `#[column(variant = "...")]`
/// to preserve UPPERCASE discriminant values matching the previous SeaORM convention.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema, toasty::Embed)]
#[column(rename_all = "UPPERCASE")]
pub enum MerchantStatus {
    // #[column(variant = "PENDING")]
    Pending,
    // #[column(variant = "ACTIVE")]
    Active,
    // #[column(variant = "SUSPENDED")]
    Suspended,
    // #[column(variant = "DELETED")]
    Deleted,
}

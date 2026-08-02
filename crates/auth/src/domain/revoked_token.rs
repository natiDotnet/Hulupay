use uuid::Uuid;

/// Access-token blocklist for server-side logout. `id` is the JWT `jti`.
/// Rows are only retained until `expires_at` (the original access token's
/// expiry), so they can be periodically cleaned up.
///
/// Because `id` must equal the JWT `jti`, the key is NOT auto-generated —
/// callers set it explicitly.
#[derive(Debug, Clone, toasty::Model)]
pub struct RevokedToken {
    #[key]
    pub id: Uuid,
    pub user_id: Uuid,
    /// When the original access token would have expired (for cleanup).
    pub expires_at: jiff::Timestamp,
    pub revoked_at: jiff::Timestamp,
}

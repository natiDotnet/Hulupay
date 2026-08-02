use uuid::Uuid;

/// Persisted refresh tokens. Each login mints one; the refresh endpoint
/// rotates it (revokes the old row, inserts a new one in the same
/// `family_id`). A reuse of a revoked token revokes the entire family
/// (detected theft).
///
/// The raw refresh token is a JWT whose `jti` matches `id`; the DB row
/// lets us revoke even though the JWT itself is otherwise valid. Because
/// `id` must equal the JWT `jti`, the key is NOT auto-generated — callers
/// set it explicitly.
#[derive(Debug, Clone, toasty::Model)]
pub struct RefreshToken {
    #[key]
    pub id: Uuid,
    pub user_id: Uuid,
    /// SHA-256 hex of the raw refresh JWT — belt-and-suspenders with the
    /// `id`/`jti` lookup.
    pub token_hash: String,
    /// Rotation family id. All refresh tokens minted from a single login
    /// share a `family_id`. Reuse of a revoked family member ⇒ revoke all.
    pub family_id: Uuid,
    pub expires_at: jiff::Timestamp,
    /// `None` while the token is active; set when revoked/rotated/used.
    pub revoked_at: Option<jiff::Timestamp>,
    pub created_at: jiff::Timestamp,
}

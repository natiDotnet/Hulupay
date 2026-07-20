use sha2::{Digest, Sha256};

/// Hash an opaque token (refresh / reset / verification) with SHA-256.
///
/// Raw tokens are only ever delivered to the user (email, JWT body) and
/// compared on use; the database stores only this hex digest.
pub fn hash_token(raw: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(raw.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Constant-time-ish comparison helper for hashed tokens.
pub fn verify_token_hash(raw: &str, stored_hash: &str) -> bool {
    hash_token(raw) == stored_hash
}

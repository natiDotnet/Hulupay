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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_known_sha256_digest() {
        // SHA-256("abc") — well-known vector.
        assert_eq!(
            hash_token("abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn empty_string_hash() {
        // SHA-256("") — well-known vector.
        assert_eq!(
            hash_token(""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn different_inputs_produce_different_hashes() {
        assert_ne!(hash_token("token-a"), hash_token("token-b"));
    }

    #[test]
    fn verify_matches_hash_of_same_token() {
        let stored = hash_token("hp_live_x9f2...");
        assert!(verify_token_hash("hp_live_x9f2...", &stored));
    }

    #[test]
    fn verify_rejects_wrong_token() {
        let stored = hash_token("hp_live_x9f2...");
        assert!(!verify_token_hash("hp_live_other...", &stored));
    }
}

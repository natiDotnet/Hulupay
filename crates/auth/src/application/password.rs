use argon2::{Argon2, PasswordHash};
use password_hash::phc::SaltString;
use password_hash::{PasswordHasher, PasswordVerifier};

pub fn hash_password(password: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate();
    let argon2 = Argon2::default();

    let hash = argon2.hash_password_with_salt(password.as_bytes(), salt.as_bytes())?;
    Ok(hash.to_string())
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    let parsed_hash = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => return false,
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_then_verify_roundtrip() {
        let hash = hash_password("s3cret-pa$$word").unwrap();
        assert!(verify_password(&hash, "s3cret-pa$$word"));
    }

    #[test]
    fn wrong_password_fails_verification() {
        let hash = hash_password("correct-horse").unwrap();
        assert!(!verify_password(&hash, "battery-staple"));
    }

    #[test]
    fn each_hash_is_uniquely_salted() {
        // Same password hashed twice must produce different PHC strings
        // (random salt) — but both must verify against the password.
        let h1 = hash_password("same-password").unwrap();
        let h2 = hash_password("same-password").unwrap();
        assert_ne!(h1, h2);
        assert!(verify_password(&h1, "same-password"));
        assert!(verify_password(&h2, "same-password"));
    }

    #[test]
    fn malformed_hash_never_verifies() {
        assert!(!verify_password("not-a-hash", "anything"));
        assert!(!verify_password("", "anything"));
    }

    #[test]
    fn empty_password_roundtrips() {
        let hash = hash_password("").unwrap();
        assert!(verify_password(&hash, ""));
        assert!(!verify_password(&hash, "x"));
    }
}

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

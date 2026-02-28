use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use base64::{engine::general_purpose, Engine as _};
use rand::Rng;
use anyhow::Result;

pub trait EncryptionService {
    fn encrypt(&self, plaintext: &str) -> Result<String>;
    fn decrypt(&self, ciphertext: &str) -> Result<String>;
}

pub struct AesEncryptionService {
    key: [u8; 32],
}

impl AesEncryptionService {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }
}

impl EncryptionService for AesEncryptionService {
    fn encrypt(&self, plaintext: &str) -> Result<String> {
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&self.key));

        let mut nonce_bytes = [0u8; 12];
        rand::rng().fill_bytes(&mut nonce_bytes);

        let nonce = Nonce::try_from(nonce_bytes)?;
        let ciphertext = cipher.encrypt(&nonce, plaintext.as_bytes())?;

        let mut combined = nonce_bytes.to_vec();
        combined.extend(ciphertext);

        Ok(general_purpose::STANDARD.encode(combined))
    }

    fn decrypt(&self, ciphertext: &str) -> Result<String> {
        let decoded = general_purpose::STANDARD.decode(ciphertext)?;

        let (nonce_bytes, cipher_bytes) = decoded.split_at(12);

        let cipher = Aes256Gcm::new(&Key::<Aes256Gcm>::try_from(self.key)?);
        let nonce = Nonce::try_from(nonce_bytes)?;

        let plaintext = cipher.decrypt(&nonce, cipher_bytes)?;

        Ok(String::from_utf8(plaintext)?)
    }
}
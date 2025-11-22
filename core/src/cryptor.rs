use crate::error::{CoreError, CoreResult};
use base64::engine::general_purpose;
use base64::Engine;
use chacha20poly1305::aead::Aead;
use chacha20poly1305::{ChaCha20Poly1305, Key, KeyInit, Nonce};
use rand::rngs::OsRng;
use rand::TryRngCore;
use std::sync::Arc;

#[derive(Clone)]
pub struct Cryptor {
    cipher: Arc<ChaCha20Poly1305>,
}

impl Cryptor {
    pub fn new(key_hex: &str) -> CoreResult<Arc<Self>> {
        let key_bytes = hex::decode(key_hex)?;
        let key = Key::from_slice(&key_bytes);
        let cipher = Arc::new(ChaCha20Poly1305::new(key));
        let cryptor = Self { cipher };
        Ok(Arc::new(cryptor))
    }

    pub fn encrypt(&self, plaintext: &str) -> CoreResult<String> {
        let mut nonce_bytes = [0u8; 12];
        OsRng.try_fill_bytes(&mut nonce_bytes)?;
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = self.cipher.encrypt(nonce, plaintext.as_bytes())?;

        let mut combined = nonce_bytes.to_vec();
        combined.extend_from_slice(&ciphertext);

        Ok(general_purpose::STANDARD.encode(combined))
    }

    pub fn decrypt(&self, encrypted: &str) -> CoreResult<String> {
        let combined = general_purpose::STANDARD.decode(encrypted)?;
        if combined.len() < 12 {
            return Err(CoreError::DecryptDataTooShort);
        };

        let (nonce_bytes, ciphertext) = combined.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext = self.cipher.decrypt(nonce, ciphertext)?;

        Ok(String::from_utf8(plaintext.to_vec())?)
    }
}

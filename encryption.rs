use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose};
use crate::{Result, VpnError};

pub struct EncryptionManager {
    cipher: Aes256Gcm,
}

impl EncryptionManager {
    pub fn new(key: &[u8; 32]) -> Self {
        let cipher = Aes256Gcm::new(key.into());
        Self { cipher }
    }

    pub fn from_password(password: &str) -> Self {
        let key = Self::derive_key(password);
        Self::new(&key)
    }

    fn derive_key(password: &str) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        let result = hasher.finalize();
        let mut key = [0u8; 32];
        key.copy_from_slice(&result);
        key
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let nonce = Nonce::from_slice(b"unique nonce"); // In production, use random nonce
        
        self.cipher
            .encrypt(nonce, data)
            .map_err(|e| VpnError::EncryptionError(format!("Encryption failed: {}", e)))
    }

    pub fn decrypt(&self, encrypted_data: &[u8]) -> Result<Vec<u8>> {
        let nonce = Nonce::from_slice(b"unique nonce");
        
        self.cipher
            .decrypt(nonce, encrypted_data)
            .map_err(|e| VpnError::EncryptionError(format!("Decryption failed: {}", e)))
    }

    pub fn encrypt_base64(&self, data: &[u8]) -> Result<String> {
        let encrypted = self.encrypt(data)?;
        Ok(general_purpose::STANDARD.encode(encrypted))
    }

    pub fn decrypt_base64(&self, encrypted_base64: &str) -> Result<Vec<u8>> {
        let encrypted = general_purpose::STANDARD
            .decode(encrypted_base64)
            .map_err(|e| VpnError::EncryptionError(format!("Base64 decode failed: {}", e)))?;
        self.decrypt(&encrypted)
    }
}

pub fn generate_random_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    use rand::RngCore;
    rand::thread_rng().fill_bytes(&mut key);
    key
}

pub fn hash_password(password: &str, salt: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.update(salt.as_bytes());
    let result = hasher.finalize();
    general_purpose::STANDARD.encode(result)
}

#[derive(Debug, Clone)]
pub enum EncryptionLevel {
    None,
    Standard,      // AES-128
    Strong,        // AES-256
    Maximum,       // AES-256 with additional layers
}

impl EncryptionLevel {
    pub fn description(&self) -> &str {
        match self {
            EncryptionLevel::None => "No encryption (not recommended)",
            EncryptionLevel::Standard => "AES-128 encryption",
            EncryptionLevel::Strong => "AES-256 encryption (recommended)",
            EncryptionLevel::Maximum => "AES-256 with double encryption",
        }
    }

    pub fn key_size(&self) -> usize {
        match self {
            EncryptionLevel::None => 0,
            EncryptionLevel::Standard => 16,
            EncryptionLevel::Strong => 32,
            EncryptionLevel::Maximum => 32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_decryption() {
        let key = generate_random_key();
        let manager = EncryptionManager::new(&key);
        
        let data = b"Hello, VPN World!";
        let encrypted = manager.encrypt(data).unwrap();
        let decrypted = manager.decrypt(&encrypted).unwrap();
        
        assert_eq!(data.to_vec(), decrypted);
    }

    #[test]
    fn test_base64_encryption() {
        let manager = EncryptionManager::from_password("test_password");
        
        let data = b"Secret VPN Data";
        let encrypted_b64 = manager.encrypt_base64(data).unwrap();
        let decrypted = manager.decrypt_base64(&encrypted_b64).unwrap();
        
        assert_eq!(data.to_vec(), decrypted);
    }
}

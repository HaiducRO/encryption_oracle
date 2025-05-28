use aes_gcm::{AeadCore, Aes256Gcm};
use aes_gcm::aead::{KeyInit, OsRng};
use base64::engine::general_purpose;
use base64::Engine;

pub fn generate_key() -> Vec<u8>{
    Aes256Gcm::generate_key(&mut OsRng).to_vec()
}

pub fn generate_nonce() -> Vec<u8>{
    Aes256Gcm::generate_nonce(&mut OsRng).to_vec()
}

// Encode binary to Base64 string
pub fn to_base64(data: &[u8]) -> String {
    general_purpose::STANDARD.encode(data)
}

// Decode Base64 string to binary
pub fn from_base64(s: &str) -> anyhow::Result<Vec<u8>> {
    general_purpose::STANDARD
        .decode(s)
        .map_err(|e| anyhow::anyhow!("Base64 decode error: {e}"))
}
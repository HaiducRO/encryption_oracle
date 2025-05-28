use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptionRequest{
    pub plaintext: String,
}
#[derive(Serialize, Deserialize)]
pub struct EncryptionResult {
    pub ciphertext: String,
    pub key: String,
    pub nonce: String,
}
#[derive(Serialize, Deserialize)]
pub struct DecryptionRequest {
    pub ciphertext: String,
    pub key: String,
    pub nonce: String,
}
#[derive(Serialize, Deserialize)]
pub struct DecryptionResult{
    pub plaintext: String,
}

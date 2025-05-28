use anyhow::{Ok, Result};
use crate::utils::{generate_key, generate_nonce, to_base64, from_base64};
use crate::models::{EncryptionRequest, EncryptionResult, DecryptionRequest, DecryptionResult};
use aes_gcm::{
    aead::{Aead, KeyInit, Nonce}, Aes256Gcm, Key,
};




pub fn encrypt(request: &EncryptionRequest) -> Result<EncryptionResult>{
    let plaintext_bytes = request.plaintext.as_bytes();
    let key_vec = generate_key();
    let nonce_vec = generate_nonce();

    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_vec);
    let nonce = aes_gcm::Nonce::from_slice(&nonce_vec);
    let cipher = Aes256Gcm::new(key);
    let ciphertext = cipher.encrypt(nonce, plaintext_bytes)
        .map_err(|e| anyhow::anyhow!("Encryption failed {e}"))?;
    let key_b64 = to_base64(&key_vec);
    let nonce_b64 = to_base64(&nonce_vec);
    let ciphertext_b64 = to_base64(&ciphertext);
    Ok(EncryptionResult { 
        ciphertext: ciphertext_b64 ,
        key: key_b64,
        nonce: nonce_b64
    })
}

pub fn decrypt(request: &DecryptionRequest) -> Result<DecryptionResult>{
    let ciphertext_bytes = from_base64(&request.ciphertext)?;
    let key_bytes = from_base64(&request.key)?;
    let nonce_bytes = from_base64(&request.nonce)?;

    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let nonce = Nonce::<Aes256Gcm>::from_slice(&nonce_bytes);
    let cipher = Aes256Gcm::new(key);

    let plaintext = cipher.decrypt(nonce, &*ciphertext_bytes)
        .map_err(|e| anyhow::anyhow!("Decryption failed {e}"))?;
    let plaintext_str = to_base64(&plaintext);
    Ok(DecryptionResult{ plaintext: plaintext_str })
}

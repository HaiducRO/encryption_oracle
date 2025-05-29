use anyhow::{Ok, Result};
use base64::prelude::{BASE64_STANDARD};
use base64::engine::general_purpose::STANDARD_NO_PAD;
use base64::Engine;
use crate::utils::{generate_key, generate_nonce};
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
    let key_b64 = BASE64_STANDARD.encode(&key_vec);
    let nonce_b64 = BASE64_STANDARD.encode(&nonce_vec);
    let ciphertext_b64 = BASE64_STANDARD.encode(&ciphertext);
    Ok(EncryptionResult { 
        ciphertext: ciphertext_b64 ,
        key: key_b64,
        nonce: nonce_b64
    })
}

pub fn decrypt(request: &DecryptionRequest) -> Result<DecryptionResult>{
    let ciphertext_bytes = STANDARD_NO_PAD.decode(&request.ciphertext)?;
    let key_bytes = STANDARD_NO_PAD.decode(&request.key)?;
    let nonce_bytes = STANDARD_NO_PAD.decode(&request.nonce)?;

    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let nonce = Nonce::<Aes256Gcm>::from_slice(&nonce_bytes);
    let cipher = Aes256Gcm::new(key);

    let plaintext = cipher.decrypt(nonce, &*ciphertext_bytes)
        .map_err(|e| anyhow::anyhow!("Decryption failed {e}"))?;
    let plaintext_str = String::from_utf8(plaintext)
        .map_err(|e| anyhow::anyhow!("Invalid UTF-8 in decrypted text: {e}"))?;
    Ok(DecryptionResult{ plaintext: plaintext_str})
}


use crate::models::{EncryptionRequest, EncryptionResult, DecryptionRequest, DecryptionResult};
use crate::encryption::{encrypt, decrypt};
use std::result::Result::Ok;
use axum::debug_handler;
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};

pub fn routes() -> Router{
    Router::new()
        .route("/", get(root))
        .route("/encrypt", post(post_encrypt))
        .route("/decrypt", post(post_decrypt))
        
}
#[debug_handler]
async fn root() -> &'static str {
    "Encryption Oracle is running!"
}
#[debug_handler]
pub async fn post_encrypt(
    Json(payload): Json<EncryptionRequest>,
) -> Result<Json<EncryptionResult>, (StatusCode, String)> {
    match encrypt(&payload) {
        Ok(result) => Ok(Json(result)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[debug_handler]
pub async fn post_decrypt(
    Json(payload): Json<DecryptionRequest>,
) -> Result<Json<DecryptionResult>, (StatusCode, String)> {
    match decrypt(&payload) {
        Ok(result) => Ok(Json(result)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
} 


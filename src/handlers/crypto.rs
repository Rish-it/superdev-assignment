use axum::{response::Result, Json};
use solana_sdk::signature::{Keypair, Signature, Signer, SeedDerivable};

use crate::{
    errors::ApiError,
    models::{
        ApiResponse, SignMessageRequest, SignMessageResponse, VerifyMessageRequest,
        VerifyMessageResponse,
    },
    utils::{decode_base58, decode_base64, encode_base64, string_to_pubkey},
};

pub async fn sign_message(
    Json(req): Json<SignMessageRequest>,
) -> Result<Json<ApiResponse<SignMessageResponse>>, ApiError> {
    let secret_key_bytes = decode_base58(&req.private_key)?;
    
    let seed_bytes = if secret_key_bytes.len() == 64 {
        &secret_key_bytes[..32]
    } else if secret_key_bytes.len() == 32 {
        &secret_key_bytes
    } else {
        return Err(ApiError::InvalidInput(
            format!("Private key must be 32 or 64 bytes, got {}", secret_key_bytes.len())
        ));
    };

    let keypair = Keypair::from_seed(seed_bytes)
        .map_err(|e| ApiError::InvalidInput(format!("Invalid private key: {}", e)))?;

    let message_bytes = req.message.as_bytes();
    let signature = keypair.sign_message(message_bytes);
    let signature_base64 = encode_base64(&signature.as_ref());

    let response = SignMessageResponse {
        signature: signature_base64,
        message: req.message,
        public_key: keypair.pubkey().to_string(),
    };

    Ok(Json(ApiResponse::success(response)))
}

pub async fn verify_message(
    Json(req): Json<VerifyMessageRequest>,
) -> Result<Json<ApiResponse<VerifyMessageResponse>>, ApiError> {
    let public_key = string_to_pubkey(&req.public_key)?;
    let signature_bytes = decode_base64(&req.signature)?;

    let signature = Signature::try_from(signature_bytes.as_slice())
        .map_err(|e| ApiError::VerificationFailed(format!("Invalid signature format: {}", e)))?;

    let message_bytes = req.message.as_bytes();
    let is_valid = signature.verify(&public_key.to_bytes(), message_bytes);

    let response = VerifyMessageResponse {
        is_valid,
        message: req.message,
        public_key: req.public_key,
    };

    Ok(Json(ApiResponse::success(response)))
}

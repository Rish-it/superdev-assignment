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
    if req.message.is_empty() || req.private_key.is_empty() {
        return Err(ApiError::InvalidInput);
    }

    let key_bytes = decode_base58(&req.private_key)?;
    
    let seed = match key_bytes.len() {
        32 => &key_bytes[..],
        64 => &key_bytes[..32],
        _ => return Err(ApiError::InvalidKey),
    };

    let keypair = Keypair::from_seed(seed).map_err(|_| ApiError::InvalidKey)?;
    let signature = keypair.sign_message(req.message.as_bytes());

    Ok(Json(ApiResponse::success(SignMessageResponse {
        signature: encode_base64(&signature.as_ref()),
        message: req.message,
        public_key: keypair.pubkey().to_string(),
    })))
}

pub async fn verify_message(
    Json(req): Json<VerifyMessageRequest>,
) -> Result<Json<ApiResponse<VerifyMessageResponse>>, ApiError> {
    if req.message.is_empty() || req.signature.is_empty() || req.public_key.is_empty() {
        return Err(ApiError::InvalidInput);
    }

    let pubkey = string_to_pubkey(&req.public_key)?;
    
    let is_valid = match decode_base64(&req.signature) {
        Ok(bytes) => match Signature::try_from(bytes.as_slice()) {
            Ok(sig) => sig.verify(&pubkey.to_bytes(), req.message.as_bytes()),
            Err(_) => false,
        },
        Err(_) => false,
    };

    Ok(Json(ApiResponse::success(VerifyMessageResponse {
        is_valid,
        message: req.message,
        public_key: req.public_key,
    })))
}

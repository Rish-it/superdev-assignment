use axum::{response::Result, Json};
use solana_sdk::signer::{keypair::Keypair, Signer};

use crate::{
    errors::ApiError,
    models::{ApiResponse, KeypairResponse},
    utils::encode_base58,
};

pub async fn generate_keypair() -> Result<Json<ApiResponse<KeypairResponse>>, ApiError> {
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey();
    let keypair_bytes = keypair.to_bytes();
    let pubkey_str = pubkey.to_string();
    let secret_str = encode_base58(&keypair_bytes);

    let response = KeypairResponse {
        pubkey: pubkey_str,
        secret: secret_str,
    };

    Ok(Json(ApiResponse::success(response)))
}

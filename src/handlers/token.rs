use axum::{response::Result, Json};
use spl_token::instruction::initialize_mint;

use crate::{
    errors::ApiError,
    models::{AccountMeta, ApiResponse, CreateTokenRequest, CreateTokenResponse},
    utils::{encode_base64, string_to_pubkey},
};

pub async fn create_token(
    Json(req): Json<CreateTokenRequest>,
) -> Result<Json<ApiResponse<CreateTokenResponse>>, ApiError> {
    if req.mint_authority.is_empty() || req.mint.is_empty() {
        return Err(ApiError::InvalidInput);
    }

    if req.decimals > 9 {
        return Err(ApiError::InvalidInput);
    }

    let mint_authority = string_to_pubkey(&req.mint_authority)?;
    let mint_pubkey = string_to_pubkey(&req.mint)?;

    let ix = initialize_mint(
        &spl_token::id(),
        &mint_pubkey,
        &mint_authority,
        Some(&mint_authority),
        req.decimals,
    ).map_err(|_| ApiError::InvalidInput)?;

    let accounts = ix.accounts.into_iter().map(|acc| AccountMeta {
        pubkey: acc.pubkey.to_string(),
        is_signer: acc.is_signer,
        is_writable: acc.is_writable,
    }).collect();

    Ok(Json(ApiResponse::success(CreateTokenResponse {
        program_id: ix.program_id.to_string(),
        accounts,
        instruction_data: encode_base64(&ix.data),
    })))
}

use axum::{response::Result, Json};
use solana_sdk::instruction::Instruction;
use spl_token::instruction::initialize_mint;

use crate::{
    errors::ApiError,
    models::{AccountMeta, ApiResponse, CreateTokenRequest, CreateTokenResponse},
    utils::{encode_base64, string_to_pubkey},
};


pub async fn create_token(
    Json(req): Json<CreateTokenRequest>,
) -> Result<Json<ApiResponse<CreateTokenResponse>>, ApiError> {

    let mint_authority = string_to_pubkey(&req.mint_authority)?;
    let mint_pubkey = string_to_pubkey(&req.mint)?;


    if req.decimals > 9 {
        return Err(ApiError::InvalidInput(
            "Token decimals must be between 0 and 9".to_string(),
        ));
    }

    let init_mint_ix = initialize_mint(
        &spl_token::id(),    
        &mint_pubkey,         
        &mint_authority,      
        Some(&mint_authority), 
        req.decimals,         
    )
    .map_err(|e| ApiError::TokenCreation(format!("Failed to create mint instruction: {}", e)))?;
    let response = instruction_to_response(init_mint_ix)?;

    Ok(Json(ApiResponse::success(response)))
}

fn instruction_to_response(instruction: Instruction) -> Result<CreateTokenResponse, ApiError> {
    let accounts: Vec<AccountMeta> = instruction
        .accounts
        .into_iter()
        .map(|acc| AccountMeta {
            pubkey: acc.pubkey.to_string(),
            is_signer: acc.is_signer,
            is_writable: acc.is_writable,
        })
        .collect();

    let instruction_data = encode_base64(&instruction.data);

    Ok(CreateTokenResponse {
        program_id: instruction.program_id.to_string(),
        accounts,
        instruction_data,
    })
}

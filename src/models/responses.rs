use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data,
        }
    }
}

#[derive(Serialize)]
pub struct KeypairResponse {
    pub pubkey: String,
    pub secret: String,
}

#[derive(Deserialize)]
pub struct CreateTokenRequest {
    #[serde(alias = "mintAuthority")]
    pub mint_authority: String,
    pub mint: String,
    pub decimals: u8,
}

#[derive(Serialize)]
pub struct AccountMeta {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Serialize)]
pub struct CreateTokenResponse {
    pub program_id: String,
    pub accounts: Vec<AccountMeta>,
    pub instruction_data: String,
}

#[derive(Deserialize)]
pub struct SignMessageRequest {
    pub message: String,
    #[serde(alias = "privateKey")]
    pub private_key: String,
}

#[derive(Serialize)]
pub struct SignMessageResponse {
    pub signature: String,
    pub message: String,
    pub public_key: String,
}

#[derive(Deserialize)]
pub struct VerifyMessageRequest {
    pub message: String,
    pub signature: String,
    #[serde(alias = "publicKey")]
    pub public_key: String,
}

#[derive(Serialize)]
pub struct VerifyMessageResponse {
    pub is_valid: bool,
    pub message: String,
    pub public_key: String,
}



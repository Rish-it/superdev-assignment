use crate::errors::ApiError;

pub fn encode_base58(data: &[u8]) -> String {
    bs58::encode(data).into_string()
}

pub fn decode_base58(data: &str) -> Result<Vec<u8>, ApiError> {
    if data.is_empty() {
        return Err(ApiError::BadEncoding);
    }
    bs58::decode(data).into_vec().map_err(ApiError::from)
}

pub fn encode_base64(data: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(data)
}

pub fn decode_base64(data: &str) -> Result<Vec<u8>, ApiError> {
    if data.is_empty() {
        return Err(ApiError::BadEncoding);
    }
    use base64::Engine;
    base64::engine::general_purpose::STANDARD
        .decode(data)
        .map_err(ApiError::from)
}

pub fn string_to_pubkey(pubkey_str: &str) -> Result<solana_sdk::pubkey::Pubkey, ApiError> {
    if pubkey_str.is_empty() {
        return Err(ApiError::InvalidKey);
    }
    pubkey_str.parse().map_err(|_| ApiError::InvalidKey)
}

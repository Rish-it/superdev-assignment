use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Token creation failed: {0}")]
    TokenCreation(String),

    #[error("Verification failed: {0}")]
    VerificationFailed(String),

    #[error("Encoding error: {0}")]
    EncodingError(String),
}



impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = StatusCode::BAD_REQUEST;
        let error_message = self.to_string();

        let body = Json(json!({
            "success": false,
            "error": error_message
        }));

        (status, body).into_response()
    }
}

impl From<bs58::decode::Error> for ApiError {
    fn from(err: bs58::decode::Error) -> Self {
        ApiError::EncodingError(format!("Base58 decode error: {}", err))
    }
}

impl From<base64::DecodeError> for ApiError {
    fn from(err: base64::DecodeError) -> Self {
        ApiError::EncodingError(format!("Base64 decode error: {}", err))
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError::InvalidInput(format!("JSON parse error: {}", err))
    }
}

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub enum ApiStatusCode {
    Ok = 200,
    BadRequest = 400,
    InternalServerError = 500,
}

impl From<ApiStatusCode> for StatusCode {
    fn from(code: ApiStatusCode) -> Self {
        match code {
            ApiStatusCode::Ok => StatusCode::OK,
            ApiStatusCode::BadRequest => StatusCode::BAD_REQUEST,
            ApiStatusCode::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

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

impl ApiError {
    fn status_code(&self) -> ApiStatusCode {
        match self {
            ApiError::InvalidInput(_) => ApiStatusCode::BadRequest,
            ApiError::TokenCreation(_) => ApiStatusCode::BadRequest,

            ApiError::VerificationFailed(_) => ApiStatusCode::BadRequest,
            ApiError::EncodingError(_) => ApiStatusCode::BadRequest,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = StatusCode::from(self.status_code());
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

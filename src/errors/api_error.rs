use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("invalid input")]
    InvalidInput,

    #[error("invalid key")]
    InvalidKey,

    #[error("bad encoding")]
    BadEncoding,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "success": false,
            "error": self.to_string()
        }));

        (StatusCode::BAD_REQUEST, body).into_response()
    }
}

impl From<bs58::decode::Error> for ApiError {
    fn from(_: bs58::decode::Error) -> Self {
        ApiError::BadEncoding
    }
}

impl From<base64::DecodeError> for ApiError {
    fn from(_: base64::DecodeError) -> Self {
        ApiError::BadEncoding
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(_: serde_json::Error) -> Self {
        ApiError::InvalidInput
    }
}

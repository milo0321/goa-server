use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use sqlx::error::Error as SqlxError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Param error")]
    ParamError(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Database error")]
    DatabaseError(#[from] SqlxError),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("InvalidRequest error: {0}")]
    InvalidRequest(String),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("InternalServerError error: {0}")]
    InternalServerError(String),
    #[error("NotImplement")]
    NotImplemented,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::ParamError(_) => (StatusCode::BAD_REQUEST, String::default()),
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::DatabaseError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            ),
            ApiError::InvalidRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            ApiError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_string()),
            ApiError::NotImplemented => (StatusCode::NOT_IMPLEMENTED, "NotImplemented".to_string()),
            ApiError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        (status, Json(ErrorResponse { message })).into_response()
    }
}

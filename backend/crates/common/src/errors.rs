//! Common error types.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

/// Standard API error response.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    /// Error code for programmatic handling
    pub code: String,
    /// Human-readable error message
    pub message: String,
    /// Request ID for support/debugging
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Detailed validation errors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<ValidationError>>,
    /// ISO 8601 timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ErrorResponse {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            request_id: None,
            details: None,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }

    pub fn with_details(mut self, details: Vec<ValidationError>) -> Self {
        self.details = Some(details);
        self
    }

    // Helper methods for common errors
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new("BAD_REQUEST", message)
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new("NOT_FOUND", message)
    }

    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::new("INTERNAL_ERROR", message)
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new("UNAUTHORIZED", message)
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::new("FORBIDDEN", message)
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let status = match self.code.as_str() {
            "BAD_REQUEST" => StatusCode::BAD_REQUEST,
            "UNAUTHORIZED" => StatusCode::UNAUTHORIZED,
            "FORBIDDEN" => StatusCode::FORBIDDEN,
            "NOT_FOUND" => StatusCode::NOT_FOUND,
            "CONFLICT" => StatusCode::CONFLICT,
            "UNPROCESSABLE_ENTITY" => StatusCode::UNPROCESSABLE_ENTITY,
            "RATE_LIMIT_EXCEEDED" => StatusCode::TOO_MANY_REQUESTS,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, Json(self)).into_response()
    }
}

/// Validation error detail.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ValidationError {
    /// Field path (e.g., "address.city")
    pub field: String,
    /// Error message for this field
    pub message: String,
    /// Error code
    pub code: String,
}

/// Application error types.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Unprocessable entity: {0}")]
    UnprocessableEntity(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("External service error: {0}")]
    ExternalService(String),
}

impl AppError {
    pub fn code(&self) -> &'static str {
        match self {
            AppError::BadRequest(_) => "BAD_REQUEST",
            AppError::Unauthorized(_) => "UNAUTHORIZED",
            AppError::Forbidden(_) => "FORBIDDEN",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::Conflict(_) => "CONFLICT",
            AppError::UnprocessableEntity(_) => "UNPROCESSABLE_ENTITY",
            AppError::RateLimitExceeded => "RATE_LIMIT_EXCEEDED",
            AppError::Internal(_) => "INTERNAL_ERROR",
            AppError::Database(_) => "DATABASE_ERROR",
            AppError::ExternalService(_) => "EXTERNAL_SERVICE_ERROR",
        }
    }

    pub fn status_code(&self) -> u16 {
        match self {
            AppError::BadRequest(_) => 400,
            AppError::Unauthorized(_) => 401,
            AppError::Forbidden(_) => 403,
            AppError::NotFound(_) => 404,
            AppError::Conflict(_) => 409,
            AppError::UnprocessableEntity(_) => 422,
            AppError::RateLimitExceeded => 429,
            AppError::Internal(_) | AppError::Database(_) | AppError::ExternalService(_) => 500,
        }
    }

    pub fn to_response(&self) -> ErrorResponse {
        ErrorResponse::new(self.code(), self.to_string())
    }
}

//! Common error types.

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

    /// Create a bad request (400) error response.
    pub fn bad_request(message: &str) -> Self {
        Self::new("BAD_REQUEST", message)
    }

    /// Create a forbidden (403) error response.
    pub fn forbidden(message: &str) -> Self {
        Self::new("FORBIDDEN", message)
    }

    /// Create a not found (404) error response.
    pub fn not_found(message: &str) -> Self {
        Self::new("NOT_FOUND", message)
    }

    /// Create an internal server error (500) response.
    pub fn internal_error(message: &str) -> Self {
        Self::new("INTERNAL_ERROR", message)
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

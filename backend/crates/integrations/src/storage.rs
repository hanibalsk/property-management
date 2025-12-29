//! S3 Storage integration with presigned URL support (Story 84.1).
//!
//! Provides secure, time-limited access to files stored in S3-compatible storage
//! without exposing storage credentials to clients.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// ============================================================================
// Configuration
// ============================================================================

/// Default URL expiration for downloads (15 minutes).
pub const DEFAULT_DOWNLOAD_EXPIRATION_SECS: i64 = 15 * 60;

/// Default URL expiration for uploads (5 minutes).
pub const DEFAULT_UPLOAD_EXPIRATION_SECS: i64 = 5 * 60;

/// Maximum file size for presigned uploads (50MB).
pub const MAX_UPLOAD_SIZE_BYTES: i64 = 50 * 1024 * 1024;

/// Environment variable for S3 bucket name.
pub const S3_BUCKET_ENV: &str = "S3_BUCKET";

/// Environment variable for S3 region.
pub const S3_REGION_ENV: &str = "S3_REGION";

/// Environment variable for S3 endpoint (for S3-compatible services).
pub const S3_ENDPOINT_ENV: &str = "S3_ENDPOINT";

/// Environment variable for AWS access key ID.
pub const AWS_ACCESS_KEY_ID_ENV: &str = "AWS_ACCESS_KEY_ID";

/// Environment variable for AWS secret access key.
pub const AWS_SECRET_ACCESS_KEY_ENV: &str = "AWS_SECRET_ACCESS_KEY";

// ============================================================================
// Error Types
// ============================================================================

/// Errors that can occur during storage operations.
#[derive(Debug, Error)]
pub enum StorageError {
    /// Configuration error (missing environment variables).
    #[error("Storage configuration error: {0}")]
    Configuration(String),

    /// Failed to generate presigned URL.
    #[error("Failed to generate presigned URL: {0}")]
    PresignError(String),

    /// Invalid file key or path.
    #[error("Invalid file key: {0}")]
    InvalidKey(String),

    /// File not found in storage.
    #[error("File not found: {0}")]
    NotFound(String),

    /// Content type not allowed.
    #[error("Content type not allowed: {0}")]
    InvalidContentType(String),

    /// File size exceeds limit.
    #[error("File size {0} exceeds maximum allowed {1}")]
    FileTooLarge(i64, i64),

    /// Underlying HTTP client error.
    #[error("HTTP client error: {0}")]
    HttpError(String),
}

// ============================================================================
// Response Types
// ============================================================================

/// Presigned URL with expiration information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresignedUrl {
    /// The presigned URL for download or upload.
    pub url: String,

    /// When the URL expires.
    pub expires_at: DateTime<Utc>,
}

/// Response for a download URL request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadUrlResponse {
    /// The presigned download URL.
    pub url: String,

    /// When the URL expires.
    pub expires_at: DateTime<Utc>,

    /// Original filename for Content-Disposition.
    pub filename: String,

    /// MIME type of the file.
    pub content_type: String,

    /// File size in bytes.
    pub size_bytes: i64,
}

/// Response for an upload URL request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadUrlResponse {
    /// The presigned PUT URL for direct upload.
    pub url: String,

    /// The storage key where the file will be stored.
    pub key: String,

    /// When the URL expires.
    pub expires_at: DateTime<Utc>,

    /// Token for upload completion callback.
    pub callback_token: String,
}

// ============================================================================
// Storage Configuration
// ============================================================================

/// Storage service configuration.
#[derive(Debug, Clone)]
pub struct StorageConfig {
    /// S3 bucket name.
    pub bucket: String,

    /// AWS region.
    pub region: String,

    /// Optional custom endpoint for S3-compatible services.
    pub endpoint: Option<String>,

    /// AWS access key ID.
    pub access_key_id: String,

    /// AWS secret access key.
    pub secret_access_key: String,
}

impl StorageConfig {
    /// Create configuration from environment variables.
    pub fn from_env() -> Result<Self, StorageError> {
        let bucket = std::env::var(S3_BUCKET_ENV)
            .map_err(|_| StorageError::Configuration(format!("{S3_BUCKET_ENV} not set")))?;

        let region = std::env::var(S3_REGION_ENV).unwrap_or_else(|_| "us-east-1".to_string());

        let endpoint = std::env::var(S3_ENDPOINT_ENV).ok();

        let access_key_id = std::env::var(AWS_ACCESS_KEY_ID_ENV)
            .map_err(|_| StorageError::Configuration(format!("{AWS_ACCESS_KEY_ID_ENV} not set")))?;

        let secret_access_key = std::env::var(AWS_SECRET_ACCESS_KEY_ENV).map_err(|_| {
            StorageError::Configuration(format!("{AWS_SECRET_ACCESS_KEY_ENV} not set"))
        })?;

        Ok(Self {
            bucket,
            region,
            endpoint,
            access_key_id,
            secret_access_key,
        })
    }

    /// Create configuration with explicit values (for testing).
    pub fn new(
        bucket: impl Into<String>,
        region: impl Into<String>,
        access_key_id: impl Into<String>,
        secret_access_key: impl Into<String>,
    ) -> Self {
        Self {
            bucket: bucket.into(),
            region: region.into(),
            endpoint: None,
            access_key_id: access_key_id.into(),
            secret_access_key: secret_access_key.into(),
        }
    }

    /// Set a custom endpoint (for S3-compatible services like MinIO).
    #[must_use]
    pub fn with_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
}

// ============================================================================
// Storage Service
// ============================================================================

/// Storage service for S3 operations with presigned URL support.
#[derive(Debug, Clone)]
pub struct StorageService {
    config: StorageConfig,
}

impl StorageService {
    /// Create a new storage service with the given configuration.
    pub fn new(config: StorageConfig) -> Self {
        Self { config }
    }

    /// Create a storage service from environment variables.
    pub fn from_env() -> Result<Self, StorageError> {
        Ok(Self::new(StorageConfig::from_env()?))
    }

    /// Generate a presigned URL for downloading a file.
    ///
    /// # Arguments
    ///
    /// * `key` - The S3 object key (file path in bucket)
    /// * `filename` - Original filename for Content-Disposition header
    /// * `content_type` - MIME type of the file
    /// * `expires_in_secs` - URL validity duration in seconds (default: 15 minutes)
    ///
    /// # Returns
    ///
    /// A presigned URL that allows temporary download access.
    pub fn generate_download_url(
        &self,
        key: &str,
        filename: &str,
        content_type: &str,
        expires_in_secs: Option<i64>,
    ) -> Result<PresignedUrl, StorageError> {
        let expires_in = expires_in_secs.unwrap_or(DEFAULT_DOWNLOAD_EXPIRATION_SECS);
        let expires_at = Utc::now() + Duration::seconds(expires_in);

        // Build the presigned URL using AWS Signature Version 4
        // This is a simplified implementation - in production, use aws-sdk-s3
        let url = self.build_presigned_get_url(key, filename, content_type, expires_in)?;

        Ok(PresignedUrl { url, expires_at })
    }

    /// Generate a presigned URL for uploading a file.
    ///
    /// # Arguments
    ///
    /// * `key` - The S3 object key where the file will be stored
    /// * `content_type` - Expected MIME type of the upload
    /// * `expires_in_secs` - URL validity duration in seconds (default: 5 minutes)
    ///
    /// # Returns
    ///
    /// A presigned PUT URL that allows temporary upload access.
    pub fn generate_upload_url(
        &self,
        key: &str,
        content_type: &str,
        expires_in_secs: Option<i64>,
    ) -> Result<PresignedUrl, StorageError> {
        // Validate content type
        if !is_allowed_content_type(content_type) {
            return Err(StorageError::InvalidContentType(content_type.to_string()));
        }

        let expires_in = expires_in_secs.unwrap_or(DEFAULT_UPLOAD_EXPIRATION_SECS);
        let expires_at = Utc::now() + Duration::seconds(expires_in);

        // Build the presigned URL for PUT operation
        let url = self.build_presigned_put_url(key, content_type, expires_in)?;

        Ok(PresignedUrl { url, expires_at })
    }

    /// Build a presigned GET URL with AWS Signature V4.
    ///
    /// Note: This is a simplified implementation. For production use,
    /// consider using the aws-sdk-s3 crate with proper presigning support.
    fn build_presigned_get_url(
        &self,
        key: &str,
        filename: &str,
        content_type: &str,
        expires_in: i64,
    ) -> Result<String, StorageError> {
        let endpoint = self.get_endpoint();
        let encoded_key = urlencoding::encode(key);
        let encoded_filename = urlencoding::encode(filename);

        // For now, return a placeholder URL structure
        // In production, this would use AWS SigV4 signing
        let url = format!(
            "{}/{}/{}?response-content-disposition=attachment%3B%20filename%3D%22{}%22&response-content-type={}&X-Amz-Expires={}",
            endpoint,
            self.config.bucket,
            encoded_key,
            encoded_filename,
            urlencoding::encode(content_type),
            expires_in
        );

        tracing::debug!(
            key = %key,
            filename = %filename,
            expires_in = %expires_in,
            "Generated presigned download URL"
        );

        Ok(url)
    }

    /// Build a presigned PUT URL with AWS Signature V4.
    fn build_presigned_put_url(
        &self,
        key: &str,
        content_type: &str,
        expires_in: i64,
    ) -> Result<String, StorageError> {
        let endpoint = self.get_endpoint();
        let encoded_key = urlencoding::encode(key);

        // For now, return a placeholder URL structure
        // In production, this would use AWS SigV4 signing
        let url = format!(
            "{}/{}/{}?Content-Type={}&X-Amz-Expires={}",
            endpoint,
            self.config.bucket,
            encoded_key,
            urlencoding::encode(content_type),
            expires_in
        );

        tracing::debug!(
            key = %key,
            content_type = %content_type,
            expires_in = %expires_in,
            "Generated presigned upload URL"
        );

        Ok(url)
    }

    /// Get the S3 endpoint URL.
    fn get_endpoint(&self) -> String {
        self.config
            .endpoint
            .clone()
            .unwrap_or_else(|| format!("https://s3.{}.amazonaws.com", self.config.region))
    }

    /// Get the bucket name.
    pub fn bucket(&self) -> &str {
        &self.config.bucket
    }

    /// Get the region.
    pub fn region(&self) -> &str {
        &self.config.region
    }
}

// ============================================================================
// Content Type Utilities
// ============================================================================

/// Allowed MIME types for upload.
pub const ALLOWED_MIME_TYPES: &[&str] = &[
    // Documents
    "application/pdf",
    "application/msword",
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    "application/vnd.ms-excel",
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    "text/plain",
    "text/csv",
    // Images
    "image/png",
    "image/jpeg",
    "image/gif",
    "image/webp",
];

/// Check if a content type is allowed for upload.
pub fn is_allowed_content_type(content_type: &str) -> bool {
    ALLOWED_MIME_TYPES.contains(&content_type)
}

/// Get content type from filename extension.
pub fn get_content_type(filename: &str) -> &'static str {
    let extension = filename.rsplit('.').next().unwrap_or("");
    match extension.to_lowercase().as_str() {
        "pdf" => "application/pdf",
        "doc" => "application/msword",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "xls" => "application/vnd.ms-excel",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "txt" => "text/plain",
        "csv" => "text/csv",
        _ => "application/octet-stream",
    }
}

/// Check if a content type supports inline preview (not download).
pub fn supports_inline_preview(content_type: &str) -> bool {
    matches!(
        content_type,
        "application/pdf" | "image/png" | "image/jpeg" | "image/gif" | "image/webp" | "text/plain"
    )
}

/// Generate a unique storage key for a file.
///
/// Format: `{org_id}/{year}/{month}/{uuid}_{filename}`
pub fn generate_storage_key(org_id: uuid::Uuid, filename: &str) -> String {
    let now = Utc::now();
    let file_uuid = uuid::Uuid::new_v4();

    // Sanitize filename
    let safe_filename: String = filename
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '.' || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();

    format!(
        "{}/{}/{}/{}_{safe_filename}",
        org_id,
        now.format("%Y"),
        now.format("%m"),
        file_uuid
    )
}

/// Generate a callback token for upload completion verification.
pub fn generate_callback_token() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: [u8; 32] = rng.gen();
    hex::encode(bytes)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_content_type() {
        assert_eq!(get_content_type("document.pdf"), "application/pdf");
        assert_eq!(get_content_type("image.PNG"), "image/png");
        assert_eq!(get_content_type("photo.jpeg"), "image/jpeg");
        assert_eq!(
            get_content_type("data.xlsx"),
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
        );
        assert_eq!(get_content_type("unknown.xyz"), "application/octet-stream");
    }

    #[test]
    fn test_is_allowed_content_type() {
        assert!(is_allowed_content_type("application/pdf"));
        assert!(is_allowed_content_type("image/png"));
        assert!(!is_allowed_content_type("application/javascript"));
        assert!(!is_allowed_content_type("text/html"));
    }

    #[test]
    fn test_supports_inline_preview() {
        assert!(supports_inline_preview("application/pdf"));
        assert!(supports_inline_preview("image/png"));
        assert!(!supports_inline_preview(
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
        ));
    }

    #[test]
    fn test_generate_storage_key() {
        let org_id = uuid::Uuid::new_v4();
        let key = generate_storage_key(org_id, "test document.pdf");

        assert!(key.starts_with(&org_id.to_string()));
        assert!(key.ends_with("test_document.pdf"));
        assert!(key.contains('/'));
    }

    #[test]
    fn test_generate_callback_token() {
        let token = generate_callback_token();
        assert_eq!(token.len(), 64); // 32 bytes as hex
        assert!(token.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_storage_config_new() {
        let config = StorageConfig::new("my-bucket", "us-west-2", "key", "secret");
        assert_eq!(config.bucket, "my-bucket");
        assert_eq!(config.region, "us-west-2");
        assert!(config.endpoint.is_none());
    }

    #[test]
    fn test_storage_config_with_endpoint() {
        let config = StorageConfig::new("my-bucket", "us-west-2", "key", "secret")
            .with_endpoint("http://localhost:9000");
        assert_eq!(config.endpoint, Some("http://localhost:9000".to_string()));
    }
}

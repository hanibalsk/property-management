//! Refresh token and login attempt models (Epic 1, Story 1.2).

use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

/// Refresh token entity from database.
#[derive(Debug, Clone, FromRow)]
pub struct RefreshToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub user_agent: Option<String>,
    /// IP address stored as String (INET type in PostgreSQL)
    pub ip_address: Option<String>,
    pub device_info: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_used_at: DateTime<Utc>,
}

impl RefreshToken {
    /// Check if token is expired.
    pub fn is_expired(&self) -> bool {
        self.expires_at < Utc::now()
    }

    /// Check if token has been revoked.
    pub fn is_revoked(&self) -> bool {
        self.revoked_at.is_some()
    }

    /// Check if token is valid (not expired and not revoked).
    pub fn is_valid(&self) -> bool {
        !self.is_expired() && !self.is_revoked()
    }
}

/// Data for creating a new refresh token.
#[derive(Debug, Clone)]
pub struct CreateRefreshToken {
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub user_agent: Option<String>,
    /// IP address as String
    pub ip_address: Option<String>,
    pub device_info: Option<String>,
}

/// Login attempt record.
#[derive(Debug, Clone, FromRow)]
pub struct LoginAttempt {
    pub id: Uuid,
    pub email: String,
    /// IP address stored as String (INET type in PostgreSQL)
    pub ip_address: String,
    pub success: bool,
    pub attempt_at: DateTime<Utc>,
}

/// Rate limit check result.
#[derive(Debug, Clone)]
pub struct RateLimitStatus {
    /// Number of failed attempts in the window
    pub failed_attempts: i64,
    /// Whether the account/IP is currently locked
    pub is_locked: bool,
    /// Time remaining until lockout expires (if locked)
    pub lockout_remaining_secs: Option<i64>,
}

impl RateLimitStatus {
    /// Maximum failed attempts before lockout.
    pub const MAX_FAILED_ATTEMPTS: i64 = 5;
    /// Lockout duration in minutes.
    pub const LOCKOUT_MINUTES: i64 = 15;

    /// Check if more attempts are allowed.
    pub fn can_attempt(&self) -> bool {
        !self.is_locked
    }
}

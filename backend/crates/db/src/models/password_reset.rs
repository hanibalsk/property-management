//! Password reset token models (Epic 1, Story 1.4).

use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

/// Password reset token entity from database.
#[derive(Debug, Clone, FromRow)]
pub struct PasswordResetToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl PasswordResetToken {
    /// Check if token is expired.
    pub fn is_expired(&self) -> bool {
        self.expires_at < Utc::now()
    }

    /// Check if token has been used.
    pub fn is_used(&self) -> bool {
        self.used_at.is_some()
    }

    /// Check if token is valid (not expired and not used).
    pub fn is_valid(&self) -> bool {
        !self.is_expired() && !self.is_used()
    }
}

/// Data for creating a new password reset token.
#[derive(Debug, Clone)]
pub struct CreatePasswordResetToken {
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
}

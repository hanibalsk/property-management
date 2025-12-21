//! Two-Factor Authentication models (Epic 9, Story 9.1).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Two-factor authentication record for a user.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct TwoFactorAuth {
    /// Record ID
    pub id: Uuid,
    /// User ID
    pub user_id: Uuid,
    /// Base32-encoded TOTP secret (encrypted in storage)
    pub secret: String,
    /// Whether 2FA is currently enabled
    pub enabled: bool,
    /// When 2FA was enabled
    pub enabled_at: Option<DateTime<Utc>>,
    /// Hashed backup codes as JSON array
    pub backup_codes: serde_json::Value,
    /// Remaining unused backup codes count
    pub backup_codes_remaining: i32,
    /// When record was created
    pub created_at: DateTime<Utc>,
    /// When record was last updated
    pub updated_at: DateTime<Utc>,
}

/// Data for creating a new 2FA setup.
#[derive(Debug, Clone)]
pub struct CreateTwoFactorAuth {
    /// User ID
    pub user_id: Uuid,
    /// Base32-encoded TOTP secret
    pub secret: String,
    /// Hashed backup codes
    pub backup_codes: Vec<String>,
}

/// Data for updating 2FA status.
#[derive(Debug, Clone)]
pub struct UpdateTwoFactorStatus {
    /// Whether to enable or disable
    pub enabled: bool,
}

/// Two-factor authentication status (public-facing, no secret).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwoFactorStatus {
    /// Whether 2FA is enabled
    pub enabled: bool,
    /// When 2FA was enabled (if enabled)
    pub enabled_at: Option<DateTime<Utc>>,
    /// Remaining backup codes count
    pub backup_codes_remaining: i32,
}

impl From<TwoFactorAuth> for TwoFactorStatus {
    fn from(auth: TwoFactorAuth) -> Self {
        Self {
            enabled: auth.enabled,
            enabled_at: auth.enabled_at,
            backup_codes_remaining: auth.backup_codes_remaining,
        }
    }
}

impl From<&TwoFactorAuth> for TwoFactorStatus {
    fn from(auth: &TwoFactorAuth) -> Self {
        Self {
            enabled: auth.enabled,
            enabled_at: auth.enabled_at,
            backup_codes_remaining: auth.backup_codes_remaining,
        }
    }
}

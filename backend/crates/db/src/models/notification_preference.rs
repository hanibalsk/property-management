//! Notification Preference models (Epic 8A, Story 8A.1).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Notification channel enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "notification_channel", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum NotificationChannel {
    Push,
    Email,
    InApp,
}

impl NotificationChannel {
    pub fn as_str(&self) -> &'static str {
        match self {
            NotificationChannel::Push => "push",
            NotificationChannel::Email => "email",
            NotificationChannel::InApp => "in_app",
        }
    }

    /// All available channels.
    pub fn all() -> Vec<NotificationChannel> {
        vec![
            NotificationChannel::Push,
            NotificationChannel::Email,
            NotificationChannel::InApp,
        ]
    }
}

impl std::fmt::Display for NotificationChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Notification preference entity from database.
#[derive(Debug, Clone, FromRow)]
pub struct NotificationPreference {
    pub id: Uuid,
    pub user_id: Uuid,
    pub channel: NotificationChannel,
    pub enabled: bool,
    pub updated_at: DateTime<Utc>,
}

/// Response DTO for a single notification preference.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotificationPreferenceResponse {
    pub channel: NotificationChannel,
    pub enabled: bool,
    pub updated_at: DateTime<Utc>,
}

impl From<NotificationPreference> for NotificationPreferenceResponse {
    fn from(pref: NotificationPreference) -> Self {
        Self {
            channel: pref.channel,
            enabled: pref.enabled,
            updated_at: pref.updated_at,
        }
    }
}

/// Response DTO for all notification preferences.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotificationPreferencesResponse {
    pub preferences: Vec<NotificationPreferenceResponse>,
    /// Warning message if all channels are disabled
    pub all_disabled_warning: Option<String>,
}

/// Request to update a notification preference.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNotificationPreferenceRequest {
    /// Whether to enable or disable this channel
    pub enabled: bool,
    /// Required confirmation when disabling all channels
    #[serde(default)]
    pub confirm_disable_all: bool,
}

/// Response when all channels would be disabled.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DisableAllWarningResponse {
    /// Warning message
    pub message: String,
    /// Whether confirmation is required
    pub requires_confirmation: bool,
    /// Which channel this update would affect
    pub channel: NotificationChannel,
}

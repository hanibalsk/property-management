//! Granular notification preference models (Epic 8B).
//!
//! Stories covered:
//! - 8B.1: Per-Event Type Preferences
//! - 8B.2: Per-Channel Delivery Preferences
//! - 8B.3: Notification Schedule (Do Not Disturb)
//! - 8B.4: Role-Based Default Preferences

use chrono::{DateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

// ============================================================================
// Event Category Enum
// ============================================================================

/// Notification event category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "notification_event_category", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum NotificationEventCategory {
    Fault,
    Vote,
    Announcement,
    Document,
    Message,
    Critical,
    Finance,
    Facility,
}

impl NotificationEventCategory {
    /// All available categories.
    pub fn all() -> Vec<Self> {
        vec![
            Self::Fault,
            Self::Vote,
            Self::Announcement,
            Self::Document,
            Self::Message,
            Self::Critical,
            Self::Finance,
            Self::Facility,
        ]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Fault => "fault",
            Self::Vote => "vote",
            Self::Announcement => "announcement",
            Self::Document => "document",
            Self::Message => "message",
            Self::Critical => "critical",
            Self::Finance => "finance",
            Self::Facility => "facility",
        }
    }
}

impl std::fmt::Display for NotificationEventCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================================
// Event Notification Preferences (Stories 8B.1 & 8B.2)
// ============================================================================

/// Per-event-type notification preference.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EventNotificationPreference {
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_type: String,
    pub event_category: NotificationEventCategory,
    pub push_enabled: bool,
    pub email_enabled: bool,
    pub in_app_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request to update event notification preference.
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEventPreferenceRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_app_enabled: Option<bool>,
}

/// Reference notification event type.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotificationEventType {
    pub event_type: String,
    pub category: NotificationEventCategory,
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub is_priority: bool,
    pub default_push: bool,
    pub default_email: bool,
    pub default_in_app: bool,
    pub created_at: DateTime<Utc>,
}

/// Event preference with event type details.
#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EventPreferenceWithDetails {
    pub event_type: String,
    pub category: NotificationEventCategory,
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub is_priority: bool,
    pub push_enabled: bool,
    pub email_enabled: bool,
    pub in_app_enabled: bool,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Response with all event preferences grouped by category.
#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EventPreferencesResponse {
    pub preferences: Vec<EventPreferenceWithDetails>,
    pub categories: Vec<CategorySummary>,
}

/// Summary of a category's preferences.
#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CategorySummary {
    pub category: NotificationEventCategory,
    pub display_name: String,
    pub total_events: i32,
    pub enabled_events: i32,
}

// ============================================================================
// Notification Schedule (Story 8B.3)
// ============================================================================

/// User's notification schedule/quiet hours settings.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotificationSchedule {
    pub id: Uuid,
    pub user_id: Uuid,

    // Quiet hours
    pub quiet_hours_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet_hours_start: Option<NaiveTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet_hours_end: Option<NaiveTime>,
    pub timezone: String,

    // Weekend settings
    pub weekend_quiet_hours_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekend_quiet_hours_start: Option<NaiveTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekend_quiet_hours_end: Option<NaiveTime>,

    // Digest settings
    pub digest_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest_frequency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest_time: Option<NaiveTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest_day_of_week: Option<i32>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request to update notification schedule.
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNotificationScheduleRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet_hours_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet_hours_start: Option<String>, // "HH:MM" format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet_hours_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekend_quiet_hours_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekend_quiet_hours_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekend_quiet_hours_end: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest_frequency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest_day_of_week: Option<i32>,
}

/// Response for notification schedule.
#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotificationScheduleResponse {
    pub schedule: NotificationSchedule,
    pub is_currently_quiet: bool,
}

// ============================================================================
// Role-Based Default Preferences (Story 8B.4)
// ============================================================================

/// Role-based notification defaults.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoleNotificationDefaults {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub role: String,
    pub event_preferences: serde_json::Value,
    pub default_quiet_hours_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_quiet_hours_start: Option<NaiveTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_quiet_hours_end: Option<NaiveTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request to create/update role notification defaults.
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRoleDefaultsRequest {
    pub event_preferences: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_quiet_hours_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_quiet_hours_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_quiet_hours_end: Option<String>,
}

/// Response listing all role defaults.
#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoleDefaultsListResponse {
    pub role_defaults: Vec<RoleNotificationDefaults>,
}

// ============================================================================
// Held Notifications (Story 8B.3)
// ============================================================================

/// Notification held during quiet hours.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HeldNotification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_type: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    pub channels: Vec<String>,
    pub held_at: DateTime<Utc>,
    pub release_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub released_at: Option<DateTime<Utc>>,
    pub is_priority: bool,
}

/// Request to create a held notification.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateHeldNotification {
    pub user_id: Uuid,
    pub event_type: String,
    pub title: String,
    pub body: Option<String>,
    pub data: Option<serde_json::Value>,
    pub channels: Vec<String>,
    pub release_at: DateTime<Utc>,
    pub is_priority: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_category_all() {
        let categories = NotificationEventCategory::all();
        assert_eq!(categories.len(), 8);
    }

    #[test]
    fn test_event_category_as_str() {
        assert_eq!(NotificationEventCategory::Fault.as_str(), "fault");
        assert_eq!(NotificationEventCategory::Critical.as_str(), "critical");
    }
}

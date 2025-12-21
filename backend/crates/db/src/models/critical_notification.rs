//! Critical Notification models (Epic 8A, Story 8A.2).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Critical notification entity from database.
#[derive(Debug, Clone, FromRow)]
pub struct CriticalNotification {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub title: String,
    pub message: String,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}

/// Critical notification acknowledgment entity.
#[derive(Debug, Clone, FromRow)]
pub struct CriticalNotificationAcknowledgment {
    pub id: Uuid,
    pub notification_id: Uuid,
    pub user_id: Uuid,
    pub acknowledged_at: DateTime<Utc>,
}

/// Critical notification with acknowledgment status.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CriticalNotificationResponse {
    pub id: Uuid,
    pub title: String,
    pub message: String,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub is_acknowledged: bool,
    pub acknowledged_at: Option<DateTime<Utc>>,
}

/// Request to create a critical notification.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateCriticalNotificationRequest {
    pub title: String,
    pub message: String,
}

/// Response after creating a critical notification.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateCriticalNotificationResponse {
    pub id: Uuid,
    pub title: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
}

/// Response for acknowledging a notification.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AcknowledgeCriticalNotificationResponse {
    pub notification_id: Uuid,
    pub acknowledged_at: DateTime<Utc>,
}

/// List of unacknowledged critical notifications.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UnacknowledgedNotificationsResponse {
    pub notifications: Vec<CriticalNotificationResponse>,
    pub count: i64,
}

/// Statistics for a critical notification (admin view).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CriticalNotificationStats {
    pub notification_id: Uuid,
    pub total_users: i64,
    pub acknowledged_count: i64,
    pub pending_count: i64,
}

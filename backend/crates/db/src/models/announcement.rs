//! Announcement model (Epic 6: Announcements & Communication).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Announcement status enum values.
pub mod announcement_status {
    pub const DRAFT: &str = "draft";
    pub const SCHEDULED: &str = "scheduled";
    pub const PUBLISHED: &str = "published";
    pub const ARCHIVED: &str = "archived";

    pub const ALL: &[&str] = &[DRAFT, SCHEDULED, PUBLISHED, ARCHIVED];
}

/// Announcement target type enum values.
pub mod target_type {
    pub const ALL: &str = "all";
    pub const BUILDING: &str = "building";
    pub const UNITS: &str = "units";
    pub const ROLES: &str = "roles";

    pub const ALL_TYPES: &[&str] = &[ALL, BUILDING, UNITS, ROLES];
}

// ============================================================================
// Announcement
// ============================================================================

/// Announcement entity from database.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Announcement {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub content: String,
    pub target_type: String,
    pub target_ids: serde_json::Value,
    pub status: String,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,
    pub pinned: bool,
    pub pinned_at: Option<DateTime<Utc>>,
    pub pinned_by: Option<Uuid>,
    pub comments_enabled: bool,
    pub acknowledgment_required: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Announcement {
    /// Check if announcement is in draft status.
    pub fn is_draft(&self) -> bool {
        self.status == announcement_status::DRAFT
    }

    /// Check if announcement is published.
    pub fn is_published(&self) -> bool {
        self.status == announcement_status::PUBLISHED
    }

    /// Check if announcement is scheduled.
    pub fn is_scheduled(&self) -> bool {
        self.status == announcement_status::SCHEDULED
    }

    /// Check if announcement can be edited.
    pub fn can_edit(&self) -> bool {
        self.status == announcement_status::DRAFT || self.status == announcement_status::SCHEDULED
    }

    /// Check if announcement is currently visible to users.
    pub fn is_visible(&self) -> bool {
        self.status == announcement_status::PUBLISHED
    }
}

/// Summary view of an announcement.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct AnnouncementSummary {
    pub id: Uuid,
    pub title: String,
    pub status: String,
    pub target_type: String,
    pub published_at: Option<DateTime<Utc>>,
    pub pinned: bool,
    pub comments_enabled: bool,
    pub acknowledgment_required: bool,
}

/// Announcement with additional details.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AnnouncementWithDetails {
    #[serde(flatten)]
    pub announcement: Announcement,
    pub author_name: String,
    pub read_count: i64,
    pub acknowledged_count: i64,
    pub comment_count: i64,
    pub attachment_count: i64,
}

/// Data for creating an announcement.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateAnnouncement {
    pub organization_id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub content: String,
    pub target_type: String,
    pub target_ids: Vec<Uuid>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub comments_enabled: Option<bool>,
    pub acknowledgment_required: Option<bool>,
}

/// Data for updating an announcement.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateAnnouncement {
    pub title: Option<String>,
    pub content: Option<String>,
    pub target_type: Option<String>,
    pub target_ids: Option<Vec<Uuid>>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub comments_enabled: Option<bool>,
    pub acknowledgment_required: Option<bool>,
}

/// Data for publishing an announcement.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PublishAnnouncement {
    /// If true, publish immediately. If false, use scheduled_at.
    pub immediate: bool,
}

/// Data for pinning an announcement.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PinAnnouncement {
    pub pinned: bool,
}

// ============================================================================
// Announcement Attachment
// ============================================================================

/// Announcement attachment entity.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct AnnouncementAttachment {
    pub id: Uuid,
    pub announcement_id: Uuid,
    pub file_key: String,
    pub file_name: String,
    pub file_type: String,
    pub file_size: i64,
    pub created_at: DateTime<Utc>,
}

/// Data for creating an attachment.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateAnnouncementAttachment {
    pub announcement_id: Uuid,
    pub file_key: String,
    pub file_name: String,
    pub file_type: String,
    pub file_size: i64,
}

// ============================================================================
// Announcement Read (Foundation for Story 6.2)
// ============================================================================

/// Announcement read record entity.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct AnnouncementRead {
    pub id: Uuid,
    pub announcement_id: Uuid,
    pub user_id: Uuid,
    pub read_at: DateTime<Utc>,
    pub acknowledged_at: Option<DateTime<Utc>>,
}

/// Data for marking an announcement as read.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MarkAnnouncementRead {
    pub announcement_id: Uuid,
    pub user_id: Uuid,
}

/// Data for acknowledging an announcement.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AcknowledgeAnnouncement {
    pub announcement_id: Uuid,
    pub user_id: Uuid,
}

// ============================================================================
// Query types
// ============================================================================

/// Query for listing announcements.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
pub struct AnnouncementListQuery {
    pub status: Option<Vec<String>>,
    pub target_type: Option<String>,
    pub author_id: Option<Uuid>,
    pub pinned: Option<bool>,
    pub from_date: Option<chrono::NaiveDate>,
    pub to_date: Option<chrono::NaiveDate>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Statistics for announcements.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct AnnouncementStatistics {
    pub total: i64,
    pub published: i64,
    pub draft: i64,
    pub scheduled: i64,
    pub archived: i64,
}

/// Acknowledgment statistics for a single announcement (Story 6.2).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AcknowledgmentStats {
    pub announcement_id: Uuid,
    pub total_targeted: i64,
    pub read_count: i64,
    pub acknowledged_count: i64,
    pub pending_count: i64,
}

/// Individual user's acknowledgment status for an announcement (Story 6.2).
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct UserAcknowledgmentStatus {
    pub user_id: Uuid,
    pub user_name: String,
    pub read_at: Option<DateTime<Utc>>,
    pub acknowledged_at: Option<DateTime<Utc>>,
}

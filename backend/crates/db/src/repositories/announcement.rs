//! Announcement repository (Epic 6: Announcements & Communication).

use crate::models::announcement::{
    announcement_status, AcknowledgmentStats, Announcement, AnnouncementAttachment,
    AnnouncementComment, AnnouncementListQuery, AnnouncementRead, AnnouncementStatistics,
    AnnouncementSummary, AnnouncementWithDetails, CommentWithAuthor, CommentWithAuthorRow,
    CreateAnnouncement, CreateAnnouncementAttachment, CreateComment, DeleteComment,
    UpdateAnnouncement, UserAcknowledgmentStatus,
};
use crate::DbPool;
use chrono::{DateTime, Utc};
use sqlx::{Error as SqlxError, FromRow};
use uuid::Uuid;

/// Row struct for announcement with details query.
#[derive(Debug, FromRow)]
struct AnnouncementDetailsRow {
    // Announcement fields
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
    // Joined fields
    pub author_name: String,
    pub read_count: i64,
    pub acknowledged_count: i64,
    pub comment_count: i64,
    pub attachment_count: i64,
}

/// Repository for announcement operations.
#[derive(Clone)]
pub struct AnnouncementRepository {
    pool: DbPool,
}

impl AnnouncementRepository {
    /// Create a new AnnouncementRepository.
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    // ========================================================================
    // Announcement CRUD
    // ========================================================================

    /// Create a new announcement (Story 6.1).
    pub async fn create(&self, data: CreateAnnouncement) -> Result<Announcement, SqlxError> {
        let target_ids_json = serde_json::to_value(&data.target_ids).unwrap_or_default();
        let comments_enabled = data.comments_enabled.unwrap_or(true);
        let acknowledgment_required = data.acknowledgment_required.unwrap_or(false);

        // Determine initial status based on scheduled_at
        let status = if data.scheduled_at.is_some() {
            announcement_status::SCHEDULED
        } else {
            announcement_status::DRAFT
        };

        let announcement = sqlx::query_as::<_, Announcement>(
            r#"
            INSERT INTO announcements (
                organization_id, author_id, title, content,
                target_type, target_ids, status, scheduled_at,
                comments_enabled, acknowledgment_required
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#,
        )
        .bind(data.organization_id)
        .bind(data.author_id)
        .bind(&data.title)
        .bind(&data.content)
        .bind(&data.target_type)
        .bind(&target_ids_json)
        .bind(status)
        .bind(data.scheduled_at)
        .bind(comments_enabled)
        .bind(acknowledgment_required)
        .fetch_one(&self.pool)
        .await?;

        Ok(announcement)
    }

    /// Find announcement by ID.
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Announcement>, SqlxError> {
        let announcement = sqlx::query_as::<_, Announcement>(
            r#"
            SELECT
                id, organization_id, author_id, title, content,
                target_type::text as target_type, target_ids,
                status::text as status, scheduled_at, published_at,
                pinned, pinned_at, pinned_by, comments_enabled,
                acknowledgment_required, created_at, updated_at
            FROM announcements WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(announcement)
    }

    /// Find announcement with full details.
    pub async fn find_by_id_with_details(
        &self,
        id: Uuid,
    ) -> Result<Option<AnnouncementWithDetails>, SqlxError> {
        let result = sqlx::query_as::<_, AnnouncementDetailsRow>(
            r#"
            SELECT
                a.id, a.organization_id, a.author_id, a.title, a.content,
                a.target_type::text as target_type, a.target_ids,
                a.status::text as status, a.scheduled_at, a.published_at,
                a.pinned, a.pinned_at, a.pinned_by, a.comments_enabled,
                a.acknowledgment_required, a.created_at, a.updated_at,
                u.name as author_name,
                (SELECT COUNT(*) FROM announcement_reads WHERE announcement_id = a.id) as read_count,
                (SELECT COUNT(*) FROM announcement_reads WHERE announcement_id = a.id AND acknowledged_at IS NOT NULL) as acknowledged_count,
                0::bigint as comment_count,
                (SELECT COUNT(*) FROM announcement_attachments WHERE announcement_id = a.id) as attachment_count
            FROM announcements a
            JOIN users u ON a.author_id = u.id
            WHERE a.id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|row| {
            let announcement = Announcement {
                id: row.id,
                organization_id: row.organization_id,
                author_id: row.author_id,
                title: row.title,
                content: row.content,
                target_type: row.target_type,
                target_ids: row.target_ids,
                status: row.status,
                scheduled_at: row.scheduled_at,
                published_at: row.published_at,
                pinned: row.pinned,
                pinned_at: row.pinned_at,
                pinned_by: row.pinned_by,
                comments_enabled: row.comments_enabled,
                acknowledgment_required: row.acknowledgment_required,
                created_at: row.created_at,
                updated_at: row.updated_at,
            };
            AnnouncementWithDetails {
                announcement,
                author_name: row.author_name,
                read_count: row.read_count,
                acknowledged_count: row.acknowledged_count,
                comment_count: row.comment_count,
                attachment_count: row.attachment_count,
            }
        }))
    }

    /// List announcements with filters.
    pub async fn list(
        &self,
        org_id: Uuid,
        query: AnnouncementListQuery,
    ) -> Result<Vec<AnnouncementSummary>, SqlxError> {
        let limit = query.limit.unwrap_or(50).min(100);
        let offset = query.offset.unwrap_or(0);

        // Build dynamic WHERE clause
        let mut conditions = vec!["organization_id = $1".to_string()];
        let mut param_idx = 2;

        if query.status.is_some() {
            conditions.push(format!(
                "status = ANY(${}::announcement_status[])",
                param_idx
            ));
            param_idx += 1;
        }
        if query.target_type.is_some() {
            conditions.push(format!("target_type = ${}", param_idx));
            param_idx += 1;
        }
        if query.author_id.is_some() {
            conditions.push(format!("author_id = ${}", param_idx));
            param_idx += 1;
        }
        if query.pinned.is_some() {
            conditions.push(format!("pinned = ${}", param_idx));
            param_idx += 1;
        }
        if query.from_date.is_some() {
            conditions.push(format!("published_at >= ${}", param_idx));
            param_idx += 1;
        }
        if query.to_date.is_some() {
            conditions.push(format!("published_at <= ${}", param_idx));
        }

        let where_clause = conditions.join(" AND ");

        let sql = format!(
            r#"
            SELECT
                id, title, status::text as status, target_type::text as target_type,
                published_at, pinned, comments_enabled, acknowledgment_required
            FROM announcements
            WHERE {}
            ORDER BY pinned DESC, COALESCE(published_at, created_at) DESC
            LIMIT {} OFFSET {}
            "#,
            where_clause, limit, offset
        );

        let mut query_builder = sqlx::query_as::<_, AnnouncementSummary>(&sql).bind(org_id);

        if let Some(ref status) = query.status {
            query_builder = query_builder.bind(status);
        }
        if let Some(ref target_type) = query.target_type {
            query_builder = query_builder.bind(target_type);
        }
        if let Some(author_id) = query.author_id {
            query_builder = query_builder.bind(author_id);
        }
        if let Some(pinned) = query.pinned {
            query_builder = query_builder.bind(pinned);
        }
        if let Some(from_date) = query.from_date {
            query_builder = query_builder.bind(from_date);
        }
        if let Some(to_date) = query.to_date {
            query_builder = query_builder.bind(to_date);
        }

        let announcements = query_builder.fetch_all(&self.pool).await?;
        Ok(announcements)
    }

    /// List published announcements for users (respecting targeting).
    pub async fn list_published(
        &self,
        org_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<AnnouncementSummary>, SqlxError> {
        let limit = limit.unwrap_or(50).min(100);
        let offset = offset.unwrap_or(0);

        let announcements = sqlx::query_as::<_, AnnouncementSummary>(
            r#"
            SELECT
                id, title, status::text as status, target_type::text as target_type,
                published_at, pinned, comments_enabled, acknowledgment_required
            FROM announcements
            WHERE organization_id = $1 AND status = 'published'
            ORDER BY pinned DESC, published_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(org_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(announcements)
    }

    /// Count announcements matching filters (for pagination).
    pub async fn count(
        &self,
        org_id: Uuid,
        query: AnnouncementListQuery,
    ) -> Result<i64, SqlxError> {
        // Build dynamic WHERE clause
        let mut conditions = vec!["organization_id = $1".to_string()];
        let mut param_idx = 2;

        if query.status.is_some() {
            conditions.push(format!(
                "status = ANY(${}::announcement_status[])",
                param_idx
            ));
            param_idx += 1;
        }
        if query.target_type.is_some() {
            conditions.push(format!("target_type = ${}", param_idx));
            param_idx += 1;
        }
        if query.author_id.is_some() {
            conditions.push(format!("author_id = ${}", param_idx));
            param_idx += 1;
        }
        if query.pinned.is_some() {
            conditions.push(format!("pinned = ${}", param_idx));
            param_idx += 1;
        }
        if query.from_date.is_some() {
            conditions.push(format!("published_at >= ${}", param_idx));
            param_idx += 1;
        }
        if query.to_date.is_some() {
            conditions.push(format!("published_at <= ${}", param_idx));
        }

        let where_clause = conditions.join(" AND ");
        let sql = format!(
            "SELECT COUNT(*) as count FROM announcements WHERE {}",
            where_clause
        );

        let mut query_builder = sqlx::query_scalar::<_, i64>(&sql).bind(org_id);

        if let Some(ref status) = query.status {
            query_builder = query_builder.bind(status);
        }
        if let Some(ref target_type) = query.target_type {
            query_builder = query_builder.bind(target_type);
        }
        if let Some(author_id) = query.author_id {
            query_builder = query_builder.bind(author_id);
        }
        if let Some(pinned) = query.pinned {
            query_builder = query_builder.bind(pinned);
        }
        if let Some(from_date) = query.from_date {
            query_builder = query_builder.bind(from_date);
        }
        if let Some(to_date) = query.to_date {
            query_builder = query_builder.bind(to_date);
        }

        let count = query_builder.fetch_one(&self.pool).await?;
        Ok(count)
    }

    /// Count published announcements (for pagination).
    pub async fn count_published(&self, org_id: Uuid) -> Result<i64, SqlxError> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM announcements WHERE organization_id = $1 AND status = 'published'",
        )
        .bind(org_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count)
    }

    /// Update announcement details (only in draft/scheduled status).
    pub async fn update(
        &self,
        id: Uuid,
        data: UpdateAnnouncement,
    ) -> Result<Announcement, SqlxError> {
        let target_ids_json = data
            .target_ids
            .as_ref()
            .map(|ids| serde_json::to_value(ids).unwrap_or_default());

        let announcement = sqlx::query_as::<_, Announcement>(
            r#"
            UPDATE announcements
            SET
                title = COALESCE($2, title),
                content = COALESCE($3, content),
                target_type = COALESCE($4, target_type),
                target_ids = COALESCE($5, target_ids),
                scheduled_at = COALESCE($6, scheduled_at),
                comments_enabled = COALESCE($7, comments_enabled),
                acknowledgment_required = COALESCE($8, acknowledgment_required),
                updated_at = NOW()
            WHERE id = $1 AND status IN ('draft', 'scheduled')
            RETURNING
                id, organization_id, author_id, title, content,
                target_type::text as target_type, target_ids,
                status::text as status, scheduled_at, published_at,
                pinned, pinned_at, pinned_by, comments_enabled,
                acknowledgment_required, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(&data.title)
        .bind(&data.content)
        .bind(&data.target_type)
        .bind(&target_ids_json)
        .bind(data.scheduled_at)
        .bind(data.comments_enabled)
        .bind(data.acknowledgment_required)
        .fetch_one(&self.pool)
        .await?;

        Ok(announcement)
    }

    /// Archive an announcement (soft delete).
    pub async fn archive(&self, id: Uuid) -> Result<Announcement, SqlxError> {
        let announcement = sqlx::query_as::<_, Announcement>(
            r#"
            UPDATE announcements
            SET status = 'archived', updated_at = NOW()
            WHERE id = $1
            RETURNING
                id, organization_id, author_id, title, content,
                target_type::text as target_type, target_ids,
                status::text as status, scheduled_at, published_at,
                pinned, pinned_at, pinned_by, comments_enabled,
                acknowledgment_required, created_at, updated_at
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(announcement)
    }

    /// Delete an announcement (only in draft status).
    pub async fn delete(&self, id: Uuid) -> Result<(), SqlxError> {
        sqlx::query("DELETE FROM announcements WHERE id = $1 AND status = 'draft'")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // ========================================================================
    // Publishing Operations (Story 6.1)
    // ========================================================================

    /// Publish an announcement immediately.
    pub async fn publish(&self, id: Uuid) -> Result<Announcement, SqlxError> {
        let announcement = sqlx::query_as::<_, Announcement>(
            r#"
            UPDATE announcements
            SET
                status = 'published',
                published_at = NOW(),
                updated_at = NOW()
            WHERE id = $1 AND status IN ('draft', 'scheduled')
            RETURNING
                id, organization_id, author_id, title, content,
                target_type::text as target_type, target_ids,
                status::text as status, scheduled_at, published_at,
                pinned, pinned_at, pinned_by, comments_enabled,
                acknowledgment_required, created_at, updated_at
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(announcement)
    }

    /// Schedule an announcement for future publishing.
    pub async fn schedule(
        &self,
        id: Uuid,
        scheduled_at: DateTime<Utc>,
    ) -> Result<Announcement, SqlxError> {
        let announcement = sqlx::query_as::<_, Announcement>(
            r#"
            UPDATE announcements
            SET
                status = 'scheduled',
                scheduled_at = $2,
                updated_at = NOW()
            WHERE id = $1 AND status = 'draft'
            RETURNING
                id, organization_id, author_id, title, content,
                target_type::text as target_type, target_ids,
                status::text as status, scheduled_at, published_at,
                pinned, pinned_at, pinned_by, comments_enabled,
                acknowledgment_required, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(scheduled_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(announcement)
    }

    /// Find scheduled announcements ready to be published.
    pub async fn find_scheduled_for_publishing(&self) -> Result<Vec<Announcement>, SqlxError> {
        let announcements = sqlx::query_as::<_, Announcement>(
            r#"
            SELECT
                id, organization_id, author_id, title, content,
                target_type::text as target_type, target_ids,
                status::text as status, scheduled_at, published_at,
                pinned, pinned_at, pinned_by, comments_enabled,
                acknowledgment_required, created_at, updated_at
            FROM announcements
            WHERE status = 'scheduled' AND scheduled_at <= NOW()
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(announcements)
    }

    /// Publish all scheduled announcements that are due.
    pub async fn publish_scheduled(&self) -> Result<Vec<Announcement>, SqlxError> {
        let announcements = sqlx::query_as::<_, Announcement>(
            r#"
            UPDATE announcements
            SET status = 'published', published_at = NOW(), updated_at = NOW()
            WHERE status = 'scheduled' AND scheduled_at <= NOW()
            RETURNING
                id, organization_id, author_id, title, content,
                target_type::text as target_type, target_ids,
                status::text as status, scheduled_at, published_at,
                pinned, pinned_at, pinned_by, comments_enabled,
                acknowledgment_required, created_at, updated_at
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(announcements)
    }

    // ========================================================================
    // Pinning Operations (Story 6.4)
    // ========================================================================

    /// Maximum number of pinned announcements per organization.
    const MAX_PINNED_PER_ORG: i64 = 3;

    /// Count pinned announcements for an organization.
    pub async fn count_pinned(&self, org_id: Uuid) -> Result<i64, SqlxError> {
        let (count,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM announcements WHERE organization_id = $1 AND pinned = true",
        )
        .bind(org_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count)
    }

    /// Pin an announcement.
    /// Returns error if max pinned limit (3) is reached.
    pub async fn pin(&self, id: Uuid, pinned_by: Uuid) -> Result<Announcement, SqlxError> {
        // Get announcement to check org_id and if already pinned
        let announcement = sqlx::query_as::<_, Announcement>(
            r#"
            SELECT
                id, organization_id, author_id, title, content,
                target_type::text as target_type, target_ids,
                status::text as status, scheduled_at, published_at,
                pinned, pinned_at, pinned_by, comments_enabled,
                acknowledgment_required, created_at, updated_at
            FROM announcements WHERE id = $1 AND status = 'published'
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        // If already pinned, just return it
        if announcement.pinned {
            return Ok(announcement);
        }

        // Check pinned count
        let pinned_count = self.count_pinned(announcement.organization_id).await?;
        if pinned_count >= Self::MAX_PINNED_PER_ORG {
            // Return a custom error - the caller should interpret this
            return Err(SqlxError::Protocol(format!(
                "Maximum of {} pinned announcements reached",
                Self::MAX_PINNED_PER_ORG
            )));
        }

        // Pin the announcement
        let pinned = sqlx::query_as::<_, Announcement>(
            r#"
            UPDATE announcements
            SET pinned = true, pinned_at = NOW(), pinned_by = $2, updated_at = NOW()
            WHERE id = $1 AND status = 'published'
            RETURNING
                id, organization_id, author_id, title, content,
                target_type::text as target_type, target_ids,
                status::text as status, scheduled_at, published_at,
                pinned, pinned_at, pinned_by, comments_enabled,
                acknowledgment_required, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(pinned_by)
        .fetch_one(&self.pool)
        .await?;

        Ok(pinned)
    }

    /// Unpin an announcement.
    pub async fn unpin(&self, id: Uuid) -> Result<Announcement, SqlxError> {
        let announcement = sqlx::query_as::<_, Announcement>(
            r#"
            UPDATE announcements
            SET pinned = false, pinned_at = NULL, pinned_by = NULL, updated_at = NOW()
            WHERE id = $1
            RETURNING
                id, organization_id, author_id, title, content,
                target_type::text as target_type, target_ids,
                status::text as status, scheduled_at, published_at,
                pinned, pinned_at, pinned_by, comments_enabled,
                acknowledgment_required, created_at, updated_at
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(announcement)
    }

    // ========================================================================
    // Attachments
    // ========================================================================

    /// Add an attachment to an announcement.
    pub async fn add_attachment(
        &self,
        data: CreateAnnouncementAttachment,
    ) -> Result<AnnouncementAttachment, SqlxError> {
        let attachment = sqlx::query_as::<_, AnnouncementAttachment>(
            r#"
            INSERT INTO announcement_attachments (announcement_id, file_key, file_name, file_type, file_size)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(data.announcement_id)
        .bind(&data.file_key)
        .bind(&data.file_name)
        .bind(&data.file_type)
        .bind(data.file_size)
        .fetch_one(&self.pool)
        .await?;

        Ok(attachment)
    }

    /// Get attachments for an announcement.
    pub async fn get_attachments(
        &self,
        announcement_id: Uuid,
    ) -> Result<Vec<AnnouncementAttachment>, SqlxError> {
        let attachments = sqlx::query_as::<_, AnnouncementAttachment>(
            r#"
            SELECT * FROM announcement_attachments
            WHERE announcement_id = $1
            ORDER BY created_at
            "#,
        )
        .bind(announcement_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(attachments)
    }

    /// Delete an attachment.
    pub async fn delete_attachment(&self, id: Uuid) -> Result<(), SqlxError> {
        sqlx::query("DELETE FROM announcement_attachments WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // ========================================================================
    // Read Tracking (Foundation for Story 6.2)
    // ========================================================================

    /// Mark an announcement as read by a user.
    pub async fn mark_read(
        &self,
        announcement_id: Uuid,
        user_id: Uuid,
    ) -> Result<AnnouncementRead, SqlxError> {
        let read = sqlx::query_as::<_, AnnouncementRead>(
            r#"
            INSERT INTO announcement_reads (announcement_id, user_id)
            VALUES ($1, $2)
            ON CONFLICT (announcement_id, user_id) DO UPDATE
            SET read_at = announcement_reads.read_at
            RETURNING *
            "#,
        )
        .bind(announcement_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(read)
    }

    /// Acknowledge an announcement.
    pub async fn acknowledge(
        &self,
        announcement_id: Uuid,
        user_id: Uuid,
    ) -> Result<AnnouncementRead, SqlxError> {
        let read = sqlx::query_as::<_, AnnouncementRead>(
            r#"
            INSERT INTO announcement_reads (announcement_id, user_id, acknowledged_at)
            VALUES ($1, $2, NOW())
            ON CONFLICT (announcement_id, user_id) DO UPDATE
            SET acknowledged_at = COALESCE(announcement_reads.acknowledged_at, NOW())
            RETURNING *
            "#,
        )
        .bind(announcement_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(read)
    }

    /// Get read status for a user.
    pub async fn get_read_status(
        &self,
        announcement_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<AnnouncementRead>, SqlxError> {
        let read = sqlx::query_as::<_, AnnouncementRead>(
            r#"
            SELECT * FROM announcement_reads
            WHERE announcement_id = $1 AND user_id = $2
            "#,
        )
        .bind(announcement_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(read)
    }

    // ========================================================================
    // Statistics
    // ========================================================================

    /// Get announcement statistics for an organization.
    pub async fn get_statistics(&self, org_id: Uuid) -> Result<AnnouncementStatistics, SqlxError> {
        let stats = sqlx::query_as::<_, AnnouncementStatistics>(
            r#"
            SELECT
                COUNT(*) as total,
                COUNT(*) FILTER (WHERE status = 'published') as published,
                COUNT(*) FILTER (WHERE status = 'draft') as draft,
                COUNT(*) FILTER (WHERE status = 'scheduled') as scheduled,
                COUNT(*) FILTER (WHERE status = 'archived') as archived
            FROM announcements
            WHERE organization_id = $1
            "#,
        )
        .bind(org_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(stats)
    }

    /// Count unread announcements for a user.
    pub async fn count_unread(&self, org_id: Uuid, user_id: Uuid) -> Result<i64, SqlxError> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM announcements a
            WHERE a.organization_id = $1
              AND a.status = 'published'
              AND NOT EXISTS (
                  SELECT 1 FROM announcement_reads ar
                  WHERE ar.announcement_id = a.id AND ar.user_id = $2
              )
            "#,
        )
        .bind(org_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count.0)
    }

    // ========================================================================
    // Acknowledgment Stats (Story 6.2)
    // ========================================================================

    /// Get acknowledgment statistics for an announcement (for managers).
    pub async fn get_acknowledgment_stats(
        &self,
        announcement_id: Uuid,
    ) -> Result<AcknowledgmentStats, SqlxError> {
        // Count reads and acknowledgments
        let (read_count,): (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM announcement_reads WHERE announcement_id = $1")
                .bind(announcement_id)
                .fetch_one(&self.pool)
                .await?;

        let (acknowledged_count,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM announcement_reads WHERE announcement_id = $1 AND acknowledged_at IS NOT NULL",
        )
        .bind(announcement_id)
        .fetch_one(&self.pool)
        .await?;

        // For now, total_targeted is based on all org users
        // In future, this should respect target_type and target_ids
        let (total_targeted,): (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(DISTINCT u.id)
            FROM users u
            JOIN announcements a ON a.organization_id = u.organization_id
            WHERE a.id = $1
            "#,
        )
        .bind(announcement_id)
        .fetch_one(&self.pool)
        .await
        .unwrap_or((0,));

        let pending_count = total_targeted - read_count;

        Ok(AcknowledgmentStats {
            announcement_id,
            total_targeted,
            read_count,
            acknowledged_count,
            pending_count: pending_count.max(0),
        })
    }

    /// Get list of users with their acknowledgment status for an announcement.
    pub async fn get_acknowledgment_list(
        &self,
        announcement_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<UserAcknowledgmentStatus>, SqlxError> {
        let limit = limit.unwrap_or(50).min(100);
        let offset = offset.unwrap_or(0);

        let users = sqlx::query_as::<_, UserAcknowledgmentStatus>(
            r#"
            SELECT
                u.id as user_id,
                u.name as user_name,
                ar.read_at,
                ar.acknowledged_at
            FROM users u
            JOIN announcements a ON a.organization_id = u.organization_id
            LEFT JOIN announcement_reads ar ON ar.announcement_id = a.id AND ar.user_id = u.id
            WHERE a.id = $1
            ORDER BY
                ar.acknowledged_at DESC NULLS LAST,
                ar.read_at DESC NULLS LAST,
                u.name
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(announcement_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    // ========================================================================
    // Comments (Story 6.3)
    // ========================================================================

    /// Create a new comment on an announcement.
    pub async fn create_comment(
        &self,
        data: CreateComment,
    ) -> Result<AnnouncementComment, SqlxError> {
        let comment = sqlx::query_as::<_, AnnouncementComment>(
            r#"
            INSERT INTO announcement_comments (announcement_id, user_id, parent_id, content, ai_training_consent)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(data.announcement_id)
        .bind(data.user_id)
        .bind(data.parent_id)
        .bind(&data.content)
        .bind(data.ai_training_consent)
        .fetch_one(&self.pool)
        .await?;

        Ok(comment)
    }

    /// Get a comment by ID.
    pub async fn get_comment(&self, id: Uuid) -> Result<Option<AnnouncementComment>, SqlxError> {
        let comment = sqlx::query_as::<_, AnnouncementComment>(
            "SELECT * FROM announcement_comments WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(comment)
    }

    /// Get comments for an announcement with author info.
    /// Returns top-level comments only; replies are fetched separately.
    pub async fn get_comments(
        &self,
        announcement_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<CommentWithAuthorRow>, SqlxError> {
        let limit = limit.unwrap_or(50).min(100);
        let offset = offset.unwrap_or(0);

        let comments = sqlx::query_as::<_, CommentWithAuthorRow>(
            r#"
            SELECT
                c.id, c.announcement_id, c.user_id, c.parent_id,
                c.content, u.name as author_name, c.deleted_at,
                c.created_at, c.updated_at
            FROM announcement_comments c
            JOIN users u ON c.user_id = u.id
            WHERE c.announcement_id = $1 AND c.parent_id IS NULL
            ORDER BY c.created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(announcement_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(comments)
    }

    /// Get replies to a comment.
    pub async fn get_comment_replies(
        &self,
        parent_id: Uuid,
    ) -> Result<Vec<CommentWithAuthorRow>, SqlxError> {
        let replies = sqlx::query_as::<_, CommentWithAuthorRow>(
            r#"
            SELECT
                c.id, c.announcement_id, c.user_id, c.parent_id,
                c.content, u.name as author_name, c.deleted_at,
                c.created_at, c.updated_at
            FROM announcement_comments c
            JOIN users u ON c.user_id = u.id
            WHERE c.parent_id = $1
            ORDER BY c.created_at ASC
            "#,
        )
        .bind(parent_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(replies)
    }

    /// Get threaded comments (top-level with nested replies).
    pub async fn get_threaded_comments(
        &self,
        announcement_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<CommentWithAuthor>, SqlxError> {
        // Get top-level comments
        let top_level = self.get_comments(announcement_id, limit, offset).await?;

        // Collect all parent IDs
        let parent_ids: Vec<Uuid> = top_level.iter().map(|c| c.id).collect();

        // Get all replies in one query
        let all_replies = if !parent_ids.is_empty() {
            sqlx::query_as::<_, CommentWithAuthorRow>(
                r#"
                SELECT
                    c.id, c.announcement_id, c.user_id, c.parent_id,
                    c.content, u.name as author_name, c.deleted_at,
                    c.created_at, c.updated_at
                FROM announcement_comments c
                JOIN users u ON c.user_id = u.id
                WHERE c.parent_id = ANY($1)
                ORDER BY c.created_at ASC
                "#,
            )
            .bind(&parent_ids)
            .fetch_all(&self.pool)
            .await?
        } else {
            vec![]
        };

        // Group replies by parent_id
        let mut replies_map: std::collections::HashMap<Uuid, Vec<CommentWithAuthor>> =
            std::collections::HashMap::new();
        for reply in all_replies {
            if let Some(parent_id) = reply.parent_id {
                replies_map
                    .entry(parent_id)
                    .or_default()
                    .push(reply.into_comment_with_author(None));
            }
        }

        // Build threaded structure
        let threaded: Vec<CommentWithAuthor> = top_level
            .into_iter()
            .map(|comment| {
                let replies = replies_map.remove(&comment.id);
                comment.into_comment_with_author(replies)
            })
            .collect();

        Ok(threaded)
    }

    /// Soft-delete a comment (author or manager moderation).
    pub async fn delete_comment(
        &self,
        data: DeleteComment,
    ) -> Result<AnnouncementComment, SqlxError> {
        let comment = sqlx::query_as::<_, AnnouncementComment>(
            r#"
            UPDATE announcement_comments
            SET deleted_at = NOW(), deleted_by = $2, deletion_reason = $3, updated_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING *
            "#,
        )
        .bind(data.comment_id)
        .bind(data.deleted_by)
        .bind(&data.deletion_reason)
        .fetch_one(&self.pool)
        .await?;

        Ok(comment)
    }

    /// Get comment count for an announcement (excluding deleted).
    pub async fn get_comment_count(&self, announcement_id: Uuid) -> Result<i64, SqlxError> {
        let (count,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM announcement_comments WHERE announcement_id = $1 AND deleted_at IS NULL",
        )
        .bind(announcement_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count)
    }
}

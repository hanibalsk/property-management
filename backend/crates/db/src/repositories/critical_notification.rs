//! Critical Notification repository (Epic 8A, Story 8A.2).

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::critical_notification::{
    CriticalNotification, CriticalNotificationAcknowledgment, CriticalNotificationStats,
};

/// Repository for critical notification operations.
#[derive(Clone)]
pub struct CriticalNotificationRepository {
    pool: PgPool,
}

impl CriticalNotificationRepository {
    /// Create a new repository.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new critical notification.
    pub async fn create(
        &self,
        organization_id: Uuid,
        title: &str,
        message: &str,
        created_by: Uuid,
    ) -> Result<CriticalNotification, sqlx::Error> {
        sqlx::query_as::<_, CriticalNotification>(
            r#"
            INSERT INTO critical_notifications (organization_id, title, message, created_by)
            VALUES ($1, $2, $3, $4)
            RETURNING id, organization_id, title, message, created_by, created_at
            "#,
        )
        .bind(organization_id)
        .bind(title)
        .bind(message)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
    }

    /// Get a critical notification by ID.
    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<CriticalNotification>, sqlx::Error> {
        sqlx::query_as::<_, CriticalNotification>(
            r#"
            SELECT id, organization_id, title, message, created_by, created_at
            FROM critical_notifications
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Get unacknowledged critical notifications for a user in an organization.
    pub async fn get_unacknowledged(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
    ) -> Result<Vec<CriticalNotification>, sqlx::Error> {
        sqlx::query_as::<_, CriticalNotification>(
            r#"
            SELECT cn.id, cn.organization_id, cn.title, cn.message, cn.created_by, cn.created_at
            FROM critical_notifications cn
            WHERE cn.organization_id = $1
            AND NOT EXISTS (
                SELECT 1 FROM critical_notification_acknowledgments cna
                WHERE cna.notification_id = cn.id AND cna.user_id = $2
            )
            ORDER BY cn.created_at DESC
            "#,
        )
        .bind(organization_id)
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
    }

    /// Get all critical notifications for an organization (with acknowledgment status for a user).
    pub async fn get_for_org_with_status(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
    ) -> Result<Vec<(CriticalNotification, Option<DateTime<Utc>>)>, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct NotificationWithAck {
            id: Uuid,
            organization_id: Uuid,
            title: String,
            message: String,
            created_by: Uuid,
            created_at: DateTime<Utc>,
            acknowledged_at: Option<DateTime<Utc>>,
        }

        let rows = sqlx::query_as::<_, NotificationWithAck>(
            r#"
            SELECT
                cn.id,
                cn.organization_id,
                cn.title,
                cn.message,
                cn.created_by,
                cn.created_at,
                cna.acknowledged_at
            FROM critical_notifications cn
            LEFT JOIN critical_notification_acknowledgments cna
                ON cna.notification_id = cn.id AND cna.user_id = $2
            WHERE cn.organization_id = $1
            ORDER BY cn.created_at DESC
            "#,
        )
        .bind(organization_id)
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| {
                (
                    CriticalNotification {
                        id: row.id,
                        organization_id: row.organization_id,
                        title: row.title,
                        message: row.message,
                        created_by: row.created_by,
                        created_at: row.created_at,
                    },
                    row.acknowledged_at,
                )
            })
            .collect())
    }

    /// Acknowledge a critical notification.
    pub async fn acknowledge(
        &self,
        notification_id: Uuid,
        user_id: Uuid,
    ) -> Result<CriticalNotificationAcknowledgment, sqlx::Error> {
        sqlx::query_as::<_, CriticalNotificationAcknowledgment>(
            r#"
            INSERT INTO critical_notification_acknowledgments (notification_id, user_id)
            VALUES ($1, $2)
            ON CONFLICT (notification_id, user_id) DO UPDATE SET acknowledged_at = NOW()
            RETURNING id, notification_id, user_id, acknowledged_at
            "#,
        )
        .bind(notification_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
    }

    /// Check if a user has acknowledged a notification.
    pub async fn is_acknowledged(
        &self,
        notification_id: Uuid,
        user_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM critical_notification_acknowledgments
            WHERE notification_id = $1 AND user_id = $2
            "#,
        )
        .bind(notification_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result > 0)
    }

    /// Get acknowledgment statistics for a notification.
    pub async fn get_stats(
        &self,
        notification_id: Uuid,
        organization_id: Uuid,
    ) -> Result<CriticalNotificationStats, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct StatsRow {
            total_users: i64,
            acknowledged_count: i64,
            pending_count: i64,
        }

        let row = sqlx::query_as::<_, StatsRow>(
            r#"
            SELECT
                COUNT(DISTINCT om.user_id) as total_users,
                COUNT(DISTINCT cna.user_id) as acknowledged_count,
                COUNT(DISTINCT om.user_id) - COUNT(DISTINCT cna.user_id) as pending_count
            FROM organization_members om
            LEFT JOIN critical_notification_acknowledgments cna
                ON cna.notification_id = $1 AND cna.user_id = om.user_id
            WHERE om.organization_id = $2
            AND om.status = 'active'
            "#,
        )
        .bind(notification_id)
        .bind(organization_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(CriticalNotificationStats {
            notification_id,
            total_users: row.total_users,
            acknowledged_count: row.acknowledged_count,
            pending_count: row.pending_count,
        })
    }
}

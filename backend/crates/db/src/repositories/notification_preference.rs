//! Notification Preference repository (Epic 8A, Story 8A.1).

use sqlx::PgPool;
use uuid::Uuid;

use crate::models::notification_preference::{NotificationChannel, NotificationPreference};

/// Repository for notification preference operations.
#[derive(Clone)]
pub struct NotificationPreferenceRepository {
    pool: PgPool,
}

impl NotificationPreferenceRepository {
    /// Create a new repository.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get all notification preferences for a user.
    pub async fn get_by_user(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<NotificationPreference>, sqlx::Error> {
        sqlx::query_as!(
            NotificationPreference,
            r#"
            SELECT
                id,
                user_id,
                channel as "channel: NotificationChannel",
                enabled,
                updated_at
            FROM notification_preferences
            WHERE user_id = $1
            ORDER BY channel
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
    }

    /// Get a specific channel preference for a user.
    pub async fn get_by_user_and_channel(
        &self,
        user_id: Uuid,
        channel: NotificationChannel,
    ) -> Result<Option<NotificationPreference>, sqlx::Error> {
        sqlx::query_as!(
            NotificationPreference,
            r#"
            SELECT
                id,
                user_id,
                channel as "channel: NotificationChannel",
                enabled,
                updated_at
            FROM notification_preferences
            WHERE user_id = $1 AND channel = $2
            "#,
            user_id,
            channel as NotificationChannel
        )
        .fetch_optional(&self.pool)
        .await
    }

    /// Update a channel preference for a user.
    pub async fn update_channel(
        &self,
        user_id: Uuid,
        channel: NotificationChannel,
        enabled: bool,
    ) -> Result<NotificationPreference, sqlx::Error> {
        sqlx::query_as!(
            NotificationPreference,
            r#"
            UPDATE notification_preferences
            SET enabled = $3, updated_at = NOW()
            WHERE user_id = $1 AND channel = $2
            RETURNING
                id,
                user_id,
                channel as "channel: NotificationChannel",
                enabled,
                updated_at
            "#,
            user_id,
            channel as NotificationChannel,
            enabled
        )
        .fetch_one(&self.pool)
        .await
    }

    /// Count how many channels are disabled for a user.
    pub async fn count_disabled(&self, user_id: Uuid) -> Result<i64, sqlx::Error> {
        let result = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) as "count!"
            FROM notification_preferences
            WHERE user_id = $1 AND enabled = false
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    /// Count how many channels are enabled for a user.
    pub async fn count_enabled(&self, user_id: Uuid) -> Result<i64, sqlx::Error> {
        let result = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) as "count!"
            FROM notification_preferences
            WHERE user_id = $1 AND enabled = true
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    /// Check if a user has any enabled notification channels.
    pub async fn has_any_enabled(&self, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let count = self.count_enabled(user_id).await?;
        Ok(count > 0)
    }

    /// Create default preferences for a user (if not exists).
    /// Normally handled by database trigger, but useful for testing.
    pub async fn create_defaults_for_user(&self, user_id: Uuid) -> Result<(), sqlx::Error> {
        for channel in NotificationChannel::all() {
            sqlx::query!(
                r#"
                INSERT INTO notification_preferences (user_id, channel, enabled)
                VALUES ($1, $2, true)
                ON CONFLICT (user_id, channel) DO NOTHING
                "#,
                user_id,
                channel as NotificationChannel
            )
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }

    /// Check if disabling a specific channel would result in all channels being disabled.
    pub async fn would_disable_all(
        &self,
        user_id: Uuid,
        channel: NotificationChannel,
    ) -> Result<bool, sqlx::Error> {
        // Get current enabled count
        let enabled_count = self.count_enabled(user_id).await?;

        // Check if the channel we're about to disable is currently enabled
        if let Some(pref) = self.get_by_user_and_channel(user_id, channel).await? {
            if pref.enabled && enabled_count == 1 {
                // This is the last enabled channel
                return Ok(true);
            }
        }

        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests would go here with a test database
    // For now, we'll test in integration tests
}

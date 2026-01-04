//! Notification Preference repository (Epic 8A, Story 8A.1).
//!
//! # RLS Integration
//!
//! This repository supports two usage patterns:
//!
//! 1. **RLS-aware** (recommended): Use methods with `_rls` suffix that accept an executor
//!    with RLS context already set (e.g., from `RlsConnection`).
//!
//! 2. **Legacy**: Use methods without suffix that use the internal pool. These do NOT
//!    enforce RLS and should be migrated to the RLS-aware pattern.
//!
//! ## Example
//!
//! ```rust,ignore
//! async fn get_preferences(
//!     mut rls: RlsConnection,
//!     State(state): State<AppState>,
//!     user_id: Uuid,
//! ) -> Result<Json<Vec<NotificationPreference>>> {
//!     let prefs = state.notification_pref_repo.get_by_user_rls(rls.conn(), user_id).await?;
//!     rls.release().await;
//!     Ok(Json(prefs))
//! }
//! ```

use sqlx::{Executor, PgPool, Postgres};
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

    // ========================================================================
    // RLS-aware methods (recommended)
    // ========================================================================

    /// Get all notification preferences for a user with RLS context.
    ///
    /// Use this method with an `RlsConnection` to ensure RLS policies are enforced.
    pub async fn get_by_user_rls<'e, E>(
        &self,
        executor: E,
        user_id: Uuid,
    ) -> Result<Vec<NotificationPreference>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as::<_, NotificationPreference>(
            r#"
            SELECT id, user_id, channel, enabled, updated_at
            FROM notification_preferences
            WHERE user_id = $1
            ORDER BY channel
            "#,
        )
        .bind(user_id)
        .fetch_all(executor)
        .await
    }

    /// Get a specific channel preference for a user with RLS context.
    pub async fn get_by_user_and_channel_rls<'e, E>(
        &self,
        executor: E,
        user_id: Uuid,
        channel: NotificationChannel,
    ) -> Result<Option<NotificationPreference>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as::<_, NotificationPreference>(
            r#"
            SELECT id, user_id, channel, enabled, updated_at
            FROM notification_preferences
            WHERE user_id = $1 AND channel = $2
            "#,
        )
        .bind(user_id)
        .bind(channel)
        .fetch_optional(executor)
        .await
    }

    /// Update a channel preference for a user with RLS context.
    pub async fn update_channel_rls<'e, E>(
        &self,
        executor: E,
        user_id: Uuid,
        channel: NotificationChannel,
        enabled: bool,
    ) -> Result<NotificationPreference, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as::<_, NotificationPreference>(
            r#"
            UPDATE notification_preferences
            SET enabled = $3, updated_at = NOW()
            WHERE user_id = $1 AND channel = $2
            RETURNING id, user_id, channel, enabled, updated_at
            "#,
        )
        .bind(user_id)
        .bind(channel)
        .bind(enabled)
        .fetch_one(executor)
        .await
    }

    /// Count how many channels are disabled for a user with RLS context.
    pub async fn count_disabled_rls<'e, E>(
        &self,
        executor: E,
        user_id: Uuid,
    ) -> Result<i64, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let result = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM notification_preferences
            WHERE user_id = $1 AND enabled = false
            "#,
        )
        .bind(user_id)
        .fetch_one(executor)
        .await?;

        Ok(result)
    }

    /// Count how many channels are enabled for a user with RLS context.
    pub async fn count_enabled_rls<'e, E>(
        &self,
        executor: E,
        user_id: Uuid,
    ) -> Result<i64, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let result = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM notification_preferences
            WHERE user_id = $1 AND enabled = true
            "#,
        )
        .bind(user_id)
        .fetch_one(executor)
        .await?;

        Ok(result)
    }

    /// Create default preferences for a user (if not exists) with RLS context.
    /// Normally handled by database trigger, but useful for testing.
    pub async fn create_defaults_for_user_rls<'e, E>(
        &self,
        executor: E,
        user_id: Uuid,
    ) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        // Note: Since executor is consumed after first use, we need to use a single query
        // that handles all channels at once. Using UNNEST for batch insert.
        let channels = NotificationChannel::all();
        let channel_strs: Vec<&str> = channels.iter().map(|c| c.as_str()).collect();

        sqlx::query(
            r#"
            INSERT INTO notification_preferences (user_id, channel, enabled)
            SELECT $1, unnest($2::text[]), true
            ON CONFLICT (user_id, channel) DO NOTHING
            "#,
        )
        .bind(user_id)
        .bind(&channel_strs)
        .execute(executor)
        .await?;

        Ok(())
    }

    // ========================================================================
    // Legacy methods (use pool directly - migrate to RLS versions)
    // ========================================================================

    /// Get all notification preferences for a user.
    ///
    /// **Deprecated**: Use `get_by_user_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use get_by_user_rls with RlsConnection instead"
    )]
    pub async fn get_by_user(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<NotificationPreference>, sqlx::Error> {
        self.get_by_user_rls(&self.pool, user_id).await
    }

    /// Get a specific channel preference for a user.
    ///
    /// **Deprecated**: Use `get_by_user_and_channel_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use get_by_user_and_channel_rls with RlsConnection instead"
    )]
    pub async fn get_by_user_and_channel(
        &self,
        user_id: Uuid,
        channel: NotificationChannel,
    ) -> Result<Option<NotificationPreference>, sqlx::Error> {
        self.get_by_user_and_channel_rls(&self.pool, user_id, channel)
            .await
    }

    /// Update a channel preference for a user.
    ///
    /// **Deprecated**: Use `update_channel_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use update_channel_rls with RlsConnection instead"
    )]
    pub async fn update_channel(
        &self,
        user_id: Uuid,
        channel: NotificationChannel,
        enabled: bool,
    ) -> Result<NotificationPreference, sqlx::Error> {
        self.update_channel_rls(&self.pool, user_id, channel, enabled)
            .await
    }

    /// Count how many channels are disabled for a user.
    ///
    /// **Deprecated**: Use `count_disabled_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use count_disabled_rls with RlsConnection instead"
    )]
    pub async fn count_disabled(&self, user_id: Uuid) -> Result<i64, sqlx::Error> {
        self.count_disabled_rls(&self.pool, user_id).await
    }

    /// Count how many channels are enabled for a user.
    ///
    /// **Deprecated**: Use `count_enabled_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use count_enabled_rls with RlsConnection instead"
    )]
    pub async fn count_enabled(&self, user_id: Uuid) -> Result<i64, sqlx::Error> {
        self.count_enabled_rls(&self.pool, user_id).await
    }

    /// Check if a user has any enabled notification channels.
    ///
    /// **Deprecated**: Use `count_enabled_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use count_enabled_rls with RlsConnection instead"
    )]
    #[allow(deprecated)]
    pub async fn has_any_enabled(&self, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let count = self.count_enabled(user_id).await?;
        Ok(count > 0)
    }

    /// Create default preferences for a user (if not exists).
    /// Normally handled by database trigger, but useful for testing.
    ///
    /// **Deprecated**: Use `create_defaults_for_user_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use create_defaults_for_user_rls with RlsConnection instead"
    )]
    pub async fn create_defaults_for_user(&self, user_id: Uuid) -> Result<(), sqlx::Error> {
        for channel in NotificationChannel::all() {
            sqlx::query(
                r#"
                INSERT INTO notification_preferences (user_id, channel, enabled)
                VALUES ($1, $2, true)
                ON CONFLICT (user_id, channel) DO NOTHING
                "#,
            )
            .bind(user_id)
            .bind(channel)
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }

    /// Check if disabling a specific channel would result in all channels being disabled.
    ///
    /// **Deprecated**: Use the RLS-aware methods `count_enabled_rls` and
    /// `get_by_user_and_channel_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use count_enabled_rls and get_by_user_and_channel_rls with RlsConnection instead"
    )]
    #[allow(deprecated)]
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
    // Tests would go here with a test database
    // For now, we'll test in integration tests
}

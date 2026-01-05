//! Two-Factor Authentication repository (Epic 9, Story 9.1).
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
//! async fn enable_2fa(
//!     mut rls: RlsConnection,
//!     State(state): State<AppState>,
//!     user_id: Uuid,
//! ) -> Result<Json<TwoFactorAuth>> {
//!     let auth = state.two_factor_auth_repo.enable_rls(rls.conn(), user_id).await?;
//!     rls.release().await;
//!     Ok(Json(auth))
//! }
//! ```

use crate::models::two_factor_auth::{CreateTwoFactorAuth, TwoFactorAuth};
use crate::DbPool;
use chrono::Utc;
use sqlx::{Error as SqlxError, Executor, Postgres};
use uuid::Uuid;

/// Repository for two-factor authentication operations.
#[derive(Clone)]
pub struct TwoFactorAuthRepository {
    pool: DbPool,
}

impl TwoFactorAuthRepository {
    /// Create a new repository instance.
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    // ========================================================================
    // RLS-aware methods (recommended)
    // ========================================================================

    /// Create a new 2FA setup for a user with RLS context.
    /// This creates the record but doesn't enable 2FA yet (pending verification).
    ///
    /// Use this method with an `RlsConnection` to ensure RLS policies are enforced.
    pub async fn create_rls<'e, E>(
        &self,
        executor: E,
        data: CreateTwoFactorAuth,
    ) -> Result<TwoFactorAuth, SqlxError>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let backup_codes_json = serde_json::to_value(&data.backup_codes)
            .map_err(|e| SqlxError::Protocol(e.to_string()))?;

        sqlx::query_as::<_, TwoFactorAuth>(
            r#"
            INSERT INTO user_2fa (user_id, secret, backup_codes, backup_codes_remaining, enabled)
            VALUES ($1, $2, $3, $4, false)
            ON CONFLICT (user_id) DO UPDATE SET
                secret = EXCLUDED.secret,
                backup_codes = EXCLUDED.backup_codes,
                backup_codes_remaining = EXCLUDED.backup_codes_remaining,
                enabled = false,
                enabled_at = NULL,
                updated_at = NOW()
            RETURNING *
            "#,
        )
        .bind(data.user_id)
        .bind(&data.secret)
        .bind(&backup_codes_json)
        .bind(data.backup_codes.len() as i32)
        .fetch_one(executor)
        .await
    }

    /// Get 2FA settings for a user with RLS context.
    pub async fn get_by_user_id_rls<'e, E>(
        &self,
        executor: E,
        user_id: Uuid,
    ) -> Result<Option<TwoFactorAuth>, SqlxError>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as::<_, TwoFactorAuth>(
            r#"
            SELECT * FROM user_2fa WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(executor)
        .await
    }

    /// Enable 2FA for a user (after successful verification) with RLS context.
    pub async fn enable_rls<'e, E>(
        &self,
        executor: E,
        user_id: Uuid,
    ) -> Result<Option<TwoFactorAuth>, SqlxError>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as::<_, TwoFactorAuth>(
            r#"
            UPDATE user_2fa
            SET enabled = true, enabled_at = $2, updated_at = NOW()
            WHERE user_id = $1
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(Utc::now())
        .fetch_optional(executor)
        .await
    }

    /// Disable 2FA for a user with RLS context.
    pub async fn disable_rls<'e, E>(
        &self,
        executor: E,
        user_id: Uuid,
    ) -> Result<Option<TwoFactorAuth>, SqlxError>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as::<_, TwoFactorAuth>(
            r#"
            UPDATE user_2fa
            SET enabled = false, enabled_at = NULL, updated_at = NOW()
            WHERE user_id = $1
            RETURNING *
            "#,
        )
        .bind(user_id)
        .fetch_optional(executor)
        .await
    }

    /// Delete 2FA record entirely with RLS context.
    pub async fn delete_rls<'e, E>(&self, executor: E, user_id: Uuid) -> Result<bool, SqlxError>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let result = sqlx::query(
            r#"
            DELETE FROM user_2fa WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .execute(executor)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Use a backup code (decrement count and mark code as used) with RLS context.
    /// Returns the updated record if successful.
    ///
    /// Note: This method requires fetching the current state first, so it uses
    /// an Acquire-capable executor to obtain multiple connections if needed.
    pub async fn use_backup_code_rls<'e, E>(
        &self,
        executor: E,
        user_id: Uuid,
        code_index: usize,
    ) -> Result<Option<TwoFactorAuth>, SqlxError>
    where
        E: Executor<'e, Database = Postgres>,
    {
        // For RLS version, we perform this in a single update using JSON functions
        // to avoid needing multiple queries
        sqlx::query_as::<_, TwoFactorAuth>(
            r#"
            WITH current_codes AS (
                SELECT user_id, backup_codes
                FROM user_2fa
                WHERE user_id = $1
            ),
            updated_codes AS (
                SELECT
                    user_id,
                    jsonb_agg(
                        CASE
                            WHEN (row_number() OVER ()) - 1 = $2 THEN ''::jsonb
                            ELSE elem
                        END
                        ORDER BY idx
                    ) as new_codes
                FROM current_codes,
                     jsonb_array_elements(backup_codes) WITH ORDINALITY AS t(elem, idx)
                GROUP BY user_id
            )
            UPDATE user_2fa u
            SET
                backup_codes = uc.new_codes,
                backup_codes_remaining = (
                    SELECT COUNT(*)
                    FROM jsonb_array_elements_text(uc.new_codes) AS code
                    WHERE code != ''
                ),
                updated_at = NOW()
            FROM updated_codes uc
            WHERE u.user_id = uc.user_id
              AND u.user_id = $1
              AND jsonb_array_length(u.backup_codes) > $2
            RETURNING u.*
            "#,
        )
        .bind(user_id)
        .bind(code_index as i64)
        .fetch_optional(executor)
        .await
    }

    /// Regenerate backup codes (replaces all existing codes) with RLS context.
    pub async fn regenerate_backup_codes_rls<'e, E>(
        &self,
        executor: E,
        user_id: Uuid,
        new_codes: Vec<String>,
    ) -> Result<Option<TwoFactorAuth>, SqlxError>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let codes_json =
            serde_json::to_value(&new_codes).map_err(|e| SqlxError::Protocol(e.to_string()))?;

        sqlx::query_as::<_, TwoFactorAuth>(
            r#"
            UPDATE user_2fa
            SET backup_codes = $2, backup_codes_remaining = $3, updated_at = NOW()
            WHERE user_id = $1
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(&codes_json)
        .bind(new_codes.len() as i32)
        .fetch_optional(executor)
        .await
    }

    // ========================================================================
    // Legacy methods (use pool directly - migrate to RLS versions)
    // ========================================================================

    /// Create a new 2FA setup for a user.
    /// This creates the record but doesn't enable 2FA yet (pending verification).
    ///
    /// **Deprecated**: Use `create_rls` with an RLS-enabled connection instead.
    #[deprecated(since = "0.2.276", note = "Use create_rls with RlsConnection instead")]
    pub async fn create(&self, data: CreateTwoFactorAuth) -> Result<TwoFactorAuth, SqlxError> {
        self.create_rls(&self.pool, data).await
    }

    /// Get 2FA settings for a user.
    ///
    /// **Deprecated**: Use `get_by_user_id_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use get_by_user_id_rls with RlsConnection instead"
    )]
    pub async fn get_by_user_id(&self, user_id: Uuid) -> Result<Option<TwoFactorAuth>, SqlxError> {
        self.get_by_user_id_rls(&self.pool, user_id).await
    }

    /// Enable 2FA for a user (after successful verification).
    ///
    /// **Deprecated**: Use `enable_rls` with an RLS-enabled connection instead.
    #[deprecated(since = "0.2.276", note = "Use enable_rls with RlsConnection instead")]
    pub async fn enable(&self, user_id: Uuid) -> Result<Option<TwoFactorAuth>, SqlxError> {
        self.enable_rls(&self.pool, user_id).await
    }

    /// Disable 2FA for a user.
    ///
    /// **Deprecated**: Use `disable_rls` with an RLS-enabled connection instead.
    #[deprecated(since = "0.2.276", note = "Use disable_rls with RlsConnection instead")]
    pub async fn disable(&self, user_id: Uuid) -> Result<Option<TwoFactorAuth>, SqlxError> {
        self.disable_rls(&self.pool, user_id).await
    }

    /// Delete 2FA record entirely.
    ///
    /// **Deprecated**: Use `delete_rls` with an RLS-enabled connection instead.
    #[deprecated(since = "0.2.276", note = "Use delete_rls with RlsConnection instead")]
    pub async fn delete(&self, user_id: Uuid) -> Result<bool, SqlxError> {
        self.delete_rls(&self.pool, user_id).await
    }

    /// Use a backup code (decrement count and mark code as used).
    /// Returns the updated record if successful.
    ///
    /// **Deprecated**: Use `use_backup_code_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use use_backup_code_rls with RlsConnection instead"
    )]
    #[allow(deprecated)]
    pub async fn use_backup_code(
        &self,
        user_id: Uuid,
        code_index: usize,
    ) -> Result<Option<TwoFactorAuth>, SqlxError> {
        // Legacy implementation uses the original multi-query approach
        // First get the current backup codes
        let current = match self.get_by_user_id(user_id).await? {
            Some(auth) => auth,
            None => return Ok(None),
        };

        // Remove the used code from the array
        let mut codes: Vec<String> = serde_json::from_value(current.backup_codes.clone())
            .map_err(|e| SqlxError::Protocol(e.to_string()))?;

        if code_index >= codes.len() {
            return Ok(None);
        }

        // Mark the code as used by replacing it with empty string
        codes[code_index] = String::new();

        let remaining = codes.iter().filter(|c| !c.is_empty()).count() as i32;
        let codes_json =
            serde_json::to_value(&codes).map_err(|e| SqlxError::Protocol(e.to_string()))?;

        sqlx::query_as::<_, TwoFactorAuth>(
            r#"
            UPDATE user_2fa
            SET backup_codes = $2, backup_codes_remaining = $3, updated_at = NOW()
            WHERE user_id = $1
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(&codes_json)
        .bind(remaining)
        .fetch_optional(&self.pool)
        .await
    }

    /// Regenerate backup codes (replaces all existing codes).
    ///
    /// **Deprecated**: Use `regenerate_backup_codes_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use regenerate_backup_codes_rls with RlsConnection instead"
    )]
    pub async fn regenerate_backup_codes(
        &self,
        user_id: Uuid,
        new_codes: Vec<String>,
    ) -> Result<Option<TwoFactorAuth>, SqlxError> {
        self.regenerate_backup_codes_rls(&self.pool, user_id, new_codes)
            .await
    }
}

//! Two-Factor Authentication repository (Epic 9, Story 9.1).

use crate::models::two_factor_auth::{CreateTwoFactorAuth, TwoFactorAuth};
use crate::DbPool;
use chrono::Utc;
use sqlx::Error as SqlxError;
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

    /// Create a new 2FA setup for a user.
    /// This creates the record but doesn't enable 2FA yet (pending verification).
    pub async fn create(&self, data: CreateTwoFactorAuth) -> Result<TwoFactorAuth, SqlxError> {
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
        .fetch_one(&self.pool)
        .await
    }

    /// Get 2FA settings for a user.
    pub async fn get_by_user_id(&self, user_id: Uuid) -> Result<Option<TwoFactorAuth>, SqlxError> {
        sqlx::query_as::<_, TwoFactorAuth>(
            r#"
            SELECT * FROM user_2fa WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Enable 2FA for a user (after successful verification).
    pub async fn enable(&self, user_id: Uuid) -> Result<Option<TwoFactorAuth>, SqlxError> {
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
        .fetch_optional(&self.pool)
        .await
    }

    /// Disable 2FA for a user.
    pub async fn disable(&self, user_id: Uuid) -> Result<Option<TwoFactorAuth>, SqlxError> {
        sqlx::query_as::<_, TwoFactorAuth>(
            r#"
            UPDATE user_2fa
            SET enabled = false, enabled_at = NULL, updated_at = NOW()
            WHERE user_id = $1
            RETURNING *
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Delete 2FA record entirely.
    pub async fn delete(&self, user_id: Uuid) -> Result<bool, SqlxError> {
        let result = sqlx::query(
            r#"
            DELETE FROM user_2fa WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Use a backup code (decrement count and mark code as used).
    /// Returns the updated record if successful.
    pub async fn use_backup_code(
        &self,
        user_id: Uuid,
        code_index: usize,
    ) -> Result<Option<TwoFactorAuth>, SqlxError> {
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
    pub async fn regenerate_backup_codes(
        &self,
        user_id: Uuid,
        new_codes: Vec<String>,
    ) -> Result<Option<TwoFactorAuth>, SqlxError> {
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
        .fetch_optional(&self.pool)
        .await
    }
}

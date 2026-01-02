//! Feature Flag repository (Epic 10B, Story 10B.2).
//!
//! Repository for feature flag management with override support.

use crate::models::platform_admin::{FeatureFlag, FeatureFlagOverride, FeatureFlagScope};
use crate::DbPool;
use sqlx::Error as SqlxError;
use uuid::Uuid;

/// Repository for feature flag operations.
#[derive(Clone)]
pub struct FeatureFlagRepository {
    pool: DbPool,
}

impl FeatureFlagRepository {
    /// Create a new FeatureFlagRepository.
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// List all feature flags with override counts.
    pub async fn list_all(&self) -> Result<Vec<FeatureFlagWithCount>, SqlxError> {
        let flags = sqlx::query_as::<_, FeatureFlagWithCount>(
            r#"
            SELECT
                f.id, f.key, f.name, f.description, f.is_enabled,
                f.created_at, f.updated_at,
                COUNT(o.id) as override_count
            FROM feature_flags f
            LEFT JOIN feature_flag_overrides o ON o.flag_id = f.id
            GROUP BY f.id, f.key, f.name, f.description, f.is_enabled, f.created_at, f.updated_at
            ORDER BY f.key
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(flags)
    }

    /// Get a feature flag by ID with all its overrides.
    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<FeatureFlagWithOverrides>, SqlxError> {
        let flag = sqlx::query_as::<_, FeatureFlag>(
            r#"
            SELECT id, key, name, description, is_enabled, created_at, updated_at
            FROM feature_flags
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        let flag = match flag {
            Some(f) => f,
            None => return Ok(None),
        };

        let overrides = sqlx::query_as::<_, FeatureFlagOverride>(
            r#"
            SELECT id, flag_id, scope_type, scope_id, is_enabled, created_at
            FROM feature_flag_overrides
            WHERE flag_id = $1
            ORDER BY scope_type, created_at
            "#,
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await?;

        Ok(Some(FeatureFlagWithOverrides { flag, overrides }))
    }

    /// Get a feature flag by key.
    pub async fn get_by_key(&self, key: &str) -> Result<Option<FeatureFlag>, SqlxError> {
        let flag = sqlx::query_as::<_, FeatureFlag>(
            r#"
            SELECT id, key, name, description, is_enabled, created_at, updated_at
            FROM feature_flags
            WHERE key = $1
            "#,
        )
        .bind(key)
        .fetch_optional(&self.pool)
        .await?;

        Ok(flag)
    }

    /// Create a new feature flag.
    pub async fn create(
        &self,
        key: &str,
        name: &str,
        description: Option<&str>,
        is_enabled: bool,
    ) -> Result<FeatureFlag, SqlxError> {
        let flag = sqlx::query_as::<_, FeatureFlag>(
            r#"
            INSERT INTO feature_flags (key, name, description, is_enabled)
            VALUES ($1, $2, $3, $4)
            RETURNING id, key, name, description, is_enabled, created_at, updated_at
            "#,
        )
        .bind(key)
        .bind(name)
        .bind(description)
        .bind(is_enabled)
        .fetch_one(&self.pool)
        .await?;

        Ok(flag)
    }

    /// Update a feature flag.
    pub async fn update(
        &self,
        id: Uuid,
        name: Option<&str>,
        description: Option<&str>,
        is_enabled: Option<bool>,
    ) -> Result<Option<FeatureFlag>, SqlxError> {
        let flag = sqlx::query_as::<_, FeatureFlag>(
            r#"
            UPDATE feature_flags
            SET
                name = COALESCE($2, name),
                description = COALESCE($3, description),
                is_enabled = COALESCE($4, is_enabled),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, key, name, description, is_enabled, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(name)
        .bind(description)
        .bind(is_enabled)
        .fetch_optional(&self.pool)
        .await?;

        Ok(flag)
    }

    /// Toggle a feature flag's global enabled state.
    pub async fn toggle(&self, id: Uuid) -> Result<Option<FeatureFlag>, SqlxError> {
        let flag = sqlx::query_as::<_, FeatureFlag>(
            r#"
            UPDATE feature_flags
            SET is_enabled = NOT is_enabled, updated_at = NOW()
            WHERE id = $1
            RETURNING id, key, name, description, is_enabled, created_at, updated_at
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(flag)
    }

    /// Delete a feature flag and all its overrides.
    pub async fn delete(&self, id: Uuid) -> Result<bool, SqlxError> {
        let result = sqlx::query("DELETE FROM feature_flags WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Create an override for a feature flag.
    pub async fn create_override(
        &self,
        flag_id: Uuid,
        scope_type: FeatureFlagScope,
        scope_id: Uuid,
        is_enabled: bool,
    ) -> Result<FeatureFlagOverride, SqlxError> {
        let override_record = sqlx::query_as::<_, FeatureFlagOverride>(
            r#"
            INSERT INTO feature_flag_overrides (flag_id, scope_type, scope_id, is_enabled)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (flag_id, scope_type, scope_id)
            DO UPDATE SET is_enabled = $4
            RETURNING id, flag_id, scope_type, scope_id, is_enabled, created_at
            "#,
        )
        .bind(flag_id)
        .bind(scope_type.as_str())
        .bind(scope_id)
        .bind(is_enabled)
        .fetch_one(&self.pool)
        .await?;

        Ok(override_record)
    }

    /// Delete an override.
    pub async fn delete_override(&self, override_id: Uuid) -> Result<bool, SqlxError> {
        let result = sqlx::query("DELETE FROM feature_flag_overrides WHERE id = $1")
            .bind(override_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Resolve whether a feature is enabled for a given context.
    /// Resolution order: user override → org override → role override → global default
    pub async fn is_enabled_for_context(
        &self,
        flag_key: &str,
        user_id: Option<Uuid>,
        org_id: Option<Uuid>,
        role_id: Option<Uuid>,
    ) -> Result<Option<bool>, SqlxError> {
        // First get the flag
        let flag = match self.get_by_key(flag_key).await? {
            Some(f) => f,
            None => return Ok(None),
        };

        // Check user override first (highest priority)
        if let Some(uid) = user_id {
            if let Some(override_value) = self.get_override_value(flag.id, "user", uid).await? {
                return Ok(Some(override_value));
            }
        }

        // Check org override
        if let Some(oid) = org_id {
            if let Some(override_value) = self
                .get_override_value(flag.id, "organization", oid)
                .await?
            {
                return Ok(Some(override_value));
            }
        }

        // Check role override
        if let Some(rid) = role_id {
            if let Some(override_value) = self.get_override_value(flag.id, "role", rid).await? {
                return Ok(Some(override_value));
            }
        }

        // Return global default
        Ok(Some(flag.is_enabled))
    }

    /// Get override value for a specific scope.
    async fn get_override_value(
        &self,
        flag_id: Uuid,
        scope_type: &str,
        scope_id: Uuid,
    ) -> Result<Option<bool>, SqlxError> {
        let result = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT is_enabled FROM feature_flag_overrides
            WHERE flag_id = $1 AND scope_type = $2 AND scope_id = $3
            "#,
        )
        .bind(flag_id)
        .bind(scope_type)
        .bind(scope_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    /// Get all flags resolved for a user context.
    pub async fn resolve_all_for_context(
        &self,
        user_id: Option<Uuid>,
        org_id: Option<Uuid>,
        role_id: Option<Uuid>,
    ) -> Result<Vec<ResolvedFeatureFlag>, SqlxError> {
        let flags = self.list_all().await?;
        let mut resolved = Vec::new();

        for flag in flags {
            let is_enabled = self
                .is_enabled_for_context(&flag.key, user_id, org_id, role_id)
                .await?
                .unwrap_or(flag.is_enabled);

            resolved.push(ResolvedFeatureFlag {
                key: flag.key,
                is_enabled,
            });
        }

        Ok(resolved)
    }
}

/// Feature flag with override count for list view.
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct FeatureFlagWithCount {
    pub id: Uuid,
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub is_enabled: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub override_count: i64,
}

impl From<FeatureFlag> for FeatureFlagWithCount {
    fn from(flag: FeatureFlag) -> Self {
        Self {
            id: flag.id,
            key: flag.key,
            name: flag.name,
            description: flag.description,
            is_enabled: flag.is_enabled,
            created_at: flag.created_at,
            updated_at: flag.updated_at,
            override_count: 0,
        }
    }
}

/// Feature flag with all its overrides.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct FeatureFlagWithOverrides {
    pub flag: FeatureFlag,
    pub overrides: Vec<FeatureFlagOverride>,
}

/// Resolved feature flag for client consumption.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResolvedFeatureFlag {
    pub key: String,
    pub is_enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolved_feature_flag() {
        let flag = ResolvedFeatureFlag {
            key: "test_feature".to_string(),
            is_enabled: true,
        };

        assert_eq!(flag.key, "test_feature");
        assert!(flag.is_enabled);
    }

    #[test]
    fn test_feature_flag_scope_as_str() {
        assert_eq!(FeatureFlagScope::Organization.as_str(), "organization");
        assert_eq!(FeatureFlagScope::User.as_str(), "user");
        assert_eq!(FeatureFlagScope::Role.as_str(), "role");
    }
}

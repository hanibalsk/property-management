//! Infrastructure repository (Epic 89).
//!
//! Repository for feature flags, health monitoring, and alerts.

use crate::models::infrastructure::{
    AlertSeverity, CreateFeatureFlag, FeatureFlag, FeatureFlagAuditAction, FeatureFlagAuditLog,
    FeatureFlagOverride, FeatureFlagOverrideType, HealthAlert, HealthCheckConfig,
    HealthCheckResult, HealthStatus, UpdateFeatureFlag,
};
use crate::DbPool;
use chrono::{DateTime, Utc};
use sqlx::Error as SqlxError;
use uuid::Uuid;

/// Repository for infrastructure operations.
#[derive(Clone)]
pub struct InfrastructureRepository {
    pool: DbPool,
}

impl InfrastructureRepository {
    /// Create a new InfrastructureRepository.
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    // ==================== Feature Flag Operations (Story 89.1) ====================

    /// List all feature flags with pagination.
    pub async fn list_feature_flags(
        &self,
        environment: Option<&str>,
        enabled: Option<bool>,
        tag: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<FeatureFlag>, i64), SqlxError> {
        // Count total
        let total = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM infrastructure_feature_flags
            WHERE ($1::VARCHAR IS NULL OR environment = $1)
              AND ($2::BOOLEAN IS NULL OR enabled = $2)
              AND ($3::TEXT IS NULL OR $3 = ANY(tags))
            "#,
        )
        .bind(environment)
        .bind(enabled)
        .bind(tag)
        .fetch_one(&self.pool)
        .await?;

        // Fetch flags
        let flags = sqlx::query_as::<_, FeatureFlag>(
            r#"
            SELECT id, key, name, description, enabled, rollout_percentage,
                   targeting_rules, default_value, value_type, environment, tags,
                   created_by, created_at, updated_at, last_evaluated_at
            FROM infrastructure_feature_flags
            WHERE ($1::VARCHAR IS NULL OR environment = $1)
              AND ($2::BOOLEAN IS NULL OR enabled = $2)
              AND ($3::TEXT IS NULL OR $3 = ANY(tags))
            ORDER BY key ASC
            LIMIT $4 OFFSET $5
            "#,
        )
        .bind(environment)
        .bind(enabled)
        .bind(tag)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok((flags, total))
    }

    /// Get a feature flag by ID.
    pub async fn get_feature_flag(&self, id: Uuid) -> Result<Option<FeatureFlag>, SqlxError> {
        let flag = sqlx::query_as::<_, FeatureFlag>(
            r#"
            SELECT id, key, name, description, enabled, rollout_percentage,
                   targeting_rules, default_value, value_type, environment, tags,
                   created_by, created_at, updated_at, last_evaluated_at
            FROM infrastructure_feature_flags
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(flag)
    }

    /// Get a feature flag by key.
    pub async fn get_feature_flag_by_key(
        &self,
        key: &str,
    ) -> Result<Option<FeatureFlag>, SqlxError> {
        let flag = sqlx::query_as::<_, FeatureFlag>(
            r#"
            SELECT id, key, name, description, enabled, rollout_percentage,
                   targeting_rules, default_value, value_type, environment, tags,
                   created_by, created_at, updated_at, last_evaluated_at
            FROM infrastructure_feature_flags
            WHERE key = $1
            "#,
        )
        .bind(key)
        .fetch_optional(&self.pool)
        .await?;

        Ok(flag)
    }

    /// Create a new feature flag.
    pub async fn create_feature_flag(
        &self,
        data: CreateFeatureFlag,
        created_by: Uuid,
    ) -> Result<FeatureFlag, SqlxError> {
        let flag = sqlx::query_as::<_, FeatureFlag>(
            r#"
            INSERT INTO infrastructure_feature_flags (
                key, name, description, enabled, rollout_percentage,
                default_value, value_type, environment, tags, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id, key, name, description, enabled, rollout_percentage,
                      targeting_rules, default_value, value_type, environment, tags,
                      created_by, created_at, updated_at, last_evaluated_at
            "#,
        )
        .bind(&data.key)
        .bind(&data.name)
        .bind(&data.description)
        .bind(data.enabled)
        .bind(data.rollout_percentage.unwrap_or(100))
        .bind(&data.default_value)
        .bind(&data.value_type)
        .bind(&data.environment)
        .bind(&data.tags)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await?;

        // Log audit event
        self.log_flag_audit(
            flag.id,
            FeatureFlagAuditAction::Created,
            Some(created_by),
            None,
            Some(serde_json::to_value(&flag).unwrap_or_default()),
            None,
        )
        .await?;

        Ok(flag)
    }

    /// Update a feature flag.
    pub async fn update_feature_flag(
        &self,
        id: Uuid,
        data: UpdateFeatureFlag,
        updated_by: Uuid,
    ) -> Result<Option<FeatureFlag>, SqlxError> {
        // Get previous state for audit
        let previous = self.get_feature_flag(id).await?;
        if previous.is_none() {
            return Ok(None);
        }

        let flag = sqlx::query_as::<_, FeatureFlag>(
            r#"
            UPDATE infrastructure_feature_flags
            SET name = COALESCE($2, name),
                description = COALESCE($3, description),
                enabled = COALESCE($4, enabled),
                rollout_percentage = COALESCE($5, rollout_percentage),
                targeting_rules = COALESCE($6, targeting_rules),
                default_value = COALESCE($7, default_value),
                tags = COALESCE($8, tags),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, key, name, description, enabled, rollout_percentage,
                      targeting_rules, default_value, value_type, environment, tags,
                      created_by, created_at, updated_at, last_evaluated_at
            "#,
        )
        .bind(id)
        .bind(&data.name)
        .bind(&data.description)
        .bind(data.enabled)
        .bind(data.rollout_percentage)
        .bind(&data.targeting_rules)
        .bind(&data.default_value)
        .bind(&data.tags)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(ref f) = flag {
            self.log_flag_audit(
                id,
                FeatureFlagAuditAction::Updated,
                Some(updated_by),
                Some(serde_json::to_value(&previous).unwrap_or_default()),
                Some(serde_json::to_value(f).unwrap_or_default()),
                None,
            )
            .await?;
        }

        Ok(flag)
    }

    /// Toggle a feature flag.
    pub async fn toggle_feature_flag(
        &self,
        id: Uuid,
        enabled: bool,
        toggled_by: Uuid,
    ) -> Result<Option<FeatureFlag>, SqlxError> {
        let previous = self.get_feature_flag(id).await?;
        if previous.is_none() {
            return Ok(None);
        }

        let flag = sqlx::query_as::<_, FeatureFlag>(
            r#"
            UPDATE infrastructure_feature_flags
            SET enabled = $2, updated_at = NOW()
            WHERE id = $1
            RETURNING id, key, name, description, enabled, rollout_percentage,
                      targeting_rules, default_value, value_type, environment, tags,
                      created_by, created_at, updated_at, last_evaluated_at
            "#,
        )
        .bind(id)
        .bind(enabled)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(ref f) = flag {
            let action = if enabled {
                FeatureFlagAuditAction::Enabled
            } else {
                FeatureFlagAuditAction::Disabled
            };
            self.log_flag_audit(
                id,
                action,
                Some(toggled_by),
                Some(serde_json::to_value(&previous).unwrap_or_default()),
                Some(serde_json::to_value(f).unwrap_or_default()),
                None,
            )
            .await?;
        }

        Ok(flag)
    }

    /// Delete a feature flag.
    pub async fn delete_feature_flag(&self, id: Uuid, deleted_by: Uuid) -> Result<bool, SqlxError> {
        // Log before deletion
        let previous = self.get_feature_flag(id).await?;
        if previous.is_some() {
            self.log_flag_audit(
                id,
                FeatureFlagAuditAction::Deleted,
                Some(deleted_by),
                Some(serde_json::to_value(&previous).unwrap_or_default()),
                None,
                None,
            )
            .await?;
        }

        let result = sqlx::query("DELETE FROM infrastructure_feature_flags WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    // ==================== Feature Flag Overrides (Story 89.2) ====================

    /// List overrides for a feature flag.
    pub async fn list_flag_overrides(
        &self,
        flag_id: Uuid,
    ) -> Result<Vec<FeatureFlagOverride>, SqlxError> {
        let overrides = sqlx::query_as::<_, FeatureFlagOverride>(
            r#"
            SELECT id, flag_id, override_type, target_id, value, enabled, expires_at, created_at
            FROM infrastructure_feature_flag_overrides
            WHERE flag_id = $1
              AND (expires_at IS NULL OR expires_at > NOW())
            ORDER BY created_at DESC
            "#,
        )
        .bind(flag_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(overrides)
    }

    /// Create a feature flag override.
    pub async fn create_flag_override(
        &self,
        flag_id: Uuid,
        override_type: FeatureFlagOverrideType,
        target_id: Option<Uuid>,
        value: serde_json::Value,
        expires_at: Option<DateTime<Utc>>,
        created_by: Uuid,
    ) -> Result<FeatureFlagOverride, SqlxError> {
        let override_record = sqlx::query_as::<_, FeatureFlagOverride>(
            r#"
            INSERT INTO infrastructure_feature_flag_overrides (
                flag_id, override_type, target_id, value, expires_at
            )
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, flag_id, override_type, target_id, value, enabled, expires_at, created_at
            "#,
        )
        .bind(flag_id)
        .bind(&override_type)
        .bind(target_id)
        .bind(&value)
        .bind(expires_at)
        .fetch_one(&self.pool)
        .await?;

        // Log audit event
        self.log_flag_audit(
            flag_id,
            FeatureFlagAuditAction::OverrideAdded,
            Some(created_by),
            None,
            Some(serde_json::to_value(&override_record).unwrap_or_default()),
            None,
        )
        .await?;

        Ok(override_record)
    }

    /// Delete a feature flag override.
    pub async fn delete_flag_override(
        &self,
        flag_id: Uuid,
        override_id: Uuid,
        deleted_by: Uuid,
    ) -> Result<bool, SqlxError> {
        // Get override for audit
        let override_record = sqlx::query_as::<_, FeatureFlagOverride>(
            "SELECT id, flag_id, override_type, target_id, value, enabled, expires_at, created_at FROM infrastructure_feature_flag_overrides WHERE id = $1 AND flag_id = $2",
        )
        .bind(override_id)
        .bind(flag_id)
        .fetch_optional(&self.pool)
        .await?;

        if override_record.is_some() {
            self.log_flag_audit(
                flag_id,
                FeatureFlagAuditAction::OverrideRemoved,
                Some(deleted_by),
                Some(serde_json::to_value(&override_record).unwrap_or_default()),
                None,
                None,
            )
            .await?;
        }

        let result = sqlx::query(
            "DELETE FROM infrastructure_feature_flag_overrides WHERE id = $1 AND flag_id = $2",
        )
        .bind(override_id)
        .bind(flag_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    // ==================== Health Check Operations (Story 89.3) ====================

    /// List health check configurations.
    pub async fn list_health_checks(&self) -> Result<Vec<HealthCheckConfig>, SqlxError> {
        let configs = sqlx::query_as::<_, HealthCheckConfig>(
            r#"
            SELECT id, name, check_type, endpoint, interval_seconds, timeout_ms,
                   failure_threshold, success_threshold, enabled, config, created_at, updated_at
            FROM health_check_configs
            ORDER BY name ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(configs)
    }

    /// Get a health check configuration by ID.
    pub async fn get_health_check(&self, id: Uuid) -> Result<Option<HealthCheckConfig>, SqlxError> {
        let config = sqlx::query_as::<_, HealthCheckConfig>(
            r#"
            SELECT id, name, check_type, endpoint, interval_seconds, timeout_ms,
                   failure_threshold, success_threshold, enabled, config, created_at, updated_at
            FROM health_check_configs
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(config)
    }

    /// Get health check results for a configuration.
    pub async fn get_health_check_results(
        &self,
        config_id: Uuid,
        limit: i64,
    ) -> Result<Vec<HealthCheckResult>, SqlxError> {
        let results = sqlx::query_as::<_, HealthCheckResult>(
            r#"
            SELECT id, config_id, status, latency_ms, error_message, response_details, checked_at
            FROM health_check_results
            WHERE config_id = $1
            ORDER BY checked_at DESC
            LIMIT $2
            "#,
        )
        .bind(config_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    /// Record a health check result.
    pub async fn record_health_check_result(
        &self,
        config_id: Uuid,
        status: HealthStatus,
        latency_ms: Option<i64>,
        error_message: Option<&str>,
        response_details: Option<serde_json::Value>,
    ) -> Result<HealthCheckResult, SqlxError> {
        let result = sqlx::query_as::<_, HealthCheckResult>(
            r#"
            INSERT INTO health_check_results (config_id, status, latency_ms, error_message, response_details)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, config_id, status, latency_ms, error_message, response_details, checked_at
            "#,
        )
        .bind(config_id)
        .bind(&status)
        .bind(latency_ms)
        .bind(error_message)
        .bind(&response_details)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    /// Cleanup old health check results.
    pub async fn cleanup_health_check_results(
        &self,
        retention_days: i32,
    ) -> Result<i64, SqlxError> {
        let result = sqlx::query_scalar::<_, i64>("SELECT cleanup_old_health_check_results($1)")
            .bind(retention_days)
            .fetch_one(&self.pool)
            .await?;

        Ok(result)
    }

    // ==================== Alert Operations (Story 89.4) ====================

    /// List alerts with pagination and filters.
    pub async fn list_alerts(
        &self,
        status: Option<&str>,
        severity: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<HealthAlert>, i64), SqlxError> {
        // Count total
        let total = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM health_alerts
            WHERE ($1::TEXT IS NULL OR status::TEXT = $1)
              AND ($2::TEXT IS NULL OR severity::TEXT = $2)
            "#,
        )
        .bind(status)
        .bind(severity)
        .fetch_one(&self.pool)
        .await?;

        // Fetch alerts
        let alerts = sqlx::query_as::<_, HealthAlert>(
            r#"
            SELECT id, rule_id, status, severity, message, context,
                   triggered_at, acknowledged_at, acknowledged_by, resolved_at
            FROM health_alerts
            WHERE ($1::TEXT IS NULL OR status::TEXT = $1)
              AND ($2::TEXT IS NULL OR severity::TEXT = $2)
            ORDER BY triggered_at DESC
            LIMIT $3 OFFSET $4
            "#,
        )
        .bind(status)
        .bind(severity)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok((alerts, total))
    }

    /// Get an alert by ID.
    pub async fn get_alert(&self, id: Uuid) -> Result<Option<HealthAlert>, SqlxError> {
        let alert = sqlx::query_as::<_, HealthAlert>(
            r#"
            SELECT id, rule_id, status, severity, message, context,
                   triggered_at, acknowledged_at, acknowledged_by, resolved_at
            FROM health_alerts
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(alert)
    }

    /// Acknowledge an alert.
    pub async fn acknowledge_alert(
        &self,
        id: Uuid,
        acknowledged_by: Uuid,
        note: Option<&str>,
    ) -> Result<Option<HealthAlert>, SqlxError> {
        let alert = sqlx::query_as::<_, HealthAlert>(
            r#"
            UPDATE health_alerts
            SET status = 'acknowledged',
                acknowledged_at = NOW(),
                acknowledged_by = $2,
                acknowledged_note = $3
            WHERE id = $1 AND status = 'active'
            RETURNING id, rule_id, status, severity, message, context,
                      triggered_at, acknowledged_at, acknowledged_by, resolved_at
            "#,
        )
        .bind(id)
        .bind(acknowledged_by)
        .bind(note)
        .fetch_optional(&self.pool)
        .await?;

        Ok(alert)
    }

    /// Resolve an alert.
    pub async fn resolve_alert(
        &self,
        id: Uuid,
        note: Option<&str>,
    ) -> Result<Option<HealthAlert>, SqlxError> {
        let alert = sqlx::query_as::<_, HealthAlert>(
            r#"
            UPDATE health_alerts
            SET status = 'resolved',
                resolved_at = NOW(),
                resolved_note = $2
            WHERE id = $1 AND status IN ('active', 'acknowledged')
            RETURNING id, rule_id, status, severity, message, context,
                      triggered_at, acknowledged_at, acknowledged_by, resolved_at
            "#,
        )
        .bind(id)
        .bind(note)
        .fetch_optional(&self.pool)
        .await?;

        Ok(alert)
    }

    /// Create a new alert.
    pub async fn create_alert(
        &self,
        rule_id: Uuid,
        severity: AlertSeverity,
        message: &str,
        context: Option<serde_json::Value>,
    ) -> Result<HealthAlert, SqlxError> {
        let alert = sqlx::query_as::<_, HealthAlert>(
            r#"
            INSERT INTO health_alerts (rule_id, severity, message, context)
            VALUES ($1, $2, $3, $4)
            RETURNING id, rule_id, status, severity, message, context,
                      triggered_at, acknowledged_at, acknowledged_by, resolved_at
            "#,
        )
        .bind(rule_id)
        .bind(&severity)
        .bind(message)
        .bind(&context)
        .fetch_one(&self.pool)
        .await?;

        Ok(alert)
    }

    // ==================== Audit Log Operations (Story 89.5) ====================

    /// Log a feature flag audit event.
    async fn log_flag_audit(
        &self,
        flag_id: Uuid,
        action: FeatureFlagAuditAction,
        performed_by: Option<Uuid>,
        previous_state: Option<serde_json::Value>,
        new_state: Option<serde_json::Value>,
        context: Option<serde_json::Value>,
    ) -> Result<FeatureFlagAuditLog, SqlxError> {
        let log = sqlx::query_as::<_, FeatureFlagAuditLog>(
            r#"
            INSERT INTO infrastructure_feature_flag_audit_logs (
                flag_id, action, performed_by, previous_state, new_state, context
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, flag_id, action, performed_by, previous_state, new_state, context, created_at
            "#,
        )
        .bind(flag_id)
        .bind(&action)
        .bind(performed_by)
        .bind(&previous_state)
        .bind(&new_state)
        .bind(&context)
        .fetch_one(&self.pool)
        .await?;

        Ok(log)
    }

    /// Get audit logs for a feature flag.
    pub async fn get_flag_audit_logs(
        &self,
        flag_id: Uuid,
        limit: i64,
    ) -> Result<Vec<FeatureFlagAuditLog>, SqlxError> {
        let logs = sqlx::query_as::<_, FeatureFlagAuditLog>(
            r#"
            SELECT id, flag_id, action, performed_by, previous_state, new_state, context, created_at
            FROM infrastructure_feature_flag_audit_logs
            WHERE flag_id = $1
            ORDER BY created_at DESC
            LIMIT $2
            "#,
        )
        .bind(flag_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(logs)
    }

    /// Evaluate a feature flag for a specific context.
    pub async fn evaluate_feature_flag(
        &self,
        key: &str,
        user_id: Option<Uuid>,
        org_id: Option<Uuid>,
    ) -> Result<Option<(bool, serde_json::Value, String)>, SqlxError> {
        // Get the flag
        let flag = match self.get_feature_flag_by_key(key).await? {
            Some(f) => f,
            None => return Ok(None),
        };

        // Update last evaluated timestamp
        let _ = sqlx::query(
            "UPDATE infrastructure_feature_flags SET last_evaluated_at = NOW() WHERE id = $1",
        )
        .bind(flag.id)
        .execute(&self.pool)
        .await;

        // Check for user-level override first
        if let Some(uid) = user_id {
            let user_override = sqlx::query_as::<_, FeatureFlagOverride>(
                r#"
                SELECT id, flag_id, override_type, target_id, value, enabled, expires_at, created_at
                FROM infrastructure_feature_flag_overrides
                WHERE flag_id = $1 AND override_type = 'user' AND target_id = $2
                  AND enabled = true
                  AND (expires_at IS NULL OR expires_at > NOW())
                "#,
            )
            .bind(flag.id)
            .bind(uid)
            .fetch_optional(&self.pool)
            .await?;

            if let Some(o) = user_override {
                return Ok(Some((
                    o.value.as_bool().unwrap_or(flag.enabled),
                    o.value.clone(),
                    "User override matched".to_string(),
                )));
            }
        }

        // Check for org-level override
        if let Some(oid) = org_id {
            let org_override = sqlx::query_as::<_, FeatureFlagOverride>(
                r#"
                SELECT id, flag_id, override_type, target_id, value, enabled, expires_at, created_at
                FROM infrastructure_feature_flag_overrides
                WHERE flag_id = $1 AND override_type = 'organization' AND target_id = $2
                  AND enabled = true
                  AND (expires_at IS NULL OR expires_at > NOW())
                "#,
            )
            .bind(flag.id)
            .bind(oid)
            .fetch_optional(&self.pool)
            .await?;

            if let Some(o) = org_override {
                return Ok(Some((
                    o.value.as_bool().unwrap_or(flag.enabled),
                    o.value.clone(),
                    "Organization override matched".to_string(),
                )));
            }
        }

        // Check rollout percentage (simple hash-based percentage rollout)
        if flag.rollout_percentage < 100 {
            // Use user_id or org_id for consistent rollout
            let rollout_key = user_id
                .or(org_id)
                .map(|id| id.to_string())
                .unwrap_or_default();
            let hash = simple_hash(&format!("{}:{}", flag.key, rollout_key));
            let percentage = (hash % 100) as i32;

            if percentage >= flag.rollout_percentage {
                return Ok(Some((
                    false,
                    serde_json::Value::Bool(false),
                    format!("Rollout percentage ({}) not met", flag.rollout_percentage),
                )));
            }
        }

        // Return flag default
        Ok(Some((
            flag.enabled,
            flag.default_value.clone(),
            if flag.enabled {
                "Flag enabled, returning default value".to_string()
            } else {
                "Flag disabled".to_string()
            },
        )))
    }

    /// Get the count of active alerts.
    pub async fn get_active_alert_count(&self) -> Result<i64, SqlxError> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM health_alerts WHERE status = 'active'",
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count)
    }

    /// Get the count of active feature flags.
    pub async fn get_active_feature_flag_count(&self) -> Result<i64, SqlxError> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM infrastructure_feature_flags WHERE enabled = true",
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count)
    }
}

/// Simple hash function for rollout percentage calculation.
fn simple_hash(s: &str) -> u64 {
    let mut hash: u64 = 5381;
    for c in s.bytes() {
        hash = ((hash << 5).wrapping_add(hash)).wrapping_add(c as u64);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_hash() {
        let hash1 = simple_hash("test:user1");
        let hash2 = simple_hash("test:user1");
        let hash3 = simple_hash("test:user2");

        // Same input = same output
        assert_eq!(hash1, hash2);
        // Different input = different output
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_hash_distribution() {
        // Test that hash values are reasonably distributed
        let mut counts = vec![0i32; 10];
        for i in 0..1000 {
            let hash = simple_hash(&format!("test:user{}", i));
            let bucket = (hash % 10) as usize;
            counts[bucket] += 1;
        }

        // Each bucket should have roughly 100 items (within reasonable variance)
        for count in counts {
            assert!(count > 50 && count < 150, "Distribution is too skewed");
        }
    }
}

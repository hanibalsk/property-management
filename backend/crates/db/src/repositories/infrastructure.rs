//! Infrastructure repository (Epic 89 & 102).
//!
//! Repository for feature flags, health monitoring, alerts, and distributed tracing.

use crate::models::infrastructure::{
    AlertSeverity, BackgroundJobTypeStats, CreateFeatureFlag, CreateHealthAlertRule, CreateSpan,
    CreateTrace, FeatureFlag, FeatureFlagAuditAction, FeatureFlagAuditLog, FeatureFlagOverride,
    FeatureFlagOverrideType, HealthAlert, HealthAlertRule, HealthCheckConfig, HealthCheckResult,
    HealthStatus, Span, Trace, TraceQuery, UpdateFeatureFlag, UpdateHealthAlertRule,
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

    // ==================== Story 102.1: Distributed Tracing ====================

    /// List traces with query filters.
    pub async fn list_traces(&self, query: TraceQuery) -> Result<(Vec<Trace>, i64), SqlxError> {
        let limit = query.limit.unwrap_or(50);
        let offset = query.offset.unwrap_or(0);

        // Count total matching traces
        let total = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM distributed_traces
            WHERE ($1::VARCHAR IS NULL OR service_name = $1)
              AND ($2::VARCHAR IS NULL OR operation_name ILIKE '%' || $2 || '%')
              AND ($3::BIGINT IS NULL OR duration_ms >= $3)
              AND ($4::BIGINT IS NULL OR duration_ms <= $4)
              AND ($5::BOOLEAN IS NULL OR has_error = $5)
              AND ($6::UUID IS NULL OR user_id = $6)
              AND ($7::UUID IS NULL OR org_id = $7)
              AND ($8::INTEGER IS NULL OR http_status_code = $8)
              AND ($9::TIMESTAMPTZ IS NULL OR started_at >= $9)
              AND ($10::TIMESTAMPTZ IS NULL OR started_at <= $10)
            "#,
        )
        .bind(&query.service_name)
        .bind(&query.operation_name)
        .bind(query.min_duration_ms)
        .bind(query.max_duration_ms)
        .bind(query.has_error)
        .bind(query.user_id)
        .bind(query.org_id)
        .bind(query.http_status_code)
        .bind(query.from_time)
        .bind(query.to_time)
        .fetch_one(&self.pool)
        .await?;

        // Fetch traces
        let traces = sqlx::query_as::<_, Trace>(
            r#"
            SELECT id, trace_id, root_span_id, service_name, operation_name,
                   http_method, http_path, http_status_code, duration_ms, has_error,
                   user_id, org_id, attributes, started_at, completed_at, created_at
            FROM distributed_traces
            WHERE ($1::VARCHAR IS NULL OR service_name = $1)
              AND ($2::VARCHAR IS NULL OR operation_name ILIKE '%' || $2 || '%')
              AND ($3::BIGINT IS NULL OR duration_ms >= $3)
              AND ($4::BIGINT IS NULL OR duration_ms <= $4)
              AND ($5::BOOLEAN IS NULL OR has_error = $5)
              AND ($6::UUID IS NULL OR user_id = $6)
              AND ($7::UUID IS NULL OR org_id = $7)
              AND ($8::INTEGER IS NULL OR http_status_code = $8)
              AND ($9::TIMESTAMPTZ IS NULL OR started_at >= $9)
              AND ($10::TIMESTAMPTZ IS NULL OR started_at <= $10)
            ORDER BY started_at DESC
            LIMIT $11 OFFSET $12
            "#,
        )
        .bind(&query.service_name)
        .bind(&query.operation_name)
        .bind(query.min_duration_ms)
        .bind(query.max_duration_ms)
        .bind(query.has_error)
        .bind(query.user_id)
        .bind(query.org_id)
        .bind(query.http_status_code)
        .bind(query.from_time)
        .bind(query.to_time)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok((traces, total))
    }

    /// Get a trace by ID.
    pub async fn get_trace(&self, id: Uuid) -> Result<Option<Trace>, SqlxError> {
        let trace = sqlx::query_as::<_, Trace>(
            r#"
            SELECT id, trace_id, root_span_id, service_name, operation_name,
                   http_method, http_path, http_status_code, duration_ms, has_error,
                   user_id, org_id, attributes, started_at, completed_at, created_at
            FROM distributed_traces
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(trace)
    }

    /// Get all spans for a trace.
    pub async fn get_trace_spans(&self, trace_id: Uuid) -> Result<Vec<Span>, SqlxError> {
        let spans = sqlx::query_as::<_, Span>(
            r#"
            SELECT id, trace_id, span_id, parent_span_id, service_name, operation_name,
                   span_kind, duration_ms, status, error_message, attributes, events,
                   started_at, ended_at, created_at
            FROM distributed_spans
            WHERE trace_id = $1
            ORDER BY started_at ASC
            "#,
        )
        .bind(trace_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(spans)
    }

    /// Create a new trace.
    pub async fn create_trace(&self, data: CreateTrace) -> Result<Trace, SqlxError> {
        let trace = sqlx::query_as::<_, Trace>(
            r#"
            INSERT INTO distributed_traces (
                trace_id, root_span_id, service_name, operation_name,
                http_method, http_path, http_status_code, duration_ms, has_error,
                user_id, org_id, attributes, started_at, completed_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            RETURNING id, trace_id, root_span_id, service_name, operation_name,
                      http_method, http_path, http_status_code, duration_ms, has_error,
                      user_id, org_id, attributes, started_at, completed_at, created_at
            "#,
        )
        .bind(&data.trace_id)
        .bind(&data.root_span_id)
        .bind(&data.service_name)
        .bind(&data.operation_name)
        .bind(&data.http_method)
        .bind(&data.http_path)
        .bind(data.http_status_code)
        .bind(data.duration_ms)
        .bind(data.has_error)
        .bind(data.user_id)
        .bind(data.org_id)
        .bind(&data.attributes)
        .bind(data.started_at)
        .bind(data.completed_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(trace)
    }

    /// Create a new span.
    pub async fn create_span(&self, data: CreateSpan) -> Result<Span, SqlxError> {
        let span = sqlx::query_as::<_, Span>(
            r#"
            INSERT INTO distributed_spans (
                trace_id, span_id, parent_span_id, service_name, operation_name,
                span_kind, duration_ms, status, error_message, attributes, events,
                started_at, ended_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING id, trace_id, span_id, parent_span_id, service_name, operation_name,
                      span_kind, duration_ms, status, error_message, attributes, events,
                      started_at, ended_at, created_at
            "#,
        )
        .bind(data.trace_id)
        .bind(&data.span_id)
        .bind(&data.parent_span_id)
        .bind(&data.service_name)
        .bind(&data.operation_name)
        .bind(&data.span_kind)
        .bind(data.duration_ms)
        .bind(&data.status)
        .bind(&data.error_message)
        .bind(&data.attributes)
        .bind(&data.events)
        .bind(data.started_at)
        .bind(data.ended_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(span)
    }

    /// Get trace statistics for dashboard.
    pub async fn get_trace_statistics(&self, hours: i32) -> Result<TraceStatistics, SqlxError> {
        let stats =
            sqlx::query_as::<_, TraceStatisticsRow>("SELECT * FROM get_trace_statistics($1)")
                .bind(hours)
                .fetch_one(&self.pool)
                .await?;

        Ok(TraceStatistics {
            total_traces: stats.total_traces,
            error_traces: stats.error_traces,
            error_rate_percent: stats.error_rate_percent,
            avg_duration_ms: stats.avg_duration_ms,
            p95_duration_ms: stats.p95_duration_ms,
            p99_duration_ms: stats.p99_duration_ms,
            requests_per_minute: stats.requests_per_minute,
        })
    }

    /// Cleanup old traces based on retention policy.
    pub async fn cleanup_old_traces(&self, retention_days: i32) -> Result<i64, SqlxError> {
        let deleted = sqlx::query_scalar::<_, i64>("SELECT cleanup_old_traces($1)")
            .bind(retention_days)
            .fetch_one(&self.pool)
            .await?;

        Ok(deleted)
    }

    /// Get recent trace count for dashboard.
    pub async fn get_recent_trace_count(&self, hours: i32) -> Result<i64, SqlxError> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM distributed_traces
            WHERE started_at > NOW() - ($1 || ' hours')::INTERVAL
            "#,
        )
        .bind(hours)
        .fetch_one(&self.pool)
        .await?;

        Ok(count)
    }

    // ==================== Story 102.2: Job Type Statistics ====================

    /// Get statistics grouped by job type.
    pub async fn get_job_type_stats(&self) -> Result<Vec<BackgroundJobTypeStats>, SqlxError> {
        let stats = sqlx::query_as::<_, JobTypeStatsRow>(
            r#"
            SELECT
                job_type,
                COUNT(*)::BIGINT AS total_count,
                COALESCE(
                    COUNT(*) FILTER (WHERE status = 'completed')::DOUBLE PRECISION /
                    NULLIF(COUNT(*) FILTER (WHERE status IN ('completed', 'failed'))::DOUBLE PRECISION, 0) * 100,
                    0
                ) AS success_rate,
                AVG(duration_ms) FILTER (WHERE status = 'completed') AS avg_duration_ms,
                COUNT(*) FILTER (WHERE status = 'pending')::BIGINT AS pending_count,
                COUNT(*) FILTER (WHERE status = 'failed')::BIGINT AS failed_count
            FROM background_jobs
            GROUP BY job_type
            ORDER BY total_count DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(stats
            .into_iter()
            .map(|s| BackgroundJobTypeStats {
                job_type: s.job_type,
                total_count: s.total_count,
                success_rate: s.success_rate,
                avg_duration_ms: s.avg_duration_ms,
                pending_count: s.pending_count,
                failed_count: s.failed_count,
            })
            .collect())
    }

    // ==================== Story 102.3: Alert Rules Management ====================

    /// List all alert rules.
    pub async fn list_alert_rules(&self) -> Result<Vec<HealthAlertRule>, SqlxError> {
        let rules = sqlx::query_as::<_, HealthAlertRule>(
            r#"
            SELECT id, name, description, condition, severity, notification_channels,
                   enabled, cooldown_seconds, last_triggered_at, created_at, updated_at
            FROM health_alert_rules
            ORDER BY name ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rules)
    }

    /// Get an alert rule by ID.
    pub async fn get_alert_rule(&self, id: Uuid) -> Result<Option<HealthAlertRule>, SqlxError> {
        let rule = sqlx::query_as::<_, HealthAlertRule>(
            r#"
            SELECT id, name, description, condition, severity, notification_channels,
                   enabled, cooldown_seconds, last_triggered_at, created_at, updated_at
            FROM health_alert_rules
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(rule)
    }

    /// Create a new alert rule.
    pub async fn create_alert_rule(
        &self,
        data: CreateHealthAlertRule,
    ) -> Result<HealthAlertRule, SqlxError> {
        let rule = sqlx::query_as::<_, HealthAlertRule>(
            r#"
            INSERT INTO health_alert_rules (
                name, description, condition, severity, notification_channels,
                enabled, cooldown_seconds
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, name, description, condition, severity, notification_channels,
                      enabled, cooldown_seconds, last_triggered_at, created_at, updated_at
            "#,
        )
        .bind(&data.name)
        .bind(&data.description)
        .bind(&data.condition)
        .bind(&data.severity)
        .bind(&data.notification_channels)
        .bind(data.enabled.unwrap_or(true))
        .bind(data.cooldown_seconds.unwrap_or(300))
        .fetch_one(&self.pool)
        .await?;

        Ok(rule)
    }

    /// Update an alert rule.
    pub async fn update_alert_rule(
        &self,
        id: Uuid,
        data: UpdateHealthAlertRule,
    ) -> Result<Option<HealthAlertRule>, SqlxError> {
        let rule = sqlx::query_as::<_, HealthAlertRule>(
            r#"
            UPDATE health_alert_rules
            SET name = COALESCE($2, name),
                description = COALESCE($3, description),
                condition = COALESCE($4, condition),
                severity = COALESCE($5, severity),
                notification_channels = COALESCE($6, notification_channels),
                enabled = COALESCE($7, enabled),
                cooldown_seconds = COALESCE($8, cooldown_seconds),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, name, description, condition, severity, notification_channels,
                      enabled, cooldown_seconds, last_triggered_at, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(&data.name)
        .bind(&data.description)
        .bind(&data.condition)
        .bind(&data.severity)
        .bind(&data.notification_channels)
        .bind(data.enabled)
        .bind(data.cooldown_seconds)
        .fetch_optional(&self.pool)
        .await?;

        Ok(rule)
    }

    /// Delete an alert rule.
    pub async fn delete_alert_rule(&self, id: Uuid) -> Result<bool, SqlxError> {
        let result = sqlx::query("DELETE FROM health_alert_rules WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Toggle an alert rule enabled/disabled.
    pub async fn toggle_alert_rule(
        &self,
        id: Uuid,
        enabled: bool,
    ) -> Result<Option<HealthAlertRule>, SqlxError> {
        let rule = sqlx::query_as::<_, HealthAlertRule>(
            r#"
            UPDATE health_alert_rules
            SET enabled = $2, updated_at = NOW()
            WHERE id = $1
            RETURNING id, name, description, condition, severity, notification_channels,
                      enabled, cooldown_seconds, last_triggered_at, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(enabled)
        .fetch_optional(&self.pool)
        .await?;

        Ok(rule)
    }

    /// Update last triggered timestamp for an alert rule.
    pub async fn update_alert_rule_last_triggered(&self, id: Uuid) -> Result<(), SqlxError> {
        sqlx::query("UPDATE health_alert_rules SET last_triggered_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // ==================== Story 102.4: Prometheus Metrics ====================

    /// Get database connection pool stats for metrics.
    pub async fn get_db_pool_stats(&self) -> DbPoolStats {
        DbPoolStats {
            active_connections: self.pool.size() as i32,
            idle_connections: self.pool.num_idle() as i32,
            max_connections: self.pool.options().get_max_connections() as i32,
        }
    }

    /// Get error rate from traces (last hour).
    pub async fn get_error_rate_percent(&self, hours: i32) -> Result<f64, SqlxError> {
        let rate = sqlx::query_scalar::<_, Option<f64>>(
            r#"
            SELECT
                CASE
                    WHEN COUNT(*) > 0 THEN
                        (COUNT(*) FILTER (WHERE has_error)::DOUBLE PRECISION / COUNT(*)::DOUBLE PRECISION) * 100
                    ELSE 0
                END
            FROM distributed_traces
            WHERE started_at > NOW() - ($1 || ' hours')::INTERVAL
            "#,
        )
        .bind(hours)
        .fetch_one(&self.pool)
        .await?;

        Ok(rate.unwrap_or(0.0))
    }

    /// Get average response time from traces (last hour).
    pub async fn get_avg_response_time_ms(&self, hours: i32) -> Result<f64, SqlxError> {
        let avg = sqlx::query_scalar::<_, Option<f64>>(
            r#"
            SELECT AVG(duration_ms)
            FROM distributed_traces
            WHERE started_at > NOW() - ($1 || ' hours')::INTERVAL
            "#,
        )
        .bind(hours)
        .fetch_one(&self.pool)
        .await?;

        Ok(avg.unwrap_or(0.0))
    }
}

/// Internal row type for trace statistics.
#[derive(sqlx::FromRow)]
struct TraceStatisticsRow {
    total_traces: i64,
    error_traces: i64,
    error_rate_percent: f64,
    avg_duration_ms: Option<f64>,
    p95_duration_ms: Option<f64>,
    p99_duration_ms: Option<f64>,
    requests_per_minute: f64,
}

/// Trace statistics for dashboard.
#[derive(Debug, Clone)]
pub struct TraceStatistics {
    pub total_traces: i64,
    pub error_traces: i64,
    pub error_rate_percent: f64,
    pub avg_duration_ms: Option<f64>,
    pub p95_duration_ms: Option<f64>,
    pub p99_duration_ms: Option<f64>,
    pub requests_per_minute: f64,
}

/// Internal row type for job type stats.
#[derive(sqlx::FromRow)]
struct JobTypeStatsRow {
    job_type: String,
    total_count: i64,
    success_rate: f64,
    avg_duration_ms: Option<f64>,
    pending_count: i64,
    failed_count: i64,
}

/// Database pool statistics for metrics.
#[derive(Debug, Clone)]
pub struct DbPoolStats {
    pub active_connections: i32,
    pub idle_connections: i32,
    pub max_connections: i32,
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

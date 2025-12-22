//! Health Monitoring repository (Epic 10B, Story 10B.3).
//!
//! Repository for platform health metrics, thresholds, and alerts.

use crate::models::platform_admin::{MetricAlert, MetricThreshold, PlatformMetric};
use crate::DbPool;
use chrono::{DateTime, Utc};
use sqlx::Error as SqlxError;
use uuid::Uuid;

/// Repository for health monitoring operations.
#[derive(Clone)]
pub struct HealthMonitoringRepository {
    pool: DbPool,
}

/// Health dashboard snapshot.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct HealthDashboard {
    pub metrics: Vec<CurrentMetric>,
    pub alerts: Vec<MetricAlert>,
    pub thresholds: Vec<MetricThreshold>,
}

/// Current metric with threshold status.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct CurrentMetric {
    pub metric_name: String,
    pub metric_type: String,
    pub value: f64,
    pub recorded_at: DateTime<Utc>,
    pub status: MetricStatus,
}

/// Metric status based on threshold.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum MetricStatus {
    Normal,
    Warning,
    Critical,
}

/// Time series data point.
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct MetricDataPoint {
    pub value: f64,
    pub recorded_at: DateTime<Utc>,
}

/// Aggregated metric statistics.
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct MetricStats {
    pub min: f64,
    pub max: f64,
    pub avg: f64,
    pub count: i64,
}

/// Metric history response.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct MetricHistory {
    pub metric_name: String,
    pub data_points: Vec<MetricDataPoint>,
    pub stats: MetricStats,
}

impl HealthMonitoringRepository {
    /// Create a new HealthMonitoringRepository.
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Record a new metric value.
    pub async fn record_metric(
        &self,
        metric_type: &str,
        metric_name: &str,
        value: f64,
        metadata: Option<serde_json::Value>,
    ) -> Result<PlatformMetric, SqlxError> {
        let metric = sqlx::query_as::<_, PlatformMetric>(
            r#"
            INSERT INTO platform_metrics (metric_type, metric_name, value, metadata)
            VALUES ($1, $2, $3, $4)
            RETURNING id, metric_type, metric_name, value::float8 as value, metadata, recorded_at
            "#,
        )
        .bind(metric_type)
        .bind(metric_name)
        .bind(value)
        .bind(metadata)
        .fetch_one(&self.pool)
        .await?;

        Ok(metric)
    }

    /// Get the most recent value for each metric.
    pub async fn get_current_metrics(&self) -> Result<Vec<PlatformMetric>, SqlxError> {
        let metrics = sqlx::query_as::<_, PlatformMetric>(
            r#"
            SELECT DISTINCT ON (metric_name)
                id, metric_type, metric_name, value::float8 as value, metadata, recorded_at
            FROM platform_metrics
            ORDER BY metric_name, recorded_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(metrics)
    }

    /// Get metric history for a time range.
    pub async fn get_metric_history(
        &self,
        metric_name: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<MetricHistory, SqlxError> {
        // Get data points
        let data_points = sqlx::query_as::<_, MetricDataPoint>(
            r#"
            SELECT value::float8 as value, recorded_at
            FROM platform_metrics
            WHERE metric_name = $1 AND recorded_at >= $2 AND recorded_at <= $3
            ORDER BY recorded_at ASC
            "#,
        )
        .bind(metric_name)
        .bind(start_time)
        .bind(end_time)
        .fetch_all(&self.pool)
        .await?;

        // Get aggregated stats
        let stats = sqlx::query_as::<_, MetricStats>(
            r#"
            SELECT
                COALESCE(MIN(value)::float8, 0) as min,
                COALESCE(MAX(value)::float8, 0) as max,
                COALESCE(AVG(value)::float8, 0) as avg,
                COUNT(*) as count
            FROM platform_metrics
            WHERE metric_name = $1 AND recorded_at >= $2 AND recorded_at <= $3
            "#,
        )
        .bind(metric_name)
        .bind(start_time)
        .bind(end_time)
        .fetch_one(&self.pool)
        .await?;

        Ok(MetricHistory {
            metric_name: metric_name.to_string(),
            data_points,
            stats,
        })
    }

    /// Get all metric thresholds.
    pub async fn get_thresholds(&self) -> Result<Vec<MetricThreshold>, SqlxError> {
        let thresholds = sqlx::query_as::<_, MetricThreshold>(
            r#"
            SELECT id, metric_name,
                   COALESCE(warning_threshold, 0)::float8 as warning_threshold,
                   COALESCE(critical_threshold, 0)::float8 as critical_threshold,
                   is_active, created_at, updated_at
            FROM metric_thresholds
            ORDER BY metric_name
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(thresholds)
    }

    /// Get threshold for a specific metric.
    pub async fn get_threshold(
        &self,
        metric_name: &str,
    ) -> Result<Option<MetricThreshold>, SqlxError> {
        let threshold = sqlx::query_as::<_, MetricThreshold>(
            r#"
            SELECT id, metric_name,
                   COALESCE(warning_threshold, 0)::float8 as warning_threshold,
                   COALESCE(critical_threshold, 0)::float8 as critical_threshold,
                   is_active, created_at, updated_at
            FROM metric_thresholds
            WHERE metric_name = $1
            "#,
        )
        .bind(metric_name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(threshold)
    }

    /// Update threshold values.
    pub async fn update_threshold(
        &self,
        metric_name: &str,
        warning_threshold: Option<f64>,
        critical_threshold: Option<f64>,
    ) -> Result<Option<MetricThreshold>, SqlxError> {
        let threshold = sqlx::query_as::<_, MetricThreshold>(
            r#"
            UPDATE metric_thresholds
            SET
                warning_threshold = COALESCE($2, warning_threshold),
                critical_threshold = COALESCE($3, critical_threshold),
                updated_at = NOW()
            WHERE metric_name = $1
            RETURNING id, metric_name,
                      COALESCE(warning_threshold, 0)::float8 as warning_threshold,
                      COALESCE(critical_threshold, 0)::float8 as critical_threshold,
                      is_active, created_at, updated_at
            "#,
        )
        .bind(metric_name)
        .bind(warning_threshold)
        .bind(critical_threshold)
        .fetch_optional(&self.pool)
        .await?;

        Ok(threshold)
    }

    /// Get active (unacknowledged) alerts.
    pub async fn get_active_alerts(&self) -> Result<Vec<MetricAlert>, SqlxError> {
        let alerts = sqlx::query_as::<_, MetricAlert>(
            r#"
            SELECT id, metric_name, threshold_type, value::float8 as value,
                   created_at, acknowledged_at, acknowledged_by
            FROM metric_alerts
            WHERE acknowledged_at IS NULL
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(alerts)
    }

    /// Get all alerts (with pagination).
    pub async fn get_alerts(&self, limit: i64, offset: i64) -> Result<Vec<MetricAlert>, SqlxError> {
        let alerts = sqlx::query_as::<_, MetricAlert>(
            r#"
            SELECT id, metric_name, threshold_type, value::float8 as value,
                   created_at, acknowledged_at, acknowledged_by
            FROM metric_alerts
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(alerts)
    }

    /// Create a new alert.
    pub async fn create_alert(
        &self,
        metric_name: &str,
        threshold_type: &str,
        value: f64,
    ) -> Result<MetricAlert, SqlxError> {
        let alert = sqlx::query_as::<_, MetricAlert>(
            r#"
            INSERT INTO metric_alerts (metric_name, threshold_type, value)
            VALUES ($1, $2, $3)
            RETURNING id, metric_name, threshold_type, value::float8 as value,
                      created_at, acknowledged_at, acknowledged_by
            "#,
        )
        .bind(metric_name)
        .bind(threshold_type)
        .bind(value)
        .fetch_one(&self.pool)
        .await?;

        Ok(alert)
    }

    /// Acknowledge an alert.
    pub async fn acknowledge_alert(
        &self,
        alert_id: Uuid,
        acknowledged_by: Uuid,
    ) -> Result<Option<MetricAlert>, SqlxError> {
        let alert = sqlx::query_as::<_, MetricAlert>(
            r#"
            UPDATE metric_alerts
            SET acknowledged_at = NOW(), acknowledged_by = $2
            WHERE id = $1 AND acknowledged_at IS NULL
            RETURNING id, metric_name, threshold_type, value::float8 as value,
                      created_at, acknowledged_at, acknowledged_by
            "#,
        )
        .bind(alert_id)
        .bind(acknowledged_by)
        .fetch_optional(&self.pool)
        .await?;

        Ok(alert)
    }

    /// Get health dashboard data.
    pub async fn get_dashboard(&self) -> Result<HealthDashboard, SqlxError> {
        let metrics = self.get_current_metrics().await?;
        let thresholds = self.get_thresholds().await?;
        let alerts = self.get_active_alerts().await?;

        // Convert metrics to CurrentMetric with status
        let current_metrics: Vec<CurrentMetric> = metrics
            .into_iter()
            .map(|m| {
                let status = self.calculate_status(&m.metric_name, m.value, &thresholds);
                CurrentMetric {
                    metric_name: m.metric_name,
                    metric_type: m.metric_type,
                    value: m.value,
                    recorded_at: m.recorded_at,
                    status,
                }
            })
            .collect();

        Ok(HealthDashboard {
            metrics: current_metrics,
            alerts,
            thresholds,
        })
    }

    /// Calculate metric status based on thresholds.
    fn calculate_status(
        &self,
        metric_name: &str,
        value: f64,
        thresholds: &[MetricThreshold],
    ) -> MetricStatus {
        calculate_metric_status(metric_name, value, thresholds)
    }

    /// Clean up old metrics (older than retention days).
    pub async fn cleanup_old_metrics(&self, retention_days: i32) -> Result<u64, SqlxError> {
        let result = sqlx::query(
            r#"
            DELETE FROM platform_metrics
            WHERE recorded_at < NOW() - ($1 || ' days')::interval
            "#,
        )
        .bind(retention_days)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    /// Get active session count for active users metric.
    pub async fn get_active_session_count(&self) -> Result<i64, SqlxError> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM sessions
            WHERE expires_at > NOW()
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count)
    }
}

/// Calculate metric status based on thresholds (standalone function for testing).
pub fn calculate_metric_status(
    metric_name: &str,
    value: f64,
    thresholds: &[MetricThreshold],
) -> MetricStatus {
    if let Some(threshold) = thresholds.iter().find(|t| t.metric_name == metric_name) {
        if threshold.is_active {
            if threshold.critical_threshold > 0.0 && value >= threshold.critical_threshold {
                return MetricStatus::Critical;
            }
            if threshold.warning_threshold > 0.0 && value >= threshold.warning_threshold {
                return MetricStatus::Warning;
            }
        }
    }
    MetricStatus::Normal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metric_status_values() {
        assert_eq!(MetricStatus::Normal, MetricStatus::Normal);
        assert_eq!(MetricStatus::Warning, MetricStatus::Warning);
        assert_eq!(MetricStatus::Critical, MetricStatus::Critical);
    }

    #[test]
    fn test_calculate_status() {
        let thresholds = vec![MetricThreshold {
            id: Uuid::new_v4(),
            metric_name: "test_metric".to_string(),
            warning_threshold: 50.0,
            critical_threshold: 80.0,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }];

        assert_eq!(
            calculate_metric_status("test_metric", 30.0, &thresholds),
            MetricStatus::Normal
        );
        assert_eq!(
            calculate_metric_status("test_metric", 60.0, &thresholds),
            MetricStatus::Warning
        );
        assert_eq!(
            calculate_metric_status("test_metric", 90.0, &thresholds),
            MetricStatus::Critical
        );
        assert_eq!(
            calculate_metric_status("unknown_metric", 100.0, &thresholds),
            MetricStatus::Normal
        );
    }
}

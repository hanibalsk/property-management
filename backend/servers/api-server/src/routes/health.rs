//! Health check endpoint (Epic 95.3 - Enhanced Health Checks).

use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;
use sqlx::Row;
use std::time::Instant;
use utoipa::ToSchema;

use crate::state::AppState;

/// Health status enumeration.
#[derive(Debug, Clone, Copy, Serialize, ToSchema, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    /// All systems operational
    Healthy,
    /// Some systems degraded but functional
    Degraded,
    /// Critical systems down
    Unhealthy,
}

/// Dependency health check result.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DependencyHealth {
    /// Name of the dependency
    pub name: String,
    /// Health status
    pub status: HealthStatus,
    /// Response time in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<u64>,
    /// Error message if unhealthy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Health check response.
#[derive(Serialize, ToSchema)]
pub struct HealthResponse {
    /// Overall service status
    pub status: HealthStatus,
    /// Service version
    pub version: String,
    /// Service name
    pub service: String,
    /// Uptime in seconds (Story 88.1)
    pub uptime_seconds: u64,
    /// Dependency health checks (Story 95.3)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<DependencyHealth>>,
    /// Current timestamp
    pub timestamp: String,
}

/// Check database connectivity and measure latency.
async fn check_database(pool: &sqlx::PgPool) -> DependencyHealth {
    let start = Instant::now();

    let result = sqlx::query("SELECT 1 as health_check")
        .fetch_one(pool)
        .await;

    let latency_ms = start.elapsed().as_millis() as u64;

    match result {
        Ok(row) => {
            // Verify the result
            let _: i32 = row.get("health_check");
            DependencyHealth {
                name: "database".to_string(),
                status: if latency_ms > 1000 {
                    HealthStatus::Degraded
                } else {
                    HealthStatus::Healthy
                },
                latency_ms: Some(latency_ms),
                error: None,
            }
        }
        Err(e) => DependencyHealth {
            name: "database".to_string(),
            status: HealthStatus::Unhealthy,
            latency_ms: Some(latency_ms),
            error: Some(format!("Database connection failed: {}", e)),
        },
    }
}

/// Determine overall health status from dependency checks.
fn determine_overall_status(dependencies: &[DependencyHealth]) -> HealthStatus {
    let has_unhealthy = dependencies
        .iter()
        .any(|d| d.status == HealthStatus::Unhealthy);
    let has_degraded = dependencies
        .iter()
        .any(|d| d.status == HealthStatus::Degraded);

    if has_unhealthy {
        HealthStatus::Unhealthy
    } else if has_degraded {
        HealthStatus::Degraded
    } else {
        HealthStatus::Healthy
    }
}

/// Health check endpoint.
///
/// Returns overall system health status including dependency checks for:
/// - Database connectivity
/// - Redis (when available)
/// - External services
#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse),
        (status = 503, description = "Service is unhealthy", body = HealthResponse)
    )
)]
pub async fn health(State(state): State<AppState>) -> (StatusCode, Json<HealthResponse>) {
    let uptime_seconds = state.boot_time.elapsed().as_secs();

    // Check all dependencies
    let db_health = check_database(&state.db).await;

    // TODO: Add Redis health check when Redis client is available
    // let redis_health = check_redis(&state.redis).await;

    let dependencies = vec![db_health];
    let overall_status = determine_overall_status(&dependencies);

    let status_code = match overall_status {
        HealthStatus::Healthy => StatusCode::OK,
        HealthStatus::Degraded => StatusCode::OK, // Still return 200 for degraded
        HealthStatus::Unhealthy => StatusCode::SERVICE_UNAVAILABLE,
    };

    let response = HealthResponse {
        status: overall_status,
        version: env!("CARGO_PKG_VERSION").to_string(),
        service: "api-server".to_string(),
        uptime_seconds,
        dependencies: Some(dependencies),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    (status_code, Json(response))
}

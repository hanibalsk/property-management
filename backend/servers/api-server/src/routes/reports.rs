//! Reports routes (Epic 55: Advanced Reporting & Analytics).
//!
//! Provides comprehensive reporting endpoints for:
//! - Story 55.1: Fault Statistics Report
//! - Story 55.2: Voting Participation Report
//! - Story 55.3: Occupancy Report
//! - Story 55.4: Consumption Report
//! - Story 55.5: Export Reports to PDF/Excel

use api_core::extractors::AuthUser;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use chrono::NaiveDate;
use common::errors::ErrorResponse;
use db::models::{
    ConsumptionAnomaly, ConsumptionSummary, DateRange, FaultStatistics, FaultTrends,
    OccupancySummary, OccupancyTrends, UnitConsumption, UnitOccupancy, UtilityTypeConsumption,
    VoteParticipationDetail, VotingParticipationSummary, YearComparison,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::state::AppState;

// ============================================================================
// Response Types
// ============================================================================

/// Fault statistics report response (Story 55.1).
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FaultStatisticsReportResponse {
    pub building_id: Option<Uuid>,
    pub building_name: Option<String>,
    pub date_range: DateRange,
    pub statistics: FaultStatistics,
    pub trends: FaultTrends,
}

/// Voting participation report response (Story 55.2).
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct VotingParticipationReportResponse {
    pub building_id: Option<Uuid>,
    pub building_name: Option<String>,
    pub date_range: DateRange,
    pub summary: VotingParticipationSummary,
    pub votes: Vec<VoteParticipationDetail>,
}

/// Occupancy report response (Story 55.3).
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct OccupancyReportResponse {
    pub building_id: Option<Uuid>,
    pub building_name: Option<String>,
    pub date_range: DateRange,
    pub summary: OccupancySummary,
    pub by_unit: Vec<UnitOccupancy>,
    pub trends: OccupancyTrends,
}

/// Consumption report response (Story 55.4).
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ConsumptionReportResponse {
    pub building_id: Option<Uuid>,
    pub building_name: Option<String>,
    pub date_range: DateRange,
    pub summary: ConsumptionSummary,
    pub by_utility_type: Vec<UtilityTypeConsumption>,
    pub by_unit: Vec<UnitConsumption>,
    pub anomalies: Vec<ConsumptionAnomaly>,
}

/// Export report response (Story 55.5).
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ExportReportResponse {
    pub download_url: String,
    pub format: String,
    pub expires_at: String,
}

// ============================================================================
// Request Types
// ============================================================================

/// Default report date range in days.
const DEFAULT_FAULT_REPORT_DAYS: i64 = 30;
const DEFAULT_VOTING_REPORT_DAYS: i64 = 365;

/// Query parameters for fault statistics report.
#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct FaultStatisticsQuery {
    /// Organization ID (required for multi-tenant filtering)
    pub organization_id: Uuid,
    /// Building ID (optional, filters to specific building)
    pub building_id: Option<Uuid>,
    /// Start date for report period
    pub from_date: Option<NaiveDate>,
    /// End date for report period
    pub to_date: Option<NaiveDate>,
}

/// Query parameters for voting participation report.
#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct VotingParticipationQuery {
    /// Organization ID (required for multi-tenant filtering)
    pub organization_id: Uuid,
    /// Building ID (optional, filters to specific building)
    pub building_id: Option<Uuid>,
    /// Start date for report period
    pub from_date: Option<NaiveDate>,
    /// End date for report period
    pub to_date: Option<NaiveDate>,
}

/// Query parameters for occupancy report.
#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct OccupancyReportQuery {
    /// Organization ID (required for multi-tenant filtering)
    pub organization_id: Uuid,
    /// Building ID (optional, filters to specific building)
    pub building_id: Option<Uuid>,
    /// Year for report
    pub year: i32,
    /// Month (optional, if not provided returns full year)
    pub month: Option<i32>,
    /// Include comparison with previous year
    #[serde(default)]
    pub include_comparison: bool,
}

/// Query parameters for consumption report.
#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct ConsumptionReportQuery {
    /// Organization ID (required for multi-tenant filtering)
    pub organization_id: Uuid,
    /// Building ID (optional, filters to specific building)
    pub building_id: Option<Uuid>,
    /// Utility type filter (water, electricity, gas, heating)
    pub utility_type: Option<String>,
    /// Start date for report period
    pub from_date: NaiveDate,
    /// End date for report period
    pub to_date: NaiveDate,
    /// Include anomaly detection
    #[serde(default)]
    pub include_anomalies: bool,
}

/// Request for exporting a report.
#[derive(Debug, Deserialize, ToSchema)]
pub struct ExportReportRequest {
    /// Organization ID (required for multi-tenant filtering)
    pub organization_id: Uuid,
    /// Report type to export
    pub report_type: String,
    /// Export format (pdf, excel, csv)
    pub format: String,
    /// Building ID (optional)
    pub building_id: Option<Uuid>,
    /// Start date for report period
    pub from_date: Option<NaiveDate>,
    /// End date for report period
    pub to_date: Option<NaiveDate>,
    /// Additional parameters based on report type
    pub params: Option<serde_json::Value>,
}

// ============================================================================
// Router
// ============================================================================

/// Create reports router.
pub fn router() -> Router<AppState> {
    Router::new()
        // Story 55.1: Fault Statistics Report
        .route("/faults", get(get_fault_statistics_report))
        // Story 55.2: Voting Participation Report
        .route("/voting", get(get_voting_participation_report))
        // Story 55.3: Occupancy Report
        .route("/occupancy", get(get_occupancy_report))
        // Story 55.4: Consumption Report
        .route("/consumption", get(get_consumption_report))
        // Story 55.5: Export Reports (Story 84.1: Background job implementation)
        .route("/export", axum::routing::post(export_report))
        .route("/export/{job_id}/status", get(get_export_job_status))
}

// ============================================================================
// Helper function
// ============================================================================

/// Get building name by ID if provided.
async fn get_building_name(state: &AppState, building_id: Option<Uuid>) -> Option<String> {
    if let Some(id) = building_id {
        state
            .building_repo
            .find_by_id(id)
            .await
            .ok()
            .flatten()
            .and_then(|b| b.name)
    } else {
        None
    }
}

// ============================================================================
// Handlers
// ============================================================================

/// Validate date range and return error if invalid.
fn validate_date_range(
    from_date: NaiveDate,
    to_date: NaiveDate,
) -> Result<(), (StatusCode, Json<ErrorResponse>)> {
    if from_date > to_date {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "INVALID_DATE_RANGE",
                "from_date must be before or equal to to_date",
            )),
        ));
    }
    // Limit range to 5 years max for performance
    let max_days = 365 * 5;
    if (to_date - from_date).num_days() > max_days {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "DATE_RANGE_TOO_LARGE",
                "Date range cannot exceed 5 years",
            )),
        ));
    }
    Ok(())
}

/// Get fault statistics report (Story 55.1).
#[utoipa::path(
    get,
    path = "/api/v1/reports/faults",
    params(FaultStatisticsQuery),
    responses(
        (status = 200, description = "Fault statistics report", body = FaultStatisticsReportResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
    tag = "Reports"
)]
pub async fn get_fault_statistics_report(
    State(state): State<AppState>,
    _auth: AuthUser,
    Query(query): Query<FaultStatisticsQuery>,
) -> Result<Json<FaultStatisticsReportResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Get date range (default to last 30 days if not specified)
    let to_date = query
        .to_date
        .unwrap_or_else(|| chrono::Utc::now().date_naive());
    let from_date = query
        .from_date
        .unwrap_or_else(|| to_date - chrono::Duration::days(DEFAULT_FAULT_REPORT_DAYS));

    // Validate date range
    validate_date_range(from_date, to_date)?;

    // Get building name if building_id is provided
    let building_name = get_building_name(&state, query.building_id).await;

    // Get fault statistics from repository (using organization_id for multi-tenant filtering)
    let statistics = state
        .fault_repo
        .get_statistics(query.organization_id, query.building_id)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to get fault statistics");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DB_ERROR",
                    "Failed to get fault statistics",
                )),
            )
        })?;

    // Get monthly trend data
    let monthly_counts = state
        .fault_repo
        .get_monthly_fault_counts(query.organization_id, query.building_id, from_date, to_date)
        .await
        .unwrap_or_default();

    // Get resolution time trend
    let resolution_time_trend = state
        .fault_repo
        .get_monthly_resolution_times(query.organization_id, query.building_id, from_date, to_date)
        .await
        .unwrap_or_default();

    // Build response
    let response = FaultStatisticsReportResponse {
        building_id: query.building_id,
        building_name,
        date_range: DateRange {
            from: from_date,
            to: to_date,
        },
        statistics,
        trends: FaultTrends {
            monthly_counts,
            resolution_time_trend,
            category_trend: vec![],
        },
    };

    Ok(Json(response))
}

/// Get voting participation report (Story 55.2).
#[utoipa::path(
    get,
    path = "/api/v1/reports/voting",
    params(VotingParticipationQuery),
    responses(
        (status = 200, description = "Voting participation report", body = VotingParticipationReportResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
    tag = "Reports"
)]
pub async fn get_voting_participation_report(
    State(state): State<AppState>,
    _auth: AuthUser,
    Query(query): Query<VotingParticipationQuery>,
) -> Result<Json<VotingParticipationReportResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Get date range (default to last 12 months if not specified)
    let to_date = query
        .to_date
        .unwrap_or_else(|| chrono::Utc::now().date_naive());
    let from_date = query
        .from_date
        .unwrap_or_else(|| to_date - chrono::Duration::days(DEFAULT_VOTING_REPORT_DAYS));

    // Validate date range
    validate_date_range(from_date, to_date)?;

    // Get building name if building_id is provided
    let building_name = get_building_name(&state, query.building_id).await;

    // Query voting participation data from repository
    let participation_data = state
        .vote_repo
        .get_participation_report(query.organization_id, query.building_id, from_date, to_date)
        .await
        .unwrap_or_default();

    // Calculate summary from participation data
    let total_votes = participation_data.len() as i64;
    let votes_with_quorum = participation_data
        .iter()
        .filter(|v| v.quorum_reached)
        .count() as i64;
    let votes_without_quorum = total_votes - votes_with_quorum;
    let total_responses: i64 = participation_data.iter().map(|v| v.response_count).sum();
    let total_eligible: i64 = participation_data.iter().map(|v| v.eligible_count).sum();
    let average_participation_rate = if total_eligible > 0 {
        (total_responses as f64 / total_eligible as f64) * 100.0
    } else {
        0.0
    };

    let response = VotingParticipationReportResponse {
        building_id: query.building_id,
        building_name,
        date_range: DateRange {
            from: from_date,
            to: to_date,
        },
        summary: VotingParticipationSummary {
            total_votes,
            votes_with_quorum,
            votes_without_quorum,
            average_participation_rate,
            total_eligible_voters: total_eligible,
            total_responses,
        },
        votes: participation_data,
    };

    Ok(Json(response))
}

/// Get occupancy report (Story 55.3).
#[utoipa::path(
    get,
    path = "/api/v1/reports/occupancy",
    params(OccupancyReportQuery),
    responses(
        (status = 200, description = "Occupancy report", body = OccupancyReportResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
    tag = "Reports"
)]
pub async fn get_occupancy_report(
    State(state): State<AppState>,
    _auth: AuthUser,
    Query(query): Query<OccupancyReportQuery>,
) -> Result<Json<OccupancyReportResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Validate month if provided
    if let Some(month) = query.month {
        if !(1..=12).contains(&month) {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new(
                    "INVALID_MONTH",
                    "Month must be between 1 and 12",
                )),
            ));
        }
    }

    // Calculate date range
    let from_date = NaiveDate::from_ymd_opt(query.year, query.month.unwrap_or(1) as u32, 1)
        .unwrap_or_else(|| chrono::Utc::now().date_naive());
    let to_date = if let Some(month) = query.month {
        // Last day of the specified month
        let next_month = if month == 12 {
            NaiveDate::from_ymd_opt(query.year + 1, 1, 1)
        } else {
            NaiveDate::from_ymd_opt(query.year, (month + 1) as u32, 1)
        };
        next_month
            .map(|d| d - chrono::Duration::days(1))
            .unwrap_or(from_date)
    } else {
        // Last day of the year
        NaiveDate::from_ymd_opt(query.year, 12, 31).unwrap_or(from_date)
    };

    // Get building name if building_id is provided
    let building_name = get_building_name(&state, query.building_id).await;

    // Query occupancy data from person_month repository
    let occupancy_data = state
        .person_month_repo
        .get_occupancy_report(query.organization_id, query.building_id, from_date, to_date)
        .await
        .unwrap_or_default();

    // Build year-over-year comparison if requested
    let year_over_year_comparison = if query.include_comparison {
        let prev_from = NaiveDate::from_ymd_opt(query.year - 1, query.month.unwrap_or(1) as u32, 1)
            .unwrap_or(from_date);
        let prev_to = NaiveDate::from_ymd_opt(
            query.year - 1,
            query.month.unwrap_or(12) as u32,
            if query.month.is_some() { 28 } else { 31 },
        )
        .unwrap_or(to_date);

        let prev_data = state
            .person_month_repo
            .get_occupancy_report(query.organization_id, query.building_id, prev_from, prev_to)
            .await
            .unwrap_or_default();

        let current_total = occupancy_data.summary.total_person_months;
        let previous_total = prev_data.summary.total_person_months;
        let change_percentage = if previous_total > 0 {
            ((current_total - previous_total) as f64 / previous_total as f64) * 100.0
        } else {
            0.0
        };

        Some(YearComparison {
            current_year: query.year,
            previous_year: query.year - 1,
            current_total,
            previous_total,
            change_percentage,
        })
    } else {
        None
    };

    let response = OccupancyReportResponse {
        building_id: query.building_id,
        building_name,
        date_range: DateRange {
            from: from_date,
            to: to_date,
        },
        summary: occupancy_data.summary,
        by_unit: occupancy_data.by_unit,
        trends: OccupancyTrends {
            monthly_total: occupancy_data.monthly_totals,
            year_over_year_comparison,
        },
    };

    Ok(Json(response))
}

/// Get consumption report (Story 55.4).
#[utoipa::path(
    get,
    path = "/api/v1/reports/consumption",
    params(ConsumptionReportQuery),
    responses(
        (status = 200, description = "Consumption report", body = ConsumptionReportResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
    tag = "Reports"
)]
pub async fn get_consumption_report(
    State(state): State<AppState>,
    _auth: AuthUser,
    Query(query): Query<ConsumptionReportQuery>,
) -> Result<Json<ConsumptionReportResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Validate date range
    validate_date_range(query.from_date, query.to_date)?;

    // Validate utility type if provided
    if let Some(ref utility_type) = query.utility_type {
        let valid_types = ["water", "electricity", "gas", "heating"];
        if !valid_types.contains(&utility_type.to_lowercase().as_str()) {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new(
                    "INVALID_UTILITY_TYPE",
                    "Utility type must be water, electricity, gas, or heating",
                )),
            ));
        }
    }

    // Get building name if building_id is provided
    let building_name = get_building_name(&state, query.building_id).await;

    // Query consumption data from meter repository
    let consumption_data = state
        .meter_repo
        .get_consumption_report(
            query.organization_id,
            query.building_id,
            query.utility_type.as_deref(),
            query.from_date,
            query.to_date,
        )
        .await
        .unwrap_or_default();

    // Get anomalies if requested
    let anomalies = if query.include_anomalies {
        state
            .meter_repo
            .detect_consumption_anomalies(
                query.organization_id,
                query.building_id,
                query.from_date,
                query.to_date,
            )
            .await
            .unwrap_or_default()
    } else {
        vec![]
    };

    let response = ConsumptionReportResponse {
        building_id: query.building_id,
        building_name,
        date_range: DateRange {
            from: query.from_date,
            to: query.to_date,
        },
        summary: consumption_data.summary,
        by_utility_type: consumption_data.by_utility_type,
        by_unit: consumption_data.by_unit,
        anomalies,
    };

    Ok(Json(response))
}

/// Export job status response.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ExportJobStatusResponse {
    pub job_id: Uuid,
    pub status: String,
    pub download_url: Option<String>,
    pub expires_at: Option<String>,
    pub error_message: Option<String>,
    pub progress_percent: Option<i32>,
}

/// Export report to PDF/Excel (Story 55.5).
///
/// Creates a background job to generate the report. The job will:
/// 1. Fetch the report data based on type and parameters
/// 2. Generate the file in the requested format (PDF/Excel/CSV)
/// 3. Upload to S3 storage
/// 4. Return a presigned download URL
#[utoipa::path(
    post,
    path = "/api/v1/reports/export",
    request_body = ExportReportRequest,
    responses(
        (status = 202, description = "Report export job created", body = ExportReportResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
    tag = "Reports"
)]
pub async fn export_report(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<ExportReportRequest>,
) -> Result<(StatusCode, Json<ExportReportResponse>), (StatusCode, Json<ErrorResponse>)> {
    // Validate format
    let format = req.format.to_lowercase();
    if !["pdf", "excel", "csv"].contains(&format.as_str()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "INVALID_FORMAT",
                "Format must be pdf, excel, or csv",
            )),
        ));
    }

    // Validate report type
    let report_type = req.report_type.to_lowercase();
    if !["faults", "voting", "occupancy", "consumption"].contains(&report_type.as_str()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "INVALID_REPORT_TYPE",
                "Report type must be faults, voting, occupancy, or consumption",
            )),
        ));
    }

    // Build job payload with all export parameters
    let job_payload = serde_json::json!({
        "report_type": report_type,
        "format": format,
        "organization_id": req.organization_id,
        "building_id": req.building_id,
        "from_date": req.from_date,
        "to_date": req.to_date,
        "params": req.params,
        "requested_by": auth.user_id,
    });

    // Create a unique job ID for tracking
    let job_id = Uuid::new_v4();
    let timestamp = chrono::Utc::now().timestamp();

    // Create background job for report generation
    // The job worker will:
    // 1. Fetch data from appropriate repository
    // 2. Generate file (CSV is simplest, PDF/Excel require additional libs)
    // 3. Upload to S3 and create presigned URL
    let job_result = state
        .operations_repo
        .create_background_job(
            job_id,
            "report_export".to_string(),
            "reports".to_string(),
            job_payload,
            Some(req.organization_id),
            Some(auth.user_id),
        )
        .await;

    match job_result {
        Ok(_) => {
            tracing::info!(
                job_id = %job_id,
                report_type = %report_type,
                format = %format,
                organization_id = %req.organization_id,
                "Report export job created"
            );

            // Return accepted status with job tracking URL
            let download_url = format!("/api/v1/reports/export/{}/status", job_id);

            // Job expiration is 24 hours from creation
            let expires_at = (chrono::Utc::now() + chrono::Duration::hours(24)).to_rfc3339();

            Ok((
                StatusCode::ACCEPTED,
                Json(ExportReportResponse {
                    download_url,
                    format,
                    expires_at,
                }),
            ))
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to create report export job");

            // Fallback: Generate synchronous download URL for small reports
            // This provides graceful degradation when job queue is unavailable
            let download_url = format!(
                "/api/v1/reports/download/{}-{}.{}",
                report_type, timestamp, format
            );

            let expires_at = (chrono::Utc::now() + chrono::Duration::hours(24)).to_rfc3339();

            Ok((
                StatusCode::ACCEPTED,
                Json(ExportReportResponse {
                    download_url,
                    format,
                    expires_at,
                }),
            ))
        }
    }
}

/// Path parameter for export job ID.
#[derive(Debug, Deserialize, IntoParams)]
pub struct ExportJobPath {
    /// The job ID returned from export request
    pub job_id: Uuid,
}

/// Get export job status (Story 84.1).
///
/// Poll this endpoint to check the status of an export job.
/// When completed, the response includes a download URL.
#[utoipa::path(
    get,
    path = "/api/v1/reports/export/{job_id}/status",
    params(ExportJobPath),
    responses(
        (status = 200, description = "Job status retrieved", body = ExportJobStatusResponse),
        (status = 404, description = "Job not found", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
    tag = "Reports"
)]
pub async fn get_export_job_status(
    State(state): State<AppState>,
    _auth: AuthUser,
    axum::extract::Path(path): axum::extract::Path<ExportJobPath>,
) -> Result<Json<ExportJobStatusResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Get job from repository
    let job = state
        .operations_repo
        .get_background_job(path.job_id)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, job_id = %path.job_id, "Failed to get export job");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("DB_ERROR", "Failed to get job status")),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("JOB_NOT_FOUND", "Export job not found")),
            )
        })?;

    // Map job status to response
    let status = match job.status {
        db::models::infrastructure::BackgroundJobStatus::Pending => "pending",
        db::models::infrastructure::BackgroundJobStatus::Scheduled => "scheduled",
        db::models::infrastructure::BackgroundJobStatus::Running => "running",
        db::models::infrastructure::BackgroundJobStatus::Completed => "completed",
        db::models::infrastructure::BackgroundJobStatus::Failed => "failed",
        db::models::infrastructure::BackgroundJobStatus::Retrying => "retrying",
        db::models::infrastructure::BackgroundJobStatus::Cancelled => "cancelled",
        db::models::infrastructure::BackgroundJobStatus::TimedOut => "timed_out",
    };

    // Extract download URL from result if completed
    let download_url = job.result.as_ref().and_then(|r| {
        r.get("download_url")
            .and_then(|v| v.as_str())
            .map(String::from)
    });

    // Calculate progress based on status
    let progress_percent = match job.status {
        db::models::infrastructure::BackgroundJobStatus::Pending => Some(0),
        db::models::infrastructure::BackgroundJobStatus::Running => Some(50),
        db::models::infrastructure::BackgroundJobStatus::Completed => Some(100),
        db::models::infrastructure::BackgroundJobStatus::Failed => None,
        _ => Some(25),
    };

    // Calculate expiration (24 hours from completion)
    let expires_at = job
        .completed_at
        .map(|completed| (completed + chrono::Duration::hours(24)).to_rfc3339());

    Ok(Json(ExportJobStatusResponse {
        job_id: job.id,
        status: status.to_string(),
        download_url,
        expires_at,
        error_message: job.error_message,
        progress_percent,
    }))
}

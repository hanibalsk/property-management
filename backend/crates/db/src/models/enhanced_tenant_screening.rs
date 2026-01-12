// Epic 135: Enhanced Tenant Screening with AI Risk Scoring
// Models for AI-powered tenant screening and risk assessment

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

// =============================================================================
// Enums
// =============================================================================

/// AI risk score category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "ai_risk_category", rename_all = "snake_case")]
pub enum AiRiskCategory {
    Excellent,
    Good,
    Fair,
    Poor,
    VeryPoor,
}

impl AiRiskCategory {
    /// Get the category from a numeric score.
    pub fn from_score(score: i32, model: &AiRiskScoringModel) -> Self {
        if score >= model.excellent_threshold {
            Self::Excellent
        } else if score >= model.good_threshold {
            Self::Good
        } else if score >= model.fair_threshold {
            Self::Fair
        } else if score >= model.poor_threshold {
            Self::Poor
        } else {
            Self::VeryPoor
        }
    }
}

/// Risk factor category (Fair Housing compliant - no protected classes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "risk_factor_category", rename_all = "snake_case")]
pub enum RiskFactorCategory {
    CreditHistory,
    RentalHistory,
    IncomeStability,
    EmploymentStability,
    EvictionHistory,
    CriminalBackground,
    IdentityVerification,
    ReferenceQuality,
}

/// Risk factor impact level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "risk_factor_impact", rename_all = "snake_case")]
pub enum RiskFactorImpact {
    Positive,
    Neutral,
    Negative,
    Critical,
}

/// Screening provider type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "screening_provider_type", rename_all = "snake_case")]
pub enum ScreeningProviderType {
    CreditBureau,
    BackgroundCheck,
    EvictionDatabase,
    IdentityVerification,
    EmploymentVerification,
    RentalHistory,
}

/// Provider integration status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "provider_integration_status", rename_all = "snake_case")]
pub enum ProviderIntegrationStatus {
    Active,
    Inactive,
    Error,
    RateLimited,
    Maintenance,
}

// =============================================================================
// AI Risk Scoring Models
// =============================================================================

/// AI risk scoring model configuration.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct AiRiskScoringModel {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub version: String,
    pub is_active: bool,

    // Factor weights
    pub credit_history_weight: Decimal,
    pub rental_history_weight: Decimal,
    pub income_stability_weight: Decimal,
    pub employment_stability_weight: Decimal,
    pub eviction_history_weight: Decimal,
    pub criminal_background_weight: Decimal,
    pub identity_verification_weight: Decimal,
    pub reference_quality_weight: Decimal,

    // Thresholds
    pub excellent_threshold: i32,
    pub good_threshold: i32,
    pub fair_threshold: i32,
    pub poor_threshold: i32,

    // Auto-approval/rejection
    pub auto_approve_threshold: Option<i32>,
    pub auto_reject_threshold: Option<i32>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
}

/// Create AI risk scoring model request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateAiRiskScoringModel {
    pub name: String,
    pub description: Option<String>,
    pub credit_history_weight: Option<Decimal>,
    pub rental_history_weight: Option<Decimal>,
    pub income_stability_weight: Option<Decimal>,
    pub employment_stability_weight: Option<Decimal>,
    pub eviction_history_weight: Option<Decimal>,
    pub criminal_background_weight: Option<Decimal>,
    pub identity_verification_weight: Option<Decimal>,
    pub reference_quality_weight: Option<Decimal>,
    pub excellent_threshold: Option<i32>,
    pub good_threshold: Option<i32>,
    pub fair_threshold: Option<i32>,
    pub poor_threshold: Option<i32>,
    pub auto_approve_threshold: Option<i32>,
    pub auto_reject_threshold: Option<i32>,
}

/// Update AI risk scoring model request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateAiRiskScoringModel {
    pub name: Option<String>,
    pub description: Option<String>,
    pub credit_history_weight: Option<Decimal>,
    pub rental_history_weight: Option<Decimal>,
    pub income_stability_weight: Option<Decimal>,
    pub employment_stability_weight: Option<Decimal>,
    pub eviction_history_weight: Option<Decimal>,
    pub criminal_background_weight: Option<Decimal>,
    pub identity_verification_weight: Option<Decimal>,
    pub reference_quality_weight: Option<Decimal>,
    pub excellent_threshold: Option<i32>,
    pub good_threshold: Option<i32>,
    pub fair_threshold: Option<i32>,
    pub poor_threshold: Option<i32>,
    pub auto_approve_threshold: Option<i32>,
    pub auto_reject_threshold: Option<i32>,
}

// =============================================================================
// Screening Provider Configuration
// =============================================================================

/// Screening provider configuration.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ScreeningProviderConfig {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub provider_name: String,
    pub provider_type: ScreeningProviderType,
    pub api_endpoint: Option<String>,
    pub status: ProviderIntegrationStatus,
    pub last_health_check: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub rate_limit_per_hour: Option<i32>,
    pub requests_this_hour: Option<i32>,
    pub hour_reset_at: Option<DateTime<Utc>>,
    pub priority: i32,
    pub cost_per_check: Option<Decimal>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create screening provider config request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateScreeningProviderConfig {
    pub provider_name: String,
    pub provider_type: ScreeningProviderType,
    pub api_endpoint: Option<String>,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub rate_limit_per_hour: Option<i32>,
    pub priority: Option<i32>,
    pub cost_per_check: Option<Decimal>,
}

/// Update screening provider config request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateScreeningProviderConfig {
    pub api_endpoint: Option<String>,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub status: Option<ProviderIntegrationStatus>,
    pub rate_limit_per_hour: Option<i32>,
    pub priority: Option<i32>,
    pub cost_per_check: Option<Decimal>,
}

// =============================================================================
// AI Screening Results
// =============================================================================

/// AI-generated screening result.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ScreeningAiResult {
    pub id: Uuid,
    pub screening_id: Uuid,
    pub organization_id: Uuid,
    pub model_id: Uuid,

    pub ai_risk_score: i32,
    pub risk_category: AiRiskCategory,

    // Component scores
    pub credit_score_component: Option<i32>,
    pub rental_history_component: Option<i32>,
    pub income_stability_component: Option<i32>,
    pub employment_stability_component: Option<i32>,
    pub eviction_history_component: Option<i32>,
    pub criminal_background_component: Option<i32>,
    pub identity_verification_component: Option<i32>,
    pub reference_quality_component: Option<i32>,

    pub recommendation: String,
    pub recommendation_reason: Option<String>,
    pub percentile_rank: Option<i32>,
    pub typical_tenant_score: Option<i32>,
    pub confidence_score: Option<Decimal>,

    pub created_at: DateTime<Utc>,
}

/// Risk factor contributing to AI score.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ScreeningRiskFactor {
    pub id: Uuid,
    pub ai_result_id: Uuid,
    pub category: RiskFactorCategory,
    pub factor_name: String,
    pub factor_description: String,
    pub impact: RiskFactorImpact,
    pub score_impact: i32,
    pub source_provider: Option<String>,
    pub source_data: Option<serde_json::Value>,
    pub is_compliant: bool,
    pub compliance_notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Create risk factor request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateScreeningRiskFactor {
    pub category: RiskFactorCategory,
    pub factor_name: String,
    pub factor_description: String,
    pub impact: RiskFactorImpact,
    pub score_impact: i32,
    pub source_provider: Option<String>,
    pub source_data: Option<serde_json::Value>,
    pub is_compliant: bool,
    pub compliance_notes: Option<String>,
}

// =============================================================================
// Credit Check Results
// =============================================================================

/// Credit check result.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ScreeningCreditResult {
    pub id: Uuid,
    pub screening_id: Uuid,
    pub organization_id: Uuid,
    pub provider_config_id: Option<Uuid>,
    pub provider_name: String,

    pub credit_score: Option<i32>,
    pub score_model: Option<String>,
    pub score_date: Option<NaiveDate>,

    pub total_accounts: Option<i32>,
    pub open_accounts: Option<i32>,
    pub delinquent_accounts: Option<i32>,
    pub collections_count: Option<i32>,
    pub total_debt: Option<Decimal>,
    pub available_credit: Option<Decimal>,
    pub utilization_ratio: Option<Decimal>,

    pub on_time_payments_pct: Option<Decimal>,
    pub late_30_days_count: Option<i32>,
    pub late_60_days_count: Option<i32>,
    pub late_90_plus_count: Option<i32>,

    pub bankruptcies_count: Option<i32>,
    pub most_recent_bankruptcy_date: Option<NaiveDate>,
    pub public_records_count: Option<i32>,

    pub requested_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Create credit result request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateScreeningCreditResult {
    pub screening_id: Uuid,
    pub provider_config_id: Option<Uuid>,
    pub provider_name: String,
    pub credit_score: Option<i32>,
    pub score_model: Option<String>,
    pub score_date: Option<NaiveDate>,
    pub total_accounts: Option<i32>,
    pub open_accounts: Option<i32>,
    pub delinquent_accounts: Option<i32>,
    pub collections_count: Option<i32>,
    pub total_debt: Option<Decimal>,
    pub available_credit: Option<Decimal>,
    pub utilization_ratio: Option<Decimal>,
    pub on_time_payments_pct: Option<Decimal>,
    pub late_30_days_count: Option<i32>,
    pub late_60_days_count: Option<i32>,
    pub late_90_plus_count: Option<i32>,
    pub bankruptcies_count: Option<i32>,
    pub most_recent_bankruptcy_date: Option<NaiveDate>,
    pub public_records_count: Option<i32>,
}

// =============================================================================
// Background Check Results
// =============================================================================

/// Background check result.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ScreeningBackgroundResult {
    pub id: Uuid,
    pub screening_id: Uuid,
    pub organization_id: Uuid,
    pub provider_config_id: Option<Uuid>,
    pub provider_name: String,

    pub has_criminal_records: Option<bool>,
    pub felony_count: Option<i32>,
    pub misdemeanor_count: Option<i32>,
    pub most_recent_offense_date: Option<NaiveDate>,
    pub offense_categories: Option<serde_json::Value>,

    pub sex_offender_check_passed: Option<bool>,
    pub watchlist_check_passed: Option<bool>,

    pub identity_verified: Option<bool>,
    pub identity_match_score: Option<Decimal>,
    pub ssn_verified: Option<bool>,
    pub address_history_verified: Option<bool>,

    pub requested_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Create background result request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateScreeningBackgroundResult {
    pub screening_id: Uuid,
    pub provider_config_id: Option<Uuid>,
    pub provider_name: String,
    pub has_criminal_records: Option<bool>,
    pub felony_count: Option<i32>,
    pub misdemeanor_count: Option<i32>,
    pub most_recent_offense_date: Option<NaiveDate>,
    pub offense_categories: Option<serde_json::Value>,
    pub sex_offender_check_passed: Option<bool>,
    pub watchlist_check_passed: Option<bool>,
    pub identity_verified: Option<bool>,
    pub identity_match_score: Option<Decimal>,
    pub ssn_verified: Option<bool>,
    pub address_history_verified: Option<bool>,
}

// =============================================================================
// Eviction Check Results
// =============================================================================

/// Eviction history result.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ScreeningEvictionResult {
    pub id: Uuid,
    pub screening_id: Uuid,
    pub organization_id: Uuid,
    pub provider_config_id: Option<Uuid>,
    pub provider_name: String,

    pub has_eviction_records: Option<bool>,
    pub eviction_count: Option<i32>,
    pub most_recent_eviction_date: Option<NaiveDate>,
    pub eviction_records: Option<serde_json::Value>,
    pub unlawful_detainer_count: Option<i32>,

    pub requested_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Create eviction result request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateScreeningEvictionResult {
    pub screening_id: Uuid,
    pub provider_config_id: Option<Uuid>,
    pub provider_name: String,
    pub has_eviction_records: Option<bool>,
    pub eviction_count: Option<i32>,
    pub most_recent_eviction_date: Option<NaiveDate>,
    pub eviction_records: Option<serde_json::Value>,
    pub unlawful_detainer_count: Option<i32>,
}

// =============================================================================
// Screening Request Queue
// =============================================================================

/// Screening request queue item.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ScreeningRequestQueueItem {
    pub id: Uuid,
    pub screening_id: Uuid,
    pub organization_id: Uuid,
    pub check_type: ScreeningProviderType,
    pub provider_config_id: Option<Uuid>,
    pub status: String,
    pub priority: i32,
    pub attempt_count: i32,
    pub max_attempts: i32,
    pub next_retry_at: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create queue item request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateScreeningQueueItem {
    pub screening_id: Uuid,
    pub check_type: ScreeningProviderType,
    pub provider_config_id: Option<Uuid>,
    pub priority: Option<i32>,
}

// =============================================================================
// Screening Reports
// =============================================================================

/// Generated screening report.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ScreeningReport {
    pub id: Uuid,
    pub screening_id: Uuid,
    pub organization_id: Uuid,
    pub report_type: String,
    pub generated_at: DateTime<Utc>,
    pub generated_by: Uuid,
    pub file_url: Option<String>,
    pub file_size_bytes: Option<i64>,
    pub content_hash: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Create report request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateScreeningReport {
    pub screening_id: Uuid,
    pub report_type: String,
    pub file_url: Option<String>,
    pub file_size_bytes: Option<i64>,
    pub content_hash: Option<String>,
}

// =============================================================================
// Dashboard and Summary Types
// =============================================================================

/// Screening summary for dashboard.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ScreeningSummary {
    pub screening_id: Uuid,
    pub applicant_name: String,
    pub unit_address: String,
    pub ai_risk_score: Option<i32>,
    pub risk_category: Option<AiRiskCategory>,
    pub recommendation: Option<String>,
    pub screening_status: String,
    pub created_at: DateTime<Utc>,
}

/// AI result with risk factors.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AiResultWithFactors {
    pub result: ScreeningAiResult,
    pub factors: Vec<ScreeningRiskFactor>,
}

/// Complete screening report data.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CompleteScreeningData {
    pub ai_result: Option<ScreeningAiResult>,
    pub risk_factors: Vec<ScreeningRiskFactor>,
    pub credit_result: Option<ScreeningCreditResult>,
    pub background_result: Option<ScreeningBackgroundResult>,
    pub eviction_result: Option<ScreeningEvictionResult>,
}

/// Screening statistics.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ScreeningStatistics {
    pub total_screenings: i64,
    pub pending_screenings: i64,
    pub completed_screenings: i64,
    pub average_risk_score: Option<Decimal>,
    pub approval_rate: Option<Decimal>,
    pub avg_processing_time_hours: Option<Decimal>,
}

/// Risk category distribution.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct RiskCategoryDistribution {
    pub category: AiRiskCategory,
    pub count: i64,
    pub percentage: Decimal,
}

/// Run AI scoring request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RunAiScoringRequest {
    pub screening_id: Uuid,
    pub model_id: Option<Uuid>,
}

/// Initiate screening request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct InitiateScreeningRequest {
    pub application_id: Uuid,
    pub check_types: Vec<ScreeningProviderType>,
}

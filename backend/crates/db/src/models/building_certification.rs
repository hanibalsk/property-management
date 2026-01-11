//! Building Certification models for Epic 137: Smart Building Certification.
//!
//! Supports tracking of building certifications like LEED, BREEAM, WELL, Fitwel,
//! Energy Star, and other green building and wellness certifications.

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Certification program types (LEED, BREEAM, WELL, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "certification_program", rename_all = "snake_case")]
pub enum CertificationProgram {
    Leed,
    Breeam,
    Well,
    Fitwel,
    EnergyStar,
    GreenGlobes,
    LivingBuildingChallenge,
    PassiveHouse,
    Nabers,
    Dgnb,
    Hqe,
    Casbee,
    Edge,
    BomaBest,
    Other,
}

/// Certification level (Certified, Silver, Gold, Platinum, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "certification_level", rename_all = "snake_case")]
pub enum CertificationLevel {
    Certified,
    Silver,
    Gold,
    Platinum,
    OneStar,
    TwoStar,
    ThreeStar,
    FourStar,
    FiveStar,
    Pending,
    Expired,
    Other,
}

/// Certification status in the lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "certification_status", rename_all = "snake_case")]
pub enum CertificationStatus {
    Planning,
    InProgress,
    UnderReview,
    Achieved,
    Renewed,
    Expired,
    Revoked,
}

/// Credit category types for certification credits
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "credit_category_type", rename_all = "snake_case")]
pub enum CreditCategoryType {
    EnergyEfficiency,
    WaterEfficiency,
    MaterialsResources,
    IndoorEnvironmentalQuality,
    SustainableSites,
    LocationTransportation,
    Innovation,
    RegionalPriority,
    Wellness,
    AirQuality,
    Light,
    ThermalComfort,
    Sound,
    Materials,
    Water,
    Nourishment,
    Movement,
    Community,
    Mind,
    Other,
}

/// Building Certification record
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct BuildingCertification {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub building_id: Uuid,

    // Certification Details
    pub program: CertificationProgram,
    pub version: Option<String>,
    pub level: CertificationLevel,
    pub status: CertificationStatus,

    // Scores
    pub total_points_possible: Option<i32>,
    pub total_points_achieved: Option<i32>,
    pub percentage_achieved: Option<rust_decimal::Decimal>,

    // Dates
    pub application_date: Option<NaiveDate>,
    pub certification_date: Option<NaiveDate>,
    pub expiration_date: Option<NaiveDate>,
    pub renewal_date: Option<NaiveDate>,

    // Reference Information
    pub certificate_number: Option<String>,
    pub project_id: Option<String>,
    pub assessor_name: Option<String>,
    pub assessor_organization: Option<String>,

    // Documentation
    pub certificate_url: Option<String>,
    pub scorecard_url: Option<String>,
    pub notes: Option<String>,

    // Cost Tracking
    pub application_fee: Option<rust_decimal::Decimal>,
    pub certification_fee: Option<rust_decimal::Decimal>,
    pub annual_fee: Option<rust_decimal::Decimal>,
    pub total_investment: Option<rust_decimal::Decimal>,

    // Metadata
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Input for creating a new building certification
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateBuildingCertification {
    pub building_id: Uuid,
    pub program: CertificationProgram,
    pub version: Option<String>,
    pub level: CertificationLevel,
    pub status: Option<CertificationStatus>,
    pub total_points_possible: Option<i32>,
    pub total_points_achieved: Option<i32>,
    pub application_date: Option<NaiveDate>,
    pub certification_date: Option<NaiveDate>,
    pub expiration_date: Option<NaiveDate>,
    pub certificate_number: Option<String>,
    pub project_id: Option<String>,
    pub assessor_name: Option<String>,
    pub assessor_organization: Option<String>,
    pub certificate_url: Option<String>,
    pub scorecard_url: Option<String>,
    pub notes: Option<String>,
    pub application_fee: Option<rust_decimal::Decimal>,
    pub certification_fee: Option<rust_decimal::Decimal>,
    pub annual_fee: Option<rust_decimal::Decimal>,
}

/// Input for updating a building certification
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateBuildingCertification {
    pub version: Option<String>,
    pub level: Option<CertificationLevel>,
    pub status: Option<CertificationStatus>,
    pub total_points_possible: Option<i32>,
    pub total_points_achieved: Option<i32>,
    pub application_date: Option<NaiveDate>,
    pub certification_date: Option<NaiveDate>,
    pub expiration_date: Option<NaiveDate>,
    pub renewal_date: Option<NaiveDate>,
    pub certificate_number: Option<String>,
    pub project_id: Option<String>,
    pub assessor_name: Option<String>,
    pub assessor_organization: Option<String>,
    pub certificate_url: Option<String>,
    pub scorecard_url: Option<String>,
    pub notes: Option<String>,
    pub application_fee: Option<rust_decimal::Decimal>,
    pub certification_fee: Option<rust_decimal::Decimal>,
    pub annual_fee: Option<rust_decimal::Decimal>,
    pub total_investment: Option<rust_decimal::Decimal>,
}

/// Certification Credit record
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CertificationCredit {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub certification_id: Uuid,

    // Credit Details
    pub category: CreditCategoryType,
    pub credit_code: Option<String>,
    pub credit_name: String,
    pub description: Option<String>,

    // Points
    pub points_possible: i32,
    pub points_achieved: i32,
    pub is_prerequisite: bool,

    // Status
    pub status: String,
    pub documentation_status: Option<String>,

    // Documentation
    pub evidence_urls: serde_json::Value,
    pub notes: Option<String>,

    // Responsible Party
    pub responsible_user_id: Option<Uuid>,
    pub due_date: Option<NaiveDate>,

    // Metadata
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Input for creating a certification credit
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateCertificationCredit {
    pub certification_id: Uuid,
    pub category: CreditCategoryType,
    pub credit_code: Option<String>,
    pub credit_name: String,
    pub description: Option<String>,
    pub points_possible: i32,
    pub points_achieved: Option<i32>,
    pub is_prerequisite: Option<bool>,
    pub status: Option<String>,
    pub documentation_status: Option<String>,
    pub evidence_urls: Option<Vec<String>>,
    pub notes: Option<String>,
    pub responsible_user_id: Option<Uuid>,
    pub due_date: Option<NaiveDate>,
}

/// Input for updating a certification credit
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateCertificationCredit {
    pub category: Option<CreditCategoryType>,
    pub credit_code: Option<String>,
    pub credit_name: Option<String>,
    pub description: Option<String>,
    pub points_possible: Option<i32>,
    pub points_achieved: Option<i32>,
    pub is_prerequisite: Option<bool>,
    pub status: Option<String>,
    pub documentation_status: Option<String>,
    pub evidence_urls: Option<Vec<String>>,
    pub notes: Option<String>,
    pub responsible_user_id: Option<Uuid>,
    pub due_date: Option<NaiveDate>,
}

/// Certification Document record
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CertificationDocument {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub certification_id: Uuid,
    pub credit_id: Option<Uuid>,

    // Document Details
    pub document_type: String,
    pub title: String,
    pub description: Option<String>,
    pub file_url: String,
    pub file_size_bytes: Option<i64>,
    pub file_type: Option<String>,

    // Version Control
    pub version: i32,
    pub is_current: bool,
    pub supersedes_id: Option<Uuid>,

    // Submission Status
    pub submitted_date: Option<DateTime<Utc>>,
    pub review_status: Option<String>,
    pub reviewer_comments: Option<String>,

    // Metadata
    pub uploaded_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Input for creating a certification document
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateCertificationDocument {
    pub certification_id: Uuid,
    pub credit_id: Option<Uuid>,
    pub document_type: String,
    pub title: String,
    pub description: Option<String>,
    pub file_url: String,
    pub file_size_bytes: Option<i64>,
    pub file_type: Option<String>,
}

/// Certification Milestone record
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CertificationMilestone {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub certification_id: Uuid,

    // Milestone Details
    pub milestone_name: String,
    pub description: Option<String>,
    pub phase: Option<String>,

    // Timeline
    pub target_date: Option<NaiveDate>,
    pub actual_date: Option<NaiveDate>,
    pub status: String,

    // Dependencies
    pub depends_on_milestone_id: Option<Uuid>,

    // Assignments
    pub assigned_to: Option<Uuid>,
    pub notes: Option<String>,

    // Metadata
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Input for creating a certification milestone
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateCertificationMilestone {
    pub certification_id: Uuid,
    pub milestone_name: String,
    pub description: Option<String>,
    pub phase: Option<String>,
    pub target_date: Option<NaiveDate>,
    pub status: Option<String>,
    pub depends_on_milestone_id: Option<Uuid>,
    pub assigned_to: Option<Uuid>,
    pub notes: Option<String>,
}

/// Input for updating a certification milestone
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateCertificationMilestone {
    pub milestone_name: Option<String>,
    pub description: Option<String>,
    pub phase: Option<String>,
    pub target_date: Option<NaiveDate>,
    pub actual_date: Option<NaiveDate>,
    pub status: Option<String>,
    pub depends_on_milestone_id: Option<Uuid>,
    pub assigned_to: Option<Uuid>,
    pub notes: Option<String>,
}

/// Certification Benchmark record
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CertificationBenchmark {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub certification_id: Uuid,

    // Benchmark Details
    pub metric_name: String,
    pub metric_unit: Option<String>,
    pub building_value: Option<rust_decimal::Decimal>,
    pub benchmark_25th_percentile: Option<rust_decimal::Decimal>,
    pub benchmark_50th_percentile: Option<rust_decimal::Decimal>,
    pub benchmark_75th_percentile: Option<rust_decimal::Decimal>,
    pub benchmark_source: Option<String>,

    // Percentile Ranking
    pub percentile_rank: Option<i32>,

    // Period
    pub measurement_period_start: Option<NaiveDate>,
    pub measurement_period_end: Option<NaiveDate>,

    // Metadata
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Input for creating a certification benchmark
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateCertificationBenchmark {
    pub certification_id: Uuid,
    pub metric_name: String,
    pub metric_unit: Option<String>,
    pub building_value: Option<rust_decimal::Decimal>,
    pub benchmark_25th_percentile: Option<rust_decimal::Decimal>,
    pub benchmark_50th_percentile: Option<rust_decimal::Decimal>,
    pub benchmark_75th_percentile: Option<rust_decimal::Decimal>,
    pub benchmark_source: Option<String>,
    pub percentile_rank: Option<i32>,
    pub measurement_period_start: Option<NaiveDate>,
    pub measurement_period_end: Option<NaiveDate>,
}

/// Certification Audit Log record
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CertificationAuditLog {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub certification_id: Uuid,

    // Audit Details
    pub action: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub previous_value: Option<serde_json::Value>,
    pub new_value: Option<serde_json::Value>,
    pub notes: Option<String>,

    // Actor
    pub performed_by: Option<Uuid>,
    pub performed_at: DateTime<Utc>,
}

/// Certification Cost record
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CertificationCost {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub certification_id: Uuid,

    // Cost Details
    pub cost_type: String,
    pub description: Option<String>,
    pub amount: rust_decimal::Decimal,
    pub currency: String,

    // Date
    pub incurred_date: Option<NaiveDate>,
    pub paid_date: Option<NaiveDate>,
    pub invoice_number: Option<String>,

    // Vendor
    pub vendor_name: Option<String>,
    pub vendor_id: Option<Uuid>,

    // Metadata
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

/// Input for creating a certification cost
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateCertificationCost {
    pub certification_id: Uuid,
    pub cost_type: String,
    pub description: Option<String>,
    pub amount: rust_decimal::Decimal,
    pub currency: Option<String>,
    pub incurred_date: Option<NaiveDate>,
    pub paid_date: Option<NaiveDate>,
    pub invoice_number: Option<String>,
    pub vendor_name: Option<String>,
    pub vendor_id: Option<Uuid>,
}

/// Certification Reminder record
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CertificationReminder {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub certification_id: Uuid,

    // Reminder Details
    pub reminder_type: String,
    pub days_before: i32,
    pub message: Option<String>,

    // Recipients
    pub notify_users: serde_json::Value,
    pub notify_roles: serde_json::Value,

    // Status
    pub is_active: bool,
    pub last_sent_at: Option<DateTime<Utc>>,

    // Metadata
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Input for creating a certification reminder
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateCertificationReminder {
    pub certification_id: Uuid,
    pub reminder_type: String,
    pub days_before: i32,
    pub message: Option<String>,
    pub notify_users: Option<Vec<Uuid>>,
    pub notify_roles: Option<Vec<String>>,
}

/// Certification Dashboard Summary
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CertificationDashboard {
    pub total_certifications: i64,
    pub active_certifications: i64,
    pub expiring_soon: i64,
    pub in_progress: i64,
    pub by_program: Vec<CertificationProgramCount>,
    pub by_level: Vec<CertificationLevelCount>,
    pub upcoming_milestones: Vec<CertificationMilestone>,
    pub total_investment: rust_decimal::Decimal,
}

/// Count by certification program
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CertificationProgramCount {
    pub program: CertificationProgram,
    pub count: i64,
}

/// Count by certification level
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CertificationLevelCount {
    pub level: CertificationLevel,
    pub count: i64,
}

/// Certification with credits summary
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CertificationWithCredits {
    #[serde(flatten)]
    pub certification: BuildingCertification,
    pub credits_achieved: i64,
    pub credits_total: i64,
    pub prerequisites_met: i64,
    pub prerequisites_total: i64,
}

/// Filter options for certifications
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct CertificationFilters {
    pub building_id: Option<Uuid>,
    pub program: Option<CertificationProgram>,
    pub level: Option<CertificationLevel>,
    pub status: Option<CertificationStatus>,
    pub expiring_within_days: Option<i32>,
}

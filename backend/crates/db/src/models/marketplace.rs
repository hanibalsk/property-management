//! Service Provider Marketplace models (Epic 68).
//!
//! Stories:
//! - 68.1: Service Provider Profiles
//! - 68.2: Search & Discovery
//! - 68.3: Request for Quote (RFQ)
//! - 68.4: Provider Verification
//! - 68.5: Reviews & Ratings

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

// ==================== Constants ====================

/// Provider profile status constants.
pub mod profile_status {
    pub const DRAFT: &str = "draft";
    pub const PENDING_REVIEW: &str = "pending_review";
    pub const ACTIVE: &str = "active";
    pub const SUSPENDED: &str = "suspended";
    pub const INACTIVE: &str = "inactive";
    pub const ALL: &[&str] = &[DRAFT, PENDING_REVIEW, ACTIVE, SUSPENDED, INACTIVE];
}

/// Service category constants for marketplace.
pub mod service_category {
    pub const PLUMBING: &str = "plumbing";
    pub const ELECTRICAL: &str = "electrical";
    pub const HVAC: &str = "hvac";
    pub const CLEANING: &str = "cleaning";
    pub const LANDSCAPING: &str = "landscaping";
    pub const SECURITY: &str = "security";
    pub const PAINTING: &str = "painting";
    pub const ROOFING: &str = "roofing";
    pub const CARPENTRY: &str = "carpentry";
    pub const LOCKSMITH: &str = "locksmith";
    pub const PEST_CONTROL: &str = "pest_control";
    pub const GENERAL_MAINTENANCE: &str = "general_maintenance";
    pub const ELEVATOR_MAINTENANCE: &str = "elevator_maintenance";
    pub const FIRE_SAFETY: &str = "fire_safety";
    pub const WASTE_MANAGEMENT: &str = "waste_management";
    pub const OTHER: &str = "other";
    pub const ALL: &[&str] = &[
        PLUMBING,
        ELECTRICAL,
        HVAC,
        CLEANING,
        LANDSCAPING,
        SECURITY,
        PAINTING,
        ROOFING,
        CARPENTRY,
        LOCKSMITH,
        PEST_CONTROL,
        GENERAL_MAINTENANCE,
        ELEVATOR_MAINTENANCE,
        FIRE_SAFETY,
        WASTE_MANAGEMENT,
        OTHER,
    ];
}

/// Pricing type constants.
pub mod pricing_type {
    pub const HOURLY: &str = "hourly";
    pub const PROJECT: &str = "project";
    pub const FIXED: &str = "fixed";
    pub const QUOTE_REQUIRED: &str = "quote_required";
    pub const ALL: &[&str] = &[HOURLY, PROJECT, FIXED, QUOTE_REQUIRED];
}

/// RFQ status constants.
pub mod rfq_status {
    pub const DRAFT: &str = "draft";
    pub const SENT: &str = "sent";
    pub const QUOTES_RECEIVED: &str = "quotes_received";
    pub const AWARDED: &str = "awarded";
    pub const CANCELLED: &str = "cancelled";
    pub const EXPIRED: &str = "expired";
    pub const ALL: &[&str] = &[DRAFT, SENT, QUOTES_RECEIVED, AWARDED, CANCELLED, EXPIRED];
}

/// Quote status constants.
pub mod quote_status {
    pub const PENDING: &str = "pending";
    pub const SUBMITTED: &str = "submitted";
    pub const ACCEPTED: &str = "accepted";
    pub const REJECTED: &str = "rejected";
    pub const WITHDRAWN: &str = "withdrawn";
    pub const EXPIRED: &str = "expired";
    pub const ALL: &[&str] = &[PENDING, SUBMITTED, ACCEPTED, REJECTED, WITHDRAWN, EXPIRED];
}

/// Verification status constants.
pub mod verification_status {
    pub const PENDING: &str = "pending";
    pub const UNDER_REVIEW: &str = "under_review";
    pub const VERIFIED: &str = "verified";
    pub const REJECTED: &str = "rejected";
    pub const EXPIRED: &str = "expired";
    pub const ALL: &[&str] = &[PENDING, UNDER_REVIEW, VERIFIED, REJECTED, EXPIRED];
}

/// Verification type constants.
pub mod verification_type {
    pub const BUSINESS_REGISTRATION: &str = "business_registration";
    pub const INSURANCE: &str = "insurance";
    pub const CERTIFICATION: &str = "certification";
    pub const LICENSE: &str = "license";
    pub const IDENTITY: &str = "identity";
    pub const ALL: &[&str] = &[
        BUSINESS_REGISTRATION,
        INSURANCE,
        CERTIFICATION,
        LICENSE,
        IDENTITY,
    ];
}

/// Badge type constants.
pub mod badge_type {
    pub const VERIFIED_BUSINESS: &str = "verified_business";
    pub const INSURED: &str = "insured";
    pub const CERTIFIED: &str = "certified";
    pub const TOP_RATED: &str = "top_rated";
    pub const FAST_RESPONDER: &str = "fast_responder";
    pub const PREFERRED: &str = "preferred";
    pub const ALL: &[&str] = &[
        VERIFIED_BUSINESS,
        INSURED,
        CERTIFIED,
        TOP_RATED,
        FAST_RESPONDER,
        PREFERRED,
    ];
}

/// Review moderation status.
pub mod review_status {
    pub const PENDING: &str = "pending";
    pub const APPROVED: &str = "approved";
    pub const FLAGGED: &str = "flagged";
    pub const REMOVED: &str = "removed";
    pub const ALL: &[&str] = &[PENDING, APPROVED, FLAGGED, REMOVED];
}

// ==================== Story 68.1: Service Provider Profiles ====================

/// Service provider marketplace profile entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ServiceProviderProfile {
    pub id: Uuid,
    pub user_id: Uuid,

    // Company information
    pub company_name: String,
    pub business_registration_number: Option<String>,
    pub tax_id: Option<String>,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub website: Option<String>,

    // Contact information
    pub contact_name: String,
    pub contact_email: String,
    pub contact_phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,

    // Services offered
    pub service_categories: Vec<String>,
    pub service_description: Option<String>,
    pub specializations: Option<Vec<String>>,

    // Coverage area
    pub coverage_postal_codes: Option<Vec<String>>,
    pub coverage_radius_km: Option<i32>,
    pub coverage_regions: Option<Vec<String>>,

    // Pricing structure
    pub pricing_type: String,
    pub hourly_rate_min: Option<Decimal>,
    pub hourly_rate_max: Option<Decimal>,
    pub currency: Option<String>,

    // Certifications and licenses
    pub certifications: Option<serde_json::Value>,
    pub licenses: Option<serde_json::Value>,

    // Availability
    pub availability_calendar: Option<serde_json::Value>,
    pub response_time_hours: Option<i32>,
    pub emergency_available: Option<bool>,

    // Portfolio
    pub portfolio_images: Option<Vec<String>>,
    pub portfolio_description: Option<String>,

    // Status and metrics
    pub status: String,
    pub is_featured: Option<bool>,
    pub average_rating: Option<Decimal>,
    pub total_reviews: Option<i32>,
    pub total_jobs_completed: Option<i32>,

    // Verification
    pub is_verified: Option<bool>,
    pub verified_at: Option<DateTime<Utc>>,
    pub badges: Option<Vec<String>>,

    // Metadata
    pub metadata: Option<serde_json::Value>,

    // Timestamps
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub last_active_at: Option<DateTime<Utc>>,
}

/// Create service provider profile request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateServiceProviderProfile {
    pub company_name: String,
    pub business_registration_number: Option<String>,
    pub tax_id: Option<String>,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub website: Option<String>,
    pub contact_name: String,
    pub contact_email: String,
    pub contact_phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub service_categories: Vec<String>,
    pub service_description: Option<String>,
    pub specializations: Option<Vec<String>>,
    pub coverage_postal_codes: Option<Vec<String>>,
    pub coverage_radius_km: Option<i32>,
    pub coverage_regions: Option<Vec<String>>,
    pub pricing_type: Option<String>,
    pub hourly_rate_min: Option<Decimal>,
    pub hourly_rate_max: Option<Decimal>,
    pub currency: Option<String>,
    pub certifications: Option<serde_json::Value>,
    pub licenses: Option<serde_json::Value>,
    pub availability_calendar: Option<serde_json::Value>,
    pub response_time_hours: Option<i32>,
    pub emergency_available: Option<bool>,
    pub portfolio_images: Option<Vec<String>>,
    pub portfolio_description: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Update service provider profile request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateServiceProviderProfile {
    pub company_name: Option<String>,
    pub business_registration_number: Option<String>,
    pub tax_id: Option<String>,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub website: Option<String>,
    pub contact_name: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub service_categories: Option<Vec<String>>,
    pub service_description: Option<String>,
    pub specializations: Option<Vec<String>>,
    pub coverage_postal_codes: Option<Vec<String>>,
    pub coverage_radius_km: Option<i32>,
    pub coverage_regions: Option<Vec<String>>,
    pub pricing_type: Option<String>,
    pub hourly_rate_min: Option<Decimal>,
    pub hourly_rate_max: Option<Decimal>,
    pub currency: Option<String>,
    pub certifications: Option<serde_json::Value>,
    pub licenses: Option<serde_json::Value>,
    pub availability_calendar: Option<serde_json::Value>,
    pub response_time_hours: Option<i32>,
    pub emergency_available: Option<bool>,
    pub portfolio_images: Option<Vec<String>>,
    pub portfolio_description: Option<String>,
    pub status: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

// ==================== Story 68.2: Search & Discovery ====================

/// Search parameters for marketplace providers.
#[derive(Debug, Default, Deserialize, ToSchema)]
pub struct MarketplaceSearchQuery {
    /// Filter by service category
    pub category: Option<String>,
    /// Filter by multiple categories
    pub categories: Option<Vec<String>>,
    /// Full-text search query
    pub query: Option<String>,
    /// Location/postal code for coverage
    pub location: Option<String>,
    /// Postal codes for coverage
    pub postal_codes: Option<Vec<String>>,
    /// Region filter
    pub region: Option<String>,
    /// Minimum rating filter
    pub min_rating: Option<Decimal>,
    /// Maximum hourly rate
    pub max_hourly_rate: Option<Decimal>,
    /// Minimum hourly rate
    pub min_hourly_rate: Option<Decimal>,
    /// Only verified providers
    pub verified_only: Option<bool>,
    /// Only providers with specific badges
    pub badges: Option<Vec<String>>,
    /// Only emergency-available providers
    pub emergency_only: Option<bool>,
    /// Sort field (rating, reviews, jobs_completed, hourly_rate)
    pub sort_by: Option<String>,
    /// Sort direction (asc, desc)
    pub sort_order: Option<String>,
    /// Pagination limit
    pub limit: Option<i32>,
    /// Pagination offset
    pub offset: Option<i32>,
}

/// Provider search result for marketplace.
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ProviderSearchResult {
    pub id: Uuid,
    pub company_name: String,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub service_categories: Vec<String>,
    pub city: Option<String>,
    pub pricing_type: String,
    pub hourly_rate_min: Option<Decimal>,
    pub hourly_rate_max: Option<Decimal>,
    pub currency: Option<String>,
    pub average_rating: Option<Decimal>,
    pub total_reviews: Option<i32>,
    pub total_jobs_completed: Option<i32>,
    pub is_verified: Option<bool>,
    pub badges: Option<Vec<String>>,
    pub emergency_available: Option<bool>,
    pub response_time_hours: Option<i32>,
}

/// Provider detail view for marketplace.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProviderDetailView {
    pub profile: ServiceProviderProfile,
    pub verifications: Vec<ProviderVerification>,
    pub recent_reviews: Vec<ProviderReviewWithResponse>,
    pub rating_breakdown: RatingBreakdown,
}

/// Marketplace statistics.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MarketplaceStatistics {
    pub total_providers: i64,
    pub verified_providers: i64,
    pub providers_by_category: Vec<CategoryCount>,
    pub average_rating: Decimal,
    pub total_reviews: i64,
    pub total_jobs_completed: i64,
}

/// Category count for statistics.
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CategoryCount {
    pub category: String,
    pub count: i64,
}

// ==================== Story 68.3: Request for Quote (RFQ) ====================

/// Request for Quote entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct RequestForQuote {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub building_id: Option<Uuid>,
    pub created_by: Uuid,

    // RFQ details
    pub title: String,
    pub description: String,
    pub service_category: String,
    pub scope_of_work: Option<String>,

    // Timeline
    pub preferred_start_date: Option<NaiveDate>,
    pub preferred_end_date: Option<NaiveDate>,
    pub is_urgent: Option<bool>,

    // Budget
    pub budget_min: Option<Decimal>,
    pub budget_max: Option<Decimal>,
    pub currency: Option<String>,

    // Attachments
    pub attachments: Option<serde_json::Value>,
    pub images: Option<Vec<String>>,

    // Status and dates
    pub status: String,
    pub quote_deadline: Option<DateTime<Utc>>,
    pub awarded_to: Option<Uuid>,
    pub awarded_quote_id: Option<Uuid>,
    pub awarded_at: Option<DateTime<Utc>>,

    // Contact preferences
    pub contact_preference: Option<String>,
    pub site_visit_required: Option<bool>,

    // Metadata
    pub metadata: Option<serde_json::Value>,

    // Timestamps
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Create RFQ request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateRequestForQuote {
    pub building_id: Option<Uuid>,
    pub title: String,
    pub description: String,
    pub service_category: String,
    pub scope_of_work: Option<String>,
    pub preferred_start_date: Option<NaiveDate>,
    pub preferred_end_date: Option<NaiveDate>,
    pub is_urgent: Option<bool>,
    pub budget_min: Option<Decimal>,
    pub budget_max: Option<Decimal>,
    pub currency: Option<String>,
    pub attachments: Option<serde_json::Value>,
    pub images: Option<Vec<String>>,
    pub quote_deadline: Option<DateTime<Utc>>,
    pub contact_preference: Option<String>,
    pub site_visit_required: Option<bool>,
    pub provider_ids: Vec<Uuid>,
    pub metadata: Option<serde_json::Value>,
}

/// Update RFQ request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateRequestForQuote {
    pub title: Option<String>,
    pub description: Option<String>,
    pub scope_of_work: Option<String>,
    pub preferred_start_date: Option<NaiveDate>,
    pub preferred_end_date: Option<NaiveDate>,
    pub is_urgent: Option<bool>,
    pub budget_min: Option<Decimal>,
    pub budget_max: Option<Decimal>,
    pub attachments: Option<serde_json::Value>,
    pub images: Option<Vec<String>>,
    pub quote_deadline: Option<DateTime<Utc>>,
    pub contact_preference: Option<String>,
    pub site_visit_required: Option<bool>,
    pub status: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// RFQ query parameters.
#[derive(Debug, Default, Deserialize, ToSchema)]
pub struct RfqQuery {
    pub status: Option<String>,
    pub service_category: Option<String>,
    pub building_id: Option<Uuid>,
    pub is_urgent: Option<bool>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// Quote submitted by a provider for an RFQ.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ProviderQuote {
    pub id: Uuid,
    pub rfq_id: Uuid,
    pub provider_id: Uuid,

    // Quote details
    pub price: Decimal,
    pub currency: String,
    pub price_breakdown: Option<serde_json::Value>,

    // Timeline
    pub estimated_start_date: Option<NaiveDate>,
    pub estimated_end_date: Option<NaiveDate>,
    pub estimated_duration_days: Option<i32>,

    // Terms
    pub terms_and_conditions: Option<String>,
    pub warranty_period_days: Option<i32>,
    pub payment_terms: Option<String>,

    // Additional info
    pub notes: Option<String>,
    pub attachments: Option<serde_json::Value>,

    // Status
    pub status: String,
    pub valid_until: Option<DateTime<Utc>>,

    // Metadata
    pub metadata: Option<serde_json::Value>,

    // Timestamps
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub submitted_at: Option<DateTime<Utc>>,
}

/// Create provider quote request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProviderQuote {
    pub rfq_id: Uuid,
    pub price: Decimal,
    pub currency: Option<String>,
    pub price_breakdown: Option<serde_json::Value>,
    pub estimated_start_date: Option<NaiveDate>,
    pub estimated_end_date: Option<NaiveDate>,
    pub estimated_duration_days: Option<i32>,
    pub terms_and_conditions: Option<String>,
    pub warranty_period_days: Option<i32>,
    pub payment_terms: Option<String>,
    pub notes: Option<String>,
    pub attachments: Option<serde_json::Value>,
    pub valid_until: Option<DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>,
}

/// Update provider quote request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProviderQuote {
    pub price: Option<Decimal>,
    pub price_breakdown: Option<serde_json::Value>,
    pub estimated_start_date: Option<NaiveDate>,
    pub estimated_end_date: Option<NaiveDate>,
    pub estimated_duration_days: Option<i32>,
    pub terms_and_conditions: Option<String>,
    pub warranty_period_days: Option<i32>,
    pub payment_terms: Option<String>,
    pub notes: Option<String>,
    pub attachments: Option<serde_json::Value>,
    pub valid_until: Option<DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>,
}

/// RFQ invitation sent to a provider.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct RfqInvitation {
    pub id: Uuid,
    pub rfq_id: Uuid,
    pub provider_id: Uuid,
    pub invited_at: DateTime<Utc>,
    pub viewed_at: Option<DateTime<Utc>>,
    pub responded_at: Option<DateTime<Utc>>,
    pub declined: Option<bool>,
    pub decline_reason: Option<String>,
}

/// Quote comparison view for managers.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct QuoteComparisonView {
    pub rfq: RequestForQuote,
    pub quotes: Vec<QuoteWithProvider>,
}

/// Quote with provider details for comparison.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct QuoteWithProvider {
    pub quote: ProviderQuote,
    pub provider: ProviderSearchResult,
}

// ==================== Story 68.4: Provider Verification ====================

/// Provider verification document/credential.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ProviderVerification {
    pub id: Uuid,
    pub provider_id: Uuid,

    // Verification details
    pub verification_type: String,
    pub document_name: String,
    pub document_number: Option<String>,
    pub issuing_authority: Option<String>,
    pub issue_date: Option<NaiveDate>,
    pub expiry_date: Option<NaiveDate>,

    // Document upload
    pub document_url: Option<String>,
    pub document_hash: Option<String>,

    // Review info
    pub status: String,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub rejection_reason: Option<String>,
    pub notes: Option<String>,

    // Metadata
    pub metadata: Option<serde_json::Value>,

    // Timestamps
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Create verification request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProviderVerification {
    pub verification_type: String,
    pub document_name: String,
    pub document_number: Option<String>,
    pub issuing_authority: Option<String>,
    pub issue_date: Option<NaiveDate>,
    pub expiry_date: Option<NaiveDate>,
    pub document_url: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Review verification request (by admin).
#[derive(Debug, Deserialize, ToSchema)]
pub struct ReviewVerificationRequest {
    pub status: String,
    pub rejection_reason: Option<String>,
    pub notes: Option<String>,
    pub award_badges: Option<Vec<String>>,
}

/// Verification queue item for admin review.
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct VerificationQueueItem {
    pub id: Uuid,
    pub provider_id: Uuid,
    pub provider_name: String,
    pub verification_type: String,
    pub document_name: String,
    pub submitted_at: Option<DateTime<Utc>>,
    pub expiry_date: Option<NaiveDate>,
    pub days_pending: i32,
}

/// Verification query parameters.
#[derive(Debug, Default, Deserialize, ToSchema)]
pub struct VerificationQuery {
    pub provider_id: Option<Uuid>,
    pub verification_type: Option<String>,
    pub status: Option<String>,
    pub expiring_days: Option<i32>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// Provider badge entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ProviderBadge {
    pub id: Uuid,
    pub provider_id: Uuid,
    pub badge_type: String,
    pub awarded_at: DateTime<Utc>,
    pub awarded_by: Option<Uuid>,
    pub expires_at: Option<DateTime<Utc>>,
    pub verification_id: Option<Uuid>,
    pub notes: Option<String>,
}

/// Expiring verification alert.
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ExpiringVerification {
    pub id: Uuid,
    pub provider_id: Uuid,
    pub provider_name: String,
    pub verification_type: String,
    pub document_name: String,
    pub expiry_date: NaiveDate,
    pub days_until_expiry: i32,
}

// ==================== Story 68.5: Reviews & Ratings ====================

/// Provider review entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ProviderReview {
    pub id: Uuid,
    pub provider_id: Uuid,
    pub reviewer_id: Uuid,
    pub organization_id: Uuid,
    pub job_id: Option<Uuid>,
    pub rfq_id: Option<Uuid>,

    // Multi-dimension ratings (1-5)
    pub quality_rating: i32,
    pub timeliness_rating: i32,
    pub communication_rating: i32,
    pub value_rating: i32,
    pub overall_rating: i32,

    // Review content
    pub review_title: Option<String>,
    pub review_text: Option<String>,

    // Moderation
    pub status: String,
    pub moderated_by: Option<Uuid>,
    pub moderated_at: Option<DateTime<Utc>>,
    pub moderation_notes: Option<String>,

    // Provider response
    pub provider_response: Option<String>,
    pub provider_responded_at: Option<DateTime<Utc>>,

    // Helpful votes
    pub helpful_count: Option<i32>,

    // Metadata
    pub metadata: Option<serde_json::Value>,

    // Timestamps
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Create provider review request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProviderReview {
    pub job_id: Option<Uuid>,
    pub rfq_id: Option<Uuid>,
    pub quality_rating: i32,
    pub timeliness_rating: i32,
    pub communication_rating: i32,
    pub value_rating: i32,
    pub review_title: Option<String>,
    pub review_text: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Update provider review request (by reviewer).
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProviderReview {
    pub quality_rating: Option<i32>,
    pub timeliness_rating: Option<i32>,
    pub communication_rating: Option<i32>,
    pub value_rating: Option<i32>,
    pub review_title: Option<String>,
    pub review_text: Option<String>,
}

/// Provider response to review.
#[derive(Debug, Deserialize, ToSchema)]
pub struct ProviderReviewResponse {
    pub response_text: String,
}

/// Moderate review request (by admin).
#[derive(Debug, Deserialize, ToSchema)]
pub struct ModerateReviewRequest {
    pub status: String,
    pub moderation_notes: Option<String>,
}

/// Review query parameters.
#[derive(Debug, Default, Deserialize, ToSchema)]
pub struct ReviewQuery {
    pub provider_id: Option<Uuid>,
    pub reviewer_id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
    pub min_rating: Option<i32>,
    pub status: Option<String>,
    pub has_response: Option<bool>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// Review with provider response for display.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProviderReviewWithResponse {
    pub review: ProviderReview,
    pub reviewer_name: String,
    pub reviewer_organization: Option<String>,
}

/// Rating breakdown for a provider.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RatingBreakdown {
    pub average_overall: Decimal,
    pub average_quality: Decimal,
    pub average_timeliness: Decimal,
    pub average_communication: Decimal,
    pub average_value: Decimal,
    pub total_reviews: i64,
    pub rating_distribution: RatingDistribution,
}

/// Distribution of ratings (1-5 stars).
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RatingDistribution {
    pub five_star: i64,
    pub four_star: i64,
    pub three_star: i64,
    pub two_star: i64,
    pub one_star: i64,
}

/// Review statistics for provider dashboard.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReviewStatistics {
    pub total_reviews: i64,
    pub average_rating: Decimal,
    pub reviews_this_month: i64,
    pub rating_trend: Decimal,
    pub response_rate: Decimal,
    pub pending_responses: i64,
}

// ==================== Composite Types ====================

/// Complete provider profile view with all related data.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProviderProfileComplete {
    pub profile: ServiceProviderProfile,
    pub verifications: Vec<ProviderVerification>,
    pub badges: Vec<ProviderBadge>,
    pub rating_breakdown: RatingBreakdown,
    pub recent_reviews: Vec<ProviderReviewWithResponse>,
    pub active_rfqs: i64,
    pub pending_quotes: i64,
}

/// Provider dashboard summary.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProviderDashboard {
    pub profile: ServiceProviderProfile,
    pub rfq_summary: RfqSummary,
    pub review_stats: ReviewStatistics,
    pub expiring_verifications: Vec<ExpiringVerification>,
    pub pending_actions: Vec<PendingAction>,
}

/// RFQ summary for provider dashboard.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RfqSummary {
    pub pending_invitations: i64,
    pub active_quotes: i64,
    pub won_quotes: i64,
    pub total_quotes_submitted: i64,
    pub win_rate: Decimal,
}

/// Pending action item for provider.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PendingAction {
    pub action_type: String,
    pub title: String,
    pub description: String,
    pub due_date: Option<DateTime<Utc>>,
    pub reference_id: Uuid,
    pub priority: String,
}

/// Manager dashboard for marketplace.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ManagerMarketplaceDashboard {
    pub active_rfqs: Vec<RequestForQuote>,
    pub pending_quotes: i64,
    pub recent_completed_jobs: i64,
    pub favorite_providers: Vec<ProviderSearchResult>,
    pub recommended_providers: Vec<ProviderSearchResult>,
}

//! Service Provider Marketplace routes (Epic 68).
//!
//! Stories:
//! - 68.1: Service Provider Profiles
//! - 68.2: Search & Discovery
//! - 68.3: Request for Quote (RFQ)
//! - 68.4: Provider Verification
//! - 68.5: Reviews & Ratings

use api_core::extractors::AuthUser;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, patch, post},
    Json, Router,
};
use chrono::NaiveDate;
use common::errors::ErrorResponse;
use db::models::{
    CategoryCount, CreateProviderQuote, CreateProviderReview, CreateProviderVerification,
    CreateRequestForQuote, CreateServiceProviderProfile, ExpiringVerification,
    ManagerMarketplaceDashboard, MarketplaceSearchQuery, MarketplaceStatistics,
    ModerateReviewRequest, PendingAction, ProviderBadge, ProviderDashboard,
    ProviderProfileComplete, ProviderQuote, ProviderReview, ProviderReviewResponse,
    ProviderReviewWithResponse, ProviderSearchResult, ProviderVerification, QuoteComparisonView,
    QuoteWithProvider, RatingBreakdown, RatingDistribution, RequestForQuote,
    ReviewVerificationRequest, RfqInvitation, RfqQuery, RfqSummary, ReviewQuery,
    ReviewStatistics, ServiceProviderProfile, UpdateProviderQuote, UpdateProviderReview,
    UpdateRequestForQuote, UpdateServiceProviderProfile, VerificationQuery, VerificationQueueItem,
};
use rust_decimal::Decimal;
use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;

use crate::state::AppState;

/// Create marketplace router with all sub-routes.
pub fn router() -> Router<AppState> {
    Router::new()
        // Provider Profile routes (Story 68.1)
        .route("/providers", post(create_profile))
        .route("/providers", get(search_providers))
        .route("/providers/me", get(get_my_profile))
        .route("/providers/me", patch(update_my_profile))
        .route("/providers/me/dashboard", get(get_provider_dashboard))
        .route("/providers/statistics", get(get_marketplace_statistics))
        .route("/providers/{id}", get(get_provider))
        .route("/providers/{id}/complete", get(get_provider_complete))
        // RFQ routes (Story 68.3)
        .route("/rfqs", post(create_rfq))
        .route("/rfqs", get(list_rfqs))
        .route("/rfqs/{id}", get(get_rfq))
        .route("/rfqs/{id}", patch(update_rfq))
        .route("/rfqs/{id}", delete(delete_rfq))
        .route("/rfqs/{id}/quotes", get(list_rfq_quotes))
        .route("/rfqs/{id}/compare", get(compare_quotes))
        .route("/rfqs/{id}/award", post(award_quote))
        .route("/rfqs/{id}/cancel", post(cancel_rfq))
        // Quote routes (Story 68.3)
        .route("/quotes", post(submit_quote))
        .route("/quotes/my", get(list_my_quotes))
        .route("/quotes/{id}", get(get_quote))
        .route("/quotes/{id}", patch(update_quote))
        .route("/quotes/{id}", delete(withdraw_quote))
        // Invitation routes (Story 68.3)
        .route("/invitations", get(list_my_invitations))
        .route("/invitations/{id}/view", post(mark_invitation_viewed))
        .route("/invitations/{id}/decline", post(decline_invitation))
        // Verification routes (Story 68.4)
        .route("/verifications", post(submit_verification))
        .route("/verifications", get(list_verifications))
        .route("/verifications/queue", get(get_verification_queue))
        .route("/verifications/expiring", get(get_expiring_verifications))
        .route("/verifications/{id}", get(get_verification))
        .route("/verifications/{id}/review", post(review_verification))
        // Badge routes (Story 68.4)
        .route("/providers/{id}/badges", get(list_provider_badges))
        .route("/providers/{id}/badges", post(award_badge))
        .route("/badges/{id}", delete(revoke_badge))
        // Review routes (Story 68.5)
        .route("/providers/{id}/reviews", post(create_review))
        .route("/providers/{id}/reviews", get(list_provider_reviews))
        .route("/providers/{id}/ratings", get(get_rating_breakdown))
        .route("/reviews", get(list_reviews))
        .route("/reviews/{id}", get(get_review))
        .route("/reviews/{id}", patch(update_review))
        .route("/reviews/{id}", delete(delete_review))
        .route("/reviews/{id}/respond", post(respond_to_review))
        .route("/reviews/{id}/moderate", post(moderate_review))
        .route("/reviews/{id}/helpful", post(mark_review_helpful))
        // Manager dashboard (Story 68.2)
        .route("/dashboard", get(get_manager_dashboard))
}

// ==================== Request/Response Types ====================

/// Organization query parameter.
#[derive(Debug, Deserialize, IntoParams)]
pub struct OrgQuery {
    pub organization_id: Uuid,
}

/// Create profile request wrapper.
#[derive(Debug, Deserialize)]
pub struct CreateProfileRequest {
    #[serde(flatten)]
    pub data: CreateServiceProviderProfile,
}

/// Search providers query with all filters.
#[derive(Debug, Deserialize, IntoParams)]
pub struct SearchProvidersQuery {
    pub category: Option<String>,
    pub query: Option<String>,
    pub location: Option<String>,
    pub region: Option<String>,
    pub min_rating: Option<Decimal>,
    pub max_hourly_rate: Option<Decimal>,
    pub min_hourly_rate: Option<Decimal>,
    pub verified_only: Option<bool>,
    pub emergency_only: Option<bool>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

impl From<&SearchProvidersQuery> for MarketplaceSearchQuery {
    fn from(q: &SearchProvidersQuery) -> Self {
        MarketplaceSearchQuery {
            category: q.category.clone(),
            categories: None,
            query: q.query.clone(),
            location: q.location.clone(),
            postal_codes: None,
            region: q.region.clone(),
            min_rating: q.min_rating,
            max_hourly_rate: q.max_hourly_rate,
            min_hourly_rate: q.min_hourly_rate,
            verified_only: q.verified_only,
            badges: None,
            emergency_only: q.emergency_only,
            sort_by: q.sort_by.clone(),
            sort_order: q.sort_order.clone(),
            limit: q.limit,
            offset: q.offset,
        }
    }
}

/// Create RFQ request wrapper.
#[derive(Debug, Deserialize)]
pub struct CreateRfqRequest {
    pub organization_id: Uuid,
    #[serde(flatten)]
    pub data: CreateRequestForQuote,
}

/// List RFQs query.
#[derive(Debug, Deserialize, IntoParams)]
pub struct ListRfqsQuery {
    pub organization_id: Uuid,
    pub status: Option<String>,
    pub service_category: Option<String>,
    pub building_id: Option<Uuid>,
    pub is_urgent: Option<bool>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

impl From<&ListRfqsQuery> for RfqQuery {
    fn from(q: &ListRfqsQuery) -> Self {
        RfqQuery {
            status: q.status.clone(),
            service_category: q.service_category.clone(),
            building_id: q.building_id,
            is_urgent: q.is_urgent,
            limit: q.limit,
            offset: q.offset,
        }
    }
}

/// Award quote request.
#[derive(Debug, Deserialize)]
pub struct AwardQuoteRequest {
    pub quote_id: Uuid,
}

/// Submit quote request wrapper.
#[derive(Debug, Deserialize)]
pub struct SubmitQuoteRequest {
    #[serde(flatten)]
    pub data: CreateProviderQuote,
}

/// List verifications query.
#[derive(Debug, Deserialize, IntoParams)]
pub struct ListVerificationsQuery {
    pub provider_id: Option<Uuid>,
    pub verification_type: Option<String>,
    pub status: Option<String>,
    pub expiring_days: Option<i32>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

impl From<&ListVerificationsQuery> for VerificationQuery {
    fn from(q: &ListVerificationsQuery) -> Self {
        VerificationQuery {
            provider_id: q.provider_id,
            verification_type: q.verification_type.clone(),
            status: q.status.clone(),
            expiring_days: q.expiring_days,
            limit: q.limit,
            offset: q.offset,
        }
    }
}

/// Expiring verifications query.
#[derive(Debug, Deserialize, IntoParams)]
pub struct ExpiringQuery {
    pub days: Option<i32>,
}

/// Award badge request.
#[derive(Debug, Deserialize)]
pub struct AwardBadgeRequest {
    pub badge_type: String,
    pub verification_id: Option<Uuid>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub notes: Option<String>,
}

/// Create review request wrapper.
#[derive(Debug, Deserialize)]
pub struct CreateReviewRequest {
    pub organization_id: Uuid,
    #[serde(flatten)]
    pub data: CreateProviderReview,
}

/// List reviews query.
#[derive(Debug, Deserialize, IntoParams)]
pub struct ListReviewsQuery {
    pub provider_id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
    pub min_rating: Option<i32>,
    pub status: Option<String>,
    pub has_response: Option<bool>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

impl From<&ListReviewsQuery> for ReviewQuery {
    fn from(q: &ListReviewsQuery) -> Self {
        ReviewQuery {
            provider_id: q.provider_id,
            reviewer_id: None,
            organization_id: q.organization_id,
            min_rating: q.min_rating,
            status: q.status.clone(),
            has_response: q.has_response,
            sort_by: q.sort_by.clone(),
            sort_order: q.sort_order.clone(),
            limit: q.limit,
            offset: q.offset,
        }
    }
}

/// Pagination query.
#[derive(Debug, Deserialize, IntoParams)]
pub struct PaginationQuery {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

// ==================== Story 68.1: Provider Profile Endpoints ====================

/// Create a new service provider profile.
async fn create_profile(
    State(_state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<CreateProfileRequest>,
) -> Result<(StatusCode, Json<ServiceProviderProfile>), (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.create_profile(user.user_id, payload.data)
    let profile = ServiceProviderProfile {
        id: Uuid::new_v4(),
        user_id: user.user_id,
        company_name: payload.data.company_name,
        business_registration_number: payload.data.business_registration_number,
        tax_id: payload.data.tax_id,
        description: payload.data.description,
        logo_url: payload.data.logo_url,
        website: payload.data.website,
        contact_name: payload.data.contact_name,
        contact_email: payload.data.contact_email,
        contact_phone: payload.data.contact_phone,
        address: payload.data.address,
        city: payload.data.city,
        postal_code: payload.data.postal_code,
        country: payload.data.country,
        service_categories: payload.data.service_categories,
        service_description: payload.data.service_description,
        specializations: payload.data.specializations,
        coverage_postal_codes: payload.data.coverage_postal_codes,
        coverage_radius_km: payload.data.coverage_radius_km,
        coverage_regions: payload.data.coverage_regions,
        pricing_type: payload.data.pricing_type.unwrap_or_else(|| "hourly".to_string()),
        hourly_rate_min: payload.data.hourly_rate_min,
        hourly_rate_max: payload.data.hourly_rate_max,
        currency: payload.data.currency,
        certifications: payload.data.certifications,
        licenses: payload.data.licenses,
        availability_calendar: payload.data.availability_calendar,
        response_time_hours: payload.data.response_time_hours,
        emergency_available: payload.data.emergency_available,
        portfolio_images: payload.data.portfolio_images,
        portfolio_description: payload.data.portfolio_description,
        status: "draft".to_string(),
        is_featured: Some(false),
        average_rating: None,
        total_reviews: Some(0),
        total_jobs_completed: Some(0),
        is_verified: Some(false),
        verified_at: None,
        badges: Some(vec![]),
        metadata: payload.data.metadata,
        created_at: Some(chrono::Utc::now()),
        updated_at: Some(chrono::Utc::now()),
        last_active_at: Some(chrono::Utc::now()),
    };

    Ok((StatusCode::CREATED, Json(profile)))
}

/// Search for service providers in the marketplace.
async fn search_providers(
    State(_state): State<AppState>,
    _user: AuthUser,
    Query(query): Query<SearchProvidersQuery>,
) -> Result<Json<Vec<ProviderSearchResult>>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.search_providers(query.into())
    let _search_query: MarketplaceSearchQuery = (&query).into();

    // Return empty results for now - actual implementation would query database
    Ok(Json(vec![]))
}

/// Get the current user's provider profile.
async fn get_my_profile(
    State(_state): State<AppState>,
    user: AuthUser,
) -> Result<Json<ServiceProviderProfile>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.find_by_user_id(user.user_id)
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new(
            "NOT_FOUND",
            &format!("Provider profile not found for user {}", user.user_id),
        )),
    ))
}

/// Update the current user's provider profile.
async fn update_my_profile(
    State(_state): State<AppState>,
    _user: AuthUser,
    Json(_data): Json<UpdateServiceProviderProfile>,
) -> Result<Json<ServiceProviderProfile>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.update_profile(user.user_id, data)
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new("NOT_FOUND", "Provider profile not found")),
    ))
}

/// Get provider dashboard with summary statistics.
async fn get_provider_dashboard(
    State(_state): State<AppState>,
    _user: AuthUser,
) -> Result<Json<ProviderDashboard>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.get_provider_dashboard(user.user_id)
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new("NOT_FOUND", "Provider profile not found")),
    ))
}

/// Get marketplace statistics.
async fn get_marketplace_statistics(
    State(_state): State<AppState>,
    _user: AuthUser,
) -> Result<Json<MarketplaceStatistics>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.get_statistics()
    Ok(Json(MarketplaceStatistics {
        total_providers: 0,
        verified_providers: 0,
        providers_by_category: vec![],
        average_rating: Decimal::ZERO,
        total_reviews: 0,
        total_jobs_completed: 0,
    }))
}

/// Get a specific provider's profile.
async fn get_provider(
    State(_state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ServiceProviderProfile>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.find_by_id(id)
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new(
            "NOT_FOUND",
            &format!("Provider {} not found", id),
        )),
    ))
}

/// Get complete provider profile with verifications, badges, and reviews.
async fn get_provider_complete(
    State(_state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ProviderProfileComplete>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.get_complete_profile(id)
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new(
            "NOT_FOUND",
            &format!("Provider {} not found", id),
        )),
    ))
}

// ==================== Story 68.3: RFQ Endpoints ====================

/// Create a new request for quote.
async fn create_rfq(
    State(_state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<CreateRfqRequest>,
) -> Result<(StatusCode, Json<RequestForQuote>), (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.create_rfq(payload.organization_id, user.user_id, payload.data)
    let rfq = RequestForQuote {
        id: Uuid::new_v4(),
        organization_id: payload.organization_id,
        building_id: payload.data.building_id,
        created_by: user.user_id,
        title: payload.data.title,
        description: payload.data.description,
        service_category: payload.data.service_category,
        scope_of_work: payload.data.scope_of_work,
        preferred_start_date: payload.data.preferred_start_date,
        preferred_end_date: payload.data.preferred_end_date,
        is_urgent: payload.data.is_urgent,
        budget_min: payload.data.budget_min,
        budget_max: payload.data.budget_max,
        currency: payload.data.currency,
        attachments: payload.data.attachments,
        images: payload.data.images,
        status: "draft".to_string(),
        quote_deadline: payload.data.quote_deadline,
        awarded_to: None,
        awarded_quote_id: None,
        awarded_at: None,
        contact_preference: payload.data.contact_preference,
        site_visit_required: payload.data.site_visit_required,
        metadata: payload.data.metadata,
        created_at: Some(chrono::Utc::now()),
        updated_at: Some(chrono::Utc::now()),
        expires_at: None,
    };

    Ok((StatusCode::CREATED, Json(rfq)))
}

/// List RFQs for an organization.
async fn list_rfqs(
    State(_state): State<AppState>,
    _user: AuthUser,
    Query(_query): Query<ListRfqsQuery>,
) -> Result<Json<Vec<RequestForQuote>>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.list_rfqs(query.organization_id, query.into())
    Ok(Json(vec![]))
}

/// Get a specific RFQ.
async fn get_rfq(
    State(_state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<RequestForQuote>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.find_rfq_by_id(id)
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new(
            "NOT_FOUND",
            &format!("RFQ {} not found", id),
        )),
    ))
}

/// Update an RFQ.
async fn update_rfq(
    State(_state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
    Json(_data): Json<UpdateRequestForQuote>,
) -> Result<Json<RequestForQuote>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.update_rfq(id, data)
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new(
            "NOT_FOUND",
            &format!("RFQ {} not found", id),
        )),
    ))
}

/// Delete an RFQ.
async fn delete_rfq(
    State(_state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.delete_rfq(id)
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new(
            "NOT_FOUND",
            &format!("RFQ {} not found", id),
        )),
    ))
}

/// List quotes for an RFQ.
async fn list_rfq_quotes(
    State(_state): State<AppState>,
    _user: AuthUser,
    Path(_id): Path<Uuid>,
) -> Result<Json<Vec<ProviderQuote>>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.list_rfq_quotes(id)
    Ok(Json(vec![]))
}

/// Compare quotes for an RFQ side-by-side.
async fn compare_quotes(
    State(_state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<QuoteComparisonView>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.get_quote_comparison(id)
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new(
            "NOT_FOUND",
            &format!("RFQ {} not found", id),
        )),
    ))
}

/// Award an RFQ to a provider.
async fn award_quote(
    State(_state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
    Json(_data): Json<AwardQuoteRequest>,
) -> Result<Json<RequestForQuote>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.award_rfq(id, data.quote_id, user.user_id)
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new(
            "NOT_FOUND",
            &format!("RFQ {} not found", id),
        )),
    ))
}

/// Cancel an RFQ.
async fn cancel_rfq(
    State(_state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<RequestForQuote>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.cancel_rfq(id)
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new(
            "NOT_FOUND",
            &format!("RFQ {} not found", id),
        )),
    ))
}

/// Submit a quote for an RFQ.
async fn submit_quote(
    State(_state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<SubmitQuoteRequest>,
) -> Result<(StatusCode, Json<ProviderQuote>), (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.submit_quote(provider_id, payload.data)
    let quote = ProviderQuote {
        id: Uuid::new_v4(),
        rfq_id: payload.data.rfq_id,
        provider_id: user.user_id, // In reality, would get provider_id from user's profile
        price: payload.data.price,
        currency: payload.data.currency.unwrap_or_else(|| "EUR".to_string()),
        price_breakdown: payload.data.price_breakdown,
        estimated_start_date: payload.data.estimated_start_date,
        estimated_end_date: payload.data.estimated_end_date,
        estimated_duration_days: payload.data.estimated_duration_days,
        terms_and_conditions: payload.data.terms_and_conditions,
        warranty_period_days: payload.data.warranty_period_days,
        payment_terms: payload.data.payment_terms,
        notes: payload.data.notes,
        attachments: payload.data.attachments,
        status: "submitted".to_string(),
        valid_until: payload.data.valid_until,
        metadata: payload.data.metadata,
        created_at: Some(chrono::Utc::now()),
        updated_at: Some(chrono::Utc::now()),
        submitted_at: Some(chrono::Utc::now()),
    };

    Ok((StatusCode::CREATED, Json(quote)))
}

/// List quotes submitted by the current provider.
async fn list_my_quotes(
    State(_state): State<AppState>,
    _user: AuthUser,
    Query(_query): Query<PaginationQuery>,
) -> Result<Json<Vec<ProviderQuote>>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.list_provider_quotes(provider_id, query)
    Ok(Json(vec![]))
}

/// Get a specific quote.
async fn get_quote(
    State(_state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ProviderQuote>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.find_quote_by_id(id)
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new(
            "NOT_FOUND",
            &format!("Quote {} not found", id),
        )),
    ))
}

/// Update a quote (before submission or while still editable).
async fn update_quote(
    State(_state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
    Json(_data): Json<UpdateProviderQuote>,
) -> Result<Json<ProviderQuote>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.update_quote(id, data)
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new(
            "NOT_FOUND",
            &format!("Quote {} not found", id),
        )),
    ))
}

/// Withdraw a submitted quote.
async fn withdraw_quote(
    State(_state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.withdraw_quote(id)
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new(
            "NOT_FOUND",
            &format!("Quote {} not found", id),
        )),
    ))
}

/// List RFQ invitations for the current provider.
async fn list_my_invitations(
    State(_state): State<AppState>,
    _user: AuthUser,
    Query(_query): Query<PaginationQuery>,
) -> Result<Json<Vec<RfqInvitation>>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.list_provider_invitations(provider_id, query)
    Ok(Json(vec![]))
}

/// Mark an invitation as viewed.
async fn mark_invitation_viewed(
    State(_state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<RfqInvitation>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.mark_invitation_viewed(id)
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new(
            "NOT_FOUND",
            &format!("Invitation {} not found", id),
        )),
    ))
}

/// Decline an RFQ invitation.
#[derive(Debug, Deserialize)]
pub struct DeclineInvitationRequest {
    pub reason: Option<String>,
}

async fn decline_invitation(
    State(_state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
    Json(_data): Json<DeclineInvitationRequest>,
) -> Result<Json<RfqInvitation>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.decline_invitation(id, data.reason)
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new(
            "NOT_FOUND",
            &format!("Invitation {} not found", id),
        )),
    ))
}

// ==================== Story 68.4: Verification Endpoints ====================

/// Submit a verification document.
async fn submit_verification(
    State(_state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<CreateProviderVerification>,
) -> Result<(StatusCode, Json<ProviderVerification>), (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.submit_verification(provider_id, payload)
    let verification = ProviderVerification {
        id: Uuid::new_v4(),
        provider_id: user.user_id, // In reality, would get provider_id from user's profile
        verification_type: payload.verification_type,
        document_name: payload.document_name,
        document_number: payload.document_number,
        issuing_authority: payload.issuing_authority,
        issue_date: payload.issue_date,
        expiry_date: payload.expiry_date,
        document_url: payload.document_url,
        document_hash: None,
        status: "pending".to_string(),
        reviewed_by: None,
        reviewed_at: None,
        rejection_reason: None,
        notes: None,
        metadata: payload.metadata,
        created_at: Some(chrono::Utc::now()),
        updated_at: Some(chrono::Utc::now()),
    };

    Ok((StatusCode::CREATED, Json(verification)))
}

/// List verifications.
async fn list_verifications(
    State(_state): State<AppState>,
    _user: AuthUser,
    Query(_query): Query<ListVerificationsQuery>,
) -> Result<Json<Vec<ProviderVerification>>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.list_verifications(query.into())
    Ok(Json(vec![]))
}

/// Get verification queue for admin review.
async fn get_verification_queue(
    State(_state): State<AppState>,
    _user: AuthUser,
    Query(_query): Query<PaginationQuery>,
) -> Result<Json<Vec<VerificationQueueItem>>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.get_verification_queue(query)
    Ok(Json(vec![]))
}

/// Get expiring verifications.
async fn get_expiring_verifications(
    State(_state): State<AppState>,
    _user: AuthUser,
    Query(query): Query<ExpiringQuery>,
) -> Result<Json<Vec<ExpiringVerification>>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.get_expiring_verifications(query.days.unwrap_or(30))
    let _days = query.days.unwrap_or(30);
    Ok(Json(vec![]))
}

/// Get a specific verification.
async fn get_verification(
    State(_state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ProviderVerification>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.find_verification_by_id(id)
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new(
            "NOT_FOUND",
            &format!("Verification {} not found", id),
        )),
    ))
}

/// Review a verification (admin only).
async fn review_verification(
    State(_state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    Json(_data): Json<ReviewVerificationRequest>,
) -> Result<Json<ProviderVerification>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.review_verification(id, user.user_id, data)
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new(
            "NOT_FOUND",
            &format!("Verification {} not found", id),
        )),
    ))
}

/// List badges for a provider.
async fn list_provider_badges(
    State(_state): State<AppState>,
    _user: AuthUser,
    Path(_id): Path<Uuid>,
) -> Result<Json<Vec<ProviderBadge>>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.list_provider_badges(id)
    Ok(Json(vec![]))
}

/// Award a badge to a provider (admin only).
async fn award_badge(
    State(_state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    Json(data): Json<AwardBadgeRequest>,
) -> Result<(StatusCode, Json<ProviderBadge>), (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.award_badge(id, user.user_id, data)
    let badge = ProviderBadge {
        id: Uuid::new_v4(),
        provider_id: id,
        badge_type: data.badge_type,
        awarded_at: chrono::Utc::now(),
        awarded_by: Some(user.user_id),
        expires_at: data.expires_at,
        verification_id: data.verification_id,
        notes: data.notes,
    };

    Ok((StatusCode::CREATED, Json(badge)))
}

/// Revoke a badge.
async fn revoke_badge(
    State(_state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.revoke_badge(id)
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new(
            "NOT_FOUND",
            &format!("Badge {} not found", id),
        )),
    ))
}

// ==================== Story 68.5: Review Endpoints ====================

/// Create a review for a provider.
async fn create_review(
    State(_state): State<AppState>,
    user: AuthUser,
    Path(provider_id): Path<Uuid>,
    Json(payload): Json<CreateReviewRequest>,
) -> Result<(StatusCode, Json<ProviderReview>), (StatusCode, Json<ErrorResponse>)> {
    // Calculate overall rating as average of dimension ratings
    let overall = (payload.data.quality_rating
        + payload.data.timeliness_rating
        + payload.data.communication_rating
        + payload.data.value_rating) as f64
        / 4.0;

    // In production, this would call state.marketplace_repo.create_review(provider_id, user.user_id, payload)
    let review = ProviderReview {
        id: Uuid::new_v4(),
        provider_id,
        reviewer_id: user.user_id,
        organization_id: payload.organization_id,
        job_id: payload.data.job_id,
        rfq_id: payload.data.rfq_id,
        quality_rating: payload.data.quality_rating,
        timeliness_rating: payload.data.timeliness_rating,
        communication_rating: payload.data.communication_rating,
        value_rating: payload.data.value_rating,
        overall_rating: overall.round() as i32,
        review_title: payload.data.review_title,
        review_text: payload.data.review_text,
        status: "pending".to_string(),
        moderated_by: None,
        moderated_at: None,
        moderation_notes: None,
        provider_response: None,
        provider_responded_at: None,
        helpful_count: Some(0),
        metadata: payload.data.metadata,
        created_at: Some(chrono::Utc::now()),
        updated_at: Some(chrono::Utc::now()),
    };

    Ok((StatusCode::CREATED, Json(review)))
}

/// List reviews for a provider.
async fn list_provider_reviews(
    State(_state): State<AppState>,
    _user: AuthUser,
    Path(_id): Path<Uuid>,
    Query(_query): Query<PaginationQuery>,
) -> Result<Json<Vec<ProviderReviewWithResponse>>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.list_provider_reviews(id, query)
    Ok(Json(vec![]))
}

/// Get rating breakdown for a provider.
async fn get_rating_breakdown(
    State(_state): State<AppState>,
    _user: AuthUser,
    Path(_id): Path<Uuid>,
) -> Result<Json<RatingBreakdown>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.get_rating_breakdown(id)
    Ok(Json(RatingBreakdown {
        average_overall: Decimal::ZERO,
        average_quality: Decimal::ZERO,
        average_timeliness: Decimal::ZERO,
        average_communication: Decimal::ZERO,
        average_value: Decimal::ZERO,
        total_reviews: 0,
        rating_distribution: RatingDistribution {
            five_star: 0,
            four_star: 0,
            three_star: 0,
            two_star: 0,
            one_star: 0,
        },
    }))
}

/// List all reviews (with filters).
async fn list_reviews(
    State(_state): State<AppState>,
    _user: AuthUser,
    Query(_query): Query<ListReviewsQuery>,
) -> Result<Json<Vec<ProviderReview>>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.list_reviews(query.into())
    Ok(Json(vec![]))
}

/// Get a specific review.
async fn get_review(
    State(_state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ProviderReview>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.find_review_by_id(id)
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new(
            "NOT_FOUND",
            &format!("Review {} not found", id),
        )),
    ))
}

/// Update a review (by the reviewer).
async fn update_review(
    State(_state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
    Json(_data): Json<UpdateProviderReview>,
) -> Result<Json<ProviderReview>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.update_review(id, user.user_id, data)
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new(
            "NOT_FOUND",
            &format!("Review {} not found", id),
        )),
    ))
}

/// Delete a review.
async fn delete_review(
    State(_state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.delete_review(id, user.user_id)
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new(
            "NOT_FOUND",
            &format!("Review {} not found", id),
        )),
    ))
}

/// Respond to a review (by the provider).
async fn respond_to_review(
    State(_state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
    Json(_data): Json<ProviderReviewResponse>,
) -> Result<Json<ProviderReview>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.respond_to_review(id, provider_id, data)
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new(
            "NOT_FOUND",
            &format!("Review {} not found", id),
        )),
    ))
}

/// Moderate a review (admin only).
async fn moderate_review(
    State(_state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    Json(_data): Json<ModerateReviewRequest>,
) -> Result<Json<ProviderReview>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.moderate_review(id, user.user_id, data)
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new(
            "NOT_FOUND",
            &format!("Review {} not found", id),
        )),
    ))
}

/// Mark a review as helpful.
async fn mark_review_helpful(
    State(_state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ProviderReview>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.mark_review_helpful(id, user.user_id)
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new(
            "NOT_FOUND",
            &format!("Review {} not found", id),
        )),
    ))
}

// ==================== Story 68.2: Manager Dashboard ====================

/// Get manager marketplace dashboard.
async fn get_manager_dashboard(
    State(_state): State<AppState>,
    _user: AuthUser,
    Query(_query): Query<OrgQuery>,
) -> Result<Json<ManagerMarketplaceDashboard>, (StatusCode, Json<ErrorResponse>)> {
    // In production, this would call state.marketplace_repo.get_manager_dashboard(query.organization_id)
    Ok(Json(ManagerMarketplaceDashboard {
        active_rfqs: vec![],
        pending_quotes: 0,
        recent_completed_jobs: 0,
        favorite_providers: vec![],
        recommended_providers: vec![],
    }))
}

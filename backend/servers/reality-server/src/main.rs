//! Reality Server - Public Real Estate Portal
//!
//! Public-facing API for the Reality Portal.
//! Serves property listings, search, favorites, and inquiries.
//! Supports SSO with Property Management system.
//!
//! Package: ppt::reality_server

// Allow dead code for stub implementations during development
#![allow(dead_code)]

use axum::{http, routing::get, Router};
use db::models::{
    CreateAgencyInvitation, CreateFeedSubscription, CreateListingInquiry, CreatePortalImportJob,
    CreateRealityAgency, CreateRealtorProfile, CreateSavedSearch, FavoritesResponse,
    InquiryMessage, ListingInquiry, PortalImportJob, PortalImportJobWithStats,
    PublicListingSearchResponse, RealityAgency, RealityAgencyInvitation, RealityAgencyMember,
    RealityFeedSubscription, RealtorProfile, SavedSearch, SavedSearchesResponse,
    SendInquiryMessage, UpdateAgencyBranding, UpdateFeedSubscription, UpdatePortalImportJob,
    UpdateRealityAgency, UpdateRealtorProfile, UpdateSavedSearch,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod handlers;
mod routes;
pub mod state;

use state::AppState;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Reality Portal API",
        version = "1.0.0",
        description = "Public API for Reality Portal - Real Estate Listings",
        contact(name = "PPT Team", email = "reality@ppt.example.com"),
        license(name = "MIT")
    ),
    servers(
        (url = "http://localhost:8081", description = "Local development"),
        (url = "https://api.reality-portal.sk", description = "Slovakia"),
        (url = "https://api.reality-portal.cz", description = "Czech Republic"),
        (url = "https://api.reality-portal.eu", description = "EU-wide")
    ),
    paths(
        routes::health::health,
        routes::listings::search,
        routes::listings::get_listing,
        routes::listings::get_suggestions,
        routes::favorites::list_favorites,
        routes::favorites::add_favorite,
        routes::favorites::remove_favorite,
        routes::favorites::check_favorite,
        routes::saved_searches::list_saved_searches,
        routes::saved_searches::create_saved_search,
        routes::saved_searches::get_saved_search,
        routes::saved_searches::update_saved_search,
        routes::saved_searches::delete_saved_search,
        routes::saved_searches::run_saved_search,
        routes::sso::sso_login,
        routes::sso::sso_callback,
        routes::sso::sso_logout,
        routes::sso::create_mobile_sso_token,
        routes::sso::validate_mobile_sso_token,
        routes::sso::get_session,
        routes::sso::refresh_session,
        // Epic 32: Agencies
        routes::agencies::create_agency,
        routes::agencies::get_agency,
        routes::agencies::get_agency_by_slug,
        routes::agencies::update_agency,
        routes::agencies::update_branding,
        routes::agencies::list_members,
        routes::agencies::create_invitation,
        routes::agencies::accept_invitation,
        // Epic 33: Realtors
        routes::realtors::get_my_profile,
        routes::realtors::get_profile,
        routes::realtors::create_profile,
        routes::realtors::update_profile,
        routes::realtors::list_inquiries,
        routes::realtors::mark_inquiry_read,
        routes::realtors::respond_to_inquiry,
        // Epic 34: Imports
        routes::imports::list_import_jobs,
        routes::imports::create_import_job,
        routes::imports::get_import_job,
        routes::imports::update_import_job,
        routes::imports::start_import_job,
        routes::imports::cancel_import_job,
        routes::imports::list_feeds,
        routes::imports::create_feed,
        routes::imports::get_feed,
        routes::imports::update_feed,
        routes::imports::sync_feed,
    ),
    components(schemas(
        routes::health::HealthResponse,
        routes::listings::ListingSearchRequest,
        routes::listings::ListingSearchResponse,
        routes::listings::ListingSummary,
        routes::listings::ListingDetail,
        routes::listings::SuggestionsResponse,
        routes::favorites::CheckFavoriteResponse,
        routes::saved_searches::RunSavedSearchResponse,
        db::models::AddFavorite,
        db::models::FavoriteWithListing,
        FavoritesResponse,
        PublicListingSearchResponse,
        CreateSavedSearch,
        UpdateSavedSearch,
        SavedSearch,
        SavedSearchesResponse,
        routes::sso::SsoError,
        routes::sso::SsoUserInfo,
        routes::sso::SessionInfo,
        routes::sso::CreateMobileSsoTokenRequest,
        routes::sso::MobileSsoTokenResponse,
        routes::sso::ValidateMobileSsoTokenRequest,
        routes::sso::SessionResponse,
        // Epic 32: Agencies
        routes::agencies::AgencyResponse,
        routes::agencies::MembersResponse,
        routes::agencies::AcceptInvitationRequest,
        CreateRealityAgency,
        UpdateRealityAgency,
        UpdateAgencyBranding,
        CreateAgencyInvitation,
        RealityAgency,
        RealityAgencyMember,
        RealityAgencyInvitation,
        // Epic 33: Realtors
        routes::realtors::ProfileResponse,
        routes::realtors::InquiriesResponse,
        routes::realtors::InquiriesQuery,
        CreateRealtorProfile,
        UpdateRealtorProfile,
        RealtorProfile,
        CreateListingInquiry,
        SendInquiryMessage,
        ListingInquiry,
        InquiryMessage,
        // Epic 34: Imports
        routes::imports::ImportJobsResponse,
        routes::imports::ImportJobResponse,
        routes::imports::ImportJobsQuery,
        routes::imports::FeedsResponse,
        routes::imports::FeedResponse,
        CreatePortalImportJob,
        UpdatePortalImportJob,
        PortalImportJob,
        PortalImportJobWithStats,
        CreateFeedSubscription,
        UpdateFeedSubscription,
        RealityFeedSubscription,
    )),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Listings", description = "Public listing search and detail"),
        (name = "SSO", description = "Single Sign-On with Property Management"),
        (name = "Users", description = "Portal user accounts (separate from PM)"),
        (name = "Favorites", description = "Save and manage favorite listings"),
        (name = "SavedSearches", description = "Saved search criteria and alerts"),
        (name = "Inquiries", description = "Contact and viewing requests"),
        (name = "Agencies", description = "Real estate agency management (Epic 32)"),
        (name = "Realtors", description = "Realtor profiles and tools (Epic 33)"),
        (name = "Imports", description = "Property import and feed management (Epic 34)")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file if present
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "reality_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Get database URL
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://localhost:5432/ppt".to_string());

    // Create database pool
    let db = db::create_pool(&database_url).await?;
    tracing::info!("Connected to database");

    // Create application state
    let state = AppState::new(db);

    // Build router with state
    let app = Router::new()
        // Health check (stateless)
        .route("/health", get(routes::health::health))
        // Public listing routes
        .nest("/api/v1/listings", routes::listings::router())
        // Portal user routes
        .nest("/api/v1/users", routes::users::router())
        // Favorites routes
        .nest("/api/v1/favorites", routes::favorites::router())
        // Saved searches routes
        .nest("/api/v1/saved-searches", routes::saved_searches::router())
        // Inquiries routes
        .nest("/api/v1/inquiries", routes::inquiries::router())
        // SSO routes (Epic 10A-SSO)
        .nest("/api/v1/sso", routes::sso::router())
        // Agency routes (Epic 32)
        .nest("/api/v1/agencies", routes::agencies::router())
        // Realtor routes (Epic 33)
        .nest("/api/v1/realtors", routes::realtors::router())
        // Import routes (Epic 34)
        .nest("/api/v1/imports", routes::imports::router())
        // Swagger UI
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // Add state
        .with_state(state)
        // Middleware
        .layer(TraceLayer::new_for_http())
        // CORS configuration - allow specific origins in production
        // TODO: Move allowed origins to configuration
        .layer(
            CorsLayer::new()
                // Allow requests from our frontends (development and production)
                .allow_origin([
                    "http://localhost:3000".parse().unwrap(), // ppt-web dev
                    "http://localhost:3001".parse().unwrap(), // reality-web dev
                    "http://localhost:8081".parse().unwrap(), // mobile dev
                    "https://ppt.three-two-bit.com".parse().unwrap(), // production
                    "https://reality.three-two-bit.com".parse().unwrap(), // reality production
                    "https://reality-portal.sk".parse().unwrap(), // Slovakia portal
                    "https://reality-portal.cz".parse().unwrap(), // Czech portal
                    "https://reality-portal.eu".parse().unwrap(), // EU portal
                ])
                // Allow common HTTP methods
                .allow_methods([
                    http::Method::GET,
                    http::Method::POST,
                    http::Method::PUT,
                    http::Method::PATCH,
                    http::Method::DELETE,
                    http::Method::OPTIONS,
                ])
                // Allow common headers
                .allow_headers(Any)
                // Allow credentials (cookies, authorization headers)
                .allow_credentials(true)
                // Cache preflight response for 1 hour
                .max_age(std::time::Duration::from_secs(3600)),
        );

    // Run server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    tracing::info!("Reality server (Public Portal) listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

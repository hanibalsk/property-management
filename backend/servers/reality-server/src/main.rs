//! Reality Server - Public Real Estate Portal
//!
//! Public-facing API for the Reality Portal.
//! Serves property listings, search, favorites, and inquiries.
//! Supports SSO with Property Management system.
//!
//! Package: ppt::reality_server

// Allow dead code for stub implementations during development
#![allow(dead_code)]

use axum::{routing::get, Router};
use db::models::{
    CreateSavedSearch, FavoritesResponse, PublicListingSearchResponse, SavedSearch,
    SavedSearchesResponse, UpdateSavedSearch,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod handlers;
mod routes;
mod state;

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
    )),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Listings", description = "Public listing search and detail"),
        (name = "Users", description = "Portal user accounts (separate from PM)"),
        (name = "Favorites", description = "Save and manage favorite listings"),
        (name = "SavedSearches", description = "Saved search criteria and alerts"),
        (name = "Inquiries", description = "Contact and viewing requests")
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
        // Swagger UI
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // Add state
        .with_state(state)
        // Middleware
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    // Run server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    tracing::info!("Reality server (Public Portal) listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

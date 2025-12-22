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
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
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
        routes::sso::sso_login,
        routes::sso::sso_callback,
        routes::sso::sso_logout,
        routes::sso::create_mobile_sso_token,
        routes::sso::validate_mobile_sso_token,
        routes::sso::get_session,
        routes::sso::refresh_session,
    ),
    components(schemas(
        routes::health::HealthResponse,
        routes::listings::ListingSearchRequest,
        routes::listings::ListingSearchResponse,
        routes::listings::ListingDetail,
        routes::sso::SsoError,
        routes::sso::SsoUserInfo,
        routes::sso::SessionInfo,
        routes::sso::CreateMobileSsoTokenRequest,
        routes::sso::MobileSsoTokenResponse,
        routes::sso::ValidateMobileSsoTokenRequest,
        routes::sso::SessionResponse,
    )),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Listings", description = "Public listing search and detail"),
        (name = "SSO", description = "Single Sign-On with Property Management"),
        (name = "Users", description = "Portal user accounts (separate from PM)"),
        (name = "Favorites", description = "Save and manage favorite listings"),
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

    // Initialize application state
    let app_state = AppState::new();

    // Build router
    let app = Router::new()
        // Health check
        .route("/health", get(routes::health::health))
        // Public listing routes
        .nest("/api/v1/listings", routes::listings::router())
        // Portal user routes
        .nest("/api/v1/users", routes::users::router())
        // Favorites routes
        .nest("/api/v1/favorites", routes::favorites::router())
        // Inquiries routes
        .nest("/api/v1/inquiries", routes::inquiries::router())
        // SSO routes (Epic 10A-SSO)
        .nest("/api/v1/sso", routes::sso::router())
        // Swagger UI
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // Application state
        .with_state(app_state)
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

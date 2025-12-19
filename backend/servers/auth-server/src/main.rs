//! Authentication Server
//!
//! Handles user authentication, registration, and session management.

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod handlers;
mod routes;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Auth API",
        version = "1.0.0",
        description = "Authentication and authorization API"
    ),
    paths(
        routes::health,
        routes::login,
        routes::register,
    ),
    components(schemas(
        routes::LoginRequest,
        routes::LoginResponse,
        routes::RegisterRequest,
    )),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Authentication", description = "User authentication")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file if present
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "auth_server=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Build router
    let app = Router::new()
        // Health check
        .route("/health", get(routes::health))
        // Auth routes
        .route("/api/v1/auth/login", post(routes::login))
        .route("/api/v1/auth/register", post(routes::register))
        // Swagger UI
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // Middleware
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    // Run server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Auth server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

//! Integration Server
//!
//! External integrations: Airbnb, Booking, real estate portals.

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "integration_server=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        // TODO: Add integration routes (rentals, listings, webhooks)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8082));
    tracing::info!("Integration server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

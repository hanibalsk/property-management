//! Database layer: models and repositories.

pub mod models;
pub mod repositories;
pub mod tenant_context;

use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

pub type DbPool = sqlx::PgPool;

// Re-export tenant context utilities
pub use tenant_context::{
    clear_request_context, set_request_context, set_tenant_context, user_has_permission,
};

/// Create database connection pool.
pub async fn create_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await
}

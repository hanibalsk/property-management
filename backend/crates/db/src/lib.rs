//! Database layer: models and repositories.

pub mod models;
pub mod repositories;

use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

pub type DbPool = sqlx::PgPool;

/// Create database connection pool.
pub async fn create_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await
}

//! Database layer: models and repositories.
//!
//! # Row-Level Security (RLS)
//!
//! This crate provides RLS-aware database access patterns to ensure tenant isolation.
//!
//! ## Key Types
//!
//! - [`RlsPool`]: Type-safe pool wrapper that enforces RLS on all connections
//! - [`RlsGuard`]: Connection guard that automatically clears context on drop
//! - [`PublicConnection`]: For unauthenticated routes (clears stale context on acquire)
//!
//! ## Security Guarantees
//!
//! 1. **Drop Safety**: If `release()` is not called, a background task clears context
//! 2. **Public Connection Safety**: `acquire_public()` clears any stale RLS context
//! 3. **CI Enforcement**: RLS violations are caught by `check-rls-enforcement.sh`
//! 4. **PR Smoke Tests**: Basic tenant isolation is verified on every PR
//!
//! ## Migration Path (Type-Level Enforcement)
//!
//! The long-term goal is type-level enforcement where repositories require
//! `RlsConnection` instead of raw `DbPool`. This eliminates the footgun of
//! accidentally using the pool without RLS context.
//!
//! Current state: Handlers use `RlsConnection` extractor, repositories still
//! accept raw pool but have RLS-aware `*_rls()` method variants.
//!
//! Future state: Replace `DbPool` with `RlsPool` in `AppState` and update
//! repository signatures to require `&mut RlsGuard` or similar.

pub mod models;
pub mod repositories;
pub mod rls_pool;
pub mod tenant_context;

use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

pub type DbPool = sqlx::PgPool;

/// Re-export sqlx error for use in route handlers.
pub use sqlx::Error as SqlxError;

// Re-export tenant context utilities
pub use tenant_context::{
    clear_request_context, set_request_context, set_tenant_context, spawn_clear_context,
    user_has_permission,
};

// Re-export RLS pool types for type-level enforcement
pub use rls_pool::{PublicConnection, RlsGuard, RlsPool};

// Re-export repositories for direct import
pub use repositories::FormRepository;

/// Create database connection pool.
pub async fn create_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await
}

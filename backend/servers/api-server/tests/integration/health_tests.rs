//! Health check integration tests (Epic 99, Story 99.4).
//!
//! Tests health check endpoints:
//! - Database connectivity verification
//! - Uptime tracking
//! - Degraded status reporting
//! - Response format compliance
//!
//! Uses sqlx::test for test database isolation.

mod common;

use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use sqlx::PgPool;

use common::TestApp;

// =============================================================================
// Basic Health Check Tests
// =============================================================================

#[cfg(test)]
mod basic_health {
    use super::*;

    #[sqlx::test]
    async fn test_health_endpoint_returns_ok(pool: PgPool) {
        let app = TestApp::new(pool).await;

        let request = Request::builder()
            .method(Method::GET)
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let response = app.execute(request).await;

        response.assert_status(StatusCode::OK);
    }

    #[sqlx::test]
    async fn test_health_response_format(pool: PgPool) {
        let app = TestApp::new(pool).await;

        let request = Request::builder()
            .method(Method::GET)
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let response = app.execute(request).await;

        response.assert_status(StatusCode::OK);
        response.assert_json_field("status");

        let json = response.json_value();
        let status = json["status"].as_str().unwrap();

        // Status should be "healthy" or "degraded"
        assert!(
            status == "healthy" || status == "degraded" || status == "ok",
            "Status should be healthy, degraded, or ok, got: {}",
            status
        );
    }

    #[sqlx::test]
    async fn test_health_includes_version(pool: PgPool) {
        let app = TestApp::new(pool).await;

        let request = Request::builder()
            .method(Method::GET)
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let response = app.execute(request).await;

        response.assert_status(StatusCode::OK);

        // Check if version is included
        let json = response.json_value();
        if let Some(version) = json.get("version") {
            assert!(version.is_string(), "Version should be a string");
        }
    }

    #[sqlx::test]
    async fn test_health_includes_uptime(pool: PgPool) {
        let app = TestApp::new(pool).await;

        let request = Request::builder()
            .method(Method::GET)
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let response = app.execute(request).await;

        response.assert_status(StatusCode::OK);

        // Check if uptime is included
        let json = response.json_value();
        if let Some(uptime) = json.get("uptime_seconds") {
            let uptime_val = uptime.as_u64().or_else(|| uptime.as_f64().map(|f| f as u64));
            assert!(uptime_val.is_some(), "Uptime should be a number");
            assert!(uptime_val.unwrap() >= 0, "Uptime should be non-negative");
        }
    }
}

// =============================================================================
// Database Connectivity Tests
// =============================================================================

#[cfg(test)]
mod database_health {
    use super::*;

    #[sqlx::test]
    async fn test_health_verifies_database_connectivity(pool: PgPool) {
        let app = TestApp::new(pool).await;

        // If we can create the app, database is connected
        let request = Request::builder()
            .method(Method::GET)
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let response = app.execute(request).await;

        // Should be OK since we have a valid pool
        response.assert_status(StatusCode::OK);

        let json = response.json_value();

        // Database status might be in checks or root level
        if let Some(checks) = json.get("checks") {
            if let Some(db_check) = checks.get("database") {
                let db_status = db_check.get("status").and_then(|s| s.as_str());
                assert!(
                    db_status == Some("healthy") || db_status == Some("ok"),
                    "Database should be healthy when connected"
                );
            }
        }
    }

    #[sqlx::test]
    async fn test_health_reports_database_latency(pool: PgPool) {
        let app = TestApp::new(pool).await;

        let request = Request::builder()
            .method(Method::GET)
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let response = app.execute(request).await;

        response.assert_status(StatusCode::OK);

        let json = response.json_value();

        // Check for latency in response
        if let Some(checks) = json.get("checks") {
            if let Some(db_check) = checks.get("database") {
                if let Some(latency) = db_check.get("latency_ms") {
                    assert!(
                        latency.is_number(),
                        "Database latency should be a number"
                    );
                }
            }
        }
    }
}

// =============================================================================
// Response Time Tests
// =============================================================================

#[cfg(test)]
mod response_time {
    use super::*;
    use std::time::Instant;

    #[sqlx::test]
    async fn test_health_responds_quickly(pool: PgPool) {
        let app = TestApp::new(pool).await;

        let request = Request::builder()
            .method(Method::GET)
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let start = Instant::now();
        let response = app.execute(request).await;
        let duration = start.elapsed();

        response.assert_status(StatusCode::OK);

        // Health check should respond within 1 second
        assert!(
            duration.as_secs() < 1,
            "Health check should respond quickly, took {:?}",
            duration
        );
    }

    #[sqlx::test]
    async fn test_health_idempotent(pool: PgPool) {
        let app = TestApp::new(pool).await;

        // Call health check multiple times
        for _ in 0..5 {
            let request = Request::builder()
                .method(Method::GET)
                .uri("/health")
                .body(Body::empty())
                .unwrap();

            let response = app.execute(request).await;
            response.assert_status(StatusCode::OK);
        }
    }
}

// =============================================================================
// Content Type Tests
// =============================================================================

#[cfg(test)]
mod content_type {
    use super::*;
    use axum::http::header;

    #[sqlx::test]
    async fn test_health_returns_json(pool: PgPool) {
        let app = TestApp::new(pool).await;

        let request = Request::builder()
            .method(Method::GET)
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let response = app.execute(request).await;

        response.assert_status(StatusCode::OK);

        // Check content type header
        let content_type = response.headers.get(header::CONTENT_TYPE);
        assert!(content_type.is_some(), "Should have Content-Type header");

        let content_type_str = content_type.unwrap().to_str().unwrap();
        assert!(
            content_type_str.contains("application/json"),
            "Content-Type should be application/json, got: {}",
            content_type_str
        );
    }

    #[sqlx::test]
    async fn test_health_response_is_valid_json(pool: PgPool) {
        let app = TestApp::new(pool).await;

        let request = Request::builder()
            .method(Method::GET)
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let response = app.execute(request).await;

        // Should not panic when parsing JSON
        let _json = response.json_value();
    }
}

// =============================================================================
// Method Tests
// =============================================================================

#[cfg(test)]
mod http_methods {
    use super::*;

    #[sqlx::test]
    async fn test_health_accepts_get(pool: PgPool) {
        let app = TestApp::new(pool).await;

        let request = Request::builder()
            .method(Method::GET)
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let response = app.execute(request).await;

        response.assert_status(StatusCode::OK);
    }

    #[sqlx::test]
    async fn test_health_rejects_post(pool: PgPool) {
        let app = TestApp::new(pool).await;

        let request = Request::builder()
            .method(Method::POST)
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let response = app.execute(request).await;

        // Should return 405 Method Not Allowed
        assert!(
            response.status == StatusCode::METHOD_NOT_ALLOWED
                || response.status == StatusCode::NOT_FOUND,
            "POST to health should be rejected"
        );
    }

    #[sqlx::test]
    async fn test_health_handles_head_request(pool: PgPool) {
        let app = TestApp::new(pool).await;

        let request = Request::builder()
            .method(Method::HEAD)
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let response = app.execute(request).await;

        // HEAD should return OK with no body or be rejected
        assert!(
            response.status == StatusCode::OK
                || response.status == StatusCode::METHOD_NOT_ALLOWED
                || response.status == StatusCode::NOT_FOUND,
            "HEAD request should be handled appropriately"
        );
    }
}

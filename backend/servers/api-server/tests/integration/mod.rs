//! Integration tests for api-server routes.
//!
//! These tests validate end-to-end HTTP flows including:
//! - Authentication (register, login, logout, token refresh)
//! - Authorization (role-based access)
//! - Error handling

pub mod auth_tests;

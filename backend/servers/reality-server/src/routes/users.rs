//! Portal user routes - separate from Property Management users.
//!
//! Supports SSO with Property Management via OAuth 2.0.

use axum::Router;

/// Create users router.
pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
    // TODO: Add user routes
    // POST /register           - Register portal user
    // POST /login              - Login
    // POST /logout             - Logout
    // GET /me                  - Get current user
    // PUT /me                  - Update profile
    // POST /oauth/pm           - OAuth login via Property Management
    // POST /link-account       - Link portal account to PM account
}

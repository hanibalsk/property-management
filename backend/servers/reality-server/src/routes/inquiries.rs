//! Inquiries routes - contact and viewing requests.

use axum::Router;

/// Create inquiries router.
pub fn router() -> Router {
    Router::new()
    // TODO: Add inquiry routes
    // POST /contact/:listing_id    - Send contact message
    // POST /viewing/:listing_id    - Request viewing
    // GET /                        - List my inquiries (authenticated)
    // GET /:id                     - Get inquiry status
}

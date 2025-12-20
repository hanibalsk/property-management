//! Listing routes (UC-31) - Real estate listing management.

use axum::Router;
use crate::state::AppState;

/// Create listings router.
pub fn router() -> Router<AppState> {
    Router::new()
    // TODO: Add listing routes
    // POST /               - Create listing
    // GET /                - List my listings
    // GET /:id             - Get listing
    // PUT /:id             - Update listing
    // DELETE /:id          - Delete listing
    // POST /:id/publish    - Publish to portal
    // POST /:id/photos     - Upload photos
    // GET /:id/viewings    - List viewings
    // POST /:id/viewings   - Schedule viewing
}

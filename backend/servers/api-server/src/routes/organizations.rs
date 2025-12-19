//! Organization routes (UC-27) - Multi-tenancy.

use axum::Router;

/// Create organizations router.
pub fn router() -> Router {
    Router::new()
    // TODO: Add organization routes
    // POST /               - Create organization
    // GET /                - List organizations
    // GET /:id             - Get organization
    // PUT /:id             - Update organization
    // DELETE /:id          - Delete organization
    // GET /:id/buildings   - List buildings in organization
    // POST /:id/buildings  - Assign building to organization
}

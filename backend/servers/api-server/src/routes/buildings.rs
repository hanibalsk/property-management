//! Building routes (UC-15).

use axum::Router;
use crate::state::AppState;

/// Create buildings router.
pub fn router() -> Router<AppState> {
    Router::new()
    // TODO: Add building routes
    // POST /               - Create building
    // GET /                - List buildings
    // GET /:id             - Get building
    // PUT /:id             - Update building
    // DELETE /:id          - Delete building
    // GET /:id/units       - List units in building
    // POST /:id/units      - Create unit in building
}

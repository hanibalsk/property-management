//! Building routes (UC-15).

use crate::state::AppState;
use axum::Router;

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

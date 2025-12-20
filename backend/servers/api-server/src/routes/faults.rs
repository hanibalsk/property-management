//! Fault routes (UC-03).

use axum::Router;
use crate::state::AppState;

/// Create faults router.
pub fn router() -> Router<AppState> {
    Router::new()
    // TODO: Add fault routes
    // POST /               - Report fault
    // GET /                - List faults
    // GET /:id             - Get fault
    // PUT /:id             - Update fault
    // POST /:id/comments   - Add comment
    // PUT /:id/status      - Update status
}

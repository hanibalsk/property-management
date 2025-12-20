//! Voting routes (UC-04).

use axum::Router;
use crate::state::AppState;

/// Create voting router.
pub fn router() -> Router<AppState> {
    Router::new()
    // TODO: Add voting routes
    // POST /               - Create vote/poll
    // GET /                - List votes
    // GET /:id             - Get vote
    // POST /:id/cast       - Cast vote
    // GET /:id/results     - Get results
    // PUT /:id/close       - Close voting
}

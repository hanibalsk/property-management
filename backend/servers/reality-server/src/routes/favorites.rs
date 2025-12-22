//! Favorites routes - save and manage favorite listings.

use axum::Router;

/// Create favorites router.
pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
    // TODO: Add favorites routes
    // GET /                - List favorites
    // POST /:listing_id    - Add to favorites
    // DELETE /:listing_id  - Remove from favorites
}

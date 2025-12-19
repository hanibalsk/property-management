//! Integration routes (UC-22, UC-32) - External portal integrations.

use axum::Router;

/// Create integrations router.
pub fn router() -> Router {
    Router::new()
    // TODO: Add integration routes
    // GET /portals                 - List connected portals
    // POST /portals/:id/connect    - Connect to portal
    // DELETE /portals/:id          - Disconnect from portal
    // POST /webhooks/airbnb        - Airbnb webhook
    // POST /webhooks/booking       - Booking.com webhook
    // POST /webhooks/portal        - Generic portal webhook
    // GET /sync/status             - Sync status
}

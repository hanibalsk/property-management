//! Rental routes (UC-29, UC-30) - Airbnb/Booking integration.

use crate::state::AppState;
use axum::Router;

/// Create rentals router.
pub fn router() -> Router<AppState> {
    Router::new()
    // TODO: Add rental routes
    // GET /reservations            - List reservations
    // GET /reservations/:id        - Get reservation
    // POST /sync/airbnb            - Sync with Airbnb
    // POST /sync/booking           - Sync with Booking.com
    // POST /guests                 - Register guest
    // GET /guests                  - List guests
    // POST /guests/:id/check-in    - Check in guest
    // POST /guests/:id/check-out   - Check out guest
}

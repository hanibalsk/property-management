//! External integrations: Airbnb, Booking, real estate portals.

pub mod airbnb;
pub mod booking;
pub mod portals;

// Re-exports
pub use airbnb::AirbnbClient;
pub use booking::BookingClient;
pub use portals::PortalClient;

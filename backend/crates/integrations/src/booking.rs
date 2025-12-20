//! Booking.com integration client.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BookingError {
    #[error("API error: {0}")]
    Api(String),
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
}

/// Booking.com API client.
#[allow(dead_code)]
pub struct BookingClient {
    client: reqwest::Client,
    api_key: String,
}

impl BookingClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
        }
    }

    /// Sync reservations from Booking.com.
    pub async fn sync_reservations(&self, _property_id: &str) -> Result<(), BookingError> {
        // TODO: Implement Booking.com API integration
        tracing::info!("Syncing Booking.com reservations");
        Ok(())
    }
}

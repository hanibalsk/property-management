//! Airbnb integration client.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AirbnbError {
    #[error("API error: {0}")]
    Api(String),
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
}

/// Airbnb API client.
#[allow(dead_code)]
pub struct AirbnbClient {
    client: reqwest::Client,
    api_key: String,
}

impl AirbnbClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
        }
    }

    /// Sync reservations from Airbnb.
    pub async fn sync_reservations(&self, _listing_id: &str) -> Result<(), AirbnbError> {
        // TODO: Implement Airbnb API integration
        tracing::info!("Syncing Airbnb reservations");
        Ok(())
    }
}

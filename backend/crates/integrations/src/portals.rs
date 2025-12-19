//! Real estate portal integration client.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum PortalError {
    #[error("API error: {0}")]
    Api(String),
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
}

/// Generic real estate portal client.
pub struct PortalClient {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
}

impl PortalClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
            api_key,
        }
    }

    /// Publish listing to portal.
    pub async fn publish_listing(&self, _listing_id: &str) -> Result<String, PortalError> {
        // TODO: Implement portal API integration
        tracing::info!("Publishing listing to portal");
        Ok("external-id".to_string())
    }

    /// Sync listing updates.
    pub async fn sync_listing(&self, _external_id: &str) -> Result<(), PortalError> {
        tracing::info!("Syncing listing with portal");
        Ok(())
    }
}

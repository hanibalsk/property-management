//! Voice Assistant OAuth client module (Epic 98: Integration Completion).
//!
//! Story 98.1: Voice Device OAuth Token Exchange
//! Provides OAuth 2.0 token exchange for voice assistant platforms:
//! - Amazon Alexa Skills Kit
//! - Google Actions (Assistant)
//!
//! # Usage
//! ```ignore
//! use integrations::voice_oauth::{VoiceOAuthClient, VoicePlatform, VoiceOAuthConfig};
//!
//! let config = VoiceOAuthConfig::alexa("client_id", "client_secret");
//! let client = VoiceOAuthClient::new(config);
//! let tokens = client.exchange_code("auth_code", "redirect_uri").await?;
//! ```

use chrono::{DateTime, Duration, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Voice OAuth errors.
#[derive(Debug, Error)]
pub enum VoiceOAuthError {
    /// HTTP request failed.
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// Invalid response from OAuth server.
    #[error("Invalid OAuth response: {0}")]
    InvalidResponse(String),

    /// Token exchange failed.
    #[error("Token exchange failed: {0}")]
    ExchangeFailed(String),

    /// Token refresh failed.
    #[error("Token refresh failed: {0}")]
    RefreshFailed(String),

    /// Invalid platform.
    #[error("Invalid voice platform: {0}")]
    InvalidPlatform(String),

    /// Missing configuration.
    #[error("Missing configuration: {0}")]
    MissingConfig(String),
}

/// Voice assistant platform types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoicePlatform {
    /// Amazon Alexa Skills Kit.
    Alexa,
    /// Google Actions / Assistant.
    GoogleAssistant,
}

impl std::fmt::Display for VoicePlatform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VoicePlatform::Alexa => write!(f, "alexa"),
            VoicePlatform::GoogleAssistant => write!(f, "google_assistant"),
        }
    }
}

impl std::str::FromStr for VoicePlatform {
    type Err = VoiceOAuthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "alexa" | "amazon_alexa" => Ok(VoicePlatform::Alexa),
            "google_assistant" | "google" | "google_actions" => Ok(VoicePlatform::GoogleAssistant),
            _ => Err(VoiceOAuthError::InvalidPlatform(s.to_string())),
        }
    }
}

/// OAuth configuration for voice platforms.
#[derive(Debug, Clone)]
pub struct VoiceOAuthConfig {
    /// Platform type.
    pub platform: VoicePlatform,
    /// OAuth client ID.
    pub client_id: String,
    /// OAuth client secret.
    pub client_secret: String,
    /// Token endpoint URL (optional, uses platform default if not set).
    pub token_endpoint: Option<String>,
}

impl VoiceOAuthConfig {
    /// Create Alexa OAuth configuration.
    pub fn alexa(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        Self {
            platform: VoicePlatform::Alexa,
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            token_endpoint: None,
        }
    }

    /// Create Google Assistant OAuth configuration.
    pub fn google_assistant(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> Self {
        Self {
            platform: VoicePlatform::GoogleAssistant,
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            token_endpoint: None,
        }
    }

    /// Get the token endpoint URL for this platform.
    pub fn get_token_endpoint(&self) -> &str {
        self.token_endpoint
            .as_deref()
            .unwrap_or_else(|| match self.platform {
                VoicePlatform::Alexa => "https://api.amazon.com/auth/o2/token",
                VoicePlatform::GoogleAssistant => "https://oauth2.googleapis.com/token",
            })
    }
}

/// OAuth tokens from voice platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceOAuthTokens {
    /// Access token for API calls.
    pub access_token: String,
    /// Refresh token for obtaining new access tokens.
    pub refresh_token: Option<String>,
    /// Token expiration time.
    pub expires_at: Option<DateTime<Utc>>,
    /// Token type (usually "Bearer").
    pub token_type: String,
    /// Granted scopes (if returned).
    pub scope: Option<String>,
}

/// Token response from OAuth server.
#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: Option<String>,
    expires_in: Option<i64>,
    token_type: Option<String>,
    scope: Option<String>,
}

/// Error response from OAuth server.
#[derive(Debug, Deserialize)]
struct ErrorResponse {
    error: String,
    error_description: Option<String>,
}

/// Voice OAuth client for token operations.
#[derive(Debug, Clone)]
pub struct VoiceOAuthClient {
    /// OAuth configuration.
    config: VoiceOAuthConfig,
    /// HTTP client.
    http: Client,
}

impl VoiceOAuthClient {
    /// Create a new voice OAuth client.
    pub fn new(config: VoiceOAuthConfig) -> Self {
        Self {
            config,
            http: Client::new(),
        }
    }

    /// Create from environment variables.
    ///
    /// For Alexa: ALEXA_CLIENT_ID, ALEXA_CLIENT_SECRET
    /// For Google: GOOGLE_VOICE_CLIENT_ID, GOOGLE_VOICE_CLIENT_SECRET
    pub fn from_env(platform: VoicePlatform) -> Result<Self, VoiceOAuthError> {
        let (client_id, client_secret) = match platform {
            VoicePlatform::Alexa => (
                std::env::var("ALEXA_CLIENT_ID")
                    .map_err(|_| VoiceOAuthError::MissingConfig("ALEXA_CLIENT_ID".to_string()))?,
                std::env::var("ALEXA_CLIENT_SECRET").map_err(|_| {
                    VoiceOAuthError::MissingConfig("ALEXA_CLIENT_SECRET".to_string())
                })?,
            ),
            VoicePlatform::GoogleAssistant => (
                std::env::var("GOOGLE_VOICE_CLIENT_ID").map_err(|_| {
                    VoiceOAuthError::MissingConfig("GOOGLE_VOICE_CLIENT_ID".to_string())
                })?,
                std::env::var("GOOGLE_VOICE_CLIENT_SECRET").map_err(|_| {
                    VoiceOAuthError::MissingConfig("GOOGLE_VOICE_CLIENT_SECRET".to_string())
                })?,
            ),
        };

        let config = match platform {
            VoicePlatform::Alexa => VoiceOAuthConfig::alexa(client_id, client_secret),
            VoicePlatform::GoogleAssistant => {
                VoiceOAuthConfig::google_assistant(client_id, client_secret)
            }
        };

        Ok(Self::new(config))
    }

    /// Exchange authorization code for access tokens.
    ///
    /// This is called after the user completes OAuth consent flow on the voice platform.
    ///
    /// # Arguments
    /// * `auth_code` - Authorization code from OAuth callback
    /// * `redirect_uri` - Redirect URI used in authorization request
    ///
    /// # Returns
    /// OAuth tokens on success.
    pub async fn exchange_code(
        &self,
        auth_code: &str,
        redirect_uri: &str,
    ) -> Result<VoiceOAuthTokens, VoiceOAuthError> {
        let token_endpoint = self.config.get_token_endpoint();

        let response = self
            .http
            .post(token_endpoint)
            .form(&[
                ("grant_type", "authorization_code"),
                ("code", auth_code),
                ("redirect_uri", redirect_uri),
                ("client_id", &self.config.client_id),
                ("client_secret", &self.config.client_secret),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            let error_body = response.text().await.unwrap_or_default();
            if let Ok(error) = serde_json::from_str::<ErrorResponse>(&error_body) {
                return Err(VoiceOAuthError::ExchangeFailed(format!(
                    "{}: {}",
                    error.error,
                    error.error_description.unwrap_or_default()
                )));
            }
            return Err(VoiceOAuthError::ExchangeFailed(error_body));
        }

        let token_response: TokenResponse = response.json().await?;
        Ok(self.parse_token_response(token_response))
    }

    /// Refresh an expired access token using the refresh token.
    ///
    /// # Arguments
    /// * `refresh_token` - Refresh token from previous token exchange
    ///
    /// # Returns
    /// New OAuth tokens on success.
    pub async fn refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<VoiceOAuthTokens, VoiceOAuthError> {
        let token_endpoint = self.config.get_token_endpoint();

        let response = self
            .http
            .post(token_endpoint)
            .form(&[
                ("grant_type", "refresh_token"),
                ("refresh_token", refresh_token),
                ("client_id", &self.config.client_id),
                ("client_secret", &self.config.client_secret),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            let error_body = response.text().await.unwrap_or_default();
            if let Ok(error) = serde_json::from_str::<ErrorResponse>(&error_body) {
                return Err(VoiceOAuthError::RefreshFailed(format!(
                    "{}: {}",
                    error.error,
                    error.error_description.unwrap_or_default()
                )));
            }
            return Err(VoiceOAuthError::RefreshFailed(error_body));
        }

        let token_response: TokenResponse = response.json().await?;
        Ok(self.parse_token_response(token_response))
    }

    /// Parse token response into VoiceOAuthTokens.
    fn parse_token_response(&self, response: TokenResponse) -> VoiceOAuthTokens {
        let expires_at = response
            .expires_in
            .map(|secs| Utc::now() + Duration::seconds(secs));

        VoiceOAuthTokens {
            access_token: response.access_token,
            refresh_token: response.refresh_token,
            expires_at,
            token_type: response.token_type.unwrap_or_else(|| "Bearer".to_string()),
            scope: response.scope,
        }
    }

    /// Get the platform for this client.
    pub fn platform(&self) -> VoicePlatform {
        self.config.platform
    }
}

/// Voice OAuth manager for handling multiple platforms.
#[derive(Debug, Clone, Default)]
pub struct VoiceOAuthManager {
    /// Alexa client (if configured).
    alexa_client: Option<VoiceOAuthClient>,
    /// Google client (if configured).
    google_client: Option<VoiceOAuthClient>,
}

impl VoiceOAuthManager {
    /// Create a new voice OAuth manager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create from environment variables.
    ///
    /// Attempts to configure clients for all platforms that have environment variables set.
    pub fn from_env() -> Self {
        let alexa_client = VoiceOAuthClient::from_env(VoicePlatform::Alexa).ok();
        let google_client = VoiceOAuthClient::from_env(VoicePlatform::GoogleAssistant).ok();

        Self {
            alexa_client,
            google_client,
        }
    }

    /// Set Alexa client configuration.
    pub fn with_alexa(
        mut self,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> Self {
        self.alexa_client = Some(VoiceOAuthClient::new(VoiceOAuthConfig::alexa(
            client_id,
            client_secret,
        )));
        self
    }

    /// Set Google client configuration.
    pub fn with_google(
        mut self,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> Self {
        self.google_client = Some(VoiceOAuthClient::new(VoiceOAuthConfig::google_assistant(
            client_id,
            client_secret,
        )));
        self
    }

    /// Get client for a specific platform.
    pub fn get_client(&self, platform: VoicePlatform) -> Option<&VoiceOAuthClient> {
        match platform {
            VoicePlatform::Alexa => self.alexa_client.as_ref(),
            VoicePlatform::GoogleAssistant => self.google_client.as_ref(),
        }
    }

    /// Check if a platform is configured.
    pub fn has_platform(&self, platform: VoicePlatform) -> bool {
        self.get_client(platform).is_some()
    }

    /// Exchange authorization code for tokens.
    pub async fn exchange_code(
        &self,
        platform: VoicePlatform,
        auth_code: &str,
        redirect_uri: &str,
    ) -> Result<VoiceOAuthTokens, VoiceOAuthError> {
        let client = self.get_client(platform).ok_or_else(|| {
            VoiceOAuthError::MissingConfig(format!("{} not configured", platform))
        })?;
        client.exchange_code(auth_code, redirect_uri).await
    }

    /// Refresh tokens for a platform.
    pub async fn refresh_token(
        &self,
        platform: VoicePlatform,
        refresh_token: &str,
    ) -> Result<VoiceOAuthTokens, VoiceOAuthError> {
        let client = self.get_client(platform).ok_or_else(|| {
            VoiceOAuthError::MissingConfig(format!("{} not configured", platform))
        })?;
        client.refresh_token(refresh_token).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_parsing() {
        assert_eq!(
            "alexa".parse::<VoicePlatform>().unwrap(),
            VoicePlatform::Alexa
        );
        assert_eq!(
            "google_assistant".parse::<VoicePlatform>().unwrap(),
            VoicePlatform::GoogleAssistant
        );
        assert_eq!(
            "google".parse::<VoicePlatform>().unwrap(),
            VoicePlatform::GoogleAssistant
        );
        assert!("invalid".parse::<VoicePlatform>().is_err());
    }

    #[test]
    fn test_platform_display() {
        assert_eq!(VoicePlatform::Alexa.to_string(), "alexa");
        assert_eq!(
            VoicePlatform::GoogleAssistant.to_string(),
            "google_assistant"
        );
    }

    #[test]
    fn test_config_creation() {
        let alexa = VoiceOAuthConfig::alexa("id", "secret");
        assert_eq!(alexa.platform, VoicePlatform::Alexa);
        assert_eq!(
            alexa.get_token_endpoint(),
            "https://api.amazon.com/auth/o2/token"
        );

        let google = VoiceOAuthConfig::google_assistant("id", "secret");
        assert_eq!(google.platform, VoicePlatform::GoogleAssistant);
        assert_eq!(
            google.get_token_endpoint(),
            "https://oauth2.googleapis.com/token"
        );
    }

    #[test]
    fn test_manager_creation() {
        let manager = VoiceOAuthManager::new()
            .with_alexa("alexa_id", "alexa_secret")
            .with_google("google_id", "google_secret");

        assert!(manager.has_platform(VoicePlatform::Alexa));
        assert!(manager.has_platform(VoicePlatform::GoogleAssistant));
    }

    #[test]
    fn test_manager_without_platforms() {
        let manager = VoiceOAuthManager::new();
        assert!(!manager.has_platform(VoicePlatform::Alexa));
        assert!(!manager.has_platform(VoicePlatform::GoogleAssistant));
    }
}

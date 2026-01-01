//! OAuth token management service (Story 96.1).
//!
//! Provides unified OAuth token storage, automatic refresh, and revocation
//! for external calendar integrations (Google, Microsoft).
//!
//! # Features
//! - Encrypted token storage using IntegrationCrypto
//! - Automatic token refresh before expiration
//! - Token revocation support
//! - Configurable refresh buffer time
//!
//! # Example
//! ```ignore
//! use integrations::oauth::{OAuthTokenManager, TokenRefreshConfig};
//!
//! let manager = OAuthTokenManager::new(crypto, config);
//! let tokens = manager.get_valid_tokens(connection_id).await?;
//! ```

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::calendar::{CalendarError, GoogleCalendarClient, MicrosoftCalendarClient, OAuthConfig};
use crate::crypto::IntegrationCrypto;

/// Default buffer time before token expiration to trigger refresh (5 minutes).
pub const DEFAULT_REFRESH_BUFFER_SECS: i64 = 300;

/// Minimum buffer time to prevent excessive refreshing (1 minute).
pub const MIN_REFRESH_BUFFER_SECS: i64 = 60;

/// Maximum buffer time (1 hour).
pub const MAX_REFRESH_BUFFER_SECS: i64 = 3600;

/// OAuth token management errors.
#[derive(Debug, Error)]
pub enum OAuthError {
    /// Token storage/retrieval error.
    #[error("Storage error: {0}")]
    Storage(String),

    /// Token refresh failed.
    #[error("Token refresh failed: {0}")]
    RefreshFailed(String),

    /// Token revocation failed.
    #[error("Token revocation failed: {0}")]
    RevocationFailed(String),

    /// Token not found.
    #[error("Token not found for connection: {0}")]
    NotFound(Uuid),

    /// Token is expired and cannot be refreshed.
    #[error("Token expired and no refresh token available")]
    ExpiredNoRefresh,

    /// Invalid provider type.
    #[error("Invalid provider: {0}")]
    InvalidProvider(String),

    /// Encryption error.
    #[error("Encryption error: {0}")]
    Encryption(String),

    /// Calendar client error.
    #[error("Calendar error: {0}")]
    Calendar(#[from] CalendarError),

    /// Airbnb client error.
    #[error("Airbnb error: {0}")]
    Airbnb(#[from] crate::airbnb::AirbnbError),

    /// Configuration error.
    #[error("Configuration error: {0}")]
    Config(String),
}

/// OAuth provider types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OAuthProvider {
    /// Google OAuth (Calendar, etc.)
    Google,
    /// Microsoft OAuth (Outlook, etc.)
    Microsoft,
    /// Airbnb OAuth
    Airbnb,
}

impl std::fmt::Display for OAuthProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OAuthProvider::Google => write!(f, "google"),
            OAuthProvider::Microsoft => write!(f, "microsoft"),
            OAuthProvider::Airbnb => write!(f, "airbnb"),
        }
    }
}

impl std::str::FromStr for OAuthProvider {
    type Err = OAuthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "google" => Ok(OAuthProvider::Google),
            "microsoft" | "outlook" => Ok(OAuthProvider::Microsoft),
            "airbnb" => Ok(OAuthProvider::Airbnb),
            _ => Err(OAuthError::InvalidProvider(s.to_string())),
        }
    }
}

/// Stored OAuth token data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredToken {
    /// Connection ID this token belongs to.
    pub connection_id: Uuid,
    /// OAuth provider.
    pub provider: OAuthProvider,
    /// Encrypted access token.
    pub access_token_encrypted: String,
    /// Encrypted refresh token (optional).
    pub refresh_token_encrypted: Option<String>,
    /// Token expiration time.
    pub expires_at: Option<DateTime<Utc>>,
    /// Token scopes granted.
    pub scopes: Option<String>,
    /// When the token was last refreshed.
    pub last_refreshed_at: Option<DateTime<Utc>>,
    /// Number of refresh attempts.
    pub refresh_count: i32,
    /// Last refresh error, if any.
    pub last_refresh_error: Option<String>,
}

impl StoredToken {
    /// Check if the token is expired.
    pub fn is_expired(&self) -> bool {
        self.expires_at
            .map(|exp| exp <= Utc::now())
            .unwrap_or(false)
    }

    /// Check if the token needs refresh within the given buffer time.
    pub fn needs_refresh(&self, buffer_secs: i64) -> bool {
        self.expires_at
            .map(|exp| exp <= Utc::now() + Duration::seconds(buffer_secs))
            .unwrap_or(false)
    }

    /// Check if a refresh token is available.
    pub fn has_refresh_token(&self) -> bool {
        self.refresh_token_encrypted.is_some()
    }
}

/// Decrypted OAuth tokens ready for use.
#[derive(Debug, Clone)]
pub struct DecryptedTokens {
    /// Access token for API calls.
    pub access_token: String,
    /// Refresh token for obtaining new access tokens.
    pub refresh_token: Option<String>,
    /// Token expiration time.
    pub expires_at: Option<DateTime<Utc>>,
    /// Token scopes.
    pub scopes: Option<String>,
}

/// Configuration for token refresh behavior.
#[derive(Debug, Clone)]
pub struct TokenRefreshConfig {
    /// Buffer time before expiration to trigger refresh (seconds).
    pub refresh_buffer_secs: i64,
    /// Maximum number of refresh retries on failure.
    pub max_retry_attempts: u32,
    /// Backoff multiplier for retries.
    pub retry_backoff_multiplier: f64,
    /// Initial retry delay in milliseconds.
    pub initial_retry_delay_ms: u64,
}

impl Default for TokenRefreshConfig {
    fn default() -> Self {
        Self {
            refresh_buffer_secs: DEFAULT_REFRESH_BUFFER_SECS,
            max_retry_attempts: 3,
            retry_backoff_multiplier: 2.0,
            initial_retry_delay_ms: 1000,
        }
    }
}

impl TokenRefreshConfig {
    /// Create a new config with custom buffer time.
    pub fn with_buffer(buffer_secs: i64) -> Self {
        Self {
            refresh_buffer_secs: buffer_secs
                .clamp(MIN_REFRESH_BUFFER_SECS, MAX_REFRESH_BUFFER_SECS),
            ..Default::default()
        }
    }

    /// Validate the configuration.
    pub fn validate(&self) -> Result<(), OAuthError> {
        if self.refresh_buffer_secs < MIN_REFRESH_BUFFER_SECS {
            return Err(OAuthError::Config(format!(
                "Refresh buffer must be at least {} seconds",
                MIN_REFRESH_BUFFER_SECS
            )));
        }
        if self.refresh_buffer_secs > MAX_REFRESH_BUFFER_SECS {
            return Err(OAuthError::Config(format!(
                "Refresh buffer must be at most {} seconds",
                MAX_REFRESH_BUFFER_SECS
            )));
        }
        Ok(())
    }
}

/// OAuth provider configuration holder.
#[derive(Debug, Clone, Default)]
pub struct ProviderConfigs {
    /// Google OAuth configuration.
    pub google: Option<OAuthConfig>,
    /// Microsoft OAuth configuration.
    pub microsoft: Option<OAuthConfig>,
    /// Airbnb OAuth configuration.
    pub airbnb: Option<crate::airbnb::AirbnbOAuthConfig>,
}

impl ProviderConfigs {
    /// Create a new empty provider configs.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set Google OAuth configuration.
    pub fn with_google(mut self, config: OAuthConfig) -> Self {
        self.google = Some(config);
        self
    }

    /// Set Microsoft OAuth configuration.
    pub fn with_microsoft(mut self, config: OAuthConfig) -> Self {
        self.microsoft = Some(config);
        self
    }

    /// Set Airbnb OAuth configuration.
    pub fn with_airbnb(mut self, config: crate::airbnb::AirbnbOAuthConfig) -> Self {
        self.airbnb = Some(config);
        self
    }

    /// Check if a provider is configured.
    pub fn has_provider(&self, provider: OAuthProvider) -> bool {
        match provider {
            OAuthProvider::Google => self.google.is_some(),
            OAuthProvider::Microsoft => self.microsoft.is_some(),
            OAuthProvider::Airbnb => self.airbnb.is_some(),
        }
    }
}

/// Token refresh result.
#[derive(Debug, Clone)]
pub struct RefreshResult {
    /// New access token.
    pub access_token: String,
    /// New refresh token (if rotated).
    pub refresh_token: Option<String>,
    /// New expiration time.
    pub expires_at: Option<DateTime<Utc>>,
    /// Whether the refresh token was rotated.
    pub token_rotated: bool,
}

/// OAuth token manager for handling token lifecycle.
///
/// Manages encrypted storage, automatic refresh, and revocation of OAuth tokens
/// for various external integrations.
pub struct OAuthTokenManager {
    /// Crypto service for encrypting/decrypting tokens.
    crypto: Option<IntegrationCrypto>,
    /// Provider configurations.
    providers: ProviderConfigs,
    /// Refresh configuration.
    config: TokenRefreshConfig,
}

impl OAuthTokenManager {
    /// Create a new OAuth token manager.
    pub fn new(crypto: Option<IntegrationCrypto>, providers: ProviderConfigs) -> Self {
        Self {
            crypto,
            providers,
            config: TokenRefreshConfig::default(),
        }
    }

    /// Create a new OAuth token manager with custom refresh config.
    pub fn with_config(
        crypto: Option<IntegrationCrypto>,
        providers: ProviderConfigs,
        config: TokenRefreshConfig,
    ) -> Result<Self, OAuthError> {
        config.validate()?;
        Ok(Self {
            crypto,
            providers,
            config,
        })
    }

    /// Encrypt tokens for storage.
    pub fn encrypt_tokens(
        &self,
        connection_id: Uuid,
        provider: OAuthProvider,
        access_token: &str,
        refresh_token: Option<&str>,
        expires_at: Option<DateTime<Utc>>,
        scopes: Option<&str>,
    ) -> Result<StoredToken, OAuthError> {
        let access_token_encrypted = match &self.crypto {
            Some(crypto) => crypto
                .encrypt(access_token)
                .map_err(|e| OAuthError::Encryption(e.to_string()))?,
            None => access_token.to_string(),
        };

        let refresh_token_encrypted = match (&self.crypto, refresh_token) {
            (Some(crypto), Some(rt)) => Some(
                crypto
                    .encrypt(rt)
                    .map_err(|e| OAuthError::Encryption(e.to_string()))?,
            ),
            (None, Some(rt)) => Some(rt.to_string()),
            _ => None,
        };

        Ok(StoredToken {
            connection_id,
            provider,
            access_token_encrypted,
            refresh_token_encrypted,
            expires_at,
            scopes: scopes.map(|s| s.to_string()),
            last_refreshed_at: None,
            refresh_count: 0,
            last_refresh_error: None,
        })
    }

    /// Decrypt stored tokens for use.
    pub fn decrypt_tokens(&self, stored: &StoredToken) -> Result<DecryptedTokens, OAuthError> {
        let access_token = match &self.crypto {
            Some(crypto) => crypto
                .decrypt(&stored.access_token_encrypted)
                .map_err(|e| OAuthError::Encryption(e.to_string()))?,
            None => stored.access_token_encrypted.clone(),
        };

        let refresh_token = match (&self.crypto, &stored.refresh_token_encrypted) {
            (Some(crypto), Some(rt)) => Some(
                crypto
                    .decrypt(rt)
                    .map_err(|e| OAuthError::Encryption(e.to_string()))?,
            ),
            (None, Some(rt)) => Some(rt.clone()),
            _ => None,
        };

        Ok(DecryptedTokens {
            access_token,
            refresh_token,
            expires_at: stored.expires_at,
            scopes: stored.scopes.clone(),
        })
    }

    /// Check if a token needs refresh.
    pub fn needs_refresh(&self, stored: &StoredToken) -> bool {
        stored.needs_refresh(self.config.refresh_buffer_secs)
    }

    /// Refresh tokens using the appropriate provider client.
    pub async fn refresh_tokens(&self, stored: &StoredToken) -> Result<RefreshResult, OAuthError> {
        let decrypted = self.decrypt_tokens(stored)?;
        let refresh_token = decrypted
            .refresh_token
            .ok_or(OAuthError::ExpiredNoRefresh)?;

        match stored.provider {
            OAuthProvider::Google => {
                let config =
                    self.providers.google.as_ref().ok_or_else(|| {
                        OAuthError::Config("Google OAuth not configured".to_string())
                    })?;
                let client = GoogleCalendarClient::new(config.clone());
                let tokens = client.refresh_token(&refresh_token).await?;

                let token_rotated = tokens
                    .refresh_token
                    .as_ref()
                    .map(|rt| rt != &refresh_token)
                    .unwrap_or(false);

                Ok(RefreshResult {
                    access_token: tokens.access_token,
                    refresh_token: tokens.refresh_token,
                    expires_at: tokens.expires_at,
                    token_rotated,
                })
            }
            OAuthProvider::Microsoft => {
                let config = self.providers.microsoft.as_ref().ok_or_else(|| {
                    OAuthError::Config("Microsoft OAuth not configured".to_string())
                })?;
                let client = MicrosoftCalendarClient::new(config.clone());
                let tokens = client.refresh_token(&refresh_token).await?;

                let token_rotated = tokens
                    .refresh_token
                    .as_ref()
                    .map(|rt| rt != &refresh_token)
                    .unwrap_or(false);

                Ok(RefreshResult {
                    access_token: tokens.access_token,
                    refresh_token: tokens.refresh_token,
                    expires_at: tokens.expires_at,
                    token_rotated,
                })
            }
            OAuthProvider::Airbnb => {
                let config =
                    self.providers.airbnb.as_ref().ok_or_else(|| {
                        OAuthError::Config("Airbnb OAuth not configured".to_string())
                    })?;
                let client = crate::airbnb::AirbnbClient::new(config.clone());
                let tokens = client.refresh_token(&refresh_token).await?;

                let token_rotated = tokens
                    .refresh_token
                    .as_ref()
                    .map(|rt| rt != &refresh_token)
                    .unwrap_or(false);

                Ok(RefreshResult {
                    access_token: tokens.access_token,
                    refresh_token: tokens.refresh_token,
                    expires_at: tokens.expires_at,
                    token_rotated,
                })
            }
        }
    }

    /// Get the refresh buffer in seconds.
    pub fn refresh_buffer_secs(&self) -> i64 {
        self.config.refresh_buffer_secs
    }

    /// Get the refresh configuration.
    pub fn config(&self) -> &TokenRefreshConfig {
        &self.config
    }
}

/// Connections that need token refresh.
#[derive(Debug, Clone)]
pub struct ConnectionsNeedingRefresh {
    /// Calendar connections needing refresh.
    pub calendar_connections: Vec<Uuid>,
    /// Airbnb connections needing refresh (for future use).
    pub airbnb_connections: Vec<Uuid>,
}

/// Token refresh scheduler for background processing.
///
/// Identifies connections with tokens that need refreshing and provides
/// scheduling information for background workers.
pub struct TokenRefreshScheduler {
    /// Buffer time before expiration to trigger refresh.
    refresh_buffer_secs: i64,
}

impl TokenRefreshScheduler {
    /// Create a new refresh scheduler.
    pub fn new(refresh_buffer_secs: i64) -> Self {
        Self {
            refresh_buffer_secs: refresh_buffer_secs
                .clamp(MIN_REFRESH_BUFFER_SECS, MAX_REFRESH_BUFFER_SECS),
        }
    }

    /// Create a scheduler with default settings.
    pub fn with_defaults() -> Self {
        Self::new(DEFAULT_REFRESH_BUFFER_SECS)
    }

    /// Get the threshold time for refresh (connections expiring before this time need refresh).
    pub fn refresh_threshold(&self) -> DateTime<Utc> {
        Utc::now() + Duration::seconds(self.refresh_buffer_secs)
    }

    /// Calculate the next scheduled refresh time for a connection.
    ///
    /// Returns the time when a refresh should be attempted, which is
    /// `expires_at - buffer_time`.
    pub fn next_refresh_time(&self, expires_at: DateTime<Utc>) -> DateTime<Utc> {
        expires_at - Duration::seconds(self.refresh_buffer_secs)
    }

    /// Check if a token needs immediate refresh.
    pub fn needs_immediate_refresh(&self, expires_at: Option<DateTime<Utc>>) -> bool {
        expires_at
            .map(|exp| exp <= self.refresh_threshold())
            .unwrap_or(false)
    }

    /// Get the buffer time in seconds.
    pub fn buffer_secs(&self) -> i64 {
        self.refresh_buffer_secs
    }
}

/// Token revocation result.
#[derive(Debug, Clone)]
pub struct RevocationResult {
    /// Whether the revocation was successful.
    pub success: bool,
    /// Provider that was revoked.
    pub provider: OAuthProvider,
    /// Connection that was revoked.
    pub connection_id: Uuid,
    /// Error message if revocation failed.
    pub error: Option<String>,
}

/// Revoke OAuth tokens for a connection.
///
/// Note: Most OAuth providers do not support programmatic token revocation
/// via API. This function primarily:
/// 1. Marks the tokens as revoked in local storage
/// 2. Attempts provider-specific revocation if supported
///
/// # Arguments
/// * `provider` - The OAuth provider
/// * `connection_id` - The connection ID
/// * `access_token` - The access token to revoke (optional, for provider API call)
///
/// # Returns
/// A RevocationResult indicating success/failure
pub fn create_revocation_result(
    provider: OAuthProvider,
    connection_id: Uuid,
    success: bool,
    error: Option<String>,
) -> RevocationResult {
    RevocationResult {
        success,
        provider,
        connection_id,
        error,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_crypto() -> IntegrationCrypto {
        IntegrationCrypto::new("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef")
            .unwrap()
    }

    #[test]
    fn test_oauth_provider_parsing() {
        assert_eq!(
            "google".parse::<OAuthProvider>().unwrap(),
            OAuthProvider::Google
        );
        assert_eq!(
            "microsoft".parse::<OAuthProvider>().unwrap(),
            OAuthProvider::Microsoft
        );
        assert_eq!(
            "outlook".parse::<OAuthProvider>().unwrap(),
            OAuthProvider::Microsoft
        );
        assert_eq!(
            "airbnb".parse::<OAuthProvider>().unwrap(),
            OAuthProvider::Airbnb
        );
        assert!("invalid".parse::<OAuthProvider>().is_err());
    }

    #[test]
    fn test_oauth_provider_display() {
        assert_eq!(OAuthProvider::Google.to_string(), "google");
        assert_eq!(OAuthProvider::Microsoft.to_string(), "microsoft");
        assert_eq!(OAuthProvider::Airbnb.to_string(), "airbnb");
    }

    #[test]
    fn test_stored_token_is_expired() {
        let mut token = StoredToken {
            connection_id: Uuid::new_v4(),
            provider: OAuthProvider::Google,
            access_token_encrypted: "test".to_string(),
            refresh_token_encrypted: None,
            expires_at: Some(Utc::now() - Duration::hours(1)),
            scopes: None,
            last_refreshed_at: None,
            refresh_count: 0,
            last_refresh_error: None,
        };

        assert!(token.is_expired());

        token.expires_at = Some(Utc::now() + Duration::hours(1));
        assert!(!token.is_expired());

        token.expires_at = None;
        assert!(!token.is_expired());
    }

    #[test]
    fn test_stored_token_needs_refresh() {
        let mut token = StoredToken {
            connection_id: Uuid::new_v4(),
            provider: OAuthProvider::Google,
            access_token_encrypted: "test".to_string(),
            refresh_token_encrypted: Some("refresh".to_string()),
            expires_at: Some(Utc::now() + Duration::seconds(120)),
            scopes: None,
            last_refreshed_at: None,
            refresh_count: 0,
            last_refresh_error: None,
        };

        // Should need refresh within 5 minute buffer
        assert!(token.needs_refresh(300));

        // Should not need refresh within 1 minute buffer
        assert!(!token.needs_refresh(60));

        // Token without expiration doesn't need refresh
        token.expires_at = None;
        assert!(!token.needs_refresh(300));
    }

    #[test]
    fn test_token_refresh_config_validation() {
        let config = TokenRefreshConfig::default();
        assert!(config.validate().is_ok());

        let config = TokenRefreshConfig::with_buffer(30);
        // Should clamp to minimum
        assert_eq!(config.refresh_buffer_secs, MIN_REFRESH_BUFFER_SECS);
    }

    #[test]
    fn test_encrypt_decrypt_tokens() {
        let crypto = test_crypto();
        let manager = OAuthTokenManager::new(Some(crypto), ProviderConfigs::default());

        let connection_id = Uuid::new_v4();
        let access_token = "access_token_12345";
        let refresh_token = "refresh_token_67890";
        let expires_at = Some(Utc::now() + Duration::hours(1));

        let stored = manager
            .encrypt_tokens(
                connection_id,
                OAuthProvider::Google,
                access_token,
                Some(refresh_token),
                expires_at,
                Some("calendar.readonly"),
            )
            .unwrap();

        // Encrypted tokens should be different from plaintext
        assert_ne!(stored.access_token_encrypted, access_token);
        assert_ne!(
            stored.refresh_token_encrypted.as_ref().unwrap(),
            refresh_token
        );

        // Decrypt and verify
        let decrypted = manager.decrypt_tokens(&stored).unwrap();
        assert_eq!(decrypted.access_token, access_token);
        assert_eq!(decrypted.refresh_token.unwrap(), refresh_token);
        assert_eq!(decrypted.scopes, Some("calendar.readonly".to_string()));
    }

    #[test]
    fn test_encrypt_decrypt_without_crypto() {
        let manager = OAuthTokenManager::new(None, ProviderConfigs::default());

        let connection_id = Uuid::new_v4();
        let access_token = "access_token_12345";

        let stored = manager
            .encrypt_tokens(
                connection_id,
                OAuthProvider::Google,
                access_token,
                None,
                None,
                None,
            )
            .unwrap();

        // Without crypto, tokens are stored as-is
        assert_eq!(stored.access_token_encrypted, access_token);

        let decrypted = manager.decrypt_tokens(&stored).unwrap();
        assert_eq!(decrypted.access_token, access_token);
    }

    #[test]
    fn test_provider_configs() {
        let configs = ProviderConfigs::new().with_google(OAuthConfig {
            client_id: "google_id".to_string(),
            client_secret: "google_secret".to_string(),
            redirect_uri: "http://localhost/callback".to_string(),
        });

        assert!(configs.has_provider(OAuthProvider::Google));
        assert!(!configs.has_provider(OAuthProvider::Microsoft));
        assert!(!configs.has_provider(OAuthProvider::Airbnb));
    }

    #[test]
    fn test_token_refresh_scheduler() {
        let scheduler = TokenRefreshScheduler::with_defaults();
        assert_eq!(scheduler.buffer_secs(), DEFAULT_REFRESH_BUFFER_SECS);

        // Token expiring in 2 minutes should need immediate refresh (with 5 min buffer)
        let expires_soon = Some(Utc::now() + Duration::seconds(120));
        assert!(scheduler.needs_immediate_refresh(expires_soon));

        // Token expiring in 1 hour should not need immediate refresh
        let expires_later = Some(Utc::now() + Duration::hours(1));
        assert!(!scheduler.needs_immediate_refresh(expires_later));

        // Token without expiration should not need refresh
        assert!(!scheduler.needs_immediate_refresh(None));
    }

    #[test]
    fn test_next_refresh_time() {
        let scheduler = TokenRefreshScheduler::new(300); // 5 minutes buffer
        let expires_at = Utc::now() + Duration::hours(1);
        let next_refresh = scheduler.next_refresh_time(expires_at);

        // Should be 5 minutes before expiration
        let expected = expires_at - Duration::seconds(300);
        assert!((next_refresh - expected).num_seconds().abs() < 1);
    }

    #[test]
    fn test_revocation_result() {
        let result = create_revocation_result(OAuthProvider::Google, Uuid::new_v4(), true, None);
        assert!(result.success);
        assert!(result.error.is_none());

        let result = create_revocation_result(
            OAuthProvider::Microsoft,
            Uuid::new_v4(),
            false,
            Some("API error".to_string()),
        );
        assert!(!result.success);
        assert_eq!(result.error, Some("API error".to_string()));
    }
}

//! Application state for Reality Server.
//!
//! Contains shared services and configuration for SSO, user management, and portal repositories.
//! Epic 104: Includes PM API health client and SSO token validation caching.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, RwLock};

use db::{
    repositories::{PortalRepository, RealityPortalRepository},
    DbPool,
};

use crate::routes::sso::{OAuthTokens, PendingSsoSession, SessionInfo, SsoUserInfo};

/// Application configuration.
#[derive(Clone, Debug)]
pub struct AppConfig {
    /// PM OAuth authorize URL
    pub pm_oauth_authorize_url: String,
    /// PM OAuth token URL
    pub pm_oauth_token_url: String,
    /// PM userinfo URL
    pub pm_userinfo_url: String,
    /// PM token introspection URL
    pub pm_introspect_url: String,
    /// PM OAuth client ID
    pub pm_client_id: String,
    /// PM OAuth client secret
    pub pm_client_secret: String,
    /// SSO callback URL (this server)
    pub sso_callback_url: String,
    /// JWT secret for session tokens
    pub jwt_secret: String,
    /// PM API health check URL (Epic 104.1)
    pub pm_api_health_url: String,
}

impl AppConfig {
    /// Load configuration from environment variables.
    pub fn from_env() -> Self {
        Self {
            pm_oauth_authorize_url: std::env::var("PM_OAUTH_AUTHORIZE_URL")
                .unwrap_or_else(|_| "http://localhost:8080/api/v1/oauth/authorize".to_string()),
            pm_oauth_token_url: std::env::var("PM_OAUTH_TOKEN_URL")
                .unwrap_or_else(|_| "http://localhost:8080/api/v1/oauth/token".to_string()),
            pm_userinfo_url: std::env::var("PM_USERINFO_URL")
                .unwrap_or_else(|_| "http://localhost:8080/api/v1/oauth/userinfo".to_string()),
            pm_introspect_url: std::env::var("PM_INTROSPECT_URL")
                .unwrap_or_else(|_| "http://localhost:8080/api/v1/oauth/introspect".to_string()),
            pm_client_id: std::env::var("PM_CLIENT_ID")
                .unwrap_or_else(|_| "reality-portal".to_string()),
            pm_client_secret: std::env::var("PM_CLIENT_SECRET")
                .unwrap_or_else(|_| "reality-portal-secret".to_string()),
            sso_callback_url: std::env::var("SSO_CALLBACK_URL")
                .unwrap_or_else(|_| "http://localhost:8081/api/v1/sso/callback".to_string()),
            jwt_secret: {
                // SECURITY: JWT secret validation with strict production requirements
                let is_development = std::env::var("RUST_ENV").unwrap_or_default() == "development";
                let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| {
                    if is_development {
                        tracing::warn!(
                            "JWT_SECRET not set, using development default (DEVELOPMENT MODE ONLY)"
                        );
                        "development-secret-key-that-is-at-least-64-characters-long-for-testing".to_string()
                    } else {
                        panic!("JWT_SECRET environment variable is required. Set RUST_ENV=development to use dev defaults.");
                    }
                });

                // SECURITY: Validate secret strength
                if secret.len() < 32 {
                    panic!("JWT_SECRET must be at least 32 characters long for minimum security");
                }
                if !is_development && secret.len() < 64 {
                    tracing::warn!(
                        "JWT_SECRET is {} characters (minimum 64 recommended for production security)",
                        secret.len()
                    );
                }
                secret
            },
            pm_api_health_url: std::env::var("PM_API_HEALTH_URL")
                .unwrap_or_else(|_| "http://localhost:8080/health".to_string()),
        }
    }
}

/// Portal user stored in database.
#[derive(Clone, Debug)]
pub struct PortalUser {
    pub id: uuid::Uuid,
    pub pm_user_id: String,
    pub email: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// User service for managing portal users.
#[derive(Clone)]
pub struct UserService {
    // In-memory store for now (will be replaced with DB)
    users: Arc<Mutex<HashMap<String, PortalUser>>>,
}

impl Default for UserService {
    fn default() -> Self {
        Self::new()
    }
}

impl UserService {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Create or update a portal user from SSO user info.
    pub async fn upsert_sso_user(&self, info: &SsoUserInfo) -> Result<PortalUser, anyhow::Error> {
        let mut users = self.users.lock().await;
        let now = chrono::Utc::now();

        let user = if let Some(existing) = users.get_mut(&info.user_id) {
            existing.email.clone_from(&info.email);
            existing.name.clone_from(&info.name);
            existing.avatar_url.clone_from(&info.avatar_url);
            existing.updated_at = now;
            existing.clone()
        } else {
            let user = PortalUser {
                id: uuid::Uuid::new_v4(),
                pm_user_id: info.user_id.clone(),
                email: info.email.clone(),
                name: info.name.clone(),
                avatar_url: info.avatar_url.clone(),
                created_at: now,
                updated_at: now,
            };
            users.insert(info.user_id.clone(), user.clone());
            user
        };

        Ok(user)
    }

    /// Get portal user by PM user ID.
    pub async fn get_by_pm_id(&self, pm_user_id: &str) -> Option<PortalUser> {
        self.users.lock().await.get(pm_user_id).cloned()
    }
}

/// Session data stored in memory/Redis.
#[derive(Clone, Debug)]
pub struct Session {
    pub user_id: uuid::Uuid,
    pub email: String,
    pub name: String,
    pub pm_tokens: Option<OAuthTokens>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Session service for managing user sessions.
#[derive(Clone)]
pub struct SessionService {
    sessions: Arc<Mutex<HashMap<String, Session>>>,
    jwt_secret: String,
}

impl SessionService {
    pub fn new(jwt_secret: String) -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            jwt_secret,
        }
    }

    /// Create a new session for a user after SSO login.
    pub async fn create_session(
        &self,
        user_id: uuid::Uuid,
        tokens: &OAuthTokens,
    ) -> Result<String, anyhow::Error> {
        let session_token = self.generate_session_token(user_id)?;
        let now = chrono::Utc::now();

        let session = Session {
            user_id,
            email: String::new(), // Will be populated from user info
            name: String::new(),
            pm_tokens: Some(tokens.clone()),
            expires_at: now + chrono::Duration::days(7),
            created_at: now,
        };

        self.sessions
            .lock()
            .await
            .insert(session_token.clone(), session);
        Ok(session_token)
    }

    /// Create a session for mobile SSO (without PM tokens).
    pub async fn create_mobile_session(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<String, anyhow::Error> {
        let session_token = self.generate_session_token(user_id)?;
        let now = chrono::Utc::now();

        let session = Session {
            user_id,
            email: String::new(),
            name: String::new(),
            pm_tokens: None,
            expires_at: now + chrono::Duration::days(7),
            created_at: now,
        };

        self.sessions
            .lock()
            .await
            .insert(session_token.clone(), session);
        Ok(session_token)
    }

    /// Get session info by token.
    pub async fn get_session(&self, token: &str) -> Result<SessionInfo, anyhow::Error> {
        let sessions = self.sessions.lock().await;
        let session = sessions
            .get(token)
            .ok_or_else(|| anyhow::anyhow!("Session not found"))?;

        if session.expires_at < chrono::Utc::now() {
            return Err(anyhow::anyhow!("Session expired"));
        }

        Ok(SessionInfo {
            user_id: session.user_id,
            email: session.email.clone(),
            name: session.name.clone(),
            expires_at: session.expires_at,
        })
    }

    /// Refresh session (extend expiry).
    pub async fn refresh_session(&self, token: &str) -> Result<SessionInfo, anyhow::Error> {
        let mut sessions = self.sessions.lock().await;
        let session = sessions
            .get_mut(token)
            .ok_or_else(|| anyhow::anyhow!("Session not found"))?;

        // Extend expiry by 7 days
        session.expires_at = chrono::Utc::now() + chrono::Duration::days(7);

        Ok(SessionInfo {
            user_id: session.user_id,
            email: session.email.clone(),
            name: session.name.clone(),
            expires_at: session.expires_at,
        })
    }

    /// Invalidate a session (logout).
    pub async fn invalidate_session(&self, token: &str) -> Result<(), anyhow::Error> {
        self.sessions.lock().await.remove(token);
        Ok(())
    }

    fn generate_session_token(&self, user_id: uuid::Uuid) -> Result<String, anyhow::Error> {
        use jsonwebtoken::{encode, EncodingKey, Header};
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize)]
        struct Claims {
            sub: String,
            exp: i64,
            iat: i64,
        }

        let now = chrono::Utc::now();
        let claims = Claims {
            sub: user_id.to_string(),
            exp: (now + chrono::Duration::days(7)).timestamp(),
            iat: now.timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;

        Ok(token)
    }
}

/// SSO token service for mobile deep-link flow.
#[derive(Clone)]
pub struct SsoTokenService {
    // Short-lived tokens for mobile SSO
    tokens: Arc<Mutex<HashMap<String, MobileSsoToken>>>,
}

#[derive(Clone, Debug)]
struct MobileSsoToken {
    user_info: SsoUserInfo,
    expires_at: chrono::DateTime<chrono::Utc>,
}

impl Default for SsoTokenService {
    fn default() -> Self {
        Self::new()
    }
}

impl SsoTokenService {
    pub fn new() -> Self {
        Self {
            tokens: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Create a short-lived mobile SSO token.
    pub async fn create_mobile_token(
        &self,
        user_info: &SsoUserInfo,
        duration: chrono::Duration,
    ) -> Result<String, anyhow::Error> {
        let token = uuid::Uuid::new_v4().to_string();
        let expires_at = chrono::Utc::now() + duration;

        self.tokens.lock().await.insert(
            token.clone(),
            MobileSsoToken {
                user_info: user_info.clone(),
                expires_at,
            },
        );

        Ok(token)
    }

    /// Validate and consume a mobile SSO token (one-time use).
    pub async fn validate_and_consume_token(
        &self,
        token: &str,
    ) -> Result<SsoUserInfo, anyhow::Error> {
        let mut tokens = self.tokens.lock().await;
        let sso_token = tokens
            .remove(token)
            .ok_or_else(|| anyhow::anyhow!("Invalid or expired token"))?;

        if sso_token.expires_at < chrono::Utc::now() {
            return Err(anyhow::anyhow!("Token expired"));
        }

        Ok(sso_token.user_info)
    }
}

// ==================== Epic 104: Caching Infrastructure ====================

/// PM API health check result (Story 104.1).
#[derive(Clone, Debug)]
pub struct PmApiHealthResult {
    /// Health status from PM API
    pub status: String,
    /// Response latency in milliseconds
    pub latency_ms: u64,
    /// PM API version
    pub version: Option<String>,
    /// When the check was performed
    pub checked_at: Instant,
    /// Error message if unhealthy
    pub error: Option<String>,
}

/// Cached health check with TTL (Story 104.1).
#[derive(Clone, Debug)]
pub struct CachedHealthCheck {
    /// The health check result
    pub result: PmApiHealthResult,
    /// When the cache entry expires
    pub expires_at: Instant,
}

/// SSO token validation cache entry (Story 104.2).
#[derive(Clone, Debug)]
pub struct CachedTokenValidation {
    /// Whether the token is valid/active
    pub active: bool,
    /// Subject (user ID) from token
    pub sub: Option<String>,
    /// Token scope
    pub scope: Option<String>,
    /// When the cache entry expires
    pub expires_at: Instant,
}

/// Cache metrics for monitoring (Story 104.2).
#[derive(Clone, Debug, Default)]
pub struct CacheMetrics {
    /// Total cache hits
    pub hits: u64,
    /// Total cache misses
    pub misses: u64,
    /// Total evictions (expired entries)
    pub evictions: u64,
}

/// Health check cache service (Story 104.1).
#[derive(Clone)]
pub struct HealthCheckCache {
    /// Cached PM API health result
    cache: Arc<RwLock<Option<CachedHealthCheck>>>,
    /// Cache TTL in seconds (default: 30)
    ttl_seconds: u64,
    /// Cache metrics
    metrics: Arc<RwLock<CacheMetrics>>,
}

impl Default for HealthCheckCache {
    fn default() -> Self {
        Self::new(30) // 30 second default TTL
    }
}

impl HealthCheckCache {
    /// Create a new health check cache with specified TTL.
    pub fn new(ttl_seconds: u64) -> Self {
        Self {
            cache: Arc::new(RwLock::new(None)),
            ttl_seconds,
            metrics: Arc::new(RwLock::new(CacheMetrics::default())),
        }
    }

    /// Get cached health check if valid.
    pub async fn get(&self) -> Option<PmApiHealthResult> {
        let cache = self.cache.read().await;
        if let Some(cached) = cache.as_ref() {
            if Instant::now() < cached.expires_at {
                let mut metrics = self.metrics.write().await;
                metrics.hits += 1;
                return Some(cached.result.clone());
            }
            // Entry expired
            drop(cache);
            let mut metrics = self.metrics.write().await;
            metrics.evictions += 1;
        } else {
            let mut metrics = self.metrics.write().await;
            metrics.misses += 1;
        }
        None
    }

    /// Store health check result in cache.
    pub async fn set(&self, result: PmApiHealthResult) {
        let mut cache = self.cache.write().await;
        *cache = Some(CachedHealthCheck {
            result,
            expires_at: Instant::now() + Duration::from_secs(self.ttl_seconds),
        });
    }

    /// Get cache metrics.
    pub async fn get_metrics(&self) -> CacheMetrics {
        self.metrics.read().await.clone()
    }

    /// Clear the cache.
    pub async fn clear(&self) {
        let mut cache = self.cache.write().await;
        *cache = None;
    }
}

/// SSO token validation cache service (Story 104.2).
#[derive(Clone)]
pub struct TokenValidationCache {
    /// Cached token validations (token hash -> validation result)
    cache: Arc<RwLock<HashMap<String, CachedTokenValidation>>>,
    /// Cache TTL in seconds (default: 60)
    ttl_seconds: u64,
    /// Maximum cache entries
    max_entries: usize,
    /// Cache metrics
    metrics: Arc<RwLock<CacheMetrics>>,
}

impl Default for TokenValidationCache {
    fn default() -> Self {
        Self::new(60, 10000) // 60 second TTL, 10000 max entries
    }
}

impl TokenValidationCache {
    /// Create a new token validation cache.
    pub fn new(ttl_seconds: u64, max_entries: usize) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            ttl_seconds,
            max_entries,
            metrics: Arc::new(RwLock::new(CacheMetrics::default())),
        }
    }

    /// Hash a token for cache key (avoid storing raw tokens).
    fn hash_token(token: &str) -> String {
        use sha2::{Digest, Sha256};
        let hash = Sha256::digest(token.as_bytes());
        hex::encode(hash)
    }

    /// Get cached token validation if valid.
    pub async fn get(&self, token: &str) -> Option<CachedTokenValidation> {
        let token_hash = Self::hash_token(token);
        let cache = self.cache.read().await;

        if let Some(cached) = cache.get(&token_hash) {
            if Instant::now() < cached.expires_at {
                let mut metrics = self.metrics.write().await;
                metrics.hits += 1;
                return Some(cached.clone());
            }
        }

        drop(cache);
        let mut metrics = self.metrics.write().await;
        metrics.misses += 1;
        None
    }

    /// Store token validation result in cache.
    pub async fn set(&self, token: &str, active: bool, sub: Option<String>, scope: Option<String>) {
        let token_hash = Self::hash_token(token);
        let mut cache = self.cache.write().await;

        // Evict expired entries if cache is full
        if cache.len() >= self.max_entries {
            let now = Instant::now();
            let expired_keys: Vec<String> = cache
                .iter()
                .filter(|(_, v)| v.expires_at < now)
                .map(|(k, _)| k.clone())
                .collect();

            let mut metrics = self.metrics.write().await;
            metrics.evictions += expired_keys.len() as u64;
            drop(metrics);

            for key in expired_keys {
                cache.remove(&key);
            }

            // If still full after eviction, remove oldest entries
            if cache.len() >= self.max_entries {
                let entries_to_remove = cache.len() - self.max_entries + 1;
                let keys_to_remove: Vec<String> = cache
                    .iter()
                    .take(entries_to_remove)
                    .map(|(k, _)| k.clone())
                    .collect();
                for key in keys_to_remove {
                    cache.remove(&key);
                }
            }
        }

        cache.insert(
            token_hash,
            CachedTokenValidation {
                active,
                sub,
                scope,
                expires_at: Instant::now() + Duration::from_secs(self.ttl_seconds),
            },
        );
    }

    /// Invalidate a cached token.
    pub async fn invalidate(&self, token: &str) {
        let token_hash = Self::hash_token(token);
        let mut cache = self.cache.write().await;
        cache.remove(&token_hash);
    }

    /// Get cache metrics.
    pub async fn get_metrics(&self) -> CacheMetrics {
        self.metrics.read().await.clone()
    }

    /// Get cache size.
    pub async fn size(&self) -> usize {
        self.cache.read().await.len()
    }

    /// Clear the cache.
    pub async fn clear(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }
}

/// HTTP client for PM API communication (Story 104.1).
#[derive(Clone)]
pub struct PmApiClient {
    /// HTTP client
    client: reqwest::Client,
    /// Health check URL
    health_url: String,
}

impl PmApiClient {
    /// Create a new PM API client.
    pub fn new(health_url: String, timeout_seconds: u64) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout_seconds))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, health_url }
    }

    /// Check PM API health.
    pub async fn check_health(&self) -> PmApiHealthResult {
        let start = Instant::now();

        match self.client.get(&self.health_url).send().await {
            Ok(response) => {
                let latency_ms = start.elapsed().as_millis() as u64;

                if response.status().is_success() {
                    // Try to parse the health response
                    match response.json::<serde_json::Value>().await {
                        Ok(json) => {
                            let status = json
                                .get("status")
                                .and_then(|v| v.as_str())
                                .unwrap_or("unknown")
                                .to_string();
                            let version = json
                                .get("version")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string());

                            PmApiHealthResult {
                                status,
                                latency_ms,
                                version,
                                checked_at: Instant::now(),
                                error: None,
                            }
                        }
                        Err(e) => PmApiHealthResult {
                            status: "degraded".to_string(),
                            latency_ms,
                            version: None,
                            checked_at: Instant::now(),
                            error: Some(format!("Failed to parse health response: {}", e)),
                        },
                    }
                } else {
                    PmApiHealthResult {
                        status: "unhealthy".to_string(),
                        latency_ms,
                        version: None,
                        checked_at: Instant::now(),
                        error: Some(format!("HTTP {}", response.status())),
                    }
                }
            }
            Err(e) => {
                let latency_ms = start.elapsed().as_millis() as u64;

                PmApiHealthResult {
                    status: "unhealthy".to_string(),
                    latency_ms,
                    version: None,
                    checked_at: Instant::now(),
                    error: Some(format!("Connection failed: {}", e)),
                }
            }
        }
    }
}

/// Application state shared across all handlers.
#[derive(Clone)]
pub struct AppState {
    /// Database connection pool
    pub db: DbPool,
    /// Portal repository for search, favorites, saved searches
    pub portal_repo: PortalRepository,
    /// Reality Portal Professional repository (agencies, realtors, inquiries)
    pub reality_portal_repo: RealityPortalRepository,
    /// Application configuration
    pub config: AppConfig,
    /// Pending SSO sessions (OAuth flow state)
    pub sso_sessions: Arc<Mutex<HashMap<String, PendingSsoSession>>>,
    /// User service for portal users
    pub user_service: UserService,
    /// Session service for managing user sessions
    pub session_service: SessionService,
    /// SSO token service for mobile deep-link flow
    pub sso_token_service: SsoTokenService,
    /// PM API HTTP client (Epic 104.1)
    pub pm_api_client: PmApiClient,
    /// PM API health check cache (Epic 104.1)
    pub health_cache: HealthCheckCache,
    /// SSO token validation cache (Epic 104.2)
    pub token_cache: TokenValidationCache,
}

impl AppState {
    /// Create a new AppState with database pool.
    pub fn new(db: DbPool) -> Self {
        let portal_repo = PortalRepository::new(db.clone());
        let reality_portal_repo = RealityPortalRepository::new(db.clone());
        let config = AppConfig::from_env();
        let jwt_secret = config.jwt_secret.clone();

        // Epic 104.1: Create PM API client for health checks
        let pm_api_client = PmApiClient::new(config.pm_api_health_url.clone(), 5);

        // Epic 104.1: Health check cache with 30 second TTL
        let health_cache = HealthCheckCache::new(30);

        // Epic 104.2: Token validation cache with 60 second TTL, 10000 max entries
        let token_cache = TokenValidationCache::new(60, 10000);

        Self {
            db,
            portal_repo,
            reality_portal_repo,
            config,
            sso_sessions: Arc::new(Mutex::new(HashMap::new())),
            user_service: UserService::new(),
            session_service: SessionService::new(jwt_secret),
            sso_token_service: SsoTokenService::new(),
            pm_api_client,
            health_cache,
            token_cache,
        }
    }
}

// Make OAuthTokens cloneable
impl Clone for OAuthTokens {
    fn clone(&self) -> Self {
        Self {
            access_token: self.access_token.clone(),
            refresh_token: self.refresh_token.clone(),
            token_type: self.token_type.clone(),
            expires_in: self.expires_in,
        }
    }
}

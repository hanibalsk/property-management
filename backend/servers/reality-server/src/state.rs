//! Application state for Reality Server.
//!
//! Contains shared services and configuration for SSO, user management, and portal repositories.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use db::{repositories::PortalRepository, DbPool};

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
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "development-secret-change-in-production".to_string()),
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

/// Application state shared across all handlers.
#[derive(Clone)]
pub struct AppState {
    /// Database connection pool
    pub db: DbPool,
    /// Portal repository for search, favorites, saved searches
    pub portal_repo: PortalRepository,
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
}

impl AppState {
    /// Create a new AppState with database pool.
    pub fn new(db: DbPool) -> Self {
        let portal_repo = PortalRepository::new(db.clone());
        let config = AppConfig::from_env();
        let jwt_secret = config.jwt_secret.clone();

        Self {
            db,
            portal_repo,
            config,
            sso_sessions: Arc::new(Mutex::new(HashMap::new())),
            user_service: UserService::new(),
            session_service: SessionService::new(jwt_secret),
            sso_token_service: SsoTokenService::new(),
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

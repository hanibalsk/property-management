//! SSO routes for Reality Portal (Epic 10A-SSO).
//!
//! Implements OIDC consumer flow to authenticate users via Property Management OAuth provider.
//! Supports both web-based authorization code flow and mobile deep-link token flow.

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::state::AppState;

/// Create SSO router.
pub fn router() -> Router<AppState> {
    Router::new()
        // Web SSO flow
        .route("/login", get(sso_login))
        .route("/callback", get(sso_callback))
        .route("/logout", post(sso_logout))
        // Mobile deep-link SSO
        .route("/mobile/token", post(create_mobile_sso_token))
        .route("/mobile/validate", post(validate_mobile_sso_token))
        // Session management
        .route("/session", get(get_session))
        .route("/refresh", post(refresh_session))
}

// ==================== Web SSO Flow ====================

/// SSO login query parameters.
#[derive(Debug, Deserialize, ToSchema)]
pub struct SsoLoginQuery {
    /// Where to redirect after successful login
    pub redirect_uri: Option<String>,
    /// Optional state for CSRF protection
    pub state: Option<String>,
}

/// Initiate SSO login - redirects to PM OAuth provider.
#[utoipa::path(
    get,
    path = "/api/v1/sso/login",
    tag = "SSO",
    params(
        ("redirect_uri" = Option<String>, Query, description = "Post-login redirect URI"),
        ("state" = Option<String>, Query, description = "CSRF state token")
    ),
    responses(
        (status = 302, description = "Redirect to PM OAuth authorize endpoint")
    )
)]
pub async fn sso_login(
    State(state): State<AppState>,
    Query(params): Query<SsoLoginQuery>,
) -> impl IntoResponse {
    // Generate PKCE code verifier and challenge
    let code_verifier = generate_code_verifier();
    let code_challenge = generate_code_challenge(&code_verifier);

    // Store code verifier in session (to be retrieved in callback)
    let session_id = uuid::Uuid::new_v4().to_string();

    // Store session data for callback
    state.sso_sessions.lock().await.insert(
        session_id.clone(),
        PendingSsoSession {
            code_verifier,
            redirect_uri: params.redirect_uri.clone(),
            state: params.state.clone(),
            created_at: chrono::Utc::now(),
        },
    );

    // Build OAuth authorize URL
    let oauth_authorize_url = format!(
        "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}&code_challenge={}&code_challenge_method=S256",
        state.config.pm_oauth_authorize_url,
        state.config.pm_client_id,
        urlencoding::encode(&state.config.sso_callback_url),
        urlencoding::encode("profile email"),
        urlencoding::encode(&session_id),
        urlencoding::encode(&code_challenge),
    );

    Redirect::temporary(&oauth_authorize_url)
}

/// SSO callback query parameters.
#[derive(Debug, Deserialize, ToSchema)]
pub struct SsoCallbackQuery {
    /// Authorization code from PM OAuth
    pub code: Option<String>,
    /// State (session ID) for CSRF verification
    pub state: Option<String>,
    /// Error code if authorization failed
    pub error: Option<String>,
    /// Error description
    pub error_description: Option<String>,
}

/// SSO callback - exchanges authorization code for tokens.
#[utoipa::path(
    get,
    path = "/api/v1/sso/callback",
    tag = "SSO",
    params(
        ("code" = Option<String>, Query, description = "Authorization code"),
        ("state" = Option<String>, Query, description = "State token"),
        ("error" = Option<String>, Query, description = "Error code"),
        ("error_description" = Option<String>, Query, description = "Error description")
    ),
    responses(
        (status = 302, description = "Redirect to original destination with session"),
        (status = 400, description = "Invalid callback parameters", body = SsoError)
    )
)]
pub async fn sso_callback(
    State(state): State<AppState>,
    Query(params): Query<SsoCallbackQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<SsoError>)> {
    // Check for OAuth errors
    if let Some(error) = params.error {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(SsoError {
                error,
                error_description: params.error_description,
            }),
        ));
    }

    // Validate required parameters
    let code = params.code.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(SsoError::new("missing_code", "Authorization code required")),
        )
    })?;

    let session_id = params.state.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(SsoError::new("missing_state", "State parameter required")),
        )
    })?;

    // Retrieve and remove pending session
    let pending_session = state
        .sso_sessions
        .lock()
        .await
        .remove(&session_id)
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                Json(SsoError::new("invalid_state", "Invalid or expired session")),
            )
        })?;

    // Check session expiry (10 minutes max)
    if chrono::Utc::now() - pending_session.created_at > chrono::Duration::minutes(10) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(SsoError::new("session_expired", "SSO session expired")),
        ));
    }

    // Exchange code for tokens with PM OAuth server
    let tokens = exchange_code_for_tokens(&state, &code, &pending_session.code_verifier)
        .await
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(SsoError::new("token_exchange_failed", &e.to_string())),
            )
        })?;

    // Get user info from PM
    let user_info = get_user_info(&state, &tokens.access_token)
        .await
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(SsoError::new("user_info_failed", &e.to_string())),
            )
        })?;

    // Create or update local portal user
    let portal_user = state
        .user_service
        .upsert_sso_user(&user_info)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SsoError::new("user_create_failed", &e.to_string())),
            )
        })?;

    // Create portal session
    let session_token = state
        .session_service
        .create_session(portal_user.id, &tokens)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SsoError::new("session_create_failed", &e.to_string())),
            )
        })?;

    // Build redirect URL with session cookie
    let redirect_uri = pending_session
        .redirect_uri
        .unwrap_or_else(|| "/".to_string());

    // Set session cookie and redirect
    Ok((
        [(
            axum::http::header::SET_COOKIE,
            format!(
                "portal_session={}; Path=/; HttpOnly; Secure; SameSite=Lax; Max-Age={}",
                session_token,
                7 * 24 * 60 * 60 // 7 days
            ),
        )],
        Redirect::temporary(&redirect_uri),
    ))
}

/// Logout from SSO session.
#[utoipa::path(
    post,
    path = "/api/v1/sso/logout",
    tag = "SSO",
    responses(
        (status = 200, description = "Logged out successfully"),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn sso_logout(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<impl IntoResponse, StatusCode> {
    // Extract session token from cookie
    let session_token = extract_session_cookie(&headers).ok_or(StatusCode::UNAUTHORIZED)?;

    // Invalidate session
    state
        .session_service
        .invalidate_session(&session_token)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Clear session cookie
    Ok((
        [(
            axum::http::header::SET_COOKIE,
            "portal_session=; Path=/; HttpOnly; Secure; SameSite=Lax; Max-Age=0".to_string(),
        )],
        StatusCode::OK,
    ))
}

// ==================== Mobile Deep-Link SSO ====================

/// Request to create mobile SSO token.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateMobileSsoTokenRequest {
    /// PM access token to verify user identity
    pub pm_access_token: String,
}

/// Mobile SSO token response.
#[derive(Debug, Serialize, ToSchema)]
pub struct MobileSsoTokenResponse {
    /// Short-lived SSO token for deep-link
    pub sso_token: String,
    /// Token expiry in seconds (5 minutes)
    pub expires_in: u64,
    /// Deep-link URL format
    pub deep_link: String,
}

/// Create a short-lived mobile SSO token.
#[utoipa::path(
    post,
    path = "/api/v1/sso/mobile/token",
    tag = "SSO",
    request_body = CreateMobileSsoTokenRequest,
    responses(
        (status = 200, description = "SSO token created", body = MobileSsoTokenResponse),
        (status = 401, description = "Invalid PM token", body = SsoError)
    )
)]
pub async fn create_mobile_sso_token(
    State(state): State<AppState>,
    Json(request): Json<CreateMobileSsoTokenRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<SsoError>)> {
    // Validate PM access token by introspecting it
    let token_info = introspect_pm_token(&state, &request.pm_access_token)
        .await
        .map_err(|e| {
            (
                StatusCode::UNAUTHORIZED,
                Json(SsoError::new("invalid_token", &e.to_string())),
            )
        })?;

    if !token_info.active {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(SsoError::new("token_inactive", "PM token is not active")),
        ));
    }

    // Get user info
    let user_info = get_user_info(&state, &request.pm_access_token)
        .await
        .map_err(|e| {
            (
                StatusCode::UNAUTHORIZED,
                Json(SsoError::new("user_info_failed", &e.to_string())),
            )
        })?;

    // Create short-lived SSO token (5 minutes, one-time use)
    let sso_token = state
        .sso_token_service
        .create_mobile_token(&user_info, chrono::Duration::minutes(5))
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SsoError::new("token_create_failed", &e.to_string())),
            )
        })?;

    Ok(Json(MobileSsoTokenResponse {
        sso_token: sso_token.clone(),
        expires_in: 300, // 5 minutes
        deep_link: format!("reality://sso?token={}", urlencoding::encode(&sso_token)),
    }))
}

/// Request to validate mobile SSO token.
#[derive(Debug, Deserialize, ToSchema)]
pub struct ValidateMobileSsoTokenRequest {
    /// SSO token from deep-link
    pub sso_token: String,
}

/// Session response after mobile SSO validation.
#[derive(Debug, Serialize, ToSchema)]
pub struct SessionResponse {
    /// Session token for API authentication
    pub session_token: String,
    /// User information
    pub user: SsoUserInfo,
    /// Session expiry in seconds
    pub expires_in: u64,
}

/// Validate mobile SSO token and create session.
#[utoipa::path(
    post,
    path = "/api/v1/sso/mobile/validate",
    tag = "SSO",
    request_body = ValidateMobileSsoTokenRequest,
    responses(
        (status = 200, description = "Session created", body = SessionResponse),
        (status = 401, description = "Invalid or expired token", body = SsoError)
    )
)]
pub async fn validate_mobile_sso_token(
    State(state): State<AppState>,
    Json(request): Json<ValidateMobileSsoTokenRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<SsoError>)> {
    // Validate and consume SSO token (one-time use)
    let user_info = state
        .sso_token_service
        .validate_and_consume_token(&request.sso_token)
        .await
        .map_err(|e| {
            (
                StatusCode::UNAUTHORIZED,
                Json(SsoError::new("invalid_token", &e.to_string())),
            )
        })?;

    // Create or update portal user
    let portal_user = state
        .user_service
        .upsert_sso_user(&user_info)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SsoError::new("user_create_failed", &e.to_string())),
            )
        })?;

    // Create session
    let session_token = state
        .session_service
        .create_mobile_session(portal_user.id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SsoError::new("session_create_failed", &e.to_string())),
            )
        })?;

    Ok(Json(SessionResponse {
        session_token,
        user: user_info,
        expires_in: 7 * 24 * 60 * 60, // 7 days
    }))
}

// ==================== Session Management ====================

/// Get current session information.
#[utoipa::path(
    get,
    path = "/api/v1/sso/session",
    tag = "SSO",
    responses(
        (status = 200, description = "Session info", body = SessionInfo),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn get_session(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<impl IntoResponse, StatusCode> {
    let session_token = extract_session_token(&headers).ok_or(StatusCode::UNAUTHORIZED)?;

    let session_info = state
        .session_service
        .get_session(&session_token)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(Json(session_info))
}

/// Refresh session with PM tokens.
#[utoipa::path(
    post,
    path = "/api/v1/sso/refresh",
    tag = "SSO",
    responses(
        (status = 200, description = "Session refreshed", body = SessionInfo),
        (status = 401, description = "Session expired or invalid")
    )
)]
pub async fn refresh_session(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<impl IntoResponse, StatusCode> {
    let session_token = extract_session_token(&headers).ok_or(StatusCode::UNAUTHORIZED)?;

    let session_info = state
        .session_service
        .refresh_session(&session_token)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(Json(session_info))
}

// ==================== Types ====================

/// SSO error response.
#[derive(Debug, Serialize, ToSchema)]
pub struct SsoError {
    pub error: String,
    pub error_description: Option<String>,
}

impl SsoError {
    pub fn new(error: &str, description: &str) -> Self {
        Self {
            error: error.to_string(),
            error_description: Some(description.to_string()),
        }
    }
}

/// User info from PM OAuth.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SsoUserInfo {
    pub user_id: String,
    pub email: String,
    pub name: String,
    pub avatar_url: Option<String>,
}

/// Session information.
#[derive(Debug, Serialize, ToSchema)]
pub struct SessionInfo {
    pub user_id: uuid::Uuid,
    pub email: String,
    pub name: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

/// Pending SSO session for PKCE flow.
#[derive(Debug)]
pub struct PendingSsoSession {
    pub code_verifier: String,
    pub redirect_uri: Option<String>,
    pub state: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Token response from PM OAuth.
#[derive(Debug, Deserialize)]
pub struct OAuthTokens {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub token_type: String,
    pub expires_in: u64,
}

/// Token introspection response.
#[derive(Debug, Deserialize)]
pub struct TokenIntrospectionResponse {
    pub active: bool,
    pub sub: Option<String>,
    pub client_id: Option<String>,
    pub scope: Option<String>,
}

// ==================== Helper Functions ====================

/// Generate PKCE code verifier (43-128 chars).
fn generate_code_verifier() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
    base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, &bytes)
}

/// Generate PKCE code challenge from verifier.
fn generate_code_challenge(verifier: &str) -> String {
    use sha2::{Digest, Sha256};
    let hash = Sha256::digest(verifier.as_bytes());
    base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, hash)
}

/// Exchange authorization code for tokens.
async fn exchange_code_for_tokens(
    state: &AppState,
    code: &str,
    code_verifier: &str,
) -> Result<OAuthTokens, anyhow::Error> {
    let client = reqwest::Client::new();
    let response = client
        .post(&state.config.pm_oauth_token_url)
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", &state.config.sso_callback_url),
            ("client_id", &state.config.pm_client_id),
            ("client_secret", &state.config.pm_client_secret),
            ("code_verifier", code_verifier),
        ])
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(anyhow::anyhow!("Token exchange failed: {}", error_text));
    }

    Ok(response.json().await?)
}

/// Get user info from PM OAuth server.
async fn get_user_info(state: &AppState, access_token: &str) -> Result<SsoUserInfo, anyhow::Error> {
    let client = reqwest::Client::new();
    let response = client
        .get(&state.config.pm_userinfo_url)
        .bearer_auth(access_token)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(anyhow::anyhow!("Failed to get user info: {}", error_text));
    }

    Ok(response.json().await?)
}

/// Introspect PM token.
async fn introspect_pm_token(
    state: &AppState,
    token: &str,
) -> Result<TokenIntrospectionResponse, anyhow::Error> {
    let client = reqwest::Client::new();
    let response = client
        .post(&state.config.pm_introspect_url)
        .form(&[
            ("token", token),
            ("client_id", &state.config.pm_client_id),
            ("client_secret", &state.config.pm_client_secret),
        ])
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(anyhow::anyhow!(
            "Token introspection failed: {}",
            error_text
        ));
    }

    Ok(response.json().await?)
}

/// Extract session cookie from headers.
fn extract_session_cookie(headers: &axum::http::HeaderMap) -> Option<String> {
    headers
        .get(axum::http::header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|cookies| {
            cookies
                .split(';')
                .find(|c| c.trim().starts_with("portal_session="))
                .map(|c| {
                    c.trim()
                        .strip_prefix("portal_session=")
                        .unwrap()
                        .to_string()
                })
        })
}

/// Extract session token from Authorization header or cookie.
fn extract_session_token(headers: &axum::http::HeaderMap) -> Option<String> {
    // Try Authorization header first
    if let Some(auth) = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
    {
        if let Some(token) = auth.strip_prefix("Bearer ") {
            return Some(token.to_string());
        }
    }

    // Fall back to cookie
    extract_session_cookie(headers)
}

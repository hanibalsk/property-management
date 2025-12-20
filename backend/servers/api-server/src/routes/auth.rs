//! Authentication routes (UC-14, Epic 1).

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use common::errors::{ErrorResponse, ValidationError};
use db::models::{CreateUser, Locale};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::services::AuthService;
use crate::state::AppState;

/// Create auth router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/verify-email", get(verify_email))
        .route("/resend-verification", post(resend_verification))
        .route("/login", post(login))
        .route("/refresh", post(refresh_token))
        .route("/logout", post(logout))
        .route("/forgot-password", post(forgot_password))
        .route("/reset-password", post(reset_password))
}

// ==================== Register (Story 1.1) ====================

/// Register request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterRequest {
    /// Email address
    pub email: String,
    /// Password (min 8 characters, 1 uppercase, 1 number)
    pub password: String,
    /// Display name
    pub name: String,
    /// Phone number (optional)
    pub phone: Option<String>,
    /// Preferred locale (sk, cs, de, en)
    pub locale: Option<String>,
}

/// Register response.
#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterResponse {
    /// Success message
    pub message: String,
    /// User ID
    pub user_id: String,
}

/// Register endpoint.
#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    tag = "Authentication",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "Registration successful", body = RegisterResponse),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 409, description = "Email already exists", body = ErrorResponse)
    )
)]
pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    // Validate email format
    if !AuthService::validate_email(&req.email) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new("INVALID_EMAIL", "Invalid email format")),
        ));
    }

    // Validate password requirements
    if let Err(errors) = AuthService::validate_password(&req.password) {
        let details: Vec<ValidationError> = errors
            .into_iter()
            .map(|msg| ValidationError {
                field: "password".to_string(),
                message: msg.clone(),
                code: "INVALID_PASSWORD".to_string(),
            })
            .collect();
        return Err((
            StatusCode::BAD_REQUEST,
            Json(
                ErrorResponse::new("VALIDATION_ERROR", "Password does not meet requirements")
                    .with_details(details),
            ),
        ));
    }

    // Validate name is not empty
    if req.name.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new("INVALID_NAME", "Name cannot be empty")),
        ));
    }

    // Check if email already exists
    match state.user_repo.email_exists(&req.email).await {
        Ok(true) => {
            // Don't reveal whether account is verified or not
            return Err((
                StatusCode::CONFLICT,
                Json(ErrorResponse::new(
                    "EMAIL_EXISTS",
                    "An account with this email already exists",
                )),
            ));
        }
        Ok(false) => {}
        Err(e) => {
            tracing::error!(error = %e, "Database error checking email");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("DATABASE_ERROR", "Failed to check email")),
            ));
        }
    }

    // Hash password
    let password_hash = match state.auth_service.hash_password(&req.password) {
        Ok(hash) => hash,
        Err(e) => {
            tracing::error!(error = %e, "Failed to hash password");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to process password")),
            ));
        }
    };

    // Determine locale from request or default to English
    let locale = req
        .locale
        .as_ref()
        .map(|l| Locale::from_str(l))
        .unwrap_or(Locale::English);

    // Create user
    let create_user = CreateUser {
        email: req.email.clone(),
        password_hash,
        name: req.name.clone(),
        phone: req.phone.clone(),
        locale: locale.clone(),
    };

    let user = match state.user_repo.create(create_user).await {
        Ok(user) => user,
        Err(e) => {
            tracing::error!(error = %e, email = %req.email, "Failed to create user");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("DATABASE_ERROR", "Failed to create account")),
            ));
        }
    };

    // Generate verification token
    let token = state.auth_service.generate_token();
    let token_hash = state.auth_service.hash_token(&token);

    // Store verification token
    if let Err(e) = state
        .user_repo
        .create_verification_token(user.id, &token_hash)
        .await
    {
        tracing::error!(error = %e, user_id = %user.id, "Failed to create verification token");
        // Continue anyway - user can request resend
    }

    // Send verification email
    if let Err(e) = state
        .email_service
        .send_verification_email(&user.email, &user.name, &token, &locale)
        .await
    {
        tracing::error!(error = %e, user_id = %user.id, "Failed to send verification email");
        // Continue anyway - user can request resend
    }

    tracing::info!(
        user_id = %user.id,
        email = %user.email,
        "User registered successfully"
    );

    Ok((
        StatusCode::CREATED,
        Json(RegisterResponse {
            message: "Check your email to verify your account".to_string(),
            user_id: user.id.to_string(),
        }),
    ))
}

// ==================== Verify Email (Story 1.1) ====================

/// Verify email query parameters.
#[derive(Debug, Deserialize, ToSchema)]
pub struct VerifyEmailQuery {
    /// Verification token from email
    pub token: String,
}

/// Verify email response.
#[derive(Debug, Serialize, ToSchema)]
pub struct VerifyEmailResponse {
    /// Success message
    pub message: String,
}

/// Verify email endpoint.
#[utoipa::path(
    get,
    path = "/api/v1/auth/verify-email",
    tag = "Authentication",
    params(
        ("token" = String, Query, description = "Email verification token")
    ),
    responses(
        (status = 200, description = "Email verified", body = VerifyEmailResponse),
        (status = 400, description = "Invalid or expired token", body = ErrorResponse)
    )
)]
pub async fn verify_email(
    State(state): State<AppState>,
    axum::extract::Query(query): axum::extract::Query<VerifyEmailQuery>,
) -> Result<Json<VerifyEmailResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Hash the token to look it up
    let token_hash = state.auth_service.hash_token(&query.token);

    // Find the token
    let verification_token = match state.user_repo.find_verification_token(&token_hash).await {
        Ok(Some(token)) => token,
        Ok(None) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new(
                    "INVALID_TOKEN",
                    "This verification link is invalid or has already been used",
                )),
            ));
        }
        Err(e) => {
            tracing::error!(error = %e, "Database error finding verification token");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("DATABASE_ERROR", "Failed to verify token")),
            ));
        }
    };

    // Check if token is expired
    if verification_token.is_expired() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "TOKEN_EXPIRED",
                "This verification link has expired. Please request a new one.",
            )),
        ));
    }

    // Mark token as used
    if let Err(e) = state
        .user_repo
        .use_verification_token(verification_token.id)
        .await
    {
        tracing::error!(error = %e, "Failed to mark verification token as used");
    }

    // Verify the user's email
    match state.user_repo.verify_email(verification_token.user_id).await {
        Ok(Some(user)) => {
            tracing::info!(user_id = %user.id, email = %user.email, "Email verified");
            Ok(Json(VerifyEmailResponse {
                message: "Your email has been verified. You can now log in.".to_string(),
            }))
        }
        Ok(None) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "USER_NOT_FOUND",
                "User account not found or already verified",
            )),
        )),
        Err(e) => {
            tracing::error!(error = %e, "Failed to verify email");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("DATABASE_ERROR", "Failed to verify email")),
            ))
        }
    }
}

// ==================== Resend Verification (Story 1.1) ====================

/// Resend verification request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct ResendVerificationRequest {
    /// Email address
    pub email: String,
}

/// Resend verification response.
#[derive(Debug, Serialize, ToSchema)]
pub struct ResendVerificationResponse {
    /// Success message
    pub message: String,
}

/// Resend verification email endpoint.
#[utoipa::path(
    post,
    path = "/api/v1/auth/resend-verification",
    tag = "Authentication",
    request_body = ResendVerificationRequest,
    responses(
        (status = 200, description = "Verification email sent (if account exists)", body = ResendVerificationResponse),
    )
)]
pub async fn resend_verification(
    State(state): State<AppState>,
    Json(req): Json<ResendVerificationRequest>,
) -> Json<ResendVerificationResponse> {
    // Always return success to prevent email enumeration
    let response = ResendVerificationResponse {
        message: "If an unverified account exists with this email, a verification link has been sent.".to_string(),
    };

    // Try to find user and send email
    match state.user_repo.find_by_email(&req.email).await {
        Ok(Some(user)) if user.status == "pending" => {
            // Invalidate old tokens
            let _ = state.user_repo.invalidate_user_tokens(user.id).await;

            // Generate new token
            let token = state.auth_service.generate_token();
            let token_hash = state.auth_service.hash_token(&token);

            // Store new token
            if let Err(e) = state
                .user_repo
                .create_verification_token(user.id, &token_hash)
                .await
            {
                tracing::error!(error = %e, user_id = %user.id, "Failed to create verification token");
                return Json(response);
            }

            // Send email
            if let Err(e) = state
                .email_service
                .send_verification_email(&user.email, &user.name, &token, &user.locale_enum())
                .await
            {
                tracing::error!(error = %e, user_id = %user.id, "Failed to send verification email");
            }

            tracing::info!(user_id = %user.id, "Resent verification email");
        }
        _ => {
            // User not found or already verified - don't reveal this
            tracing::debug!(email = %req.email, "Resend verification request for non-pending account");
        }
    }

    Json(response)
}

// ==================== Login (Story 1.2) ====================

/// Login request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    /// Email address
    pub email: String,
    /// Password
    pub password: String,
    /// 2FA code (optional, for future use)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_factor_code: Option<String>,
}

/// Login response.
#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    /// JWT access token
    pub access_token: String,
    /// Refresh token
    pub refresh_token: String,
    /// Access token expiration in seconds
    pub expires_in: i64,
    /// Token type (always "Bearer")
    pub token_type: String,
}

/// Login endpoint.
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "Authentication",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 429, description = "Too many failed attempts", body = ErrorResponse)
    )
)]
pub async fn login(
    State(state): State<AppState>,
    axum::extract::ConnectInfo(addr): axum::extract::ConnectInfo<std::net::SocketAddr>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<ErrorResponse>)> {
    let ip_address = addr.ip().to_string();

    // Check rate limiting
    match state.session_repo.check_rate_limit(&req.email).await {
        Ok(status) if !status.can_attempt() => {
            let remaining = status.lockout_remaining_secs.unwrap_or(900);
            tracing::warn!(
                email = %req.email,
                ip = %ip_address,
                remaining_secs = remaining,
                "Login attempt blocked due to rate limiting"
            );
            return Err((
                StatusCode::TOO_MANY_REQUESTS,
                Json(ErrorResponse::new(
                    "RATE_LIMITED",
                    format!("Too many failed login attempts. Please try again in {} minutes.", remaining / 60 + 1),
                )),
            ));
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to check rate limit");
            // Continue anyway - don't block login due to rate limit check failure
        }
        _ => {}
    }

    // Find user by email
    let user = match state.user_repo.find_by_email(&req.email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            // Record failed attempt (user not found)
            let _ = state.session_repo.record_login_attempt(&req.email, &ip_address, false).await;
            tracing::debug!(email = %req.email, "Login failed: user not found");
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse::new("INVALID_CREDENTIALS", "Invalid email or password")),
            ));
        }
        Err(e) => {
            tracing::error!(error = %e, "Database error finding user");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("DATABASE_ERROR", "Login failed")),
            ));
        }
    };

    // Check if user can log in
    if user.status == "suspended" {
        let _ = state.session_repo.record_login_attempt(&req.email, &ip_address, false).await;
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse::new(
                "ACCOUNT_SUSPENDED",
                "Account suspended. Contact support.",
            )),
        ));
    }

    if !user.is_verified() {
        let _ = state.session_repo.record_login_attempt(&req.email, &ip_address, false).await;
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse::new(
                "EMAIL_NOT_VERIFIED",
                "Please verify your email first",
            )),
        ));
    }

    // Verify password
    let password_valid = match state.auth_service.verify_password(&req.password, &user.password_hash) {
        Ok(valid) => valid,
        Err(e) => {
            tracing::error!(error = %e, "Password verification error");
            let _ = state.session_repo.record_login_attempt(&req.email, &ip_address, false).await;
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Login failed")),
            ));
        }
    };

    if !password_valid {
        let _ = state.session_repo.record_login_attempt(&req.email, &ip_address, false).await;
        tracing::debug!(email = %req.email, "Login failed: invalid password");
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse::new("INVALID_CREDENTIALS", "Invalid email or password")),
        ));
    }

    // TODO: Check 2FA if enabled (future story)
    // if user.has_2fa_enabled() && req.two_factor_code.is_none() { ... }

    // Record successful login attempt
    let _ = state.session_repo.record_login_attempt(&req.email, &ip_address, true).await;

    // Generate access token
    let access_token = match state.jwt_service.generate_access_token(
        user.id,
        &user.email,
        &user.name,
        None, // org_id - will be set when org context is selected
        None, // roles - will be set when org context is selected
    ) {
        Ok(token) => token,
        Err(e) => {
            tracing::error!(error = %e, "Failed to generate access token");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("TOKEN_ERROR", "Failed to create session")),
            ));
        }
    };

    // Generate refresh token
    let (refresh_token, token_hash, expires_at) = match state.jwt_service.generate_refresh_token(
        user.id,
        &user.email,
        &user.name,
    ) {
        Ok(result) => result,
        Err(e) => {
            tracing::error!(error = %e, "Failed to generate refresh token");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("TOKEN_ERROR", "Failed to create session")),
            ));
        }
    };

    // Store refresh token in database
    use db::models::CreateRefreshToken;
    let create_token = CreateRefreshToken {
        user_id: user.id,
        token_hash,
        expires_at,
        user_agent: None, // TODO: Extract from headers in Story 1.5
        ip_address: Some(ip_address.clone()),
        device_info: None,
    };

    if let Err(e) = state.session_repo.create_refresh_token(create_token).await {
        tracing::error!(error = %e, "Failed to store refresh token");
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::new("DATABASE_ERROR", "Failed to create session")),
        ));
    }

    tracing::info!(
        user_id = %user.id,
        email = %user.email,
        "User logged in successfully"
    );

    Ok(Json(LoginResponse {
        access_token,
        refresh_token,
        expires_in: state.jwt_service.access_token_lifetime(),
        token_type: "Bearer".to_string(),
    }))
}

// ==================== Refresh Token (Story 1.3) ====================

/// Refresh token request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct RefreshTokenRequest {
    /// The refresh token to exchange for new tokens
    pub refresh_token: String,
}

/// Refresh token endpoint.
#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    tag = "Authentication",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed", body = LoginResponse),
        (status = 401, description = "Invalid or expired refresh token", body = ErrorResponse)
    )
)]
pub async fn refresh_token(
    State(state): State<AppState>,
    Json(req): Json<RefreshTokenRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Validate the refresh token JWT
    let claims = match state.jwt_service.validate_refresh_token(&req.refresh_token) {
        Ok(claims) => claims,
        Err(e) => {
            tracing::debug!(error = %e, "Invalid refresh token");
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse::new("INVALID_TOKEN", "Invalid or expired refresh token")),
            ));
        }
    };

    // Hash the token to look it up in database
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(req.refresh_token.as_bytes());
    let token_hash = hex::encode(hasher.finalize());

    // Find the token in database
    let stored_token = match state.session_repo.find_by_token_hash(&token_hash).await {
        Ok(Some(token)) => token,
        Ok(None) => {
            tracing::warn!(user_id = %claims.sub, "Refresh token not found in database (possibly revoked)");
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse::new("TOKEN_REVOKED", "This session has been revoked")),
            ));
        }
        Err(e) => {
            tracing::error!(error = %e, "Database error finding refresh token");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("DATABASE_ERROR", "Failed to validate session")),
            ));
        }
    };

    // Check if token is still valid
    if !stored_token.is_valid() {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse::new("TOKEN_EXPIRED", "Session has expired")),
        ));
    }

    // Parse user ID from claims
    let user_id: uuid::Uuid = claims.sub.parse().map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse::new("INVALID_TOKEN", "Invalid token format")),
        )
    })?;

    // Verify user still exists and is active
    let user = match state.user_repo.find_by_id(user_id).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse::new("USER_NOT_FOUND", "User account no longer exists")),
            ));
        }
        Err(e) => {
            tracing::error!(error = %e, "Database error finding user");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("DATABASE_ERROR", "Failed to validate user")),
            ));
        }
    };

    if user.status != "active" {
        // Revoke the token since user is no longer active
        let _ = state.session_repo.revoke_token(stored_token.id).await;
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse::new("ACCOUNT_INACTIVE", "Account is no longer active")),
        ));
    }

    // Token rotation: revoke the old token
    if let Err(e) = state.session_repo.revoke_token(stored_token.id).await {
        tracing::error!(error = %e, "Failed to revoke old refresh token");
        // Continue anyway - better to issue new token than fail
    }

    // Generate new access token
    let access_token = match state.jwt_service.generate_access_token(
        user.id,
        &user.email,
        &user.name,
        None,
        None,
    ) {
        Ok(token) => token,
        Err(e) => {
            tracing::error!(error = %e, "Failed to generate access token");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("TOKEN_ERROR", "Failed to create session")),
            ));
        }
    };

    // Generate new refresh token (rotation)
    let (new_refresh_token, new_token_hash, expires_at) = match state.jwt_service.generate_refresh_token(
        user.id,
        &user.email,
        &user.name,
    ) {
        Ok(result) => result,
        Err(e) => {
            tracing::error!(error = %e, "Failed to generate refresh token");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("TOKEN_ERROR", "Failed to create session")),
            ));
        }
    };

    // Store new refresh token
    use db::models::CreateRefreshToken;
    let create_token = CreateRefreshToken {
        user_id: user.id,
        token_hash: new_token_hash,
        expires_at,
        user_agent: stored_token.user_agent.clone(),
        ip_address: stored_token.ip_address.clone(),
        device_info: stored_token.device_info.clone(),
    };

    if let Err(e) = state.session_repo.create_refresh_token(create_token).await {
        tracing::error!(error = %e, "Failed to store new refresh token");
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::new("DATABASE_ERROR", "Failed to create session")),
        ));
    }

    tracing::info!(user_id = %user.id, "Token refreshed successfully");

    Ok(Json(LoginResponse {
        access_token,
        refresh_token: new_refresh_token,
        expires_in: state.jwt_service.access_token_lifetime(),
        token_type: "Bearer".to_string(),
    }))
}

// ==================== Logout (Story 1.3) ====================

/// Logout request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct LogoutRequest {
    /// The refresh token to revoke
    pub refresh_token: String,
}

/// Logout response.
#[derive(Debug, Serialize, ToSchema)]
pub struct LogoutResponse {
    /// Success message
    pub message: String,
}

/// Logout endpoint.
#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    tag = "Authentication",
    request_body = LogoutRequest,
    responses(
        (status = 200, description = "Logout successful", body = LogoutResponse),
        (status = 401, description = "Invalid token")
    )
)]
pub async fn logout(
    State(state): State<AppState>,
    Json(req): Json<LogoutRequest>,
) -> Result<Json<LogoutResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Hash the token to look it up
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(req.refresh_token.as_bytes());
    let token_hash = hex::encode(hasher.finalize());

    // Find and revoke the token
    match state.session_repo.find_by_token_hash(&token_hash).await {
        Ok(Some(token)) => {
            if let Err(e) = state.session_repo.revoke_token(token.id).await {
                tracing::error!(error = %e, "Failed to revoke token");
            } else {
                tracing::info!(user_id = %token.user_id, "User logged out");
            }
        }
        Ok(None) => {
            // Token not found - might already be revoked, that's fine
            tracing::debug!("Logout requested for unknown/revoked token");
        }
        Err(e) => {
            tracing::error!(error = %e, "Database error during logout");
        }
    }

    // Always return success to prevent token enumeration
    Ok(Json(LogoutResponse {
        message: "Logged out successfully".to_string(),
    }))
}

// ==================== Forgot Password (Story 1.4) ====================

/// Forgot password request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct ForgotPasswordRequest {
    /// Email address
    pub email: String,
}

/// Forgot password response.
#[derive(Debug, Serialize, ToSchema)]
pub struct ForgotPasswordResponse {
    /// Success message (always same to prevent enumeration)
    pub message: String,
}

/// Forgot password endpoint - initiates password reset.
#[utoipa::path(
    post,
    path = "/api/v1/auth/forgot-password",
    tag = "Authentication",
    request_body = ForgotPasswordRequest,
    responses(
        (status = 200, description = "Password reset email sent (if account exists)", body = ForgotPasswordResponse),
    )
)]
pub async fn forgot_password(
    State(state): State<AppState>,
    Json(req): Json<ForgotPasswordRequest>,
) -> Json<ForgotPasswordResponse> {
    // Always return success to prevent email enumeration
    let response = ForgotPasswordResponse {
        message: "If an account exists with this email, a password reset link has been sent.".to_string(),
    };

    // Try to find user and send email
    match state.user_repo.find_by_email(&req.email).await {
        Ok(Some(user)) if user.status == "active" => {
            // Invalidate any existing reset tokens for this user
            let _ = state.password_reset_repo.invalidate_user_tokens(user.id).await;

            // Generate reset token (1 hour expiry)
            let token = state.auth_service.generate_token();
            let token_hash = state.auth_service.hash_token(&token);
            let expires_at = chrono::Utc::now() + chrono::Duration::hours(1);

            // Store token
            use db::models::CreatePasswordResetToken;
            let create_token = CreatePasswordResetToken {
                user_id: user.id,
                token_hash,
                expires_at,
            };

            if let Err(e) = state.password_reset_repo.create(create_token).await {
                tracing::error!(error = %e, user_id = %user.id, "Failed to create password reset token");
                return Json(response);
            }

            // Send reset email
            if let Err(e) = state
                .email_service
                .send_password_reset_email(&user.email, &user.name, &token, &user.locale_enum())
                .await
            {
                tracing::error!(error = %e, user_id = %user.id, "Failed to send password reset email");
            }

            tracing::info!(user_id = %user.id, "Password reset email sent");
        }
        Ok(Some(user)) => {
            // User exists but not active (pending/suspended/deleted)
            tracing::debug!(
                email = %req.email,
                status = %user.status,
                "Password reset requested for non-active account"
            );
        }
        Ok(None) => {
            // User not found - don't reveal this
            tracing::debug!(email = %req.email, "Password reset requested for unknown email");
        }
        Err(e) => {
            tracing::error!(error = %e, "Database error finding user for password reset");
        }
    }

    Json(response)
}

// ==================== Reset Password (Story 1.4) ====================

/// Reset password request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct ResetPasswordRequest {
    /// Reset token from email
    pub token: String,
    /// New password (min 8 characters, 1 uppercase, 1 number)
    pub new_password: String,
}

/// Reset password response.
#[derive(Debug, Serialize, ToSchema)]
pub struct ResetPasswordResponse {
    /// Success message
    pub message: String,
}

/// Reset password endpoint - completes password reset.
#[utoipa::path(
    post,
    path = "/api/v1/auth/reset-password",
    tag = "Authentication",
    request_body = ResetPasswordRequest,
    responses(
        (status = 200, description = "Password reset successful", body = ResetPasswordResponse),
        (status = 400, description = "Invalid or expired token", body = ErrorResponse),
        (status = 400, description = "Password does not meet requirements", body = ErrorResponse)
    )
)]
pub async fn reset_password(
    State(state): State<AppState>,
    Json(req): Json<ResetPasswordRequest>,
) -> Result<Json<ResetPasswordResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Validate new password requirements
    if let Err(errors) = AuthService::validate_password(&req.new_password) {
        let details: Vec<ValidationError> = errors
            .into_iter()
            .map(|msg| ValidationError {
                field: "new_password".to_string(),
                message: msg.clone(),
                code: "INVALID_PASSWORD".to_string(),
            })
            .collect();
        return Err((
            StatusCode::BAD_REQUEST,
            Json(
                ErrorResponse::new("VALIDATION_ERROR", "Password does not meet requirements")
                    .with_details(details),
            ),
        ));
    }

    // Hash the token to look it up
    let token_hash = state.auth_service.hash_token(&req.token);

    // Find the reset token
    let reset_token = match state.password_reset_repo.find_by_token_hash(&token_hash).await {
        Ok(Some(token)) => token,
        Ok(None) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new(
                    "INVALID_TOKEN",
                    "This password reset link is invalid or has already been used",
                )),
            ));
        }
        Err(e) => {
            tracing::error!(error = %e, "Database error finding password reset token");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("DATABASE_ERROR", "Failed to validate reset token")),
            ));
        }
    };

    // Check if token is expired
    if reset_token.is_expired() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "TOKEN_EXPIRED",
                "This password reset link has expired. Please request a new one.",
            )),
        ));
    }

    // Find the user
    let user = match state.user_repo.find_by_id(reset_token.user_id).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new("USER_NOT_FOUND", "User account no longer exists")),
            ));
        }
        Err(e) => {
            tracing::error!(error = %e, "Database error finding user");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("DATABASE_ERROR", "Failed to reset password")),
            ));
        }
    };

    // Check if user is still active
    if user.status != "active" {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "ACCOUNT_INACTIVE",
                "Cannot reset password for inactive account",
            )),
        ));
    }

    // Hash new password
    let password_hash = match state.auth_service.hash_password(&req.new_password) {
        Ok(hash) => hash,
        Err(e) => {
            tracing::error!(error = %e, "Failed to hash new password");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to process password")),
            ));
        }
    };

    // Update password
    if let Err(e) = state.user_repo.update_password(user.id, &password_hash).await {
        tracing::error!(error = %e, user_id = %user.id, "Failed to update password");
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::new("DATABASE_ERROR", "Failed to update password")),
        ));
    }

    // Mark token as used
    if let Err(e) = state.password_reset_repo.mark_used(reset_token.id).await {
        tracing::error!(error = %e, "Failed to mark reset token as used");
        // Continue anyway - password was changed successfully
    }

    // Revoke all refresh tokens for security (force re-login)
    if let Err(e) = state.session_repo.revoke_all_user_tokens(user.id, None).await {
        tracing::error!(error = %e, "Failed to revoke user sessions");
        // Continue anyway - password was changed
    }

    tracing::info!(user_id = %user.id, "Password reset successfully");

    Ok(Json(ResetPasswordResponse {
        message: "Password has been reset. Please log in with your new password.".to_string(),
    }))
}

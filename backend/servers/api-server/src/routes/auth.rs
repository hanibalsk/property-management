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
        .route("/logout", post(logout))
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

// ==================== Login (Story 1.2 - placeholder) ====================

/// Login request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    /// Email address
    pub email: String,
    /// Password
    pub password: String,
    /// 2FA code (optional)
    pub two_factor_code: Option<String>,
}

/// Login response.
#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    /// JWT access token
    pub access_token: String,
    /// Refresh token
    pub refresh_token: String,
    /// Token expiration in seconds
    pub expires_in: i32,
}

/// Login endpoint.
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "Authentication",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse)
    )
)]
pub async fn login(
    State(_state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Implement in Story 1.2
    tracing::info!(email = %req.email, "Login attempt");

    Ok(Json(LoginResponse {
        access_token: "placeholder-token".to_string(),
        refresh_token: "placeholder-refresh".to_string(),
        expires_in: 3600,
    }))
}

// ==================== Logout (Story 1.3 - placeholder) ====================

/// Logout endpoint.
#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    tag = "Authentication",
    responses(
        (status = 200, description = "Logout successful"),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn logout(State(_state): State<AppState>) -> StatusCode {
    // TODO: Implement in Story 1.3
    StatusCode::OK
}

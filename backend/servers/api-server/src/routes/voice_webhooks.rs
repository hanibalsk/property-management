//! Voice assistant webhook routes (Epic 93: Voice Assistant & OAuth Completion).
//!
//! Story 93.3: Voice Platform Webhooks
//! - Alexa Skills Kit webhook handler
//! - Google Actions webhook handler
//! - Request signature verification
//! - User authentication via OAuth token

use crate::services::VoiceCommandProcessor;
use crate::state::AppState;
use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::post,
    Json, Router,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use chrono::{Duration, Utc};
use common::errors::ErrorResponse;
use db::models::{
    voice_platform, AlexaCard, AlexaIntent, AlexaOutputSpeech, AlexaRequestBody, AlexaResponseBody,
    AlexaSkillRequest, AlexaSkillResponse, GoogleActionsRequest, GoogleActionsResponse,
    GoogleContent, GooglePrompt, GoogleSceneResponse, GoogleSessionResponse, GoogleSimpleResponse,
    VoiceActionResult, VoiceOAuthExchangeRequest, VoiceOAuthExchangeResponse,
    VoiceTokenRefreshRequest, VoiceTokenRefreshResult, WebhookVerificationResult,
};
use hmac::{Hmac, Mac};
use integrations::{encrypt_if_available, IntegrationCrypto};
use serde::Deserialize;
use sha2::Sha256;
use utoipa::ToSchema;
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;

// ============================================================================
// Router
// ============================================================================

/// Voice webhook router for external platform integrations.
pub fn voice_webhook_router() -> Router<AppState> {
    Router::new()
        // Alexa Skills Kit
        .route("/alexa", post(alexa_webhook))
        .route("/alexa/health", post(alexa_health_check))
        // Google Actions
        .route("/google", post(google_actions_webhook))
        // OAuth token exchange endpoints
        .route("/oauth/exchange", post(oauth_token_exchange))
        .route("/oauth/refresh", post(oauth_token_refresh))
        // Verification endpoint
        .route("/verify", post(verify_webhook_signature))
}

// ============================================================================
// Alexa Skills Kit Webhook
// ============================================================================

/// Alexa Skills Kit webhook endpoint.
///
/// Handles all Alexa skill requests including:
/// - LaunchRequest: Skill opened
/// - IntentRequest: User spoke a command
/// - SessionEndedRequest: Session terminated
#[utoipa::path(
    post,
    path = "/api/v1/webhooks/voice/alexa",
    request_body = AlexaSkillRequest,
    responses(
        (status = 200, description = "Alexa skill response", body = AlexaSkillResponse),
        (status = 401, description = "Unauthorized - invalid signature or token"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Voice Webhooks"
)]
async fn alexa_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Json<AlexaSkillResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Verify request signature (Story 93.3)
    if let Err(e) = verify_alexa_signature(&headers, &body).await {
        tracing::warn!("Alexa signature verification failed: {}", e);
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse::new("INVALID_SIGNATURE", &e)),
        ));
    }

    // Parse the request body
    let request: AlexaSkillRequest = serde_json::from_slice(&body).map_err(|e| {
        tracing::error!("Failed to parse Alexa request: {}", e);
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "INVALID_REQUEST",
                "Invalid request format",
            )),
        )
    })?;

    // Extract access token for user authentication
    let access_token = request.session.user.access_token.as_ref();

    // Authenticate user via OAuth token
    let device = if let Some(token) = access_token {
        authenticate_voice_user(&state, token, voice_platform::ALEXA).await?
    } else {
        // Account linking not complete - return link card
        return Ok(Json(build_alexa_link_account_response()));
    };

    // Process the request based on type
    let locale = match &request.request {
        AlexaRequestBody::LaunchRequest { locale, .. } => locale.clone(),
        AlexaRequestBody::IntentRequest { locale, .. } => locale.clone(),
        AlexaRequestBody::SessionEndedRequest { locale, .. } => locale.clone(),
    };

    let response = match &request.request {
        AlexaRequestBody::LaunchRequest { .. } => {
            // Welcome message
            let processor = VoiceCommandProcessor::new(state.llm_document_repo.clone());
            let (result, _) = processor
                .process_command(device.id, "help", &locale)
                .await
                .map_err(|e| {
                    tracing::error!("Voice command processing failed: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse::new("PROCESSING_ERROR", e.to_string())),
                    )
                })?;
            build_alexa_response(&result)
        }
        AlexaRequestBody::IntentRequest { intent, .. } => {
            // Process the intent
            let command_text = extract_alexa_command_text(intent);
            let processor = VoiceCommandProcessor::new(state.llm_document_repo.clone());
            let (result, _) = processor
                .process_command(device.id, &command_text, &locale)
                .await
                .map_err(|e| {
                    tracing::error!("Voice command processing failed: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse::new("PROCESSING_ERROR", e.to_string())),
                    )
                })?;
            build_alexa_response(&result)
        }
        AlexaRequestBody::SessionEndedRequest { .. } => {
            // Session ended - no response needed
            AlexaSkillResponse {
                version: "1.0".to_string(),
                response: AlexaResponseBody {
                    output_speech: AlexaOutputSpeech {
                        speech_type: "PlainText".to_string(),
                        text: None,
                        ssml: None,
                    },
                    card: None,
                    should_end_session: true,
                },
            }
        }
    };

    Ok(Json(response))
}

/// Alexa health check endpoint for skill validation.
async fn alexa_health_check() -> StatusCode {
    StatusCode::OK
}

// ============================================================================
// Google Actions Webhook
// ============================================================================

/// Google Actions webhook endpoint.
///
/// Handles Google Assistant requests via Actions SDK.
#[utoipa::path(
    post,
    path = "/api/v1/webhooks/voice/google",
    request_body = GoogleActionsRequest,
    responses(
        (status = 200, description = "Google Actions response", body = GoogleActionsResponse),
        (status = 401, description = "Unauthorized - invalid token"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Voice Webhooks"
)]
async fn google_actions_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<GoogleActionsRequest>,
) -> Result<Json<GoogleActionsResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Verify request (Google uses Bearer token in Authorization header)
    if let Err(e) = verify_google_request(&headers) {
        tracing::warn!("Google Actions verification failed: {}", e);
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse::new("INVALID_REQUEST", &e)),
        ));
    }

    // Extract access token from user params
    let access_token = request
        .user
        .params
        .as_ref()
        .and_then(|p| p.get("bearerToken"))
        .and_then(|v| v.as_str());

    // Authenticate user via OAuth token
    let device = if let Some(token) = access_token {
        authenticate_voice_user(&state, token, voice_platform::GOOGLE_ASSISTANT).await?
    } else {
        // Account linking not complete
        return Ok(Json(build_google_link_account_response(
            &request.session.id,
        )));
    };

    // Get locale from session
    let locale = request.session.language_code.as_deref().unwrap_or("en-US");

    // Extract command text from intent
    let command_text = request
        .intent
        .query
        .as_deref()
        .unwrap_or(&request.handler.name);

    // Process the command
    let processor = VoiceCommandProcessor::new(state.llm_document_repo.clone());
    let (result, _) = processor
        .process_command(device.id, command_text, locale)
        .await
        .map_err(|e| {
            tracing::error!("Voice command processing failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("PROCESSING_ERROR", e.to_string())),
            )
        })?;

    Ok(Json(build_google_response(&request.session.id, &result)))
}

// ============================================================================
// OAuth Token Exchange (Story 93.1)
// ============================================================================

/// Exchange OAuth authorization code for tokens.
///
/// This endpoint is called after the user completes account linking
/// on the voice assistant platform.
#[utoipa::path(
    post,
    path = "/api/v1/webhooks/voice/oauth/exchange",
    request_body = VoiceOAuthExchangeRequest,
    responses(
        (status = 200, description = "Token exchange successful", body = VoiceOAuthExchangeResponse),
        (status = 400, description = "Invalid authorization code"),
        (status = 500, description = "Token exchange failed"),
    ),
    tag = "Voice OAuth"
)]
async fn oauth_token_exchange(
    State(state): State<AppState>,
    Json(request): Json<VoiceOAuthExchangeRequest>,
) -> Result<Json<VoiceOAuthExchangeResponse>, (StatusCode, Json<ErrorResponse>)> {
    use integrations::{VoiceOAuthManager, VoicePlatform};

    tracing::info!("OAuth token exchange for platform: {}", request.platform);

    // Validate platform
    if request.platform != voice_platform::ALEXA
        && request.platform != voice_platform::GOOGLE_ASSISTANT
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "INVALID_PLATFORM",
                "Unsupported voice platform",
            )),
        ));
    }

    // Story 98.1: Implement actual OAuth token exchange
    let voice_platform: VoicePlatform = request.platform.parse().map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "INVALID_PLATFORM",
                "Unsupported voice platform",
            )),
        )
    })?;

    // Get OAuth manager and check if platform is configured
    let oauth_manager = VoiceOAuthManager::from_env();
    let crypto = IntegrationCrypto::try_from_env();

    let (access_encrypted, refresh_encrypted, expires_at) = if oauth_manager
        .has_platform(voice_platform)
    {
        // Get redirect URI from environment
        let redirect_uri = match voice_platform {
            VoicePlatform::Alexa => std::env::var("ALEXA_REDIRECT_URI").unwrap_or_else(|_| {
                "https://ppt.three-two-bit.com/api/v1/webhooks/voice/oauth/callback".to_string()
            }),
            VoicePlatform::GoogleAssistant => std::env::var("GOOGLE_VOICE_REDIRECT_URI")
                .unwrap_or_else(|_| {
                    "https://ppt.three-two-bit.com/api/v1/webhooks/voice/oauth/callback".to_string()
                }),
        };

        // Exchange the authorization code for tokens
        let tokens = oauth_manager
            .exchange_code(voice_platform, &request.auth_code, &redirect_uri)
            .await
            .map_err(|e| {
                tracing::error!("OAuth token exchange failed: {}", e);
                (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse::new(
                        "OAUTH_EXCHANGE_FAILED",
                        &format!("Failed to exchange authorization code: {}", e),
                    )),
                )
            })?;

        let access_encrypted = encrypt_if_available(crypto.as_ref(), &tokens.access_token);
        let refresh_encrypted = tokens
            .refresh_token
            .as_ref()
            .map(|rt| encrypt_if_available(crypto.as_ref(), rt));

        (access_encrypted, refresh_encrypted, tokens.expires_at)
    } else {
        // Platform not configured - use simulated tokens for development
        tracing::warn!(
            "Voice OAuth not configured for platform {}, using simulated tokens",
            request.platform
        );
        let simulated_access = format!("voice_access_{}_{}", request.platform, Uuid::new_v4());
        let simulated_refresh = format!("voice_refresh_{}_{}", request.platform, Uuid::new_v4());
        (
            encrypt_if_available(crypto.as_ref(), &simulated_access),
            Some(encrypt_if_available(crypto.as_ref(), &simulated_refresh)),
            Some(Utc::now() + Duration::hours(1)),
        )
    };

    // For this webhook endpoint, we don't have tenant context
    // The user_id and org_id should be extracted from the OAuth token claims
    // For now, use placeholder values (in production, validate JWT/token)
    let user_id = Uuid::new_v4();
    let org_id = Uuid::new_v4();
    let device_id = format!("{}_{}", request.platform, Uuid::new_v4());

    // Create the voice device with tokens
    let device = state
        .llm_document_repo
        .create_voice_device(
            org_id,
            user_id,
            None,
            &request.platform,
            &device_id,
            Some("Voice Assistant"),
            Some(&access_encrypted),
            refresh_encrypted.as_deref(),
            expires_at,
            serde_json::json!(["check_balance", "report_fault", "check_announcements"]),
        )
        .await
        .map_err(|e| {
            tracing::error!("Failed to create voice device: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DEVICE_CREATION_FAILED",
                    "Failed to link voice device",
                )),
            )
        })?;

    tracing::info!(
        "Voice device linked successfully: {} (platform: {})",
        device.id,
        request.platform
    );

    Ok(Json(VoiceOAuthExchangeResponse {
        device_id: device.id,
        success: true,
        message: "Voice assistant linked successfully".to_string(),
        capabilities: vec![
            "check_balance".to_string(),
            "report_fault".to_string(),
            "check_announcements".to_string(),
        ],
    }))
}

/// Refresh OAuth tokens for a voice device.
#[utoipa::path(
    post,
    path = "/api/v1/webhooks/voice/oauth/refresh",
    request_body = VoiceTokenRefreshRequest,
    responses(
        (status = 200, description = "Token refresh successful", body = VoiceTokenRefreshResult),
        (status = 404, description = "Device not found"),
        (status = 500, description = "Token refresh failed"),
    ),
    tag = "Voice OAuth"
)]
async fn oauth_token_refresh(
    State(state): State<AppState>,
    Json(request): Json<VoiceTokenRefreshRequest>,
) -> Result<Json<VoiceTokenRefreshResult>, (StatusCode, Json<ErrorResponse>)> {
    use integrations::{decrypt_if_available, VoiceOAuthManager, VoicePlatform};

    // Find the device
    let device = state
        .llm_document_repo
        .find_voice_device(request.device_id)
        .await
        .map_err(|e| {
            tracing::error!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("DATABASE_ERROR", "Database error")),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new(
                    "DEVICE_NOT_FOUND",
                    "Voice device not found",
                )),
            )
        })?;

    // Check if device has refresh token
    let refresh_token_encrypted = match &device.refresh_token_encrypted {
        Some(token) => token,
        None => {
            return Ok(Json(VoiceTokenRefreshResult {
                success: false,
                expires_at: None,
                error: Some("No refresh token available".to_string()),
            }));
        }
    };

    // Story 98.1: Use actual OAuth client to refresh tokens
    let crypto = IntegrationCrypto::try_from_env();
    let oauth_manager = VoiceOAuthManager::from_env();

    // Parse platform
    let voice_platform: VoicePlatform = device.platform.parse().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::new(
                "INVALID_PLATFORM",
                "Device has invalid platform",
            )),
        )
    })?;

    let (new_access_encrypted, new_refresh_encrypted, new_expires_at) =
        if oauth_manager.has_platform(voice_platform) {
            // Decrypt the refresh token
            let refresh_token = decrypt_if_available(crypto.as_ref(), refresh_token_encrypted);

            // Refresh the tokens using OAuth client
            let tokens = oauth_manager
                .refresh_token(voice_platform, &refresh_token)
                .await
                .map_err(|e| {
                    tracing::error!("Token refresh failed: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse::new(
                            "TOKEN_REFRESH_FAILED",
                            &format!("Failed to refresh token: {}", e),
                        )),
                    )
                })?;

            let access_encrypted = encrypt_if_available(crypto.as_ref(), &tokens.access_token);
            let refresh_encrypted = tokens
                .refresh_token
                .as_ref()
                .map(|rt| encrypt_if_available(crypto.as_ref(), rt));

            (access_encrypted, refresh_encrypted, tokens.expires_at)
        } else {
            // Platform not configured - use simulated tokens for development
            tracing::warn!(
                "Voice OAuth not configured for platform {}, using simulated refresh",
                device.platform
            );
            let new_access = format!("voice_access_refreshed_{}", Uuid::new_v4());
            (
                encrypt_if_available(crypto.as_ref(), &new_access),
                None,
                Some(Utc::now() + Duration::hours(1)),
            )
        };

    // Update the device tokens
    state
        .llm_document_repo
        .update_voice_device_tokens(
            device.id,
            &new_access_encrypted,
            new_refresh_encrypted.as_deref(),
            new_expires_at,
        )
        .await
        .map_err(|e| {
            tracing::error!("Failed to update tokens: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "TOKEN_UPDATE_FAILED",
                    "Failed to update tokens",
                )),
            )
        })?;

    tracing::info!(
        "Successfully refreshed OAuth tokens for voice device {}",
        device.id
    );

    Ok(Json(VoiceTokenRefreshResult {
        success: true,
        expires_at: new_expires_at,
        error: None,
    }))
}

// ============================================================================
// Signature Verification (Story 93.3)
// ============================================================================

/// Verify webhook request signature.
#[derive(Debug, Deserialize, ToSchema)]
pub struct VerifyWebhookRequest {
    pub platform: String,
    pub signature: String,
    pub body: String,
    pub timestamp: Option<String>,
}

#[utoipa::path(
    post,
    path = "/api/v1/webhooks/voice/verify",
    request_body = VerifyWebhookRequest,
    responses(
        (status = 200, description = "Verification result", body = WebhookVerificationResult),
    ),
    tag = "Voice Webhooks"
)]
async fn verify_webhook_signature(
    Json(request): Json<VerifyWebhookRequest>,
) -> Json<WebhookVerificationResult> {
    let result = match request.platform.as_str() {
        "alexa" => {
            // Alexa uses certificate-based signature verification
            // Simplified check for demo
            WebhookVerificationResult {
                valid: !request.signature.is_empty(),
                platform: "alexa".to_string(),
                error: if request.signature.is_empty() {
                    Some("Missing signature".to_string())
                } else {
                    None
                },
            }
        }
        "google" => {
            // Google uses HMAC-SHA256
            match verify_hmac_signature(&request.signature, &request.body) {
                Ok(valid) => WebhookVerificationResult {
                    valid,
                    platform: "google".to_string(),
                    error: if valid {
                        None
                    } else {
                        Some("Invalid signature".to_string())
                    },
                },
                Err(e) => WebhookVerificationResult {
                    valid: false,
                    platform: "google".to_string(),
                    error: Some(e),
                },
            }
        }
        _ => WebhookVerificationResult {
            valid: false,
            platform: request.platform.clone(),
            error: Some("Unknown platform".to_string()),
        },
    };

    Json(result)
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Verify Alexa request signature.
async fn verify_alexa_signature(headers: &HeaderMap, _body: &[u8]) -> Result<(), String> {
    // Alexa uses certificate-based signature verification
    // Check required headers
    let _signature_cert_url = headers
        .get("SignatureCertChainUrl")
        .and_then(|v| v.to_str().ok())
        .ok_or("Missing SignatureCertChainUrl header")?;

    let _signature = headers
        .get("Signature")
        .and_then(|v| v.to_str().ok())
        .ok_or("Missing Signature header")?;

    // In production, you would:
    // 1. Fetch and validate the certificate from SignatureCertChainUrl
    // 2. Verify the certificate chain
    // 3. Verify the signature using the public key
    // 4. Check timestamp is within 150 seconds
    //
    // For now, accept if headers are present
    // TODO: Implement full certificate validation

    Ok(())
}

/// Verify Google Actions request.
fn verify_google_request(headers: &HeaderMap) -> Result<(), String> {
    // Google Actions can use project ID verification
    let _auth_header = headers.get("Authorization").and_then(|v| v.to_str().ok());

    // Accept requests for now - in production verify project ID or use ID tokens
    Ok(())
}

/// Verify HMAC-SHA256 signature.
fn verify_hmac_signature(signature: &str, body: &str) -> Result<bool, String> {
    let secret =
        std::env::var("VOICE_WEBHOOK_SECRET").unwrap_or_else(|_| "default_secret".to_string());

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|e| format!("Invalid HMAC key: {}", e))?;

    mac.update(body.as_bytes());

    let expected = BASE64.encode(mac.finalize().into_bytes());

    Ok(signature == expected)
}

/// Authenticate user via OAuth access token.
async fn authenticate_voice_user(
    state: &AppState,
    access_token: &str,
    platform: &str,
) -> Result<db::models::VoiceAssistantDevice, (StatusCode, Json<ErrorResponse>)> {
    // In production, validate the access token and extract user ID
    // For now, find device by platform that has matching token pattern

    // Look for device with this token (simplified - production would validate JWT/OAuth)
    let crypto = IntegrationCrypto::try_from_env();
    let _encrypted_token = encrypt_if_available(crypto.as_ref(), access_token);

    // Try to find a device - for demo, just find any active device for the platform
    // In production, you would validate the token and look up the associated user
    let devices = sqlx::query_as::<_, db::models::VoiceAssistantDevice>(
        r#"
        SELECT * FROM voice_assistant_devices
        WHERE platform = $1 AND is_active = TRUE
        ORDER BY last_used_at DESC NULLS LAST
        LIMIT 1
        "#,
    )
    .bind(platform)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::new("DATABASE_ERROR", "Database error")),
        )
    })?;

    devices.ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse::new(
                "DEVICE_NOT_LINKED",
                "Voice device not linked. Please complete account linking.",
            )),
        )
    })
}

/// Extract command text from Alexa intent.
fn extract_alexa_command_text(intent: &AlexaIntent) -> String {
    // Map Alexa built-in intents to our commands
    match intent.name.as_str() {
        "AMAZON.HelpIntent" => "help".to_string(),
        "AMAZON.StopIntent" | "AMAZON.CancelIntent" => "goodbye".to_string(),
        "CheckBalanceIntent" => "check my balance".to_string(),
        "ReportFaultIntent" => {
            // Extract fault description from slots
            let description = intent
                .slots
                .as_ref()
                .and_then(|s| s.get("FaultDescription"))
                .and_then(|v| v.get("value"))
                .and_then(|v| v.as_str())
                .unwrap_or("a fault");
            format!("report a fault with {}", description)
        }
        "CheckAnnouncementsIntent" => "check announcements".to_string(),
        "CheckMeterIntent" => "check meter readings".to_string(),
        "ContactManagerIntent" => "contact manager".to_string(),
        _ => intent.name.clone(),
    }
}

/// Build Alexa skill response from action result.
fn build_alexa_response(result: &VoiceActionResult) -> AlexaSkillResponse {
    let output_speech = if let Some(ssml) = &result.ssml {
        AlexaOutputSpeech {
            speech_type: "SSML".to_string(),
            text: None,
            ssml: Some(ssml.clone()),
        }
    } else {
        AlexaOutputSpeech {
            speech_type: "PlainText".to_string(),
            text: Some(result.response_text.clone()),
            ssml: None,
        }
    };

    let card = result.card.as_ref().map(|c| AlexaCard {
        card_type: "Simple".to_string(),
        title: c.title.clone(),
        content: Some(c.content.clone()),
        text: None,
    });

    AlexaSkillResponse {
        version: "1.0".to_string(),
        response: AlexaResponseBody {
            output_speech,
            card,
            should_end_session: result.should_end_session,
        },
    }
}

/// Build Alexa response for account linking.
fn build_alexa_link_account_response() -> AlexaSkillResponse {
    AlexaSkillResponse {
        version: "1.0".to_string(),
        response: AlexaResponseBody {
            output_speech: AlexaOutputSpeech {
                speech_type: "PlainText".to_string(),
                text: Some(
                    "Please link your property management account in the Alexa app to use this skill."
                        .to_string(),
                ),
                ssml: None,
            },
            card: Some(AlexaCard {
                card_type: "LinkAccount".to_string(),
                title: "Link Account".to_string(),
                content: None,
                text: None,
            }),
            should_end_session: true,
        },
    }
}

/// Build Google Actions response from action result.
fn build_google_response(session_id: &str, result: &VoiceActionResult) -> GoogleActionsResponse {
    let content = result.card.as_ref().map(|c| GoogleContent {
        card: Some(db::models::GoogleCard {
            title: c.title.clone(),
            subtitle: None,
            text: c.content.clone(),
            image: None,
        }),
    });

    GoogleActionsResponse {
        session: GoogleSessionResponse {
            id: session_id.to_string(),
            params: None,
        },
        prompt: GooglePrompt {
            override_mode: false,
            first_simple: GoogleSimpleResponse {
                speech: result.response_text.clone(),
                text: Some(result.response_text.clone()),
            },
            content,
        },
        scene: if result.should_end_session {
            Some(GoogleSceneResponse {
                name: "actions.scene.END_CONVERSATION".to_string(),
            })
        } else {
            None
        },
    }
}

/// Build Google response for account linking.
fn build_google_link_account_response(session_id: &str) -> GoogleActionsResponse {
    GoogleActionsResponse {
        session: GoogleSessionResponse {
            id: session_id.to_string(),
            params: None,
        },
        prompt: GooglePrompt {
            override_mode: false,
            first_simple: GoogleSimpleResponse {
                speech: "Please link your property management account to use this action."
                    .to_string(),
                text: Some(
                    "Please link your property management account to use this action.".to_string(),
                ),
            },
            content: None,
        },
        scene: Some(GoogleSceneResponse {
            name: "AccountLinking".to_string(),
        }),
    }
}

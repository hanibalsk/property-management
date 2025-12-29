//! E-Signature API routes (Story 7B.3).
//!
//! Provides endpoints for managing electronic signature workflows on documents.

use api_core::AuthUser;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use common::errors::ErrorResponse;
use db::models::{
    CancelSignatureRequestRequest, CancelSignatureRequestResponse, CreateSignatureRequest,
    CreateSignatureRequestResponse, ListSignatureRequestsResponse, Locale, SendReminderRequest,
    SendReminderResponse, SignatureRequestResponse, SignatureWebhookEvent, WebhookResponse,
};
use tracing::{info, warn};
use uuid::Uuid;

use crate::state::AppState;

/// Create router for signature endpoints.
pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(list_signature_requests).post(create_signature_request),
        )
        .route("/:id", get(get_signature_request))
        .route("/:id/remind", post(send_reminder))
        .route("/:id/cancel", post(cancel_signature_request))
        .route("/webhook/:provider", post(handle_webhook))
}

/// Create a new signature request for a document.
pub async fn create_signature_request(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(document_id): Path<Uuid>,
    Json(request): Json<CreateSignatureRequest>,
) -> Result<(StatusCode, Json<CreateSignatureRequestResponse>), (StatusCode, Json<ErrorResponse>)> {
    // Validate request
    if request.signers.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "VALIDATION_ERROR",
                "At least one signer is required",
            )),
        ));
    }

    // Get the document to verify it exists and get organization_id
    let document = state
        .document_repo
        .find_by_id(document_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("DATABASE_ERROR", e.to_string())),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Document not found")),
            )
        })?;

    // Check if document already has pending signature request
    let existing = state
        .signature_request_repo
        .find_by_document(document_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("DATABASE_ERROR", e.to_string())),
            )
        })?;

    if existing.iter().any(|r| {
        matches!(
            r.status,
            db::models::SignatureRequestStatus::Pending
                | db::models::SignatureRequestStatus::InProgress
        )
    }) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "PENDING_REQUEST_EXISTS",
                "Document already has a pending signature request",
            )),
        ));
    }

    let created_by = auth.user_id;

    let signature_request = state
        .signature_request_repo
        .create(document_id, document.organization_id, created_by, &request)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("DATABASE_ERROR", e.to_string())),
            )
        })?;

    info!(
        signature_request_id = %signature_request.id,
        document_id = %document_id,
        signers_count = request.signers.len(),
        "Created signature request"
    );

    // Send invitation emails to signers
    let base_url =
        std::env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let subject = signature_request
        .subject
        .clone()
        .unwrap_or_else(|| "You have been requested to sign a document".to_string());

    for signer in &signature_request.signers {
        let sign_url = format!(
            "{}/sign?request_id={}&email={}",
            base_url, signature_request.id, signer.email
        );
        let email_body = format!(
            "Hello {},\n\nYou have been requested to electronically sign a document.\n\n{}\n\nPlease click the link below to review and sign the document:\n\n{}\n\nIf you have any questions, please contact the person who sent this request.\n\nBest regards,\nProperty Management System",
            signer.name,
            signature_request.message.as_deref().unwrap_or(""),
            sign_url
        );

        if let Err(e) = state
            .email_service
            .send_notification_email(
                &signer.email,
                &signer.name,
                &subject,
                &email_body,
                &Locale::English,
            )
            .await
        {
            warn!(
                error = %e,
                email = %signer.email,
                signature_request_id = %signature_request.id,
                "Failed to send signature request email to signer"
            );
            // Continue sending to other signers even if one fails
        }
    }

    Ok((
        StatusCode::CREATED,
        Json(CreateSignatureRequestResponse {
            signature_request,
            message: "Signature request created. Signers will receive email invitations.".into(),
        }),
    ))
}

/// List signature requests for a document.
pub async fn list_signature_requests(
    State(state): State<AppState>,
    Path(document_id): Path<Uuid>,
) -> Result<Json<ListSignatureRequestsResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Verify document exists
    let _document = state
        .document_repo
        .find_by_id(document_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("DATABASE_ERROR", e.to_string())),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Document not found")),
            )
        })?;

    let requests = state
        .signature_request_repo
        .find_by_document(document_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("DATABASE_ERROR", e.to_string())),
            )
        })?;

    let total = requests.len() as i64;

    Ok(Json(ListSignatureRequestsResponse {
        signature_requests: requests,
        total,
    }))
}

/// Get a signature request by ID.
pub async fn get_signature_request(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<SignatureRequestResponse>, (StatusCode, Json<ErrorResponse>)> {
    let request = state
        .signature_request_repo
        .find_by_id(id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("DATABASE_ERROR", e.to_string())),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new(
                    "NOT_FOUND",
                    "Signature request not found",
                )),
            )
        })?;

    let signer_counts = request.signer_counts();

    Ok(Json(SignatureRequestResponse {
        signature_request: request,
        signer_counts,
    }))
}

/// Send reminder to pending signers.
pub async fn send_reminder(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<SendReminderRequest>,
) -> Result<Json<SendReminderResponse>, (StatusCode, Json<ErrorResponse>)> {
    let signature_request = state
        .signature_request_repo
        .find_by_id(id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("DATABASE_ERROR", e.to_string())),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new(
                    "NOT_FOUND",
                    "Signature request not found",
                )),
            )
        })?;

    // Check if request is still active
    if !signature_request.can_cancel() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "INVALID_STATE",
                "Cannot send reminders for completed or cancelled requests",
            )),
        ));
    }

    // Find pending signers
    let pending_signers: Vec<_> = signature_request
        .signers
        .iter()
        .filter(|s| !s.is_complete())
        .filter(|s| {
            request.signer_emails.is_empty()
                || request
                    .signer_emails
                    .iter()
                    .any(|e| e.eq_ignore_ascii_case(&s.email))
        })
        .collect();

    if pending_signers.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "NO_PENDING_SIGNERS",
                "No pending signers to remind",
            )),
        ));
    }

    // Send reminder emails to pending signers
    let base_url =
        std::env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let subject = format!(
        "Reminder: {}",
        signature_request
            .subject
            .as_deref()
            .unwrap_or("Signature request pending")
    );
    let mut reminders_sent = 0i32;

    for signer in pending_signers {
        let sign_url = format!(
            "{}/sign?request_id={}&email={}",
            base_url, signature_request.id, signer.email
        );
        let email_body = format!(
            "Hello {},\n\nThis is a reminder that you have a pending signature request.\n\nPlease click the link below to review and sign the document:\n\n{}\n\nIf you have any questions, please contact the person who sent this request.\n\nBest regards,\nProperty Management System",
            signer.name, sign_url
        );

        if let Err(e) = state
            .email_service
            .send_notification_email(
                &signer.email,
                &signer.name,
                &subject,
                &email_body,
                &Locale::English,
            )
            .await
        {
            warn!(
                error = %e,
                email = %signer.email,
                signature_request_id = %id,
                "Failed to send reminder email to signer"
            );
            // Continue sending to other signers even if one fails
        } else {
            reminders_sent += 1;
        }
    }

    info!(
        signature_request_id = %id,
        reminders_sent = reminders_sent,
        "Sent signature reminders"
    );

    Ok(Json(SendReminderResponse {
        reminders_sent,
        message: format!("Sent {} reminder(s)", reminders_sent),
    }))
}

/// Cancel a signature request.
pub async fn cancel_signature_request(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<CancelSignatureRequestRequest>,
) -> Result<Json<CancelSignatureRequestResponse>, (StatusCode, Json<ErrorResponse>)> {
    let signature_request = state
        .signature_request_repo
        .cancel(id, request.reason.as_deref())
        .await
        .map_err(|e| {
            let err_msg = e.to_string();
            if err_msg.contains("RowNotFound") {
                (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse::new(
                        "INVALID_STATE",
                        "Cannot cancel: request not found or already completed/cancelled",
                    )),
                )
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse::new("DATABASE_ERROR", &err_msg)),
                )
            }
        })?;

    info!(
        signature_request_id = %id,
        reason = ?request.reason,
        "Cancelled signature request"
    );

    // Notify signers of cancellation
    let subject = format!(
        "Signature Request Cancelled: {}",
        signature_request
            .subject
            .as_deref()
            .unwrap_or("Document signing")
    );
    let reason_text = request
        .reason
        .as_deref()
        .map(|r| format!("\n\nReason: {}", r))
        .unwrap_or_default();

    for signer in &signature_request.signers {
        // Only notify signers who haven't completed signing yet
        if !signer.is_complete() {
            let email_body = format!(
                "Hello {},\n\nThe signature request you received has been cancelled.{}\n\nNo further action is required.\n\nBest regards,\nProperty Management System",
                signer.name, reason_text
            );

            if let Err(e) = state
                .email_service
                .send_notification_email(
                    &signer.email,
                    &signer.name,
                    &subject,
                    &email_body,
                    &Locale::English,
                )
                .await
            {
                warn!(
                    error = %e,
                    email = %signer.email,
                    signature_request_id = %id,
                    "Failed to send cancellation notification to signer"
                );
            }
        }
    }

    Ok(Json(CancelSignatureRequestResponse {
        signature_request,
        message: "Signature request cancelled".into(),
    }))
}

/// Handle webhook from e-signature provider.
pub async fn handle_webhook(
    State(state): State<AppState>,
    Path(provider): Path<String>,
    Json(event): Json<SignatureWebhookEvent>,
) -> Result<Json<WebhookResponse>, (StatusCode, Json<ErrorResponse>)> {
    info!(
        provider = %provider,
        event_type = %event.event_type,
        provider_request_id = %event.provider_request_id,
        "Received signature webhook"
    );

    // Find the signature request by provider request ID
    let signature_request = state
        .signature_request_repo
        .find_by_provider_request_id(&provider, &event.provider_request_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("DATABASE_ERROR", e.to_string())),
            )
        })?
        .ok_or_else(|| {
            warn!(
                provider = %provider,
                provider_request_id = %event.provider_request_id,
                "Signature request not found for webhook"
            );
            (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new(
                    "NOT_FOUND",
                    "Signature request not found",
                )),
            )
        })?;

    // Process signer-specific events
    if let (Some(signer_email), Some(signer_status)) = (&event.signer_email, &event.signer_status) {
        state
            .signature_request_repo
            .update_signer_status(
                signature_request.id,
                signer_email,
                *signer_status,
                event.decline_reason.as_deref(),
            )
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse::new("DATABASE_ERROR", e.to_string())),
                )
            })?;

        info!(
            signature_request_id = %signature_request.id,
            signer_email = %signer_email,
            new_status = ?signer_status,
            "Updated signer status from webhook"
        );
    }

    // Handle completion event with signed document
    if event.event_type == "completed" {
        if let Some(_signed_url) = &event.signed_document_url {
            // TODO: Download signed document and store it
            // For now, just log
            info!(
                signature_request_id = %signature_request.id,
                "Signature request completed, signed document available"
            );
        }
    }

    Ok(Json(WebhookResponse {
        success: true,
        message: "Webhook processed successfully".into(),
    }))
}

// Helper function to create document-scoped signature routes
pub fn document_signature_router() -> Router<AppState> {
    Router::new().route(
        "/signature-requests",
        get(list_signature_requests_for_doc).post(create_signature_request_for_doc),
    )
}

/// Create a signature request for a specific document (nested route version).
pub async fn create_signature_request_for_doc(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(document_id): Path<Uuid>,
    Json(request): Json<CreateSignatureRequest>,
) -> Result<(StatusCode, Json<CreateSignatureRequestResponse>), (StatusCode, Json<ErrorResponse>)> {
    create_signature_request(State(state), auth, Path(document_id), Json(request)).await
}

/// List signature requests for a specific document (nested route version).
pub async fn list_signature_requests_for_doc(
    State(state): State<AppState>,
    Path(document_id): Path<Uuid>,
) -> Result<Json<ListSignatureRequestsResponse>, (StatusCode, Json<ErrorResponse>)> {
    list_signature_requests(State(state), Path(document_id)).await
}

//! E-Signature API routes (Story 7B.3).
//!
//! Provides endpoints for managing electronic signature workflows on documents.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use common::errors::ErrorResponse;
use db::models::{
    CancelSignatureRequestRequest, CancelSignatureRequestResponse, CreateSignatureRequest,
    CreateSignatureRequestResponse, ListSignatureRequestsResponse, SendReminderRequest,
    SendReminderResponse, SignatureRequestResponse, SignatureWebhookEvent, WebhookResponse,
};
use tracing::{info, warn};
use uuid::Uuid;

use crate::state::AppState;

/// Create router for signature endpoints.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_signature_requests).post(create_signature_request))
        .route("/:id", get(get_signature_request))
        .route("/:id/remind", post(send_reminder))
        .route("/:id/cancel", post(cancel_signature_request))
        .route("/webhook/:provider", post(handle_webhook))
}

/// Create a new signature request for a document.
pub async fn create_signature_request(
    State(state): State<AppState>,
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
                Json(ErrorResponse::new("DATABASE_ERROR", &e.to_string())),
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
                Json(ErrorResponse::new("DATABASE_ERROR", &e.to_string())),
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

    // TODO: Get actual user ID from auth context
    let created_by = Uuid::nil();

    let signature_request = state
        .signature_request_repo
        .create(document_id, document.organization_id, created_by, &request)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("DATABASE_ERROR", &e.to_string())),
            )
        })?;

    info!(
        signature_request_id = %signature_request.id,
        document_id = %document_id,
        signers_count = request.signers.len(),
        "Created signature request"
    );

    // TODO: Send emails to signers via email service

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
                Json(ErrorResponse::new("DATABASE_ERROR", &e.to_string())),
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
                Json(ErrorResponse::new("DATABASE_ERROR", &e.to_string())),
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
                Json(ErrorResponse::new("DATABASE_ERROR", &e.to_string())),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Signature request not found")),
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
                Json(ErrorResponse::new("DATABASE_ERROR", &e.to_string())),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Signature request not found")),
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

    // TODO: Actually send reminder emails via email service
    let reminders_sent = pending_signers.len() as i32;

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

    // TODO: Notify signers of cancellation

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
                Json(ErrorResponse::new("DATABASE_ERROR", &e.to_string())),
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
                Json(ErrorResponse::new("NOT_FOUND", "Signature request not found")),
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
                    Json(ErrorResponse::new("DATABASE_ERROR", &e.to_string())),
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
    Path(document_id): Path<Uuid>,
    Json(request): Json<CreateSignatureRequest>,
) -> Result<(StatusCode, Json<CreateSignatureRequestResponse>), (StatusCode, Json<ErrorResponse>)> {
    create_signature_request(State(state), Path(document_id), Json(request)).await
}

/// List signature requests for a specific document (nested route version).
pub async fn list_signature_requests_for_doc(
    State(state): State<AppState>,
    Path(document_id): Path<Uuid>,
) -> Result<Json<ListSignatureRequestsResponse>, (StatusCode, Json<ErrorResponse>)> {
    list_signature_requests(State(state), Path(document_id)).await
}

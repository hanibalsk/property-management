//! Document routes (Epic 7A: Basic Document Management, Epic 7B: Document Versioning).

use crate::state::AppState;
use api_core::{AuthUser, TenantExtractor};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::{DateTime, Utc};
use common::errors::ErrorResponse;
use db::models::{
    access_scope, document_category, share_type, CreateDocument, CreateDocumentVersion,
    CreateFolder, CreateShare, Document, DocumentFolder, DocumentListQuery, DocumentSummary,
    DocumentVersion, DocumentVersionHistory, DocumentWithDetails, FolderTreeNode, FolderWithCount,
    LogShareAccess, MoveDocument, ShareWithDocument, UpdateDocument, UpdateFolder,
    ALLOWED_MIME_TYPES, MAX_FILE_SIZE,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

// ============================================================================
// Constants
// ============================================================================

/// Maximum allowed title length (characters).
const MAX_TITLE_LENGTH: usize = 500;

/// Maximum allowed description length (characters).
const MAX_DESCRIPTION_LENGTH: usize = 5000;

/// Maximum allowed folder name length (characters).
const MAX_FOLDER_NAME_LENGTH: usize = 255;

// ============================================================================
// Response Types
// ============================================================================

/// Response for document creation.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateDocumentResponse {
    pub id: Uuid,
    pub message: String,
}

/// Response for document list with pagination.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DocumentListResponse {
    pub documents: Vec<DocumentSummary>,
    pub count: usize,
    pub total: i64,
}

/// Response for document details.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DocumentDetailResponse {
    pub document: DocumentWithDetails,
}

/// Response for document action.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DocumentActionResponse {
    pub message: String,
    pub document: Document,
}

/// Response for folder creation.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateFolderResponse {
    pub id: Uuid,
    pub message: String,
}

/// Response for folder list.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FolderListResponse {
    pub folders: Vec<FolderWithCount>,
}

/// Response for folder tree.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FolderTreeResponse {
    pub tree: Vec<FolderTreeNode>,
}

/// Response for folder details.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FolderDetailResponse {
    pub folder: DocumentFolder,
}

/// Response for folder action.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FolderActionResponse {
    pub message: String,
    pub folder: DocumentFolder,
}

/// Response for share creation.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateShareResponse {
    pub id: Uuid,
    pub share_token: Option<String>,
    pub share_url: Option<String>,
    pub message: String,
}

/// Response for share list.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ShareListResponse {
    pub shares: Vec<ShareWithDocument>,
}

/// Response for download/preview URL.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UrlResponse {
    pub url: String,
    pub expires_at: DateTime<Utc>,
}

/// Response for shared document access (no auth required).
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SharedDocumentResponse {
    pub document: DocumentSummary,
    pub download_url: String,
    pub preview_url: Option<String>,
}

/// Response for version list (Story 7B.1).
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct VersionHistoryResponse {
    pub history: DocumentVersionHistory,
}

/// Response for single version (Story 7B.1).
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct VersionResponse {
    pub version: DocumentVersion,
}

/// Response for creating/restoring a version (Story 7B.1).
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateVersionResponse {
    pub id: Uuid,
    pub version_number: i32,
    pub message: String,
}

// ============================================================================
// Request Types
// ============================================================================

/// Request for creating a document.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateDocumentRequest {
    pub title: String,
    pub description: Option<String>,
    pub category: String,
    pub folder_id: Option<Uuid>,
    pub file_key: String,
    pub file_name: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub access_scope: Option<String>,
    pub access_target_ids: Option<Vec<Uuid>>,
    pub access_roles: Option<Vec<String>>,
}

/// Request for updating a document.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateDocumentRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub folder_id: Option<Uuid>,
    pub access_scope: Option<String>,
    pub access_target_ids: Option<Vec<Uuid>>,
    pub access_roles: Option<Vec<String>>,
}

/// Request for updating document access.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateAccessRequest {
    pub access_scope: String,
    pub access_target_ids: Option<Vec<Uuid>>,
    pub access_roles: Option<Vec<String>>,
}

/// Request for moving a document.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MoveDocumentRequest {
    pub folder_id: Option<Uuid>,
}

/// Request for creating a folder.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateFolderRequest {
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
}

/// Request for updating a folder.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateFolderRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
}

/// Request for deleting a folder.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteFolderRequest {
    #[serde(default)]
    pub cascade: bool,
}

/// Request for creating a share.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateShareRequest {
    pub share_type: String,
    pub target_id: Option<Uuid>,
    pub target_role: Option<String>,
    pub password: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Request for accessing a password-protected share.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AccessShareRequest {
    pub password: String,
}

/// Request for uploading a new document version (Story 7B.1).
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UploadVersionRequest {
    pub file_key: String,
    pub file_name: String,
    pub mime_type: String,
    pub size_bytes: i64,
}

/// Query for listing documents.
#[derive(Debug, Serialize, Deserialize, ToSchema, Default, utoipa::IntoParams)]
pub struct ListDocumentsQuery {
    pub folder_id: Option<Uuid>,
    pub category: Option<String>,
    pub created_by: Option<Uuid>,
    pub search: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Query for listing folders.
#[derive(Debug, Serialize, Deserialize, ToSchema, Default, utoipa::IntoParams)]
pub struct ListFoldersQuery {
    pub parent_id: Option<Uuid>,
}

// ============================================================================
// Router
// ============================================================================

/// Create documents router.
pub fn router() -> Router<AppState> {
    Router::new()
        // Document CRUD
        .route("/", post(create_document))
        .route("/", get(list_documents))
        .route("/{id}", get(get_document))
        .route("/{id}", put(update_document))
        .route("/{id}", delete(delete_document))
        // Document actions
        .route("/{id}/move", post(move_document))
        .route("/{id}/access", put(update_document_access))
        // Download/Preview (Story 7A.4)
        .route("/{id}/download", get(get_download_url))
        .route("/{id}/preview", get(get_preview_url))
        // Versioning (Story 7B.1)
        .route("/{id}/versions", get(get_version_history))
        .route("/{id}/versions", post(create_version))
        .route("/{id}/versions/{version_id}", get(get_version))
        .route("/{id}/versions/{version_id}/restore", post(restore_version))
        // Shares (Story 7A.5)
        .route("/{id}/shares", get(list_shares))
        .route("/{id}/shares", post(create_share))
        .route("/{id}/shares/{share_id}", delete(revoke_share))
        // Folders (Story 7A.2)
        .route("/folders", get(list_folders))
        .route("/folders", post(create_folder))
        .route("/folders/tree", get(get_folder_tree))
        .route("/folders/{id}", get(get_folder))
        .route("/folders/{id}", put(update_folder))
        .route("/folders/{id}", delete(delete_folder))
    // Public shared document access (no auth required - separate route in main.rs)
    // .route("/shared/{token}", get(access_shared_document))
    // .route("/shared/{token}/access", post(access_protected_share))
}

/// Create public routes for shared documents (no auth required).
pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/shared/{token}", get(access_shared_document))
        .route("/shared/{token}/access", post(access_protected_share))
}

// ============================================================================
// Document Handlers
// ============================================================================

/// Create a new document (Story 7A.1).
#[utoipa::path(
    post,
    path = "/api/v1/documents",
    request_body = CreateDocumentRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 201, description = "Document created", body = CreateDocumentResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
    tag = "Documents"
)]
async fn create_document(
    State(state): State<AppState>,
    auth: AuthUser,
    tenant: TenantExtractor,
    Json(req): Json<CreateDocumentRequest>,
) -> Result<(StatusCode, Json<CreateDocumentResponse>), (StatusCode, Json<ErrorResponse>)> {
    let user_id = auth.user_id;
    let org_id = tenant.tenant_id;

    // Validate title length
    if req.title.len() > MAX_TITLE_LENGTH {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "BAD_REQUEST",
                format!(
                    "Title exceeds maximum length of {} characters",
                    MAX_TITLE_LENGTH
                ),
            )),
        ));
    }

    // Validate description length
    if let Some(ref desc) = req.description {
        if desc.len() > MAX_DESCRIPTION_LENGTH {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new(
                    "BAD_REQUEST",
                    format!(
                        "Description exceeds maximum length of {} characters",
                        MAX_DESCRIPTION_LENGTH
                    ),
                )),
            ));
        }
    }

    // Validate file size
    if req.size_bytes > MAX_FILE_SIZE {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "FILE_TOO_LARGE",
                format!(
                    "File exceeds maximum size of {} bytes (50MB)",
                    MAX_FILE_SIZE
                ),
            )),
        ));
    }

    // Validate MIME type
    if !ALLOWED_MIME_TYPES.contains(&req.mime_type.as_str()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "UNSUPPORTED_FILE_TYPE",
                format!(
                    "File type '{}' is not supported. Allowed types: PDF, DOC, DOCX, XLS, XLSX, PNG, JPG, GIF, WEBP, TXT",
                    req.mime_type
                ),
            )),
        ));
    }

    // Validate category
    if !document_category::ALL.contains(&req.category.as_str()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new("BAD_REQUEST", "Invalid category")),
        ));
    }

    // Validate access scope if provided
    if let Some(ref scope) = req.access_scope {
        if !access_scope::ALL.contains(&scope.as_str()) {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new("BAD_REQUEST", "Invalid access_scope")),
            ));
        }

        // Validate access_target_ids are required for non-organization scope
        if scope != access_scope::ORGANIZATION
            && scope != access_scope::ROLE
            && req.access_target_ids.as_ref().is_none_or(|v| v.is_empty())
        {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new(
                    "BAD_REQUEST",
                    "access_target_ids required for building, unit, or users scope",
                )),
            ));
        }

        // Validate access_roles are required for role scope
        if scope == access_scope::ROLE && req.access_roles.as_ref().is_none_or(|v| v.is_empty()) {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new(
                    "BAD_REQUEST",
                    "access_roles required for role scope",
                )),
            ));
        }
    }

    let data = CreateDocument {
        organization_id: org_id,
        folder_id: req.folder_id,
        title: req.title,
        description: req.description,
        category: req.category,
        file_key: req.file_key,
        file_name: req.file_name,
        mime_type: req.mime_type,
        size_bytes: req.size_bytes,
        access_scope: req.access_scope,
        access_target_ids: req.access_target_ids,
        access_roles: req.access_roles,
        created_by: user_id,
    };

    match state.document_repo.create(data).await {
        Ok(document) => Ok((
            StatusCode::CREATED,
            Json(CreateDocumentResponse {
                id: document.id,
                message: "Document created successfully".to_string(),
            }),
        )),
        Err(e) => {
            tracing::error!("Failed to create document: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to create document",
                )),
            ))
        }
    }
}

/// List documents with filters.
#[utoipa::path(
    get,
    path = "/api/v1/documents",
    params(ListDocumentsQuery),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Document list", body = DocumentListResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
    tag = "Documents"
)]
async fn list_documents(
    State(state): State<AppState>,
    auth: AuthUser,
    tenant: TenantExtractor,
    Query(query): Query<ListDocumentsQuery>,
) -> Result<Json<DocumentListResponse>, (StatusCode, Json<ErrorResponse>)> {
    let org_id = tenant.tenant_id;
    let user_id = auth.user_id;

    // For managers, show all documents; for others, show only accessible documents
    let is_manager = tenant.role.is_manager();

    let list_query = DocumentListQuery {
        folder_id: query.folder_id,
        category: query.category.clone(),
        created_by: query.created_by,
        search: query.search.clone(),
        limit: query.limit,
        offset: query.offset,
    };

    let (documents, total) = if is_manager {
        let docs = state
            .document_repo
            .list(org_id, list_query.clone())
            .await
            .unwrap_or_default();
        let total = state
            .document_repo
            .count(org_id, list_query)
            .await
            .unwrap_or(0);
        (docs, total)
    } else {
        // Use simplified access control for non-managers
        // Shows: org-wide documents + own documents + role-based documents
        // TODO: Full implementation needs building/unit context from TenantContext
        let user_role = tenant.role.to_string().to_lowercase().replace(' ', "_");
        let docs = state
            .document_repo
            .list_accessible_simple(org_id, user_id, &user_role, list_query.clone())
            .await
            .unwrap_or_default();
        let total = state
            .document_repo
            .count_accessible_simple(org_id, user_id, &user_role, list_query)
            .await
            .unwrap_or(0);
        (docs, total)
    };

    let count = documents.len();
    Ok(Json(DocumentListResponse {
        documents,
        count,
        total,
    }))
}

/// Get document details.
#[utoipa::path(
    get,
    path = "/api/v1/documents/{id}",
    params(
        ("id" = Uuid, Path, description = "Document ID")
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Document details", body = DocumentDetailResponse),
        (status = 404, description = "Document not found", body = ErrorResponse),
    ),
    tag = "Documents"
)]
async fn get_document(
    State(state): State<AppState>,
    _auth: AuthUser,
    _tenant: TenantExtractor,
    Path(id): Path<Uuid>,
) -> Result<Json<DocumentDetailResponse>, (StatusCode, Json<ErrorResponse>)> {
    match state.document_repo.find_by_id_with_details(id).await {
        Ok(Some(document)) => Ok(Json(DocumentDetailResponse { document })),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Document not found")),
        )),
        Err(e) => {
            tracing::error!("Failed to get document: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to get document",
                )),
            ))
        }
    }
}

/// Update a document.
#[utoipa::path(
    put,
    path = "/api/v1/documents/{id}",
    params(
        ("id" = Uuid, Path, description = "Document ID")
    ),
    request_body = UpdateDocumentRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Document updated", body = DocumentActionResponse),
        (status = 404, description = "Document not found", body = ErrorResponse),
    ),
    tag = "Documents"
)]
async fn update_document(
    State(state): State<AppState>,
    auth: AuthUser,
    tenant: TenantExtractor,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateDocumentRequest>,
) -> Result<Json<DocumentActionResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Check document exists
    let existing = match state.document_repo.find_by_id(id).await {
        Ok(Some(d)) => d,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Document not found")),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to find document: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to find document",
                )),
            ));
        }
    };

    // Only creator or manager can update
    if existing.created_by != auth.user_id && !tenant.role.is_manager() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse::new(
                "FORBIDDEN",
                "Only the document creator or managers can update",
            )),
        ));
    }

    // Validate inputs
    if let Some(ref title) = req.title {
        if title.len() > MAX_TITLE_LENGTH {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new(
                    "BAD_REQUEST",
                    format!(
                        "Title exceeds maximum length of {} characters",
                        MAX_TITLE_LENGTH
                    ),
                )),
            ));
        }
    }

    if let Some(ref category) = req.category {
        if !document_category::ALL.contains(&category.as_str()) {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new("BAD_REQUEST", "Invalid category")),
            ));
        }
    }

    let data = UpdateDocument {
        title: req.title,
        description: req.description,
        category: req.category,
        folder_id: req.folder_id,
        access_scope: req.access_scope,
        access_target_ids: req.access_target_ids,
        access_roles: req.access_roles,
    };

    match state.document_repo.update(id, data).await {
        Ok(document) => Ok(Json(DocumentActionResponse {
            message: "Document updated".to_string(),
            document,
        })),
        Err(e) => {
            tracing::error!("Failed to update document: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to update document",
                )),
            ))
        }
    }
}

/// Delete a document (soft delete).
#[utoipa::path(
    delete,
    path = "/api/v1/documents/{id}",
    params(
        ("id" = Uuid, Path, description = "Document ID")
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 204, description = "Document deleted"),
        (status = 404, description = "Document not found", body = ErrorResponse),
    ),
    tag = "Documents"
)]
async fn delete_document(
    State(state): State<AppState>,
    auth: AuthUser,
    tenant: TenantExtractor,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    // Check document exists
    let existing = match state.document_repo.find_by_id(id).await {
        Ok(Some(d)) => d,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Document not found")),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to find document: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to find document",
                )),
            ));
        }
    };

    // Only creator or manager can delete
    if existing.created_by != auth.user_id && !tenant.role.is_manager() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse::new(
                "FORBIDDEN",
                "Only the document creator or managers can delete",
            )),
        ));
    }

    match state.document_repo.delete(id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => {
            tracing::error!("Failed to delete document: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to delete document",
                )),
            ))
        }
    }
}

/// Move a document to a folder.
#[utoipa::path(
    post,
    path = "/api/v1/documents/{id}/move",
    params(
        ("id" = Uuid, Path, description = "Document ID")
    ),
    request_body = MoveDocumentRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Document moved", body = DocumentActionResponse),
        (status = 404, description = "Document not found", body = ErrorResponse),
    ),
    tag = "Documents"
)]
async fn move_document(
    State(state): State<AppState>,
    auth: AuthUser,
    tenant: TenantExtractor,
    Path(id): Path<Uuid>,
    Json(req): Json<MoveDocumentRequest>,
) -> Result<Json<DocumentActionResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Check document exists
    let existing = match state.document_repo.find_by_id(id).await {
        Ok(Some(d)) => d,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Document not found")),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to find document: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to find document",
                )),
            ));
        }
    };

    // Only creator or manager can move
    if existing.created_by != auth.user_id && !tenant.role.is_manager() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse::new(
                "FORBIDDEN",
                "Only the document creator or managers can move",
            )),
        ));
    }

    let data = MoveDocument {
        document_id: id,
        folder_id: req.folder_id,
    };

    match state.document_repo.move_document(data).await {
        Ok(document) => Ok(Json(DocumentActionResponse {
            message: "Document moved".to_string(),
            document,
        })),
        Err(e) => {
            tracing::error!("Failed to move document: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to move document",
                )),
            ))
        }
    }
}

/// Update document access permissions.
#[utoipa::path(
    put,
    path = "/api/v1/documents/{id}/access",
    params(
        ("id" = Uuid, Path, description = "Document ID")
    ),
    request_body = UpdateAccessRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Access updated", body = DocumentActionResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 404, description = "Document not found", body = ErrorResponse),
    ),
    tag = "Documents"
)]
async fn update_document_access(
    State(state): State<AppState>,
    auth: AuthUser,
    tenant: TenantExtractor,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateAccessRequest>,
) -> Result<Json<DocumentActionResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Check document exists
    let existing = match state.document_repo.find_by_id(id).await {
        Ok(Some(d)) => d,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Document not found")),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to find document: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to find document",
                )),
            ));
        }
    };

    // Only creator or manager can update access
    if existing.created_by != auth.user_id && !tenant.role.is_manager() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse::new(
                "FORBIDDEN",
                "Only the document creator or managers can update access",
            )),
        ));
    }

    // Validate access scope
    if !access_scope::ALL.contains(&req.access_scope.as_str()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new("BAD_REQUEST", "Invalid access_scope")),
        ));
    }

    let data = UpdateDocument {
        title: None,
        description: None,
        category: None,
        folder_id: None,
        access_scope: Some(req.access_scope),
        access_target_ids: req.access_target_ids,
        access_roles: req.access_roles,
    };

    match state.document_repo.update(id, data).await {
        Ok(document) => Ok(Json(DocumentActionResponse {
            message: "Document access updated".to_string(),
            document,
        })),
        Err(e) => {
            tracing::error!("Failed to update document access: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to update document access",
                )),
            ))
        }
    }
}

// ============================================================================
// Download/Preview Handlers (Story 7A.4)
// ============================================================================

/// Get download URL for a document.
#[utoipa::path(
    get,
    path = "/api/v1/documents/{id}/download",
    params(
        ("id" = Uuid, Path, description = "Document ID")
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Download URL", body = UrlResponse),
        (status = 404, description = "Document not found", body = ErrorResponse),
    ),
    tag = "Documents"
)]
async fn get_download_url(
    State(state): State<AppState>,
    _auth: AuthUser,
    _tenant: TenantExtractor,
    Path(id): Path<Uuid>,
) -> Result<Json<UrlResponse>, (StatusCode, Json<ErrorResponse>)> {
    let document = match state.document_repo.find_by_id(id).await {
        Ok(Some(d)) => d,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Document not found")),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to find document: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to find document",
                )),
            ));
        }
    };

    // TODO: Generate actual S3 presigned URL
    // For now, return a placeholder
    let expires_at = chrono::Utc::now() + chrono::Duration::minutes(15);
    let url = format!("/api/v1/storage/{}", document.file_key);

    Ok(Json(UrlResponse { url, expires_at }))
}

/// Get preview URL for a document.
#[utoipa::path(
    get,
    path = "/api/v1/documents/{id}/preview",
    params(
        ("id" = Uuid, Path, description = "Document ID")
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Preview URL", body = UrlResponse),
        (status = 404, description = "Document not found", body = ErrorResponse),
    ),
    tag = "Documents"
)]
async fn get_preview_url(
    State(state): State<AppState>,
    _auth: AuthUser,
    _tenant: TenantExtractor,
    Path(id): Path<Uuid>,
) -> Result<Json<UrlResponse>, (StatusCode, Json<ErrorResponse>)> {
    let document = match state.document_repo.find_by_id(id).await {
        Ok(Some(d)) => d,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Document not found")),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to find document: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to find document",
                )),
            ));
        }
    };

    if !document.supports_preview() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "PREVIEW_NOT_SUPPORTED",
                "Preview is not supported for this file type. Use download instead.",
            )),
        ));
    }

    // TODO: Generate actual S3 presigned URL for inline viewing
    let expires_at = chrono::Utc::now() + chrono::Duration::hours(1);
    let url = format!("/api/v1/storage/preview/{}", document.file_key);

    Ok(Json(UrlResponse { url, expires_at }))
}

// ============================================================================
// Version Handlers (Story 7B.1)
// ============================================================================

/// Get version history for a document.
#[utoipa::path(
    get,
    path = "/api/v1/documents/{id}/versions",
    params(
        ("id" = Uuid, Path, description = "Document ID")
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Version history", body = VersionHistoryResponse),
        (status = 404, description = "Document not found", body = ErrorResponse),
    ),
    tag = "Documents"
)]
async fn get_version_history(
    State(state): State<AppState>,
    _auth: AuthUser,
    _tenant: TenantExtractor,
    Path(id): Path<Uuid>,
) -> Result<Json<VersionHistoryResponse>, (StatusCode, Json<ErrorResponse>)> {
    match state.document_repo.get_version_history(id).await {
        Ok(history) => Ok(Json(VersionHistoryResponse { history })),
        Err(sqlx::Error::RowNotFound) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Document not found")),
        )),
        Err(e) => {
            tracing::error!("Failed to get version history: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to get version history",
                )),
            ))
        }
    }
}

/// Upload a new version of a document.
#[utoipa::path(
    post,
    path = "/api/v1/documents/{id}/versions",
    params(
        ("id" = Uuid, Path, description = "Document ID")
    ),
    request_body = UploadVersionRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 201, description = "Version created", body = CreateVersionResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 404, description = "Document not found", body = ErrorResponse),
    ),
    tag = "Documents"
)]
async fn create_version(
    State(state): State<AppState>,
    auth: AuthUser,
    tenant: TenantExtractor,
    Path(id): Path<Uuid>,
    Json(req): Json<UploadVersionRequest>,
) -> Result<(StatusCode, Json<CreateVersionResponse>), (StatusCode, Json<ErrorResponse>)> {
    let user_id = auth.user_id;

    // Check document exists
    let existing = match state.document_repo.find_by_id(id).await {
        Ok(Some(d)) => d,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Document not found")),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to find document: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to find document",
                )),
            ));
        }
    };

    // Only creator or manager can upload new versions
    if existing.created_by != user_id && !tenant.role.is_manager() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse::new(
                "FORBIDDEN",
                "Only the document creator or managers can upload new versions",
            )),
        ));
    }

    // Validate file size
    if req.size_bytes > MAX_FILE_SIZE {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "FILE_TOO_LARGE",
                format!(
                    "File exceeds maximum size of {} bytes (50MB)",
                    MAX_FILE_SIZE
                ),
            )),
        ));
    }

    // Validate MIME type
    if !ALLOWED_MIME_TYPES.contains(&req.mime_type.as_str()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "UNSUPPORTED_FILE_TYPE",
                format!(
                    "File type '{}' is not supported",
                    req.mime_type
                ),
            )),
        ));
    }

    let data = CreateDocumentVersion {
        file_key: req.file_key,
        file_name: req.file_name,
        mime_type: req.mime_type,
        size_bytes: req.size_bytes,
        created_by: user_id,
    };

    match state.document_repo.create_version(id, data).await {
        Ok(new_version) => Ok((
            StatusCode::CREATED,
            Json(CreateVersionResponse {
                id: new_version.id,
                version_number: new_version.version_number,
                message: format!("Version {} created successfully", new_version.version_number),
            }),
        )),
        Err(e) => {
            tracing::error!("Failed to create version: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to create version",
                )),
            ))
        }
    }
}

/// Get a specific version of a document.
#[utoipa::path(
    get,
    path = "/api/v1/documents/{id}/versions/{version_id}",
    params(
        ("id" = Uuid, Path, description = "Document ID"),
        ("version_id" = Uuid, Path, description = "Version ID")
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Version details", body = VersionResponse),
        (status = 404, description = "Version not found", body = ErrorResponse),
    ),
    tag = "Documents"
)]
async fn get_version(
    State(state): State<AppState>,
    _auth: AuthUser,
    _tenant: TenantExtractor,
    Path((id, version_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<VersionResponse>, (StatusCode, Json<ErrorResponse>)> {
    match state.document_repo.get_version(id, version_id).await {
        Ok(Some(version)) => Ok(Json(VersionResponse { version })),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Version not found")),
        )),
        Err(e) => {
            tracing::error!("Failed to get version: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to get version",
                )),
            ))
        }
    }
}

/// Restore a previous version to become the current version.
#[utoipa::path(
    post,
    path = "/api/v1/documents/{id}/versions/{version_id}/restore",
    params(
        ("id" = Uuid, Path, description = "Document ID"),
        ("version_id" = Uuid, Path, description = "Version ID to restore")
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 201, description = "Version restored", body = CreateVersionResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 404, description = "Version not found", body = ErrorResponse),
    ),
    tag = "Documents"
)]
async fn restore_version(
    State(state): State<AppState>,
    auth: AuthUser,
    tenant: TenantExtractor,
    Path((id, version_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<CreateVersionResponse>), (StatusCode, Json<ErrorResponse>)> {
    let user_id = auth.user_id;

    // Check document exists
    let existing = match state.document_repo.find_by_id(id).await {
        Ok(Some(d)) => d,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Document not found")),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to find document: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to find document",
                )),
            ));
        }
    };

    // Only creator or manager can restore versions
    if existing.created_by != user_id && !tenant.role.is_manager() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse::new(
                "FORBIDDEN",
                "Only the document creator or managers can restore versions",
            )),
        ));
    }

    // Check version exists in the same chain
    match state.document_repo.get_version(id, version_id).await {
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Version not found in this document")),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to find version: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to find version",
                )),
            ));
        }
        Ok(Some(_)) => {}
    }

    match state.document_repo.restore_version(id, version_id, user_id).await {
        Ok(new_version) => Ok((
            StatusCode::CREATED,
            Json(CreateVersionResponse {
                id: new_version.id,
                version_number: new_version.version_number,
                message: format!(
                    "Version restored successfully as version {}",
                    new_version.version_number
                ),
            }),
        )),
        Err(sqlx::Error::RowNotFound) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Version not found")),
        )),
        Err(e) => {
            tracing::error!("Failed to restore version: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to restore version",
                )),
            ))
        }
    }
}

// ============================================================================
// Folder Handlers (Story 7A.2)
// ============================================================================

/// List folders.
#[utoipa::path(
    get,
    path = "/api/v1/documents/folders",
    params(ListFoldersQuery),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Folder list", body = FolderListResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
    tag = "Documents"
)]
async fn list_folders(
    State(state): State<AppState>,
    _auth: AuthUser,
    tenant: TenantExtractor,
    Query(query): Query<ListFoldersQuery>,
) -> Result<Json<FolderListResponse>, (StatusCode, Json<ErrorResponse>)> {
    let org_id = tenant.tenant_id;

    match state
        .document_repo
        .get_folders(org_id, query.parent_id)
        .await
    {
        Ok(folders) => Ok(Json(FolderListResponse { folders })),
        Err(e) => {
            tracing::error!("Failed to list folders: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to list folders",
                )),
            ))
        }
    }
}

/// Get folder tree.
#[utoipa::path(
    get,
    path = "/api/v1/documents/folders/tree",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Folder tree", body = FolderTreeResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
    tag = "Documents"
)]
async fn get_folder_tree(
    State(state): State<AppState>,
    _auth: AuthUser,
    tenant: TenantExtractor,
) -> Result<Json<FolderTreeResponse>, (StatusCode, Json<ErrorResponse>)> {
    let org_id = tenant.tenant_id;

    match state.document_repo.get_folder_tree(org_id).await {
        Ok(tree) => Ok(Json(FolderTreeResponse { tree })),
        Err(e) => {
            tracing::error!("Failed to get folder tree: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to get folder tree",
                )),
            ))
        }
    }
}

/// Create a folder.
#[utoipa::path(
    post,
    path = "/api/v1/documents/folders",
    request_body = CreateFolderRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 201, description = "Folder created", body = CreateFolderResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 403, description = "Forbidden - requires manager role", body = ErrorResponse),
    ),
    tag = "Documents"
)]
async fn create_folder(
    State(state): State<AppState>,
    auth: AuthUser,
    tenant: TenantExtractor,
    Json(req): Json<CreateFolderRequest>,
) -> Result<(StatusCode, Json<CreateFolderResponse>), (StatusCode, Json<ErrorResponse>)> {
    // Only managers can create folders
    if !tenant.role.is_manager() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse::new(
                "FORBIDDEN",
                "Only managers can create folders",
            )),
        ));
    }

    // Validate name
    if req.name.is_empty() || req.name.len() > MAX_FOLDER_NAME_LENGTH {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "BAD_REQUEST",
                format!(
                    "Folder name must be 1-{} characters",
                    MAX_FOLDER_NAME_LENGTH
                ),
            )),
        ));
    }

    let org_id = tenant.tenant_id;
    let user_id = auth.user_id;

    let data = CreateFolder {
        organization_id: org_id,
        parent_id: req.parent_id,
        name: req.name,
        description: req.description,
        created_by: user_id,
    };

    match state.document_repo.create_folder(data).await {
        Ok(folder) => Ok((
            StatusCode::CREATED,
            Json(CreateFolderResponse {
                id: folder.id,
                message: "Folder created successfully".to_string(),
            }),
        )),
        Err(e) => {
            // Check for depth violation
            if e.to_string().contains("Maximum folder depth") {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse::new(
                        "MAX_DEPTH_EXCEEDED",
                        "Maximum folder depth of 5 levels exceeded",
                    )),
                ));
            }
            tracing::error!("Failed to create folder: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to create folder",
                )),
            ))
        }
    }
}

/// Get folder details.
#[utoipa::path(
    get,
    path = "/api/v1/documents/folders/{id}",
    params(
        ("id" = Uuid, Path, description = "Folder ID")
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Folder details", body = FolderDetailResponse),
        (status = 404, description = "Folder not found", body = ErrorResponse),
    ),
    tag = "Documents"
)]
async fn get_folder(
    State(state): State<AppState>,
    _auth: AuthUser,
    _tenant: TenantExtractor,
    Path(id): Path<Uuid>,
) -> Result<Json<FolderDetailResponse>, (StatusCode, Json<ErrorResponse>)> {
    match state.document_repo.find_folder_by_id(id).await {
        Ok(Some(folder)) => Ok(Json(FolderDetailResponse { folder })),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Folder not found")),
        )),
        Err(e) => {
            tracing::error!("Failed to get folder: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to get folder")),
            ))
        }
    }
}

/// Update a folder.
#[utoipa::path(
    put,
    path = "/api/v1/documents/folders/{id}",
    params(
        ("id" = Uuid, Path, description = "Folder ID")
    ),
    request_body = UpdateFolderRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Folder updated", body = FolderActionResponse),
        (status = 403, description = "Forbidden - requires manager role", body = ErrorResponse),
        (status = 404, description = "Folder not found", body = ErrorResponse),
    ),
    tag = "Documents"
)]
async fn update_folder(
    State(state): State<AppState>,
    _auth: AuthUser,
    tenant: TenantExtractor,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateFolderRequest>,
) -> Result<Json<FolderActionResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Only managers can update folders
    if !tenant.role.is_manager() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse::new(
                "FORBIDDEN",
                "Only managers can update folders",
            )),
        ));
    }

    // Check folder exists
    match state.document_repo.find_folder_by_id(id).await {
        Ok(Some(_)) => {}
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Folder not found")),
            ));
        }
        Err(e) => {
            tracing::error!("Failed to find folder: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to find folder",
                )),
            ));
        }
    }

    // Validate name if provided
    if let Some(ref name) = req.name {
        if name.is_empty() || name.len() > MAX_FOLDER_NAME_LENGTH {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new(
                    "BAD_REQUEST",
                    format!(
                        "Folder name must be 1-{} characters",
                        MAX_FOLDER_NAME_LENGTH
                    ),
                )),
            ));
        }
    }

    // Validate parent_id to prevent circular references
    if let Some(new_parent_id) = req.parent_id {
        // Cannot set a folder as its own parent
        if new_parent_id == id {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new(
                    "CIRCULAR_REFERENCE",
                    "A folder cannot be its own parent",
                )),
            ));
        }

        // Check that new parent is not a descendant of this folder
        match state
            .document_repo
            .is_descendant_of(new_parent_id, id)
            .await
        {
            Ok(true) => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse::new(
                        "CIRCULAR_REFERENCE",
                        "Cannot move a folder into one of its descendants",
                    )),
                ));
            }
            Ok(false) => {}
            Err(e) => {
                tracing::error!("Failed to check folder hierarchy: {}", e);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse::new(
                        "INTERNAL_ERROR",
                        "Failed to validate folder hierarchy",
                    )),
                ));
            }
        }
    }

    let data = UpdateFolder {
        name: req.name,
        description: req.description,
        parent_id: req.parent_id,
    };

    match state.document_repo.update_folder(id, data).await {
        Ok(folder) => Ok(Json(FolderActionResponse {
            message: "Folder updated".to_string(),
            folder,
        })),
        Err(e) => {
            tracing::error!("Failed to update folder: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to update folder",
                )),
            ))
        }
    }
}

/// Delete a folder.
#[utoipa::path(
    delete,
    path = "/api/v1/documents/folders/{id}",
    params(
        ("id" = Uuid, Path, description = "Folder ID")
    ),
    request_body = Option<DeleteFolderRequest>,
    security(("bearer_auth" = [])),
    responses(
        (status = 204, description = "Folder deleted"),
        (status = 403, description = "Forbidden - requires manager role", body = ErrorResponse),
        (status = 404, description = "Folder not found", body = ErrorResponse),
    ),
    tag = "Documents"
)]
async fn delete_folder(
    State(state): State<AppState>,
    _auth: AuthUser,
    tenant: TenantExtractor,
    Path(id): Path<Uuid>,
    body: Option<Json<DeleteFolderRequest>>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    // Only managers can delete folders
    if !tenant.role.is_manager() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse::new(
                "FORBIDDEN",
                "Only managers can delete folders",
            )),
        ));
    }

    // Check folder exists
    match state.document_repo.find_folder_by_id(id).await {
        Ok(Some(_)) => {}
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Folder not found")),
            ));
        }
        Err(e) => {
            tracing::error!("Failed to find folder: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to find folder",
                )),
            ));
        }
    }

    let cascade = body.map(|b| b.cascade).unwrap_or(false);

    // Check if folder has documents and warn if not cascading
    if !cascade {
        let doc_count = state
            .document_repo
            .count_documents_in_folder(id)
            .await
            .unwrap_or(0);
        if doc_count > 0 {
            // Documents will be moved to root
            tracing::info!(
                folder_id = %id,
                document_count = doc_count,
                "Moving documents to root folder before deleting folder"
            );
        }
    }

    match state.document_repo.delete_folder(id, cascade).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => {
            tracing::error!("Failed to delete folder: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to delete folder",
                )),
            ))
        }
    }
}

// ============================================================================
// Share Handlers (Story 7A.5)
// ============================================================================

/// List shares for a document.
#[utoipa::path(
    get,
    path = "/api/v1/documents/{id}/shares",
    params(
        ("id" = Uuid, Path, description = "Document ID")
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Share list", body = ShareListResponse),
        (status = 404, description = "Document not found", body = ErrorResponse),
    ),
    tag = "Documents"
)]
async fn list_shares(
    State(state): State<AppState>,
    auth: AuthUser,
    tenant: TenantExtractor,
    Path(id): Path<Uuid>,
) -> Result<Json<ShareListResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Check document exists
    let existing = match state.document_repo.find_by_id(id).await {
        Ok(Some(d)) => d,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Document not found")),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to find document: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to find document",
                )),
            ));
        }
    };

    // Only creator or manager can view shares
    if existing.created_by != auth.user_id && !tenant.role.is_manager() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse::new(
                "FORBIDDEN",
                "Only the document creator or managers can view shares",
            )),
        ));
    }

    match state.document_repo.get_shares(id).await {
        Ok(shares) => Ok(Json(ShareListResponse { shares })),
        Err(e) => {
            tracing::error!("Failed to list shares: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to list shares",
                )),
            ))
        }
    }
}

/// Create a share.
#[utoipa::path(
    post,
    path = "/api/v1/documents/{id}/shares",
    params(
        ("id" = Uuid, Path, description = "Document ID")
    ),
    request_body = CreateShareRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 201, description = "Share created", body = CreateShareResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 404, description = "Document not found", body = ErrorResponse),
    ),
    tag = "Documents"
)]
async fn create_share(
    State(state): State<AppState>,
    auth: AuthUser,
    tenant: TenantExtractor,
    Path(id): Path<Uuid>,
    Json(req): Json<CreateShareRequest>,
) -> Result<(StatusCode, Json<CreateShareResponse>), (StatusCode, Json<ErrorResponse>)> {
    // Check document exists
    let existing = match state.document_repo.find_by_id(id).await {
        Ok(Some(d)) => d,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Document not found")),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to find document: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to find document",
                )),
            ));
        }
    };

    // Only creator or manager can create shares
    if existing.created_by != auth.user_id && !tenant.role.is_manager() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse::new(
                "FORBIDDEN",
                "Only the document creator or managers can create shares",
            )),
        ));
    }

    // Validate share type
    if !share_type::ALL.contains(&req.share_type.as_str()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new("BAD_REQUEST", "Invalid share_type")),
        ));
    }

    // Validate target_id for non-link shares
    if req.share_type != share_type::LINK && req.target_id.is_none() && req.target_role.is_none() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "BAD_REQUEST",
                "target_id or target_role required for non-link shares",
            )),
        ));
    }

    let data = CreateShare {
        document_id: id,
        share_type: req.share_type.clone(),
        target_id: req.target_id,
        target_role: req.target_role,
        shared_by: auth.user_id,
        password: req.password,
        expires_at: req.expires_at,
    };

    match state.document_repo.create_share(data).await {
        Ok(share) => {
            let share_url = share
                .share_token
                .as_ref()
                .map(|token| format!("/documents/shared/{}", token));

            Ok((
                StatusCode::CREATED,
                Json(CreateShareResponse {
                    id: share.id,
                    share_token: share.share_token,
                    share_url,
                    message: "Share created successfully".to_string(),
                }),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to create share: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to create share",
                )),
            ))
        }
    }
}

/// Revoke a share.
#[utoipa::path(
    delete,
    path = "/api/v1/documents/{id}/shares/{share_id}",
    params(
        ("id" = Uuid, Path, description = "Document ID"),
        ("share_id" = Uuid, Path, description = "Share ID")
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 204, description = "Share revoked"),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 404, description = "Share not found", body = ErrorResponse),
    ),
    tag = "Documents"
)]
async fn revoke_share(
    State(state): State<AppState>,
    auth: AuthUser,
    tenant: TenantExtractor,
    Path((id, share_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    // Check document exists
    let existing = match state.document_repo.find_by_id(id).await {
        Ok(Some(d)) => d,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Document not found")),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to find document: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to find document",
                )),
            ));
        }
    };

    // Only creator or manager can revoke shares
    if existing.created_by != auth.user_id && !tenant.role.is_manager() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse::new(
                "FORBIDDEN",
                "Only the document creator or managers can revoke shares",
            )),
        ));
    }

    // Check share exists
    match state.document_repo.find_share_by_id(share_id).await {
        Ok(Some(_)) => {}
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Share not found")),
            ));
        }
        Err(e) => {
            tracing::error!("Failed to find share: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to find share")),
            ));
        }
    }

    match state.document_repo.revoke_share(share_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => {
            tracing::error!("Failed to revoke share: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to revoke share",
                )),
            ))
        }
    }
}

// ============================================================================
// Public Share Access (No Auth Required)
// ============================================================================

/// Access a shared document via token.
#[utoipa::path(
    get,
    path = "/api/v1/documents/shared/{token}",
    params(
        ("token" = String, Path, description = "Share token")
    ),
    responses(
        (status = 200, description = "Shared document access", body = SharedDocumentResponse),
        (status = 401, description = "Password required", body = ErrorResponse),
        (status = 404, description = "Share not found or expired", body = ErrorResponse),
    ),
    tag = "Documents"
)]
async fn access_shared_document(
    State(state): State<AppState>,
    axum::extract::ConnectInfo(addr): axum::extract::ConnectInfo<std::net::SocketAddr>,
    Path(token): Path<String>,
) -> Result<Json<SharedDocumentResponse>, (StatusCode, Json<ErrorResponse>)> {
    let ip_address = addr.ip().to_string();
    // Find share by token
    let share = match state.document_repo.find_share_by_token(&token).await {
        Ok(Some(s)) => s,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new(
                    "NOT_FOUND",
                    "Share not found or expired",
                )),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to find share: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to access share",
                )),
            ));
        }
    };

    // Check if password protected
    if share.has_password() {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse::new(
                "PASSWORD_REQUIRED",
                "This share is password protected",
            )),
        ));
    }

    // Get document
    let document = match state.document_repo.find_by_id(share.document_id).await {
        Ok(Some(d)) => d,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Document not found")),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to find document: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to access document",
                )),
            ));
        }
    };

    // Log access
    let _ = state
        .document_repo
        .log_share_access(LogShareAccess {
            share_id: share.id,
            accessed_by: None,
            ip_address: Some(ip_address),
        })
        .await;

    // Generate URLs
    let download_url = format!("/api/v1/storage/{}", document.file_key);
    let preview_url = if document.supports_preview() {
        Some(format!("/api/v1/storage/preview/{}", document.file_key))
    } else {
        None
    };

    Ok(Json(SharedDocumentResponse {
        document: DocumentSummary {
            id: document.id,
            title: document.title,
            category: document.category,
            file_name: document.file_name,
            mime_type: document.mime_type,
            size_bytes: document.size_bytes,
            folder_id: document.folder_id,
            created_at: document.created_at,
        },
        download_url,
        preview_url,
    }))
}

/// Access a password-protected shared document.
#[utoipa::path(
    post,
    path = "/api/v1/documents/shared/{token}/access",
    params(
        ("token" = String, Path, description = "Share token")
    ),
    request_body = AccessShareRequest,
    responses(
        (status = 200, description = "Shared document access", body = SharedDocumentResponse),
        (status = 401, description = "Invalid password", body = ErrorResponse),
        (status = 404, description = "Share not found or expired", body = ErrorResponse),
    ),
    tag = "Documents"
)]
async fn access_protected_share(
    State(state): State<AppState>,
    axum::extract::ConnectInfo(addr): axum::extract::ConnectInfo<std::net::SocketAddr>,
    Path(token): Path<String>,
    Json(req): Json<AccessShareRequest>,
) -> Result<Json<SharedDocumentResponse>, (StatusCode, Json<ErrorResponse>)> {
    let ip_address = addr.ip().to_string();
    // Find share by token
    let share = match state.document_repo.find_share_by_token(&token).await {
        Ok(Some(s)) => s,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new(
                    "NOT_FOUND",
                    "Share not found or expired",
                )),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to find share: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to access share",
                )),
            ));
        }
    };

    // Verify password
    if !state
        .document_repo
        .verify_share_password(share.id, &req.password)
        .await
        .unwrap_or(false)
    {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse::new("INVALID_PASSWORD", "Invalid password")),
        ));
    }

    // Get document
    let document = match state.document_repo.find_by_id(share.document_id).await {
        Ok(Some(d)) => d,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Document not found")),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to find document: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to access document",
                )),
            ));
        }
    };

    // Log access
    let _ = state
        .document_repo
        .log_share_access(LogShareAccess {
            share_id: share.id,
            accessed_by: None,
            ip_address: Some(ip_address),
        })
        .await;

    // Generate URLs
    let download_url = format!("/api/v1/storage/{}", document.file_key);
    let preview_url = if document.supports_preview() {
        Some(format!("/api/v1/storage/preview/{}", document.file_key))
    } else {
        None
    };

    Ok(Json(SharedDocumentResponse {
        document: DocumentSummary {
            id: document.id,
            title: document.title,
            category: document.category,
            file_name: document.file_name,
            mime_type: document.mime_type,
            size_bytes: document.size_bytes,
            folder_id: document.folder_id,
            created_at: document.created_at,
        },
        download_url,
        preview_url,
    }))
}

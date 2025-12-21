//! Document model (Epic 7A: Basic Document Management).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

// ============================================================================
// Constants
// ============================================================================

/// Maximum file size in bytes (50MB).
pub const MAX_FILE_SIZE: i64 = 50 * 1024 * 1024;

/// Allowed MIME types for document upload.
pub const ALLOWED_MIME_TYPES: &[&str] = &[
    // Documents
    "application/pdf",
    "application/msword",
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    "application/vnd.ms-excel",
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    "text/plain",
    // Images
    "image/png",
    "image/jpeg",
    "image/gif",
    "image/webp",
];

/// Document category enum values.
pub mod document_category {
    pub const CONTRACTS: &str = "contracts";
    pub const INVOICES: &str = "invoices";
    pub const REPORTS: &str = "reports";
    pub const MANUALS: &str = "manuals";
    pub const CERTIFICATES: &str = "certificates";
    pub const OTHER: &str = "other";

    pub const ALL: &[&str] = &[CONTRACTS, INVOICES, REPORTS, MANUALS, CERTIFICATES, OTHER];
}

/// Access scope enum values.
pub mod access_scope {
    pub const ORGANIZATION: &str = "organization";
    pub const BUILDING: &str = "building";
    pub const UNIT: &str = "unit";
    pub const ROLE: &str = "role";
    pub const USERS: &str = "users";

    pub const ALL: &[&str] = &[ORGANIZATION, BUILDING, UNIT, ROLE, USERS];
}

/// Share type enum values.
pub mod share_type {
    pub const USER: &str = "user";
    pub const ROLE: &str = "role";
    pub const BUILDING: &str = "building";
    pub const LINK: &str = "link";

    pub const ALL: &[&str] = &[USER, ROLE, BUILDING, LINK];
}

// ============================================================================
// Document Folder (Story 7A.2)
// ============================================================================

/// Document folder entity from database.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct DocumentFolder {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl DocumentFolder {
    /// Check if folder is deleted (soft-deleted).
    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    /// Check if this is a root folder.
    pub fn is_root(&self) -> bool {
        self.parent_id.is_none()
    }
}

/// Folder with document count for display.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FolderWithCount {
    #[serde(flatten)]
    pub folder: DocumentFolder,
    pub document_count: i64,
    pub subfolder_count: i64,
}

/// Folder tree node for hierarchical display.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FolderTreeNode {
    pub id: Uuid,
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub document_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<FolderTreeNode>>,
}

/// Data for creating a folder.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateFolder {
    pub organization_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub created_by: Uuid,
}

/// Data for updating a folder.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateFolder {
    pub name: Option<String>,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
}

// ============================================================================
// Document (Story 7A.1)
// ============================================================================

/// Document entity from database.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Document {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub folder_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub category: String,
    pub file_key: String,
    pub file_name: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub access_scope: String,
    pub access_target_ids: serde_json::Value,
    pub access_roles: serde_json::Value,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Document {
    /// Check if document is deleted (soft-deleted).
    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    /// Check if MIME type supports inline preview.
    pub fn supports_preview(&self) -> bool {
        matches!(
            self.mime_type.as_str(),
            "application/pdf" | "image/png" | "image/jpeg" | "image/gif" | "image/webp"
        )
    }

    /// Check if this is an image file.
    pub fn is_image(&self) -> bool {
        self.mime_type.starts_with("image/")
    }

    /// Check if this is a PDF file.
    pub fn is_pdf(&self) -> bool {
        self.mime_type == "application/pdf"
    }

    /// Get file extension from file name.
    pub fn extension(&self) -> Option<&str> {
        self.file_name.rsplit('.').next()
    }
}

/// Document summary for list display.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct DocumentSummary {
    pub id: Uuid,
    pub title: String,
    pub category: String,
    pub file_name: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub folder_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

/// Document with additional details.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DocumentWithDetails {
    #[serde(flatten)]
    pub document: Document,
    pub created_by_name: String,
    pub folder_name: Option<String>,
    pub share_count: i64,
}

/// Data for creating a document.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateDocument {
    pub organization_id: Uuid,
    pub folder_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub category: String,
    pub file_key: String,
    pub file_name: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub access_scope: Option<String>,
    pub access_target_ids: Option<Vec<Uuid>>,
    pub access_roles: Option<Vec<String>>,
    pub created_by: Uuid,
}

/// Data for updating a document.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateDocument {
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub folder_id: Option<Uuid>,
    pub access_scope: Option<String>,
    pub access_target_ids: Option<Vec<Uuid>>,
    pub access_roles: Option<Vec<String>>,
}

/// Query for listing documents.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
pub struct DocumentListQuery {
    pub folder_id: Option<Uuid>,
    pub category: Option<String>,
    pub created_by: Option<Uuid>,
    pub search: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Data for moving a document to a folder.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MoveDocument {
    pub document_id: Uuid,
    pub folder_id: Option<Uuid>,
}

// ============================================================================
// Document Share (Story 7A.5)
// ============================================================================

/// Document share entity from database.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct DocumentShare {
    pub id: Uuid,
    pub document_id: Uuid,
    pub share_type: String,
    pub target_id: Option<Uuid>,
    pub target_role: Option<String>,
    pub shared_by: Uuid,
    pub share_token: Option<String>,
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl DocumentShare {
    /// Check if share is active (not revoked and not expired).
    pub fn is_active(&self) -> bool {
        if self.revoked_at.is_some() {
            return false;
        }
        if let Some(expires_at) = self.expires_at {
            if expires_at < Utc::now() {
                return false;
            }
        }
        true
    }

    /// Check if share is a link share.
    pub fn is_link_share(&self) -> bool {
        self.share_type == share_type::LINK
    }

    /// Check if share has password protection.
    pub fn has_password(&self) -> bool {
        self.password_hash.is_some()
    }
}

/// Share with document info for display.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ShareWithDocument {
    #[serde(flatten)]
    pub share: DocumentShare,
    pub document_title: String,
    pub shared_by_name: String,
}

/// Data for creating a share.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateShare {
    pub document_id: Uuid,
    pub share_type: String,
    pub target_id: Option<Uuid>,
    pub target_role: Option<String>,
    pub shared_by: Uuid,
    pub password: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Data for revoking a share.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RevokeShare {
    pub share_id: Uuid,
}

// ============================================================================
// Document Share Access Log
// ============================================================================

/// Access log entry for document shares.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct ShareAccessLog {
    pub id: Uuid,
    pub share_id: Uuid,
    pub accessed_by: Option<Uuid>,
    pub accessed_at: DateTime<Utc>,
    pub ip_address: Option<String>,
}

/// Data for logging share access.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogShareAccess {
    pub share_id: Uuid,
    pub accessed_by: Option<Uuid>,
    pub ip_address: Option<String>,
}

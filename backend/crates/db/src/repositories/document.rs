//! Document repository (Epic 7A: Basic Document Management, Epic 7B: Document Versioning).

use crate::models::{
    access_scope, CreateDocument, CreateDocumentVersion, CreateFolder, CreateShare, Document,
    DocumentFolder, DocumentListQuery, DocumentShare, DocumentSummary, DocumentVersion,
    DocumentVersionHistory, DocumentWithDetails, FolderTreeNode, FolderWithCount, LogShareAccess,
    MoveDocument, ShareAccessLog, ShareWithDocument, UpdateDocument, UpdateFolder,
};
use sqlx::{Error as SqlxError, PgPool, Row};
use uuid::Uuid;

/// Repository for document operations.
#[derive(Debug, Clone)]
pub struct DocumentRepository {
    pool: PgPool,
}

impl DocumentRepository {
    /// Create a new document repository.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // ========================================================================
    // Folder Operations (Story 7A.2)
    // ========================================================================

    /// Create a new folder.
    pub async fn create_folder(&self, data: CreateFolder) -> Result<DocumentFolder, SqlxError> {
        sqlx::query_as::<_, DocumentFolder>(
            r#"
            INSERT INTO document_folders (organization_id, parent_id, name, description, created_by)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(data.organization_id)
        .bind(data.parent_id)
        .bind(&data.name)
        .bind(&data.description)
        .bind(data.created_by)
        .fetch_one(&self.pool)
        .await
    }

    /// Find folder by ID.
    pub async fn find_folder_by_id(&self, id: Uuid) -> Result<Option<DocumentFolder>, SqlxError> {
        sqlx::query_as::<_, DocumentFolder>(
            r#"
            SELECT * FROM document_folders
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Get all folders for an organization.
    pub async fn get_folders(
        &self,
        org_id: Uuid,
        parent_id: Option<Uuid>,
    ) -> Result<Vec<FolderWithCount>, SqlxError> {
        let rows = sqlx::query(
            r#"
            SELECT
                f.id, f.organization_id, f.parent_id, f.name, f.description,
                f.created_by, f.created_at, f.updated_at, f.deleted_at,
                COALESCE(doc_count.count, 0)::bigint as document_count,
                COALESCE(sub_count.count, 0)::bigint as subfolder_count
            FROM document_folders f
            LEFT JOIN (
                SELECT folder_id, COUNT(*) as count
                FROM documents
                WHERE deleted_at IS NULL
                GROUP BY folder_id
            ) doc_count ON doc_count.folder_id = f.id
            LEFT JOIN (
                SELECT parent_id, COUNT(*) as count
                FROM document_folders
                WHERE deleted_at IS NULL
                GROUP BY parent_id
            ) sub_count ON sub_count.parent_id = f.id
            WHERE f.organization_id = $1
              AND f.deleted_at IS NULL
              AND (f.parent_id = $2 OR ($2 IS NULL AND f.parent_id IS NULL))
            ORDER BY f.name
            "#,
        )
        .bind(org_id)
        .bind(parent_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| FolderWithCount {
                folder: DocumentFolder {
                    id: r.get("id"),
                    organization_id: r.get("organization_id"),
                    parent_id: r.get("parent_id"),
                    name: r.get("name"),
                    description: r.get("description"),
                    created_by: r.get("created_by"),
                    created_at: r.get("created_at"),
                    updated_at: r.get("updated_at"),
                    deleted_at: r.get("deleted_at"),
                },
                document_count: r.get("document_count"),
                subfolder_count: r.get("subfolder_count"),
            })
            .collect())
    }

    /// Get folder tree for an organization.
    pub async fn get_folder_tree(&self, org_id: Uuid) -> Result<Vec<FolderTreeNode>, SqlxError> {
        let rows = sqlx::query(
            r#"
            WITH folder_counts AS (
                SELECT folder_id, COUNT(*) as doc_count
                FROM documents
                WHERE organization_id = $1 AND deleted_at IS NULL
                GROUP BY folder_id
            )
            SELECT
                f.id,
                f.name,
                f.parent_id,
                COALESCE(fc.doc_count, 0) as document_count
            FROM document_folders f
            LEFT JOIN folder_counts fc ON fc.folder_id = f.id
            WHERE f.organization_id = $1 AND f.deleted_at IS NULL
            ORDER BY f.parent_id NULLS FIRST, f.name
            "#,
        )
        .bind(org_id)
        .fetch_all(&self.pool)
        .await?;

        // Build tree structure
        let nodes: Vec<FolderTreeNode> = rows
            .iter()
            .map(|row| FolderTreeNode {
                id: row.get("id"),
                name: row.get("name"),
                parent_id: row.get("parent_id"),
                document_count: row.get("document_count"),
                children: None,
            })
            .collect();

        Ok(build_folder_tree(nodes))
    }

    /// Update a folder.
    pub async fn update_folder(
        &self,
        id: Uuid,
        data: UpdateFolder,
    ) -> Result<DocumentFolder, SqlxError> {
        sqlx::query_as::<_, DocumentFolder>(
            r#"
            UPDATE document_folders
            SET
                name = COALESCE($2, name),
                description = COALESCE($3, description),
                parent_id = COALESCE($4, parent_id),
                updated_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&data.name)
        .bind(&data.description)
        .bind(data.parent_id)
        .fetch_one(&self.pool)
        .await
    }

    /// Check if a folder is a descendant of another folder.
    /// Used to prevent circular references when updating parent_id.
    pub async fn is_descendant_of(
        &self,
        folder_id: Uuid,
        potential_ancestor_id: Uuid,
    ) -> Result<bool, SqlxError> {
        let row = sqlx::query(
            r#"
            WITH RECURSIVE ancestors AS (
                SELECT id, parent_id FROM document_folders WHERE id = $1
                UNION ALL
                SELECT f.id, f.parent_id FROM document_folders f
                JOIN ancestors a ON f.id = a.parent_id
                WHERE f.deleted_at IS NULL
            )
            SELECT EXISTS(SELECT 1 FROM ancestors WHERE id = $2) as is_descendant
            "#,
        )
        .bind(folder_id)
        .bind(potential_ancestor_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.get("is_descendant"))
    }

    /// Delete a folder (soft delete).
    pub async fn delete_folder(&self, id: Uuid, cascade: bool) -> Result<(), SqlxError> {
        if cascade {
            // Delete all documents in folder
            sqlx::query(
                r#"
                UPDATE documents
                SET deleted_at = NOW()
                WHERE folder_id = $1 AND deleted_at IS NULL
                "#,
            )
            .bind(id)
            .execute(&self.pool)
            .await?;

            // Delete all subfolders recursively
            sqlx::query(
                r#"
                WITH RECURSIVE subfolders AS (
                    SELECT id FROM document_folders WHERE id = $1
                    UNION ALL
                    SELECT f.id FROM document_folders f
                    JOIN subfolders s ON f.parent_id = s.id
                )
                UPDATE document_folders
                SET deleted_at = NOW()
                WHERE id IN (SELECT id FROM subfolders)
                "#,
            )
            .bind(id)
            .execute(&self.pool)
            .await?;
        } else {
            // Just delete the folder, move documents to root
            sqlx::query(
                r#"
                UPDATE documents
                SET folder_id = NULL
                WHERE folder_id = $1 AND deleted_at IS NULL
                "#,
            )
            .bind(id)
            .execute(&self.pool)
            .await?;

            sqlx::query(
                r#"
                UPDATE document_folders
                SET deleted_at = NOW()
                WHERE id = $1
                "#,
            )
            .bind(id)
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }

    /// Count documents in a folder.
    pub async fn count_documents_in_folder(&self, folder_id: Uuid) -> Result<i64, SqlxError> {
        let row = sqlx::query(
            r#"
            SELECT COUNT(*) as count
            FROM documents
            WHERE folder_id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(folder_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.get("count"))
    }

    // ========================================================================
    // Document Operations (Story 7A.1)
    // ========================================================================

    /// Create a new document.
    pub async fn create(&self, data: CreateDocument) -> Result<Document, SqlxError> {
        let access_target_ids =
            serde_json::to_value(data.access_target_ids.unwrap_or_default()).unwrap();
        let access_roles = serde_json::to_value(data.access_roles.unwrap_or_default()).unwrap();

        sqlx::query_as::<_, Document>(
            r#"
            INSERT INTO documents (
                organization_id, folder_id, title, description, category,
                file_key, file_name, mime_type, size_bytes,
                access_scope, access_target_ids, access_roles, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING *
            "#,
        )
        .bind(data.organization_id)
        .bind(data.folder_id)
        .bind(&data.title)
        .bind(&data.description)
        .bind(&data.category)
        .bind(&data.file_key)
        .bind(&data.file_name)
        .bind(&data.mime_type)
        .bind(data.size_bytes)
        .bind(
            data.access_scope
                .as_deref()
                .unwrap_or(access_scope::ORGANIZATION),
        )
        .bind(&access_target_ids)
        .bind(&access_roles)
        .bind(data.created_by)
        .fetch_one(&self.pool)
        .await
    }

    /// Find document by ID.
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Document>, SqlxError> {
        sqlx::query_as::<_, Document>(
            r#"
            SELECT * FROM documents
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Find document by ID with details.
    pub async fn find_by_id_with_details(
        &self,
        id: Uuid,
    ) -> Result<Option<DocumentWithDetails>, SqlxError> {
        let row = sqlx::query(
            r#"
            SELECT
                d.*,
                u.full_name as created_by_name,
                f.name as folder_name,
                COALESCE(s.share_count, 0) as share_count
            FROM documents d
            LEFT JOIN users u ON u.id = d.created_by
            LEFT JOIN document_folders f ON f.id = d.folder_id
            LEFT JOIN (
                SELECT document_id, COUNT(*) as share_count
                FROM document_shares
                WHERE revoked_at IS NULL
                GROUP BY document_id
            ) s ON s.document_id = d.id
            WHERE d.id = $1 AND d.deleted_at IS NULL
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| DocumentWithDetails {
            document: Document {
                id: r.get("id"),
                organization_id: r.get("organization_id"),
                folder_id: r.get("folder_id"),
                title: r.get("title"),
                description: r.get("description"),
                category: r.get("category"),
                file_key: r.get("file_key"),
                file_name: r.get("file_name"),
                mime_type: r.get("mime_type"),
                size_bytes: r.get("size_bytes"),
                access_scope: r.get("access_scope"),
                access_target_ids: r.get("access_target_ids"),
                access_roles: r.get("access_roles"),
                created_by: r.get("created_by"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
                deleted_at: r.get("deleted_at"),
                version_number: r.get("version_number"),
                parent_document_id: r.get("parent_document_id"),
                is_current_version: r.get("is_current_version"),
                template_id: r.get("template_id"),
                generation_metadata: r.get("generation_metadata"),
            },
            created_by_name: r.get("created_by_name"),
            folder_name: r.get("folder_name"),
            share_count: r.get("share_count"),
        }))
    }

    /// List documents for an organization with filters.
    pub async fn list(
        &self,
        org_id: Uuid,
        query: DocumentListQuery,
    ) -> Result<Vec<DocumentSummary>, SqlxError> {
        let limit = query.limit.unwrap_or(50).min(100);
        let offset = query.offset.unwrap_or(0);

        sqlx::query_as::<_, DocumentSummary>(
            r#"
            SELECT
                id, title, category, file_name, mime_type, size_bytes, folder_id, created_at
            FROM documents
            WHERE organization_id = $1
              AND deleted_at IS NULL
              AND ($2::uuid IS NULL OR folder_id = $2)
              AND ($3::text IS NULL OR category = $3)
              AND ($4::uuid IS NULL OR created_by = $4)
              AND ($5::text IS NULL OR title ILIKE '%' || $5 || '%')
            ORDER BY created_at DESC
            LIMIT $6 OFFSET $7
            "#,
        )
        .bind(org_id)
        .bind(query.folder_id)
        .bind(&query.category)
        .bind(query.created_by)
        .bind(&query.search)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
    }

    /// Count documents matching query.
    pub async fn count(&self, org_id: Uuid, query: DocumentListQuery) -> Result<i64, SqlxError> {
        let row = sqlx::query(
            r#"
            SELECT COUNT(*) as count
            FROM documents
            WHERE organization_id = $1
              AND deleted_at IS NULL
              AND ($2::uuid IS NULL OR folder_id = $2)
              AND ($3::text IS NULL OR category = $3)
              AND ($4::uuid IS NULL OR created_by = $4)
              AND ($5::text IS NULL OR title ILIKE '%' || $5 || '%')
            "#,
        )
        .bind(org_id)
        .bind(query.folder_id)
        .bind(&query.category)
        .bind(query.created_by)
        .bind(&query.search)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.get("count"))
    }

    /// List documents accessible by a specific user (Story 7A.3).
    pub async fn list_accessible(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        user_building_ids: &[Uuid],
        user_unit_ids: &[Uuid],
        user_roles: &[String],
        query: DocumentListQuery,
    ) -> Result<Vec<DocumentSummary>, SqlxError> {
        let limit = query.limit.unwrap_or(50).min(100);
        let offset = query.offset.unwrap_or(0);

        let building_ids_json = serde_json::to_value(user_building_ids).unwrap();
        let unit_ids_json = serde_json::to_value(user_unit_ids).unwrap();
        let roles_json = serde_json::to_value(user_roles).unwrap();

        sqlx::query_as::<_, DocumentSummary>(
            r#"
            SELECT
                id, title, category, file_name, mime_type, size_bytes, folder_id, created_at
            FROM documents
            WHERE organization_id = $1
              AND deleted_at IS NULL
              AND (
                -- Creator always has access
                created_by = $2
                -- Organization-wide access
                OR access_scope = 'organization'
                -- Building-based access
                OR (access_scope = 'building' AND access_target_ids ?| $3)
                -- Unit-based access
                OR (access_scope = 'unit' AND access_target_ids ?| $4)
                -- Role-based access
                OR (access_scope = 'role' AND access_roles ?| $5)
                -- Specific user access
                OR (access_scope = 'users' AND access_target_ids ? $2::text)
              )
              AND ($6::uuid IS NULL OR folder_id = $6)
              AND ($7::text IS NULL OR category = $7)
              AND ($8::text IS NULL OR title ILIKE '%' || $8 || '%')
            ORDER BY created_at DESC
            LIMIT $9 OFFSET $10
            "#,
        )
        .bind(org_id)
        .bind(user_id)
        .bind(&building_ids_json)
        .bind(&unit_ids_json)
        .bind(&roles_json)
        .bind(query.folder_id)
        .bind(&query.category)
        .bind(&query.search)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
    }

    /// List documents accessible by user (simplified - org-wide + own documents + role-based).
    /// Used when building/unit context is not available.
    pub async fn list_accessible_simple(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        user_role: &str,
        query: DocumentListQuery,
    ) -> Result<Vec<DocumentSummary>, SqlxError> {
        let limit = query.limit.unwrap_or(50).min(100);
        let offset = query.offset.unwrap_or(0);

        sqlx::query_as::<_, DocumentSummary>(
            r#"
            SELECT
                id, title, category, file_name, mime_type, size_bytes, folder_id, created_at
            FROM documents
            WHERE organization_id = $1
              AND deleted_at IS NULL
              AND (
                -- Creator always has access
                created_by = $2
                -- Organization-wide access
                OR access_scope = 'organization'
                -- Role-based access (check if user's role is in the access_roles array)
                OR (access_scope = 'role' AND access_roles ? $3)
              )
              AND ($4::uuid IS NULL OR folder_id = $4)
              AND ($5::text IS NULL OR category = $5)
              AND ($6::text IS NULL OR title ILIKE '%' || $6 || '%')
            ORDER BY created_at DESC
            LIMIT $7 OFFSET $8
            "#,
        )
        .bind(org_id)
        .bind(user_id)
        .bind(user_role)
        .bind(query.folder_id)
        .bind(&query.category)
        .bind(&query.search)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
    }

    /// Count documents accessible by user (simplified).
    pub async fn count_accessible_simple(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        user_role: &str,
        query: DocumentListQuery,
    ) -> Result<i64, SqlxError> {
        let row = sqlx::query(
            r#"
            SELECT COUNT(*) as count
            FROM documents
            WHERE organization_id = $1
              AND deleted_at IS NULL
              AND (
                created_by = $2
                OR access_scope = 'organization'
                OR (access_scope = 'role' AND access_roles ? $3)
              )
              AND ($4::uuid IS NULL OR folder_id = $4)
              AND ($5::text IS NULL OR category = $5)
              AND ($6::text IS NULL OR title ILIKE '%' || $6 || '%')
            "#,
        )
        .bind(org_id)
        .bind(user_id)
        .bind(user_role)
        .bind(query.folder_id)
        .bind(&query.category)
        .bind(&query.search)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.get("count"))
    }

    /// Update a document.
    pub async fn update(&self, id: Uuid, data: UpdateDocument) -> Result<Document, SqlxError> {
        let access_target_ids = data
            .access_target_ids
            .map(|v| serde_json::to_value(v).unwrap());
        let access_roles = data.access_roles.map(|v| serde_json::to_value(v).unwrap());

        sqlx::query_as::<_, Document>(
            r#"
            UPDATE documents
            SET
                title = COALESCE($2, title),
                description = COALESCE($3, description),
                category = COALESCE($4, category),
                folder_id = COALESCE($5, folder_id),
                access_scope = COALESCE($6, access_scope),
                access_target_ids = COALESCE($7, access_target_ids),
                access_roles = COALESCE($8, access_roles),
                updated_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&data.title)
        .bind(&data.description)
        .bind(&data.category)
        .bind(data.folder_id)
        .bind(&data.access_scope)
        .bind(&access_target_ids)
        .bind(&access_roles)
        .fetch_one(&self.pool)
        .await
    }

    /// Move a document to a folder.
    pub async fn move_document(&self, data: MoveDocument) -> Result<Document, SqlxError> {
        sqlx::query_as::<_, Document>(
            r#"
            UPDATE documents
            SET folder_id = $2, updated_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING *
            "#,
        )
        .bind(data.document_id)
        .bind(data.folder_id)
        .fetch_one(&self.pool)
        .await
    }

    /// Delete a document (soft delete).
    pub async fn delete(&self, id: Uuid) -> Result<(), SqlxError> {
        sqlx::query(
            r#"
            UPDATE documents
            SET deleted_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Check if user has access to a document (Story 7A.3).
    pub async fn check_access(
        &self,
        document_id: Uuid,
        user_id: Uuid,
        user_building_ids: &[Uuid],
        user_unit_ids: &[Uuid],
        user_roles: &[String],
    ) -> Result<bool, SqlxError> {
        let building_ids_json = serde_json::to_value(user_building_ids).unwrap();
        let unit_ids_json = serde_json::to_value(user_unit_ids).unwrap();
        let roles_json = serde_json::to_value(user_roles).unwrap();

        let row = sqlx::query(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM documents
                WHERE id = $1
                  AND deleted_at IS NULL
                  AND (
                    created_by = $2
                    OR access_scope = 'organization'
                    OR (access_scope = 'building' AND access_target_ids ?| $3)
                    OR (access_scope = 'unit' AND access_target_ids ?| $4)
                    OR (access_scope = 'role' AND access_roles ?| $5)
                    OR (access_scope = 'users' AND access_target_ids ? $2::text)
                  )
            ) as has_access
            "#,
        )
        .bind(document_id)
        .bind(user_id)
        .bind(&building_ids_json)
        .bind(&unit_ids_json)
        .bind(&roles_json)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.get("has_access"))
    }

    // ========================================================================
    // Share Operations (Story 7A.5)
    // ========================================================================

    /// Create a new share.
    pub async fn create_share(&self, data: CreateShare) -> Result<DocumentShare, SqlxError> {
        let share_token = if data.share_type == crate::models::share_type::LINK {
            Some(generate_share_token())
        } else {
            None
        };

        let password_hash = match data.password.as_ref() {
            Some(password) => Some(hash_password(password)?),
            None => None,
        };

        sqlx::query_as::<_, DocumentShare>(
            r#"
            INSERT INTO document_shares (
                document_id, share_type, target_id, target_role, shared_by,
                share_token, password_hash, expires_at
            )
            VALUES ($1, $2::document_share_type, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#,
        )
        .bind(data.document_id)
        .bind(&data.share_type)
        .bind(data.target_id)
        .bind(&data.target_role)
        .bind(data.shared_by)
        .bind(&share_token)
        .bind(&password_hash)
        .bind(data.expires_at)
        .fetch_one(&self.pool)
        .await
    }

    /// Find share by ID.
    pub async fn find_share_by_id(&self, id: Uuid) -> Result<Option<DocumentShare>, SqlxError> {
        sqlx::query_as::<_, DocumentShare>(
            r#"
            SELECT * FROM document_shares
            WHERE id = $1 AND revoked_at IS NULL
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Find share by token.
    pub async fn find_share_by_token(
        &self,
        token: &str,
    ) -> Result<Option<DocumentShare>, SqlxError> {
        sqlx::query_as::<_, DocumentShare>(
            r#"
            SELECT * FROM document_shares
            WHERE share_token = $1
              AND revoked_at IS NULL
              AND (expires_at IS NULL OR expires_at > NOW())
            "#,
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await
    }

    /// Get shares for a document.
    pub async fn get_shares(&self, document_id: Uuid) -> Result<Vec<ShareWithDocument>, SqlxError> {
        let rows = sqlx::query(
            r#"
            SELECT
                s.*,
                d.title as document_title,
                u.full_name as shared_by_name
            FROM document_shares s
            JOIN documents d ON d.id = s.document_id
            JOIN users u ON u.id = s.shared_by
            WHERE s.document_id = $1 AND s.revoked_at IS NULL
            ORDER BY s.created_at DESC
            "#,
        )
        .bind(document_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| ShareWithDocument {
                share: DocumentShare {
                    id: r.get("id"),
                    document_id: r.get("document_id"),
                    share_type: r.get("share_type"),
                    target_id: r.get("target_id"),
                    target_role: r.get("target_role"),
                    shared_by: r.get("shared_by"),
                    share_token: r.get("share_token"),
                    password_hash: r.get("password_hash"),
                    expires_at: r.get("expires_at"),
                    revoked_at: r.get("revoked_at"),
                    created_at: r.get("created_at"),
                },
                document_title: r.get("document_title"),
                shared_by_name: r.get("shared_by_name"),
            })
            .collect())
    }

    /// Revoke a share.
    pub async fn revoke_share(&self, id: Uuid) -> Result<(), SqlxError> {
        sqlx::query(
            r#"
            UPDATE document_shares
            SET revoked_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Verify share password.
    pub async fn verify_share_password(
        &self,
        share_id: Uuid,
        password: &str,
    ) -> Result<bool, SqlxError> {
        let share = self.find_share_by_id(share_id).await?;
        match share {
            Some(s) => {
                if let Some(hash) = s.password_hash {
                    Ok(verify_password(password, &hash))
                } else {
                    Ok(true) // No password required
                }
            }
            None => Ok(false),
        }
    }

    /// Log share access.
    pub async fn log_share_access(
        &self,
        data: LogShareAccess,
    ) -> Result<ShareAccessLog, SqlxError> {
        sqlx::query_as::<_, ShareAccessLog>(
            r#"
            INSERT INTO document_share_access_log (share_id, accessed_by, ip_address)
            VALUES ($1, $2, $3::inet)
            RETURNING *
            "#,
        )
        .bind(data.share_id)
        .bind(data.accessed_by)
        .bind(&data.ip_address)
        .fetch_one(&self.pool)
        .await
    }

    /// Get share access log.
    pub async fn get_share_access_log(
        &self,
        share_id: Uuid,
        limit: Option<i64>,
    ) -> Result<Vec<ShareAccessLog>, SqlxError> {
        let limit = limit.unwrap_or(100).min(500);

        sqlx::query_as::<_, ShareAccessLog>(
            r#"
            SELECT * FROM document_share_access_log
            WHERE share_id = $1
            ORDER BY accessed_at DESC
            LIMIT $2
            "#,
        )
        .bind(share_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
    }

    // ========================================================================
    // Version Operations (Story 7B.1)
    // ========================================================================

    /// Create a new version of an existing document.
    ///
    /// This creates a new document record with:
    /// - An incremented version number
    /// - Reference to the original document (parent_document_id)
    /// - is_current_version set to true (previous versions are auto-updated to false via trigger)
    pub async fn create_version(
        &self,
        document_id: Uuid,
        data: CreateDocumentVersion,
    ) -> Result<Document, SqlxError> {
        // First, get the original document to copy metadata
        let original = self
            .find_by_id(document_id)
            .await?
            .ok_or_else(|| SqlxError::RowNotFound)?;

        // Determine the root document ID (the first version in the chain)
        let root_id = original.root_document_id();

        // Get the next version number using the database function
        let next_version: i32 = sqlx::query_scalar("SELECT get_next_document_version($1)")
            .bind(document_id)
            .fetch_one(&self.pool)
            .await?;

        // Create the new version with copied metadata
        sqlx::query_as::<_, Document>(
            r#"
            INSERT INTO documents (
                organization_id, folder_id, title, description, category,
                file_key, file_name, mime_type, size_bytes,
                access_scope, access_target_ids, access_roles, created_by,
                version_number, parent_document_id, is_current_version
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, true)
            RETURNING *
            "#,
        )
        .bind(original.organization_id)
        .bind(original.folder_id)
        .bind(&original.title)
        .bind(&original.description)
        .bind(&original.category)
        .bind(&data.file_key)
        .bind(&data.file_name)
        .bind(&data.mime_type)
        .bind(data.size_bytes)
        .bind(&original.access_scope)
        .bind(&original.access_target_ids)
        .bind(&original.access_roles)
        .bind(data.created_by)
        .bind(next_version)
        .bind(root_id)
        .fetch_one(&self.pool)
        .await
    }

    /// Get version history for a document.
    ///
    /// Returns all versions in the chain, ordered by version number (descending).
    pub async fn get_version_history(
        &self,
        document_id: Uuid,
    ) -> Result<DocumentVersionHistory, SqlxError> {
        // First get the document to find the root
        let doc = self
            .find_by_id(document_id)
            .await?
            .ok_or_else(|| SqlxError::RowNotFound)?;

        let root_id = doc.root_document_id();

        // Get all versions in the chain
        let versions = sqlx::query_as::<_, DocumentVersion>(
            r#"
            SELECT
                d.id,
                d.version_number,
                d.is_current_version,
                d.file_key,
                d.file_name,
                d.mime_type,
                d.size_bytes,
                d.created_by,
                CONCAT(u.first_name, ' ', u.last_name) as created_by_name,
                d.created_at
            FROM documents d
            JOIN users u ON u.id = d.created_by
            WHERE d.deleted_at IS NULL
              AND (d.id = $1 OR d.parent_document_id = $1)
            ORDER BY d.version_number DESC
            "#,
        )
        .bind(root_id)
        .fetch_all(&self.pool)
        .await?;

        let total_versions = versions.len() as i32;

        Ok(DocumentVersionHistory {
            document_id: root_id,
            title: doc.title,
            total_versions,
            versions,
        })
    }

    /// Get a specific version of a document.
    pub async fn get_version(
        &self,
        document_id: Uuid,
        version_id: Uuid,
    ) -> Result<Option<DocumentVersion>, SqlxError> {
        // First verify the version belongs to the document chain
        let doc = match self.find_by_id(document_id).await? {
            Some(d) => d,
            None => return Ok(None),
        };

        let root_id = doc.root_document_id();

        sqlx::query_as::<_, DocumentVersion>(
            r#"
            SELECT
                d.id,
                d.version_number,
                d.is_current_version,
                d.file_key,
                d.file_name,
                d.mime_type,
                d.size_bytes,
                d.created_by,
                CONCAT(u.first_name, ' ', u.last_name) as created_by_name,
                d.created_at
            FROM documents d
            JOIN users u ON u.id = d.created_by
            WHERE d.id = $1
              AND d.deleted_at IS NULL
              AND (d.id = $2 OR d.parent_document_id = $2)
            "#,
        )
        .bind(version_id)
        .bind(root_id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Restore a previous version to become the current version.
    ///
    /// This creates a new version entry with the content from the old version,
    /// making it non-destructive (preserving full history).
    pub async fn restore_version(
        &self,
        document_id: Uuid,
        version_id: Uuid,
        restored_by: Uuid,
    ) -> Result<Document, SqlxError> {
        // Get the version to restore
        let version_to_restore = self
            .find_by_id(version_id)
            .await?
            .ok_or_else(|| SqlxError::RowNotFound)?;

        // Verify it belongs to the same document chain
        let original_doc = self
            .find_by_id(document_id)
            .await?
            .ok_or_else(|| SqlxError::RowNotFound)?;

        if version_to_restore.root_document_id() != original_doc.root_document_id() {
            return Err(SqlxError::RowNotFound);
        }

        // Create a new version based on the old version's content
        let create_version_data = CreateDocumentVersion {
            file_key: version_to_restore.file_key,
            file_name: version_to_restore.file_name,
            mime_type: version_to_restore.mime_type,
            size_bytes: version_to_restore.size_bytes,
            created_by: restored_by,
        };

        self.create_version(document_id, create_version_data).await
    }

    /// Get the current (latest) version of a document.
    pub async fn get_current_version(&self, document_id: Uuid) -> Result<Option<Document>, SqlxError> {
        // Get the document to find the root
        let doc = match self.find_by_id(document_id).await? {
            Some(d) => d,
            None => return Ok(None),
        };

        let root_id = doc.root_document_id();

        sqlx::query_as::<_, Document>(
            r#"
            SELECT * FROM documents
            WHERE deleted_at IS NULL
              AND is_current_version = true
              AND (id = $1 OR parent_document_id = $1)
            "#,
        )
        .bind(root_id)
        .fetch_optional(&self.pool)
        .await
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Build folder tree from flat list.
fn build_folder_tree(nodes: Vec<FolderTreeNode>) -> Vec<FolderTreeNode> {
    use std::collections::HashMap;

    let mut node_map: HashMap<Uuid, FolderTreeNode> = HashMap::new();
    let mut root_ids: Vec<Uuid> = Vec::new();

    // First pass: create all nodes
    for node in nodes {
        if node.parent_id.is_none() {
            root_ids.push(node.id);
        }
        node_map.insert(node.id, node);
    }

    // Second pass: build parent-child relationships
    let mut children_map: HashMap<Uuid, Vec<FolderTreeNode>> = HashMap::new();
    for node in node_map.values() {
        if let Some(parent_id) = node.parent_id {
            children_map
                .entry(parent_id)
                .or_default()
                .push(node.clone());
        }
    }

    // Third pass: attach children to parents
    fn attach_children(
        node: &mut FolderTreeNode,
        children_map: &HashMap<Uuid, Vec<FolderTreeNode>>,
    ) {
        if let Some(children) = children_map.get(&node.id) {
            let mut child_nodes: Vec<FolderTreeNode> = children.clone();
            for child in &mut child_nodes {
                attach_children(child, children_map);
            }
            node.children = Some(child_nodes);
        }
    }

    root_ids
        .iter()
        .filter_map(|id| {
            node_map.get(id).cloned().map(|mut node| {
                attach_children(&mut node, &children_map);
                node
            })
        })
        .collect()
}

/// Generate a secure random share token.
fn generate_share_token() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    (0..32)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// Hash a password using Argon2.
fn hash_password(password: &str) -> Result<String, SqlxError> {
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2,
    };
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| {
            tracing::error!("Failed to hash password: {}", e);
            SqlxError::Protocol("Password hashing failed".to_string())
        })
}

/// Verify a password against a hash.
fn verify_password(password: &str, hash: &str) -> bool {
    use argon2::{
        password_hash::{PasswordHash, PasswordVerifier},
        Argon2,
    };
    let parsed_hash = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => return false,
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

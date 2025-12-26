//! External Integrations repository (Epic 61).
//!
//! Repository for calendar sync, accounting exports, e-signatures, video conferencing, and webhooks.
//!
//! # Security
//! This repository supports encryption of sensitive data (OAuth tokens, webhook secrets)
//! using AES-256-GCM. Enable encryption by setting the INTEGRATION_ENCRYPTION_KEY
//! environment variable to a 64-character hex string (32 bytes).
//!
//! Generate a key with: `openssl rand -hex 32`

use crate::models::integration::*;
use crate::DbPool;
use chrono::{Duration, Utc};
use integrations::{decrypt_if_available, encrypt_if_available, IntegrationCrypto};
use sqlx::Error as SqlxError;
use std::str::FromStr;
use uuid::Uuid;

/// Integration error types.
#[derive(Debug, thiserror::Error)]
pub enum IntegrationError {
    #[error("Database error: {0}")]
    Database(#[from] SqlxError),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Invalid provider: {0}")]
    InvalidProvider(String),

    #[error("Invalid webhook URL: {0}")]
    InvalidWebhookUrl(String),
}

/// Repository for external integration operations.
///
/// Supports optional encryption for sensitive data like OAuth tokens and webhook secrets.
/// If INTEGRATION_ENCRYPTION_KEY is not set, data will be stored unencrypted (dev mode).
#[derive(Clone)]
pub struct IntegrationRepository {
    pool: DbPool,
    /// Optional crypto service for encrypting/decrypting sensitive data.
    crypto: Option<IntegrationCrypto>,
}

impl IntegrationRepository {
    /// Create a new IntegrationRepository.
    ///
    /// Automatically attempts to initialize encryption from environment.
    /// If INTEGRATION_ENCRYPTION_KEY is not set, data will be stored unencrypted.
    pub fn new(pool: DbPool) -> Self {
        let crypto = IntegrationCrypto::try_from_env();
        if crypto.is_none() {
            tracing::warn!(
                "INTEGRATION_ENCRYPTION_KEY not set - OAuth tokens and webhook secrets \
                 will be stored in plaintext. Set this variable in production!"
            );
        }
        Self { pool, crypto }
    }

    /// Create a new IntegrationRepository with explicit crypto configuration.
    pub fn with_crypto(pool: DbPool, crypto: Option<IntegrationCrypto>) -> Self {
        Self { pool, crypto }
    }

    /// Encrypt a value using the configured crypto, or return plaintext if not configured.
    fn encrypt(&self, value: &str) -> String {
        encrypt_if_available(self.crypto.as_ref(), value)
    }

    /// Decrypt a value using the configured crypto, or return as-is if not configured.
    fn decrypt(&self, value: &str) -> String {
        decrypt_if_available(self.crypto.as_ref(), value)
    }

    /// Encrypt an optional value.
    fn encrypt_optional(&self, value: Option<&str>) -> Option<String> {
        value.map(|v| self.encrypt(v))
    }

    /// Decrypt an optional value from an Option<String>.
    fn decrypt_optional_owned(&self, value: Option<String>) -> Option<String> {
        value.map(|v| self.decrypt(&v))
    }

    /// Decrypt sensitive fields in a CalendarConnection.
    fn decrypt_calendar_connection(&self, mut conn: CalendarConnection) -> CalendarConnection {
        conn.access_token = self.decrypt_optional_owned(conn.access_token);
        conn.refresh_token = self.decrypt_optional_owned(conn.refresh_token);
        conn
    }

    /// Decrypt sensitive fields in a VideoConferenceConnection.
    fn decrypt_video_connection(
        &self,
        mut conn: VideoConferenceConnection,
    ) -> VideoConferenceConnection {
        conn.access_token = self.decrypt_optional_owned(conn.access_token);
        conn.refresh_token = self.decrypt_optional_owned(conn.refresh_token);
        conn
    }

    /// Decrypt sensitive fields in a WebhookSubscription.
    fn decrypt_webhook_subscription(&self, mut sub: WebhookSubscription) -> WebhookSubscription {
        sub.secret = self.decrypt_optional_owned(sub.secret);
        sub
    }

    // ========================================================================
    // Calendar Connections (Story 61.1)
    // ========================================================================

    /// Create a calendar connection.
    ///
    /// Validates that the provider is a valid calendar provider type.
    pub async fn create_calendar_connection(
        &self,
        organization_id: Uuid,
        user_id: Uuid,
        data: CreateCalendarConnection,
    ) -> Result<CalendarConnection, IntegrationError> {
        // Validate provider
        CalendarProvider::from_str(&data.provider).map_err(IntegrationError::InvalidProvider)?;

        sqlx::query_as::<_, CalendarConnection>(
            r#"
            INSERT INTO calendar_connections (
                organization_id, user_id, provider, calendar_id, sync_direction, sync_status
            )
            VALUES ($1, $2, $3, $4, COALESCE($5, 'bidirectional'), 'active')
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(user_id)
        .bind(&data.provider)
        .bind(&data.calendar_id)
        .bind(&data.sync_direction)
        .fetch_one(&self.pool)
        .await
        .map_err(IntegrationError::Database)
    }

    /// Get calendar connection by ID.
    ///
    /// Returns the connection with decrypted access_token and refresh_token.
    pub async fn get_calendar_connection(
        &self,
        id: Uuid,
    ) -> Result<Option<CalendarConnection>, SqlxError> {
        let conn = sqlx::query_as::<_, CalendarConnection>(
            "SELECT * FROM calendar_connections WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(conn.map(|c| self.decrypt_calendar_connection(c)))
    }

    /// List calendar connections for a user.
    ///
    /// Returns connections with decrypted access_token and refresh_token.
    pub async fn list_calendar_connections(
        &self,
        organization_id: Uuid,
        user_id: Option<Uuid>,
    ) -> Result<Vec<CalendarConnection>, SqlxError> {
        let connections = if let Some(uid) = user_id {
            sqlx::query_as::<_, CalendarConnection>(
                r#"
                SELECT * FROM calendar_connections
                WHERE organization_id = $1 AND user_id = $2
                ORDER BY created_at DESC
                "#,
            )
            .bind(organization_id)
            .bind(uid)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as::<_, CalendarConnection>(
                r#"
                SELECT * FROM calendar_connections
                WHERE organization_id = $1
                ORDER BY created_at DESC
                "#,
            )
            .bind(organization_id)
            .fetch_all(&self.pool)
            .await?
        };

        Ok(connections
            .into_iter()
            .map(|c| self.decrypt_calendar_connection(c))
            .collect())
    }

    /// Update calendar connection.
    ///
    /// Returns connection with decrypted tokens.
    pub async fn update_calendar_connection(
        &self,
        id: Uuid,
        data: UpdateCalendarConnection,
    ) -> Result<CalendarConnection, SqlxError> {
        let conn = sqlx::query_as::<_, CalendarConnection>(
            r#"
            UPDATE calendar_connections SET
                calendar_id = COALESCE($2, calendar_id),
                sync_direction = COALESCE($3, sync_direction),
                sync_status = COALESCE($4, sync_status),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&data.calendar_id)
        .bind(&data.sync_direction)
        .bind(&data.sync_status)
        .fetch_one(&self.pool)
        .await?;

        Ok(self.decrypt_calendar_connection(conn))
    }

    /// Delete calendar connection.
    pub async fn delete_calendar_connection(&self, id: Uuid) -> Result<bool, SqlxError> {
        let result = sqlx::query("DELETE FROM calendar_connections WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    /// Update calendar connection tokens.
    ///
    /// Encrypts access_token and refresh_token before storage.
    pub async fn update_calendar_tokens(
        &self,
        id: Uuid,
        access_token: &str,
        refresh_token: Option<&str>,
        expires_at: Option<chrono::DateTime<Utc>>,
    ) -> Result<(), SqlxError> {
        // Encrypt tokens before storage
        let encrypted_access = self.encrypt(access_token);
        let encrypted_refresh = self.encrypt_optional(refresh_token);

        sqlx::query(
            r#"
            UPDATE calendar_connections SET
                access_token = $2,
                refresh_token = COALESCE($3, refresh_token),
                token_expires_at = $4,
                updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(&encrypted_access)
        .bind(&encrypted_refresh)
        .bind(expires_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Update sync status.
    pub async fn update_sync_status(
        &self,
        id: Uuid,
        status: &str,
        error: Option<&str>,
    ) -> Result<(), SqlxError> {
        sqlx::query(
            r#"
            UPDATE calendar_connections SET
                sync_status = $2,
                last_sync_at = CASE WHEN $2 = 'active' THEN NOW() ELSE last_sync_at END,
                last_error = $3,
                updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(status)
        .bind(error)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    // ========================================================================
    // Calendar Events (Story 61.1)
    // ========================================================================

    /// Create a calendar event.
    pub async fn create_calendar_event(
        &self,
        data: CreateCalendarEvent,
    ) -> Result<CalendarEvent, SqlxError> {
        sqlx::query_as::<_, CalendarEvent>(
            r#"
            INSERT INTO calendar_events (
                connection_id, external_event_id, source_type, source_id, title, description,
                location, start_time, end_time, all_day, recurrence_rule, attendees
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, COALESCE($10, false), $11, $12)
            RETURNING *
            "#,
        )
        .bind(data.connection_id)
        .bind(&data.external_event_id)
        .bind(&data.source_type)
        .bind(data.source_id)
        .bind(&data.title)
        .bind(&data.description)
        .bind(&data.location)
        .bind(data.start_time)
        .bind(data.end_time)
        .bind(data.all_day)
        .bind(&data.recurrence_rule)
        .bind(&data.attendees)
        .fetch_one(&self.pool)
        .await
    }

    /// Upsert a calendar event - insert if external_event_id doesn't exist, skip if it does.
    /// Returns true if a new event was created, false if skipped (duplicate).
    pub async fn upsert_calendar_event(
        &self,
        data: CreateCalendarEvent,
    ) -> Result<bool, SqlxError> {
        // If external_event_id is provided, check for existing event first
        if let Some(ref external_id) = data.external_event_id {
            let existing = sqlx::query_scalar::<_, i64>(
                r#"
                SELECT COUNT(*) FROM calendar_events
                WHERE connection_id = $1 AND external_event_id = $2
                "#,
            )
            .bind(data.connection_id)
            .bind(external_id)
            .fetch_one(&self.pool)
            .await?;

            if existing > 0 {
                return Ok(false); // Event already exists, skip
            }
        }

        // Insert the new event
        self.create_calendar_event(data).await?;
        Ok(true)
    }

    /// List calendar events for a connection.
    pub async fn list_calendar_events(
        &self,
        connection_id: Uuid,
        from: Option<chrono::DateTime<Utc>>,
        to: Option<chrono::DateTime<Utc>>,
    ) -> Result<Vec<CalendarEvent>, SqlxError> {
        let from_date = from.unwrap_or_else(Utc::now);
        let to_date = to.unwrap_or_else(|| Utc::now() + Duration::days(30));

        sqlx::query_as::<_, CalendarEvent>(
            r#"
            SELECT * FROM calendar_events
            WHERE connection_id = $1
              AND start_time >= $2
              AND end_time <= $3
            ORDER BY start_time
            "#,
        )
        .bind(connection_id)
        .bind(from_date)
        .bind(to_date)
        .fetch_all(&self.pool)
        .await
    }

    // ========================================================================
    // Accounting Exports (Story 61.2)
    // ========================================================================

    /// Create an accounting export.
    ///
    /// Validates that the system_type is a valid accounting system.
    pub async fn create_accounting_export(
        &self,
        organization_id: Uuid,
        exported_by: Uuid,
        data: CreateAccountingExport,
    ) -> Result<AccountingExport, IntegrationError> {
        // Validate accounting system type
        AccountingSystem::from_str(&data.system_type).map_err(IntegrationError::InvalidProvider)?;

        sqlx::query_as::<_, AccountingExport>(
            r#"
            INSERT INTO accounting_exports (
                organization_id, system_type, export_type, period_start, period_end,
                status, exported_by
            )
            VALUES ($1, $2, $3, $4, $5, 'pending', $6)
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(&data.system_type)
        .bind(&data.export_type)
        .bind(data.period_start)
        .bind(data.period_end)
        .bind(exported_by)
        .fetch_one(&self.pool)
        .await
        .map_err(IntegrationError::Database)
    }

    /// Get accounting export by ID.
    pub async fn get_accounting_export(
        &self,
        id: Uuid,
    ) -> Result<Option<AccountingExport>, SqlxError> {
        sqlx::query_as::<_, AccountingExport>("SELECT * FROM accounting_exports WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    /// List accounting exports for an organization.
    pub async fn list_accounting_exports(
        &self,
        organization_id: Uuid,
        system_type: Option<&str>,
        limit: i32,
    ) -> Result<Vec<AccountingExport>, SqlxError> {
        if let Some(st) = system_type {
            sqlx::query_as::<_, AccountingExport>(
                r#"
                SELECT * FROM accounting_exports
                WHERE organization_id = $1 AND system_type = $2
                ORDER BY created_at DESC
                LIMIT $3
                "#,
            )
            .bind(organization_id)
            .bind(st)
            .bind(limit)
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query_as::<_, AccountingExport>(
                r#"
                SELECT * FROM accounting_exports
                WHERE organization_id = $1
                ORDER BY created_at DESC
                LIMIT $2
                "#,
            )
            .bind(organization_id)
            .bind(limit)
            .fetch_all(&self.pool)
            .await
        }
    }

    /// Update accounting export status.
    pub async fn update_accounting_export_status(
        &self,
        id: Uuid,
        status: &str,
        file_path: Option<&str>,
        file_size: Option<i64>,
        record_count: Option<i32>,
        error_message: Option<&str>,
    ) -> Result<AccountingExport, SqlxError> {
        sqlx::query_as::<_, AccountingExport>(
            r#"
            UPDATE accounting_exports SET
                status = $2,
                file_path = COALESCE($3, file_path),
                file_size = COALESCE($4, file_size),
                record_count = COALESCE($5, record_count),
                error_message = $6,
                completed_at = CASE WHEN $2 IN ('completed', 'failed') THEN NOW() ELSE completed_at END
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(status)
        .bind(file_path)
        .bind(file_size)
        .bind(record_count)
        .bind(error_message)
        .fetch_one(&self.pool)
        .await
    }

    /// Get or create accounting export settings.
    pub async fn get_accounting_export_settings(
        &self,
        organization_id: Uuid,
        system_type: &str,
    ) -> Result<AccountingExportSettings, SqlxError> {
        // Try to get existing settings
        let existing = sqlx::query_as::<_, AccountingExportSettings>(
            "SELECT * FROM accounting_export_settings WHERE organization_id = $1 AND system_type = $2",
        )
        .bind(organization_id)
        .bind(system_type)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(settings) = existing {
            Ok(settings)
        } else {
            // Create default settings
            sqlx::query_as::<_, AccountingExportSettings>(
                r#"
                INSERT INTO accounting_export_settings (organization_id, system_type, auto_export_enabled)
                VALUES ($1, $2, false)
                RETURNING *
                "#,
            )
            .bind(organization_id)
            .bind(system_type)
            .fetch_one(&self.pool)
            .await
        }
    }

    /// Update accounting export settings.
    pub async fn update_accounting_export_settings(
        &self,
        organization_id: Uuid,
        system_type: &str,
        data: UpdateAccountingExportSettings,
    ) -> Result<AccountingExportSettings, SqlxError> {
        sqlx::query_as::<_, AccountingExportSettings>(
            r#"
            UPDATE accounting_export_settings SET
                default_cost_center = COALESCE($3, default_cost_center),
                account_mappings = COALESCE($4, account_mappings),
                vat_settings = COALESCE($5, vat_settings),
                auto_export_enabled = COALESCE($6, auto_export_enabled),
                auto_export_schedule = COALESCE($7, auto_export_schedule),
                updated_at = NOW()
            WHERE organization_id = $1 AND system_type = $2
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(system_type)
        .bind(&data.default_cost_center)
        .bind(&data.account_mappings)
        .bind(&data.vat_settings)
        .bind(data.auto_export_enabled)
        .bind(&data.auto_export_schedule)
        .fetch_one(&self.pool)
        .await
    }

    // ========================================================================
    // E-Signature Workflows (Story 61.3)
    // ========================================================================

    /// Create an e-signature workflow.
    ///
    /// Validates that the provider (if specified) is a valid e-signature provider.
    pub async fn create_esignature_workflow(
        &self,
        organization_id: Uuid,
        created_by: Uuid,
        data: CreateESignatureWorkflow,
    ) -> Result<ESignatureWorkflow, IntegrationError> {
        // Validate provider if specified
        if let Some(ref provider) = data.provider {
            ESignatureProvider::from_str(provider).map_err(IntegrationError::InvalidProvider)?;
        }

        let expires_at = data
            .expires_in_days
            .map(|days| Utc::now() + Duration::days(days as i64));

        sqlx::query_as::<_, ESignatureWorkflow>(
            r#"
            INSERT INTO esignature_workflows (
                organization_id, document_id, provider, title, message, status,
                expires_at, reminder_enabled, reminder_days, created_by
            )
            VALUES ($1, $2, COALESCE($3, 'internal'), $4, $5, 'draft', $6, COALESCE($7, false), $8, $9)
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(data.document_id)
        .bind(&data.provider)
        .bind(&data.title)
        .bind(&data.message)
        .bind(expires_at)
        .bind(data.reminder_enabled)
        .bind(data.reminder_days)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(IntegrationError::Database)
    }

    /// Add recipient to e-signature workflow.
    pub async fn add_esignature_recipient(
        &self,
        workflow_id: Uuid,
        data: CreateESignatureRecipient,
    ) -> Result<ESignatureRecipient, SqlxError> {
        sqlx::query_as::<_, ESignatureRecipient>(
            r#"
            INSERT INTO esignature_recipients (
                workflow_id, email, name, role, signing_order, status
            )
            VALUES ($1, $2, $3, $4, COALESCE($5, 1), 'pending')
            RETURNING *
            "#,
        )
        .bind(workflow_id)
        .bind(&data.email)
        .bind(&data.name)
        .bind(&data.role)
        .bind(data.signing_order)
        .fetch_one(&self.pool)
        .await
    }

    /// Get e-signature workflow by ID.
    pub async fn get_esignature_workflow(
        &self,
        id: Uuid,
    ) -> Result<Option<ESignatureWorkflow>, SqlxError> {
        sqlx::query_as::<_, ESignatureWorkflow>("SELECT * FROM esignature_workflows WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    /// Get e-signature workflow with recipients.
    pub async fn get_esignature_workflow_with_recipients(
        &self,
        id: Uuid,
    ) -> Result<Option<ESignatureWorkflowWithRecipients>, SqlxError> {
        let workflow = self.get_esignature_workflow(id).await?;
        if let Some(w) = workflow {
            let recipients = self.list_esignature_recipients(id).await?;
            Ok(Some(ESignatureWorkflowWithRecipients {
                workflow: w,
                recipients,
            }))
        } else {
            Ok(None)
        }
    }

    /// List e-signature recipients for a workflow.
    pub async fn list_esignature_recipients(
        &self,
        workflow_id: Uuid,
    ) -> Result<Vec<ESignatureRecipient>, SqlxError> {
        sqlx::query_as::<_, ESignatureRecipient>(
            "SELECT * FROM esignature_recipients WHERE workflow_id = $1 ORDER BY signing_order",
        )
        .bind(workflow_id)
        .fetch_all(&self.pool)
        .await
    }

    /// List e-signature workflows for an organization.
    pub async fn list_esignature_workflows(
        &self,
        organization_id: Uuid,
        status: Option<&str>,
        limit: i32,
    ) -> Result<Vec<ESignatureWorkflow>, SqlxError> {
        if let Some(s) = status {
            sqlx::query_as::<_, ESignatureWorkflow>(
                r#"
                SELECT * FROM esignature_workflows
                WHERE organization_id = $1 AND status = $2
                ORDER BY created_at DESC
                LIMIT $3
                "#,
            )
            .bind(organization_id)
            .bind(s)
            .bind(limit)
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query_as::<_, ESignatureWorkflow>(
                r#"
                SELECT * FROM esignature_workflows
                WHERE organization_id = $1
                ORDER BY created_at DESC
                LIMIT $2
                "#,
            )
            .bind(organization_id)
            .bind(limit)
            .fetch_all(&self.pool)
            .await
        }
    }

    /// Update e-signature workflow status.
    pub async fn update_esignature_workflow_status(
        &self,
        id: Uuid,
        status: &str,
    ) -> Result<ESignatureWorkflow, SqlxError> {
        sqlx::query_as::<_, ESignatureWorkflow>(
            r#"
            UPDATE esignature_workflows SET
                status = $2,
                completed_at = CASE WHEN $2 IN ('completed', 'voided', 'expired') THEN NOW() ELSE completed_at END,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(status)
        .fetch_one(&self.pool)
        .await
    }

    /// Update e-signature recipient status.
    pub async fn update_esignature_recipient_status(
        &self,
        id: Uuid,
        status: &str,
        decline_reason: Option<&str>,
    ) -> Result<ESignatureRecipient, SqlxError> {
        sqlx::query_as::<_, ESignatureRecipient>(
            r#"
            UPDATE esignature_recipients SET
                status = $2,
                signed_at = CASE WHEN $2 = 'signed' THEN NOW() ELSE signed_at END,
                declined_at = CASE WHEN $2 = 'declined' THEN NOW() ELSE declined_at END,
                decline_reason = $3,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(status)
        .bind(decline_reason)
        .fetch_one(&self.pool)
        .await
    }

    /// Update e-signature workflow status by external envelope ID.
    ///
    /// Used by webhook handlers to update workflow status based on external provider events.
    pub async fn update_esignature_workflow_by_external_id(
        &self,
        external_envelope_id: &str,
        status: &str,
    ) -> Result<Option<ESignatureWorkflow>, SqlxError> {
        sqlx::query_as::<_, ESignatureWorkflow>(
            r#"
            UPDATE esignature_workflows SET
                status = $2,
                completed_at = CASE WHEN $2 IN ('completed', 'voided', 'expired', 'declined') THEN NOW() ELSE completed_at END,
                updated_at = NOW()
            WHERE external_envelope_id = $1
            RETURNING *
            "#,
        )
        .bind(external_envelope_id)
        .bind(status)
        .fetch_optional(&self.pool)
        .await
    }

    // ========================================================================
    // Video Conference Connections (Story 61.4)
    // ========================================================================

    /// Create a video conference connection.
    ///
    /// Validates that the provider is a valid video conferencing provider.
    pub async fn create_video_conference_connection(
        &self,
        organization_id: Uuid,
        user_id: Uuid,
        data: CreateVideoConferenceConnection,
    ) -> Result<VideoConferenceConnection, IntegrationError> {
        // Validate provider
        VideoProvider::from_str(&data.provider).map_err(IntegrationError::InvalidProvider)?;

        sqlx::query_as::<_, VideoConferenceConnection>(
            r#"
            INSERT INTO video_conference_connections (
                organization_id, user_id, provider, is_active
            )
            VALUES ($1, $2, $3, true)
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(user_id)
        .bind(&data.provider)
        .fetch_one(&self.pool)
        .await
        .map_err(IntegrationError::Database)
    }

    /// Get video conference connection by ID.
    ///
    /// Returns connection with decrypted access_token and refresh_token.
    pub async fn get_video_conference_connection(
        &self,
        id: Uuid,
    ) -> Result<Option<VideoConferenceConnection>, SqlxError> {
        let conn = sqlx::query_as::<_, VideoConferenceConnection>(
            "SELECT * FROM video_conference_connections WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(conn.map(|c| self.decrypt_video_connection(c)))
    }

    /// List video conference connections for a user.
    ///
    /// Returns connections with decrypted access_token and refresh_token.
    pub async fn list_video_conference_connections(
        &self,
        organization_id: Uuid,
        user_id: Option<Uuid>,
    ) -> Result<Vec<VideoConferenceConnection>, SqlxError> {
        let connections = if let Some(uid) = user_id {
            sqlx::query_as::<_, VideoConferenceConnection>(
                r#"
                SELECT * FROM video_conference_connections
                WHERE organization_id = $1 AND user_id = $2 AND is_active = true
                ORDER BY created_at DESC
                "#,
            )
            .bind(organization_id)
            .bind(uid)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as::<_, VideoConferenceConnection>(
                r#"
                SELECT * FROM video_conference_connections
                WHERE organization_id = $1 AND is_active = true
                ORDER BY created_at DESC
                "#,
            )
            .bind(organization_id)
            .fetch_all(&self.pool)
            .await?
        };

        Ok(connections
            .into_iter()
            .map(|c| self.decrypt_video_connection(c))
            .collect())
    }

    /// Update video conference connection tokens.
    ///
    /// Encrypts access_token and refresh_token before storage.
    pub async fn update_video_connection_tokens(
        &self,
        id: Uuid,
        access_token: &str,
        refresh_token: Option<&str>,
        expires_at: Option<chrono::DateTime<Utc>>,
    ) -> Result<(), SqlxError> {
        // Encrypt tokens before storage
        let encrypted_access = self.encrypt(access_token);
        let encrypted_refresh = self.encrypt_optional(refresh_token);

        sqlx::query(
            r#"
            UPDATE video_conference_connections SET
                access_token = $2,
                refresh_token = COALESCE($3, refresh_token),
                token_expires_at = $4,
                updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(&encrypted_access)
        .bind(&encrypted_refresh)
        .bind(expires_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Delete video conference connection.
    pub async fn delete_video_conference_connection(&self, id: Uuid) -> Result<bool, SqlxError> {
        let result = sqlx::query(
            "UPDATE video_conference_connections SET is_active = false, updated_at = NOW() WHERE id = $1",
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    // ========================================================================
    // Video Meetings (Story 61.4)
    // ========================================================================

    /// Create a video meeting.
    pub async fn create_video_meeting(
        &self,
        organization_id: Uuid,
        created_by: Uuid,
        data: CreateVideoMeeting,
    ) -> Result<VideoMeeting, SqlxError> {
        sqlx::query_as::<_, VideoMeeting>(
            r#"
            INSERT INTO video_meetings (
                organization_id, connection_id, source_type, source_id, title, description,
                start_time, duration_minutes, timezone, status, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'scheduled', $10)
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(data.connection_id)
        .bind(&data.source_type)
        .bind(data.source_id)
        .bind(&data.title)
        .bind(&data.description)
        .bind(data.start_time)
        .bind(data.duration_minutes)
        .bind(&data.timezone)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
    }

    /// Get video meeting by ID.
    pub async fn get_video_meeting(&self, id: Uuid) -> Result<Option<VideoMeeting>, SqlxError> {
        sqlx::query_as::<_, VideoMeeting>("SELECT * FROM video_meetings WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    /// List video meetings for an organization.
    pub async fn list_video_meetings(
        &self,
        organization_id: Uuid,
        from: Option<chrono::DateTime<Utc>>,
        status: Option<&str>,
        limit: i32,
    ) -> Result<Vec<VideoMeeting>, SqlxError> {
        let from_date = from.unwrap_or_else(Utc::now);

        if let Some(s) = status {
            sqlx::query_as::<_, VideoMeeting>(
                r#"
                SELECT * FROM video_meetings
                WHERE organization_id = $1 AND start_time >= $2 AND status = $3
                ORDER BY start_time
                LIMIT $4
                "#,
            )
            .bind(organization_id)
            .bind(from_date)
            .bind(s)
            .bind(limit)
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query_as::<_, VideoMeeting>(
                r#"
                SELECT * FROM video_meetings
                WHERE organization_id = $1 AND start_time >= $2
                ORDER BY start_time
                LIMIT $3
                "#,
            )
            .bind(organization_id)
            .bind(from_date)
            .bind(limit)
            .fetch_all(&self.pool)
            .await
        }
    }

    /// Update video meeting.
    pub async fn update_video_meeting(
        &self,
        id: Uuid,
        data: UpdateVideoMeeting,
    ) -> Result<VideoMeeting, SqlxError> {
        sqlx::query_as::<_, VideoMeeting>(
            r#"
            UPDATE video_meetings SET
                title = COALESCE($2, title),
                description = COALESCE($3, description),
                start_time = COALESCE($4, start_time),
                duration_minutes = COALESCE($5, duration_minutes),
                status = COALESCE($6, status),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&data.title)
        .bind(&data.description)
        .bind(data.start_time)
        .bind(data.duration_minutes)
        .bind(&data.status)
        .fetch_one(&self.pool)
        .await
    }

    /// Update video meeting join URLs.
    pub async fn update_video_meeting_urls(
        &self,
        id: Uuid,
        external_meeting_id: &str,
        join_url: &str,
        host_url: Option<&str>,
        password: Option<&str>,
    ) -> Result<VideoMeeting, SqlxError> {
        sqlx::query_as::<_, VideoMeeting>(
            r#"
            UPDATE video_meetings SET
                external_meeting_id = $2,
                join_url = $3,
                host_url = $4,
                password = $5,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(external_meeting_id)
        .bind(join_url)
        .bind(host_url)
        .bind(password)
        .fetch_one(&self.pool)
        .await
    }

    /// Delete video meeting.
    pub async fn delete_video_meeting(&self, id: Uuid) -> Result<bool, SqlxError> {
        let result = sqlx::query("DELETE FROM video_meetings WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    // ========================================================================
    // Webhook Subscriptions (Story 61.5)
    // ========================================================================

    /// Create a webhook subscription.
    ///
    /// Validates the webhook URL for security:
    /// - Must use HTTPS (HTTP only allowed for localhost in development)
    /// - Must not target private IP ranges
    /// - Must not target localhost in production
    ///
    /// Encrypts the webhook secret before storage.
    pub async fn create_webhook_subscription(
        &self,
        organization_id: Uuid,
        created_by: Uuid,
        data: CreateWebhookSubscription,
        is_production: bool,
    ) -> Result<WebhookSubscription, IntegrationError> {
        // Validate webhook URL
        let validation = validate_webhook_url(&data.url, is_production);
        if !validation.is_valid {
            return Err(IntegrationError::InvalidWebhookUrl(
                validation
                    .error
                    .unwrap_or_else(|| "Invalid URL".to_string()),
            ));
        }

        // Encrypt the secret before storage
        let encrypted_secret = self.encrypt_optional(data.secret.as_deref());

        let sub = sqlx::query_as::<_, WebhookSubscription>(
            r#"
            INSERT INTO webhook_subscriptions (
                organization_id, name, description, url, secret, events, status,
                headers, retry_policy, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, 'active', $7, $8, $9)
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(&data.name)
        .bind(&data.description)
        .bind(&data.url)
        .bind(&encrypted_secret)
        .bind(&data.events)
        .bind(&data.headers)
        .bind(serde_json::to_value(&data.retry_policy).ok())
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(IntegrationError::Database)?;

        Ok(self.decrypt_webhook_subscription(sub))
    }

    /// Get webhook subscription by ID.
    ///
    /// Returns subscription with decrypted secret.
    pub async fn get_webhook_subscription(
        &self,
        id: Uuid,
    ) -> Result<Option<WebhookSubscription>, SqlxError> {
        let sub = sqlx::query_as::<_, WebhookSubscription>(
            "SELECT * FROM webhook_subscriptions WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(sub.map(|s| self.decrypt_webhook_subscription(s)))
    }

    /// List webhook subscriptions for an organization.
    ///
    /// Returns subscriptions with decrypted secrets.
    pub async fn list_webhook_subscriptions(
        &self,
        organization_id: Uuid,
    ) -> Result<Vec<WebhookSubscription>, SqlxError> {
        let subs = sqlx::query_as::<_, WebhookSubscription>(
            r#"
            SELECT * FROM webhook_subscriptions
            WHERE organization_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(organization_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(subs
            .into_iter()
            .map(|s| self.decrypt_webhook_subscription(s))
            .collect())
    }

    /// Update webhook subscription.
    ///
    /// Validates the webhook URL if it's being updated.
    /// Encrypts the secret if it's being updated.
    pub async fn update_webhook_subscription(
        &self,
        id: Uuid,
        data: UpdateWebhookSubscription,
        is_production: bool,
    ) -> Result<WebhookSubscription, IntegrationError> {
        // Validate new URL if provided
        if let Some(ref url) = data.url {
            let validation = validate_webhook_url(url, is_production);
            if !validation.is_valid {
                return Err(IntegrationError::InvalidWebhookUrl(
                    validation
                        .error
                        .unwrap_or_else(|| "Invalid URL".to_string()),
                ));
            }
        }

        // Encrypt the new secret if provided
        let encrypted_secret = self.encrypt_optional(data.secret.as_deref());

        let sub = sqlx::query_as::<_, WebhookSubscription>(
            r#"
            UPDATE webhook_subscriptions SET
                name = COALESCE($2, name),
                description = COALESCE($3, description),
                url = COALESCE($4, url),
                events = COALESCE($5, events),
                secret = COALESCE($6, secret),
                headers = COALESCE($7, headers),
                status = COALESCE($8, status),
                retry_policy = COALESCE($9, retry_policy),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&data.name)
        .bind(&data.description)
        .bind(&data.url)
        .bind(&data.events)
        .bind(&encrypted_secret)
        .bind(&data.headers)
        .bind(&data.status)
        .bind(serde_json::to_value(&data.retry_policy).ok())
        .fetch_one(&self.pool)
        .await
        .map_err(IntegrationError::Database)?;

        Ok(self.decrypt_webhook_subscription(sub))
    }

    /// Delete webhook subscription.
    pub async fn delete_webhook_subscription(&self, id: Uuid) -> Result<bool, SqlxError> {
        let result = sqlx::query("DELETE FROM webhook_subscriptions WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    /// Get subscriptions for a specific event.
    ///
    /// Returns subscriptions with decrypted secrets.
    pub async fn get_subscriptions_for_event(
        &self,
        organization_id: Uuid,
        event_type: &str,
    ) -> Result<Vec<WebhookSubscription>, SqlxError> {
        let subs = sqlx::query_as::<_, WebhookSubscription>(
            r#"
            SELECT * FROM webhook_subscriptions
            WHERE organization_id = $1
              AND status = 'active'
              AND $2 = ANY(events)
            "#,
        )
        .bind(organization_id)
        .bind(event_type)
        .fetch_all(&self.pool)
        .await?;

        Ok(subs
            .into_iter()
            .map(|s| self.decrypt_webhook_subscription(s))
            .collect())
    }

    // ========================================================================
    // Webhook Delivery Logs (Story 61.5)
    // ========================================================================

    /// Create a webhook delivery log.
    pub async fn create_webhook_delivery_log(
        &self,
        subscription_id: Uuid,
        event_type: &str,
        event_id: Uuid,
        payload: serde_json::Value,
    ) -> Result<WebhookDeliveryLog, SqlxError> {
        sqlx::query_as::<_, WebhookDeliveryLog>(
            r#"
            INSERT INTO webhook_delivery_logs (
                subscription_id, event_type, event_id, payload, status, attempts
            )
            VALUES ($1, $2, $3, $4, 'pending', 0)
            RETURNING *
            "#,
        )
        .bind(subscription_id)
        .bind(event_type)
        .bind(event_id)
        .bind(&payload)
        .fetch_one(&self.pool)
        .await
    }

    /// Update webhook delivery status.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_webhook_delivery_status(
        &self,
        id: Uuid,
        status: &str,
        response_status: Option<i32>,
        response_body: Option<&str>,
        error_message: Option<&str>,
        duration_ms: Option<i32>,
        next_retry_at: Option<chrono::DateTime<Utc>>,
    ) -> Result<WebhookDeliveryLog, SqlxError> {
        sqlx::query_as::<_, WebhookDeliveryLog>(
            r#"
            UPDATE webhook_delivery_logs SET
                status = $2,
                attempts = attempts + 1,
                last_attempt_at = NOW(),
                response_status = $3,
                response_body = $4,
                error_message = $5,
                duration_ms = $6,
                next_retry_at = $7
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(status)
        .bind(response_status)
        .bind(response_body)
        .bind(error_message)
        .bind(duration_ms)
        .bind(next_retry_at)
        .fetch_one(&self.pool)
        .await
    }

    /// List webhook delivery logs.
    pub async fn list_webhook_delivery_logs(
        &self,
        query: WebhookDeliveryQuery,
    ) -> Result<Vec<WebhookDeliveryLog>, SqlxError> {
        let limit = query.limit.unwrap_or(50);
        let offset = query.offset.unwrap_or(0);

        sqlx::query_as::<_, WebhookDeliveryLog>(
            r#"
            SELECT * FROM webhook_delivery_logs
            WHERE ($1::uuid IS NULL OR subscription_id = $1)
              AND ($2::text IS NULL OR event_type = $2)
              AND ($3::text IS NULL OR status = $3)
              AND ($4::timestamptz IS NULL OR created_at >= $4)
              AND ($5::timestamptz IS NULL OR created_at <= $5)
            ORDER BY created_at DESC
            LIMIT $6 OFFSET $7
            "#,
        )
        .bind(query.subscription_id)
        .bind(&query.event_type)
        .bind(&query.status)
        .bind(query.from_date)
        .bind(query.to_date)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
    }

    /// Get webhook statistics for a subscription.
    pub async fn get_webhook_statistics(
        &self,
        subscription_id: Uuid,
    ) -> Result<WebhookStatistics, SqlxError> {
        let stats = sqlx::query_as::<_, (i64, i64, i64, i64, Option<f64>)>(
            r#"
            SELECT
                COUNT(*) as total,
                COUNT(*) FILTER (WHERE status = 'delivered') as successful,
                COUNT(*) FILTER (WHERE status = 'failed') as failed,
                COUNT(*) FILTER (WHERE status IN ('pending', 'retrying')) as pending,
                AVG(duration_ms)::float8 as avg_duration
            FROM webhook_delivery_logs
            WHERE subscription_id = $1
            "#,
        )
        .bind(subscription_id)
        .fetch_one(&self.pool)
        .await?;

        let success_rate = if stats.0 > 0 {
            (stats.1 as f64) / (stats.0 as f64) * 100.0
        } else {
            0.0
        };

        Ok(WebhookStatistics {
            total_deliveries: stats.0,
            successful_deliveries: stats.1,
            failed_deliveries: stats.2,
            pending_deliveries: stats.3,
            average_response_time_ms: stats.4,
            success_rate,
        })
    }

    /// Get pending webhook deliveries for retry.
    pub async fn get_pending_webhook_deliveries(
        &self,
        limit: i32,
    ) -> Result<Vec<WebhookDeliveryLog>, SqlxError> {
        sqlx::query_as::<_, WebhookDeliveryLog>(
            r#"
            SELECT wdl.* FROM webhook_delivery_logs wdl
            JOIN webhook_subscriptions ws ON ws.id = wdl.subscription_id
            WHERE wdl.status IN ('pending', 'retrying')
              AND ws.status = 'active'
              AND (wdl.next_retry_at IS NULL OR wdl.next_retry_at <= NOW())
            ORDER BY wdl.created_at
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
    }

    // ========================================================================
    // Integration Statistics
    // ========================================================================

    /// Get integration statistics for an organization.
    ///
    /// Logs errors for individual statistics queries but continues collecting other stats.
    /// This ensures partial data is returned even if some queries fail.
    pub async fn get_integration_statistics(
        &self,
        organization_id: Uuid,
    ) -> Result<IntegrationStatistics, SqlxError> {
        // Calendar stats
        let calendar_stats = sqlx::query_as::<_, (i64, i64)>(
            r#"
            SELECT
                COUNT(*) as total,
                COUNT(*) FILTER (WHERE sync_status = 'active') as active
            FROM calendar_connections
            WHERE organization_id = $1
            "#,
        )
        .bind(organization_id)
        .fetch_one(&self.pool)
        .await
        .unwrap_or_else(|e| {
            tracing::warn!(
                error = %e,
                org_id = %organization_id,
                "Failed to fetch calendar connection stats, using defaults"
            );
            (0, 0)
        });

        // Accounting exports this month
        let export_count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM accounting_exports
            WHERE organization_id = $1
              AND created_at >= date_trunc('month', NOW())
            "#,
        )
        .bind(organization_id)
        .fetch_one(&self.pool)
        .await
        .unwrap_or_else(|e| {
            tracing::warn!(
                error = %e,
                org_id = %organization_id,
                "Failed to fetch accounting export stats, using defaults"
            );
            0
        });

        // E-signature stats
        let esig_stats = sqlx::query_as::<_, (i64, i64)>(
            r#"
            SELECT
                COUNT(*) FILTER (WHERE status IN ('draft', 'sent', 'viewed')) as pending,
                COUNT(*) FILTER (WHERE status = 'completed') as completed
            FROM esignature_workflows
            WHERE organization_id = $1
            "#,
        )
        .bind(organization_id)
        .fetch_one(&self.pool)
        .await
        .unwrap_or_else(|e| {
            tracing::warn!(
                error = %e,
                org_id = %organization_id,
                "Failed to fetch e-signature workflow stats, using defaults"
            );
            (0, 0)
        });

        // Video meetings scheduled
        let meeting_count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM video_meetings
            WHERE organization_id = $1
              AND status = 'scheduled'
              AND start_time >= NOW()
            "#,
        )
        .bind(organization_id)
        .fetch_one(&self.pool)
        .await
        .unwrap_or_else(|e| {
            tracing::warn!(
                error = %e,
                org_id = %organization_id,
                "Failed to fetch video meeting stats, using defaults"
            );
            0
        });

        // Webhook stats
        let webhook_count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM webhook_subscriptions
            WHERE organization_id = $1 AND status = 'active'
            "#,
        )
        .bind(organization_id)
        .fetch_one(&self.pool)
        .await
        .unwrap_or_else(|e| {
            tracing::warn!(
                error = %e,
                org_id = %organization_id,
                "Failed to fetch webhook subscription stats, using defaults"
            );
            0
        });

        let webhook_delivery_stats = sqlx::query_as::<_, (i64, i64)>(
            r#"
            SELECT
                COUNT(*) as total,
                COUNT(*) FILTER (WHERE status = 'delivered') as successful
            FROM webhook_delivery_logs wdl
            JOIN webhook_subscriptions ws ON ws.id = wdl.subscription_id
            WHERE ws.organization_id = $1
              AND wdl.created_at >= NOW() - INTERVAL '1 day'
            "#,
        )
        .bind(organization_id)
        .fetch_one(&self.pool)
        .await
        .unwrap_or_else(|e| {
            tracing::warn!(
                error = %e,
                org_id = %organization_id,
                "Failed to fetch webhook delivery stats, using defaults"
            );
            (0, 0)
        });

        let success_rate = if webhook_delivery_stats.0 > 0 {
            (webhook_delivery_stats.1 as f64) / (webhook_delivery_stats.0 as f64) * 100.0
        } else {
            100.0
        };

        Ok(IntegrationStatistics {
            calendar_connections: calendar_stats.0 as i32,
            active_calendar_syncs: calendar_stats.1 as i32,
            accounting_exports_this_month: export_count as i32,
            esignature_workflows_pending: esig_stats.0 as i32,
            esignature_workflows_completed: esig_stats.1 as i32,
            video_meetings_scheduled: meeting_count as i32,
            webhook_subscriptions: webhook_count as i32,
            webhook_deliveries_today: webhook_delivery_stats.0,
            webhook_success_rate: success_rate,
        })
    }
}

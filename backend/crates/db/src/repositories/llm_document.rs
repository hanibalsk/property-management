//! LLM Document repository (Epic 64: Advanced AI & LLM Capabilities).

use crate::models::llm_document::{
    generation_status, AiEscalationConfig, AiUsageStatistics, DocumentEmbedding,
    GeneratedListingDescription, LlmGenerationRequest, LlmPromptTemplate, PhotoEnhancement,
    ProviderStats, RequestTypeStats, UpdateEscalationConfig, VoiceAssistantDevice,
    VoiceCommandHistory,
};
use crate::DbPool;
use chrono::{DateTime, Utc};
use sqlx::Error as SqlxError;
use uuid::Uuid;

/// Repository for LLM document generation operations.
#[derive(Clone)]
pub struct LlmDocumentRepository {
    pool: DbPool,
}

impl LlmDocumentRepository {
    /// Create a new LlmDocumentRepository.
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    // =========================================================================
    // LLM Generation Requests
    // =========================================================================

    /// Create a new LLM generation request.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_generation_request(
        &self,
        organization_id: Uuid,
        user_id: Uuid,
        request_type: &str,
        provider: &str,
        model: &str,
        input_data: serde_json::Value,
        prompt_template_id: Option<Uuid>,
    ) -> Result<LlmGenerationRequest, SqlxError> {
        sqlx::query_as::<_, LlmGenerationRequest>(
            r#"
            INSERT INTO llm_generation_requests (
                organization_id, user_id, request_type, provider, model,
                input_data, prompt_template_id, status
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(user_id)
        .bind(request_type)
        .bind(provider)
        .bind(model)
        .bind(&input_data)
        .bind(prompt_template_id)
        .bind(generation_status::PENDING)
        .fetch_one(&self.pool)
        .await
    }

    /// Find a generation request by ID.
    pub async fn find_generation_request(
        &self,
        id: Uuid,
    ) -> Result<Option<LlmGenerationRequest>, SqlxError> {
        sqlx::query_as::<_, LlmGenerationRequest>(
            "SELECT * FROM llm_generation_requests WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Update generation request status.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_generation_status(
        &self,
        id: Uuid,
        status: &str,
        result: Option<serde_json::Value>,
        error_message: Option<&str>,
        tokens_used: Option<i32>,
        cost_cents: Option<i32>,
        latency_ms: Option<i32>,
    ) -> Result<Option<LlmGenerationRequest>, SqlxError> {
        let completed_at =
            if status == generation_status::COMPLETED || status == generation_status::FAILED {
                Some(Utc::now())
            } else {
                None
            };

        sqlx::query_as::<_, LlmGenerationRequest>(
            r#"
            UPDATE llm_generation_requests SET
                status = $2,
                result = COALESCE($3, result),
                error_message = $4,
                tokens_used = COALESCE($5, tokens_used),
                cost_cents = COALESCE($6, cost_cents),
                latency_ms = COALESCE($7, latency_ms),
                completed_at = COALESCE($8, completed_at)
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(status)
        .bind(&result)
        .bind(error_message)
        .bind(tokens_used)
        .bind(cost_cents)
        .bind(latency_ms)
        .bind(completed_at)
        .fetch_optional(&self.pool)
        .await
    }

    /// List generation requests for an organization.
    pub async fn list_generation_requests(
        &self,
        organization_id: Uuid,
        request_type: Option<&str>,
        status: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<LlmGenerationRequest>, SqlxError> {
        sqlx::query_as::<_, LlmGenerationRequest>(
            r#"
            SELECT * FROM llm_generation_requests
            WHERE organization_id = $1
              AND ($2::text IS NULL OR request_type = $2)
              AND ($3::text IS NULL OR status = $3)
            ORDER BY created_at DESC
            LIMIT $4 OFFSET $5
            "#,
        )
        .bind(organization_id)
        .bind(request_type)
        .bind(status)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
    }

    // =========================================================================
    // Prompt Templates
    // =========================================================================

    /// Create a prompt template.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_prompt_template(
        &self,
        organization_id: Option<Uuid>,
        name: &str,
        description: Option<&str>,
        request_type: &str,
        system_prompt: &str,
        user_prompt_template: &str,
        variables: serde_json::Value,
        provider: &str,
        model: &str,
        temperature: Option<f32>,
        max_tokens: Option<i32>,
    ) -> Result<LlmPromptTemplate, SqlxError> {
        sqlx::query_as::<_, LlmPromptTemplate>(
            r#"
            INSERT INTO llm_prompt_templates (
                organization_id, name, description, request_type,
                system_prompt, user_prompt_template, variables,
                provider, model, temperature, max_tokens, is_system
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(name)
        .bind(description)
        .bind(request_type)
        .bind(system_prompt)
        .bind(user_prompt_template)
        .bind(&variables)
        .bind(provider)
        .bind(model)
        .bind(temperature)
        .bind(max_tokens)
        .bind(organization_id.is_none()) // is_system = true if no org
        .fetch_one(&self.pool)
        .await
    }

    /// Find a prompt template by ID.
    pub async fn find_prompt_template(
        &self,
        id: Uuid,
    ) -> Result<Option<LlmPromptTemplate>, SqlxError> {
        sqlx::query_as::<_, LlmPromptTemplate>("SELECT * FROM llm_prompt_templates WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    /// Find the default template for a request type.
    pub async fn find_default_template(
        &self,
        organization_id: Uuid,
        request_type: &str,
    ) -> Result<Option<LlmPromptTemplate>, SqlxError> {
        // First try org-specific, then system default
        sqlx::query_as::<_, LlmPromptTemplate>(
            r#"
            SELECT * FROM llm_prompt_templates
            WHERE request_type = $2
              AND is_active = TRUE
              AND (organization_id = $1 OR is_system = TRUE)
            ORDER BY
                CASE WHEN organization_id = $1 THEN 0 ELSE 1 END,
                version DESC
            LIMIT 1
            "#,
        )
        .bind(organization_id)
        .bind(request_type)
        .fetch_optional(&self.pool)
        .await
    }

    /// List prompt templates.
    pub async fn list_prompt_templates(
        &self,
        organization_id: Option<Uuid>,
        request_type: Option<&str>,
    ) -> Result<Vec<LlmPromptTemplate>, SqlxError> {
        sqlx::query_as::<_, LlmPromptTemplate>(
            r#"
            SELECT * FROM llm_prompt_templates
            WHERE ($1::uuid IS NULL OR organization_id = $1 OR is_system = TRUE)
              AND ($2::text IS NULL OR request_type = $2)
              AND is_active = TRUE
            ORDER BY is_system DESC, name
            "#,
        )
        .bind(organization_id)
        .bind(request_type)
        .fetch_all(&self.pool)
        .await
    }

    /// Update a prompt template.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_prompt_template(
        &self,
        id: Uuid,
        name: Option<&str>,
        description: Option<&str>,
        system_prompt: Option<&str>,
        user_prompt_template: Option<&str>,
        variables: Option<serde_json::Value>,
        provider: Option<&str>,
        model: Option<&str>,
        temperature: Option<f32>,
        max_tokens: Option<i32>,
        is_active: Option<bool>,
    ) -> Result<Option<LlmPromptTemplate>, SqlxError> {
        sqlx::query_as::<_, LlmPromptTemplate>(
            r#"
            UPDATE llm_prompt_templates SET
                name = COALESCE($2, name),
                description = COALESCE($3, description),
                system_prompt = COALESCE($4, system_prompt),
                user_prompt_template = COALESCE($5, user_prompt_template),
                variables = COALESCE($6, variables),
                provider = COALESCE($7, provider),
                model = COALESCE($8, model),
                temperature = COALESCE($9, temperature),
                max_tokens = COALESCE($10, max_tokens),
                is_active = COALESCE($11, is_active),
                version = version + 1,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(name)
        .bind(description)
        .bind(system_prompt)
        .bind(user_prompt_template)
        .bind(&variables)
        .bind(provider)
        .bind(model)
        .bind(temperature)
        .bind(max_tokens)
        .bind(is_active)
        .fetch_optional(&self.pool)
        .await
    }

    // =========================================================================
    // Generated Listing Descriptions
    // =========================================================================

    /// Create a generated listing description.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_listing_description(
        &self,
        organization_id: Uuid,
        listing_id: Option<Uuid>,
        user_id: Uuid,
        language: &str,
        original_description: &str,
        property_details: serde_json::Value,
        photo_analysis: Option<serde_json::Value>,
        generation_request_id: Uuid,
    ) -> Result<GeneratedListingDescription, SqlxError> {
        sqlx::query_as::<_, GeneratedListingDescription>(
            r#"
            INSERT INTO generated_listing_descriptions (
                organization_id, listing_id, user_id, language,
                original_description, property_details, photo_analysis,
                generation_request_id, generated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW())
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(listing_id)
        .bind(user_id)
        .bind(language)
        .bind(original_description)
        .bind(&property_details)
        .bind(&photo_analysis)
        .bind(generation_request_id)
        .fetch_one(&self.pool)
        .await
    }

    /// Find a generated description by ID.
    pub async fn find_listing_description(
        &self,
        id: Uuid,
    ) -> Result<Option<GeneratedListingDescription>, SqlxError> {
        sqlx::query_as::<_, GeneratedListingDescription>(
            "SELECT * FROM generated_listing_descriptions WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    /// List descriptions for a listing.
    pub async fn list_listing_descriptions(
        &self,
        listing_id: Uuid,
    ) -> Result<Vec<GeneratedListingDescription>, SqlxError> {
        sqlx::query_as::<_, GeneratedListingDescription>(
            "SELECT * FROM generated_listing_descriptions WHERE listing_id = $1 ORDER BY generated_at DESC",
        )
        .bind(listing_id)
        .fetch_all(&self.pool)
        .await
    }

    /// Update edited description.
    pub async fn update_edited_description(
        &self,
        id: Uuid,
        edited_description: &str,
        edited_by: Uuid,
    ) -> Result<Option<GeneratedListingDescription>, SqlxError> {
        sqlx::query_as::<_, GeneratedListingDescription>(
            r#"
            UPDATE generated_listing_descriptions SET
                edited_description = $2,
                edited_at = NOW(),
                edited_by = $3
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(edited_description)
        .bind(edited_by)
        .fetch_optional(&self.pool)
        .await
    }

    /// Mark description as published.
    pub async fn publish_description(
        &self,
        id: Uuid,
    ) -> Result<Option<GeneratedListingDescription>, SqlxError> {
        sqlx::query_as::<_, GeneratedListingDescription>(
            "UPDATE generated_listing_descriptions SET is_published = TRUE WHERE id = $1 RETURNING *",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    // =========================================================================
    // Document Embeddings (RAG)
    // =========================================================================

    /// Create a document embedding.
    pub async fn create_embedding(
        &self,
        organization_id: Uuid,
        document_id: Uuid,
        chunk_index: i32,
        chunk_text: &str,
        embedding: Option<Vec<f32>>,
        metadata: serde_json::Value,
    ) -> Result<DocumentEmbedding, SqlxError> {
        sqlx::query_as::<_, DocumentEmbedding>(
            r#"
            INSERT INTO document_embeddings (
                organization_id, document_id, chunk_index, chunk_text, embedding, metadata
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(document_id)
        .bind(chunk_index)
        .bind(chunk_text)
        // TODO: For production RAG systems, migrate to pgvector extension for efficient vector storage.
        // Current JSONB approach is inefficient for vector similarity operations.
        // See: https://github.com/pgvector/pgvector
        .bind(
            embedding
                .as_ref()
                .and_then(|e| serde_json::to_value(e).ok()),
        )
        .bind(&metadata)
        .fetch_one(&self.pool)
        .await
    }

    /// Find embeddings for a document.
    pub async fn find_document_embeddings(
        &self,
        document_id: Uuid,
    ) -> Result<Vec<DocumentEmbedding>, SqlxError> {
        sqlx::query_as::<_, DocumentEmbedding>(
            "SELECT * FROM document_embeddings WHERE document_id = $1 ORDER BY chunk_index",
        )
        .bind(document_id)
        .fetch_all(&self.pool)
        .await
    }

    /// Delete embeddings for a document.
    pub async fn delete_document_embeddings(&self, document_id: Uuid) -> Result<u64, SqlxError> {
        let result = sqlx::query("DELETE FROM document_embeddings WHERE document_id = $1")
            .bind(document_id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }

    /// Search documents by text (simple text search, not semantic/vector similarity).
    ///
    /// TODO: For production RAG capability, implement semantic similarity search using pgvector.
    /// Current ILIKE text matching doesn't provide contextually relevant document retrieval.
    /// Proper RAG requires cosine similarity search on embedding vectors.
    /// See: https://github.com/pgvector/pgvector
    pub async fn search_documents_by_text(
        &self,
        organization_id: Uuid,
        search_text: &str,
        limit: i32,
    ) -> Result<Vec<DocumentEmbedding>, SqlxError> {
        sqlx::query_as::<_, DocumentEmbedding>(
            r#"
            SELECT * FROM document_embeddings
            WHERE organization_id = $1
              AND chunk_text ILIKE '%' || $2 || '%'
            ORDER BY created_at DESC
            LIMIT $3
            "#,
        )
        .bind(organization_id)
        .bind(search_text)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
    }

    // =========================================================================
    // Escalation Configuration
    // =========================================================================

    /// Get or create escalation config for an organization.
    pub async fn get_escalation_config(
        &self,
        organization_id: Uuid,
    ) -> Result<AiEscalationConfig, SqlxError> {
        // Try to find existing config
        let existing = sqlx::query_as::<_, AiEscalationConfig>(
            "SELECT * FROM ai_escalation_configs WHERE organization_id = $1",
        )
        .bind(organization_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(config) = existing {
            return Ok(config);
        }

        // Create default config with 80% threshold
        sqlx::query_as::<_, AiEscalationConfig>(
            r#"
            INSERT INTO ai_escalation_configs (
                organization_id, confidence_threshold, auto_escalate_topics
            )
            VALUES ($1, 0.80, '[]')
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .fetch_one(&self.pool)
        .await
    }

    /// Update escalation config.
    pub async fn update_escalation_config(
        &self,
        organization_id: Uuid,
        update: UpdateEscalationConfig,
    ) -> Result<AiEscalationConfig, SqlxError> {
        let topics_json = update
            .auto_escalate_topics
            .map(|t| serde_json::to_value(&t).unwrap_or_default());

        sqlx::query_as::<_, AiEscalationConfig>(
            r#"
            UPDATE ai_escalation_configs SET
                confidence_threshold = COALESCE($2, confidence_threshold),
                escalation_email = COALESCE($3, escalation_email),
                escalation_webhook_url = COALESCE($4, escalation_webhook_url),
                auto_escalate_topics = COALESCE($5, auto_escalate_topics),
                updated_at = NOW()
            WHERE organization_id = $1
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(update.confidence_threshold)
        .bind(&update.escalation_email)
        .bind(&update.escalation_webhook_url)
        .bind(&topics_json)
        .fetch_one(&self.pool)
        .await
    }

    // =========================================================================
    // Photo Enhancement
    // =========================================================================

    /// Create a photo enhancement record.
    pub async fn create_photo_enhancement(
        &self,
        organization_id: Uuid,
        listing_id: Option<Uuid>,
        user_id: Uuid,
        original_photo_url: &str,
        enhancement_type: &str,
        metadata: serde_json::Value,
    ) -> Result<PhotoEnhancement, SqlxError> {
        sqlx::query_as::<_, PhotoEnhancement>(
            r#"
            INSERT INTO photo_enhancements (
                organization_id, listing_id, user_id, original_photo_url,
                enhancement_type, status, metadata
            )
            VALUES ($1, $2, $3, $4, $5, 'pending', $6)
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(listing_id)
        .bind(user_id)
        .bind(original_photo_url)
        .bind(enhancement_type)
        .bind(&metadata)
        .fetch_one(&self.pool)
        .await
    }

    /// Find photo enhancement by ID.
    pub async fn find_photo_enhancement(
        &self,
        id: Uuid,
    ) -> Result<Option<PhotoEnhancement>, SqlxError> {
        sqlx::query_as::<_, PhotoEnhancement>("SELECT * FROM photo_enhancements WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    /// Update photo enhancement status and result.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_photo_enhancement(
        &self,
        id: Uuid,
        status: &str,
        enhanced_photo_url: Option<&str>,
        thumbnail_url: Option<&str>,
        error_message: Option<&str>,
        processing_time_ms: Option<i32>,
        cost_cents: Option<i32>,
    ) -> Result<Option<PhotoEnhancement>, SqlxError> {
        let completed_at = if status == "completed" || status == "failed" {
            Some(Utc::now())
        } else {
            None
        };

        sqlx::query_as::<_, PhotoEnhancement>(
            r#"
            UPDATE photo_enhancements SET
                status = $2,
                enhanced_photo_url = COALESCE($3, enhanced_photo_url),
                thumbnail_url = COALESCE($4, thumbnail_url),
                error_message = $5,
                processing_time_ms = COALESCE($6, processing_time_ms),
                cost_cents = COALESCE($7, cost_cents),
                completed_at = COALESCE($8, completed_at)
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(status)
        .bind(enhanced_photo_url)
        .bind(thumbnail_url)
        .bind(error_message)
        .bind(processing_time_ms)
        .bind(cost_cents)
        .bind(completed_at)
        .fetch_optional(&self.pool)
        .await
    }

    /// List photo enhancements for a listing.
    pub async fn list_photo_enhancements(
        &self,
        listing_id: Uuid,
    ) -> Result<Vec<PhotoEnhancement>, SqlxError> {
        sqlx::query_as::<_, PhotoEnhancement>(
            "SELECT * FROM photo_enhancements WHERE listing_id = $1 ORDER BY created_at DESC",
        )
        .bind(listing_id)
        .fetch_all(&self.pool)
        .await
    }

    // =========================================================================
    // Voice Assistant Devices
    // =========================================================================

    /// Create a voice assistant device link.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_voice_device(
        &self,
        organization_id: Uuid,
        user_id: Uuid,
        unit_id: Option<Uuid>,
        platform: &str,
        device_id: &str,
        device_name: Option<&str>,
        access_token_encrypted: Option<&str>,
        refresh_token_encrypted: Option<&str>,
        token_expires_at: Option<DateTime<Utc>>,
        capabilities: serde_json::Value,
    ) -> Result<VoiceAssistantDevice, SqlxError> {
        sqlx::query_as::<_, VoiceAssistantDevice>(
            r#"
            INSERT INTO voice_assistant_devices (
                organization_id, user_id, unit_id, platform, device_id,
                device_name, access_token_encrypted, refresh_token_encrypted,
                token_expires_at, capabilities, linked_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, NOW())
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(user_id)
        .bind(unit_id)
        .bind(platform)
        .bind(device_id)
        .bind(device_name)
        .bind(access_token_encrypted)
        .bind(refresh_token_encrypted)
        .bind(token_expires_at)
        .bind(&capabilities)
        .fetch_one(&self.pool)
        .await
    }

    /// Find voice device by ID.
    pub async fn find_voice_device(
        &self,
        id: Uuid,
    ) -> Result<Option<VoiceAssistantDevice>, SqlxError> {
        sqlx::query_as::<_, VoiceAssistantDevice>(
            "SELECT * FROM voice_assistant_devices WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Find voice device by external device ID.
    pub async fn find_voice_device_by_device_id(
        &self,
        platform: &str,
        device_id: &str,
    ) -> Result<Option<VoiceAssistantDevice>, SqlxError> {
        sqlx::query_as::<_, VoiceAssistantDevice>(
            "SELECT * FROM voice_assistant_devices WHERE platform = $1 AND device_id = $2 AND is_active = TRUE",
        )
        .bind(platform)
        .bind(device_id)
        .fetch_optional(&self.pool)
        .await
    }

    /// List voice devices for a user.
    pub async fn list_user_voice_devices(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<VoiceAssistantDevice>, SqlxError> {
        sqlx::query_as::<_, VoiceAssistantDevice>(
            "SELECT * FROM voice_assistant_devices WHERE user_id = $1 AND is_active = TRUE ORDER BY linked_at DESC",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
    }

    /// Update voice device last used timestamp.
    pub async fn update_voice_device_last_used(&self, id: Uuid) -> Result<(), SqlxError> {
        sqlx::query("UPDATE voice_assistant_devices SET last_used_at = NOW(), updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Deactivate a voice device.
    pub async fn deactivate_voice_device(&self, id: Uuid) -> Result<bool, SqlxError> {
        let result = sqlx::query(
            "UPDATE voice_assistant_devices SET is_active = FALSE, updated_at = NOW() WHERE id = $1",
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    // =========================================================================
    // Voice Command History
    // =========================================================================

    /// Create a voice command history entry.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_voice_command(
        &self,
        device_id: Uuid,
        user_id: Uuid,
        command_text: &str,
        intent_detected: Option<&str>,
        response_text: &str,
        action_taken: Option<&str>,
        success: bool,
        error_message: Option<&str>,
        processing_time_ms: i32,
    ) -> Result<VoiceCommandHistory, SqlxError> {
        sqlx::query_as::<_, VoiceCommandHistory>(
            r#"
            INSERT INTO voice_command_history (
                device_id, user_id, command_text, intent_detected,
                response_text, action_taken, success, error_message, processing_time_ms
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#,
        )
        .bind(device_id)
        .bind(user_id)
        .bind(command_text)
        .bind(intent_detected)
        .bind(response_text)
        .bind(action_taken)
        .bind(success)
        .bind(error_message)
        .bind(processing_time_ms)
        .fetch_one(&self.pool)
        .await
    }

    /// List voice command history for a device.
    pub async fn list_voice_commands(
        &self,
        device_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VoiceCommandHistory>, SqlxError> {
        sqlx::query_as::<_, VoiceCommandHistory>(
            r#"
            SELECT * FROM voice_command_history
            WHERE device_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(device_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
    }

    // =========================================================================
    // Statistics
    // =========================================================================

    /// Get AI usage statistics for an organization.
    pub async fn get_usage_statistics(
        &self,
        organization_id: Uuid,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
    ) -> Result<AiUsageStatistics, SqlxError> {
        let start = start_date.unwrap_or_else(|| Utc::now() - chrono::Duration::days(30));
        let end = end_date.unwrap_or_else(Utc::now);

        // Get totals
        let totals: (i64, i64, i64, i64, i64, f64) = sqlx::query_as(
            r#"
            SELECT
                COUNT(*),
                COUNT(*) FILTER (WHERE status = 'completed'),
                COUNT(*) FILTER (WHERE status = 'failed'),
                COALESCE(SUM(tokens_used), 0),
                COALESCE(SUM(cost_cents), 0),
                COALESCE(AVG(latency_ms), 0)
            FROM llm_generation_requests
            WHERE organization_id = $1
              AND created_at >= $2
              AND created_at <= $3
            "#,
        )
        .bind(organization_id)
        .bind(start)
        .bind(end)
        .fetch_one(&self.pool)
        .await?;

        // Get by request type
        let by_type: Vec<(String, i64, i64, i64)> = sqlx::query_as(
            r#"
            SELECT
                request_type,
                COUNT(*),
                COALESCE(SUM(tokens_used), 0),
                COALESCE(SUM(cost_cents), 0)
            FROM llm_generation_requests
            WHERE organization_id = $1
              AND created_at >= $2
              AND created_at <= $3
            GROUP BY request_type
            "#,
        )
        .bind(organization_id)
        .bind(start)
        .bind(end)
        .fetch_all(&self.pool)
        .await?;

        // Get by provider
        let by_provider: Vec<(String, i64, i64, i64, f64)> = sqlx::query_as(
            r#"
            SELECT
                provider,
                COUNT(*),
                COALESCE(SUM(tokens_used), 0),
                COALESCE(SUM(cost_cents), 0),
                COALESCE(AVG(latency_ms), 0)
            FROM llm_generation_requests
            WHERE organization_id = $1
              AND created_at >= $2
              AND created_at <= $3
            GROUP BY provider
            "#,
        )
        .bind(organization_id)
        .bind(start)
        .bind(end)
        .fetch_all(&self.pool)
        .await?;

        Ok(AiUsageStatistics {
            total_generations: totals.0,
            successful_generations: totals.1,
            failed_generations: totals.2,
            total_tokens_used: totals.3,
            total_cost_cents: totals.4,
            average_latency_ms: totals.5,
            by_request_type: by_type
                .into_iter()
                .map(|(request_type, count, tokens, cost)| RequestTypeStats {
                    request_type,
                    count,
                    tokens_used: tokens,
                    cost_cents: cost,
                })
                .collect(),
            by_provider: by_provider
                .into_iter()
                .map(|(provider, count, tokens, cost, latency)| ProviderStats {
                    provider,
                    count,
                    tokens_used: tokens,
                    cost_cents: cost,
                    average_latency_ms: latency,
                })
                .collect(),
        })
    }
}

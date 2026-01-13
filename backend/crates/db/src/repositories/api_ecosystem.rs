//! API Ecosystem repository (Epic 150).
//!
//! Provides database operations for Integration Marketplace, Connector Framework,
//! Webhooks, and Developer Portal.

use crate::models::api_ecosystem::*;
use crate::DbPool;
use common::AppError;
use uuid::Uuid;

/// Repository for API Ecosystem operations.
#[derive(Clone)]
pub struct ApiEcosystemRepository {
    pool: DbPool,
}

impl ApiEcosystemRepository {
    /// Create a new repository instance.
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    // ============================================
    // Story 150.1: Integration Marketplace
    // ============================================

    /// List marketplace integrations with optional filtering.
    pub async fn list_marketplace_integrations(
        &self,
        query: &MarketplaceIntegrationQuery,
    ) -> Result<Vec<MarketplaceIntegrationSummary>, AppError> {
        let limit = query.limit.unwrap_or(50).min(100) as i64;
        let offset = query.offset.unwrap_or(0) as i64;

        let integrations = sqlx::query_as::<_, MarketplaceIntegrationSummary>(
            r#"
            SELECT id, slug, name, description, category, icon_url, vendor_name,
                   status, rating_average, rating_count, install_count, is_featured, is_premium
            FROM marketplace_integrations
            WHERE ($1::text IS NULL OR category = $1)
              AND ($2::text IS NULL OR status = $2)
              AND ($3::bool IS NULL OR is_featured = $3)
              AND ($4::bool IS NULL OR is_premium = $4)
              AND ($5::text IS NULL OR name ILIKE '%' || $5 || '%' OR description ILIKE '%' || $5 || '%')
            ORDER BY
                CASE WHEN $6 = 'rating' THEN rating_average END DESC NULLS LAST,
                CASE WHEN $6 = 'installs' THEN install_count END DESC,
                CASE WHEN $6 = 'created' THEN created_at END DESC,
                CASE WHEN $6 IS NULL OR $6 = 'name' THEN name END ASC
            LIMIT $7 OFFSET $8
            "#,
        )
        .bind(query.category.as_ref())
        .bind(query.status.as_ref())
        .bind(query.featured_only)
        .bind(query.premium_only)
        .bind(query.search.as_ref())
        .bind(query.sort_by.as_ref())
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(integrations)
    }

    /// Get a marketplace integration by ID.
    pub async fn get_marketplace_integration(
        &self,
        id: Uuid,
    ) -> Result<Option<MarketplaceIntegration>, AppError> {
        let integration = sqlx::query_as::<_, MarketplaceIntegration>(
            r#"
            SELECT * FROM marketplace_integrations WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(integration)
    }

    /// Get a marketplace integration by slug.
    pub async fn get_marketplace_integration_by_slug(
        &self,
        slug: &str,
    ) -> Result<Option<MarketplaceIntegration>, AppError> {
        let integration = sqlx::query_as::<_, MarketplaceIntegration>(
            r#"
            SELECT * FROM marketplace_integrations WHERE slug = $1
            "#,
        )
        .bind(slug)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(integration)
    }

    /// Create a new marketplace integration (admin only).
    pub async fn create_marketplace_integration(
        &self,
        req: &CreateMarketplaceIntegration,
    ) -> Result<MarketplaceIntegration, AppError> {
        let integration = sqlx::query_as::<_, MarketplaceIntegration>(
            r#"
            INSERT INTO marketplace_integrations (
                slug, name, description, long_description, category, icon_url, banner_url,
                vendor_name, vendor_url, documentation_url, support_url, version,
                features, requirements, pricing_info, is_premium, required_scopes
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            RETURNING *
            "#,
        )
        .bind(&req.slug)
        .bind(&req.name)
        .bind(&req.description)
        .bind(&req.long_description)
        .bind(&req.category)
        .bind(&req.icon_url)
        .bind(&req.banner_url)
        .bind(&req.vendor_name)
        .bind(&req.vendor_url)
        .bind(&req.documentation_url)
        .bind(&req.support_url)
        .bind(&req.version)
        .bind(&req.features)
        .bind(&req.requirements)
        .bind(&req.pricing_info)
        .bind(req.is_premium.unwrap_or(false))
        .bind(&req.required_scopes)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(integration)
    }

    /// Update a marketplace integration.
    pub async fn update_marketplace_integration(
        &self,
        id: Uuid,
        req: &UpdateMarketplaceIntegration,
    ) -> Result<Option<MarketplaceIntegration>, AppError> {
        let integration = sqlx::query_as::<_, MarketplaceIntegration>(
            r#"
            UPDATE marketplace_integrations SET
                name = COALESCE($2, name),
                description = COALESCE($3, description),
                long_description = COALESCE($4, long_description),
                category = COALESCE($5, category),
                icon_url = COALESCE($6, icon_url),
                banner_url = COALESCE($7, banner_url),
                vendor_url = COALESCE($8, vendor_url),
                documentation_url = COALESCE($9, documentation_url),
                support_url = COALESCE($10, support_url),
                version = COALESCE($11, version),
                status = COALESCE($12, status),
                features = COALESCE($13, features),
                requirements = COALESCE($14, requirements),
                pricing_info = COALESCE($15, pricing_info),
                is_featured = COALESCE($16, is_featured),
                is_premium = COALESCE($17, is_premium),
                required_scopes = COALESCE($18, required_scopes)
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&req.name)
        .bind(&req.description)
        .bind(&req.long_description)
        .bind(&req.category)
        .bind(&req.icon_url)
        .bind(&req.banner_url)
        .bind(&req.vendor_url)
        .bind(&req.documentation_url)
        .bind(&req.support_url)
        .bind(&req.version)
        .bind(&req.status)
        .bind(&req.features)
        .bind(&req.requirements)
        .bind(&req.pricing_info)
        .bind(req.is_featured)
        .bind(req.is_premium)
        .bind(&req.required_scopes)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(integration)
    }

    /// Delete a marketplace integration.
    pub async fn delete_marketplace_integration(&self, id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM marketplace_integrations WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    /// Get integration category counts.
    pub async fn get_integration_category_counts(
        &self,
    ) -> Result<Vec<IntegrationCategoryCount>, AppError> {
        let counts = sqlx::query_as::<_, IntegrationCategoryCount>(
            r#"
            SELECT category, COUNT(*) as count
            FROM marketplace_integrations
            WHERE status = 'available'
            GROUP BY category
            ORDER BY count DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(counts)
    }

    // ============================================
    // Organization Integrations
    // ============================================

    /// Install an integration for an organization.
    pub async fn install_integration(
        &self,
        organization_id: Uuid,
        user_id: Uuid,
        req: &InstallIntegration,
    ) -> Result<OrganizationIntegration, AppError> {
        // Encrypt credentials if provided (in production, use proper encryption)
        let credentials_encrypted = req
            .credentials
            .as_ref()
            .map(|c| serde_json::to_string(c).unwrap_or_default());

        let installation = sqlx::query_as::<_, OrganizationIntegration>(
            r#"
            INSERT INTO organization_integrations (
                organization_id, integration_id, status, configuration,
                credentials_encrypted, installed_by
            ) VALUES ($1, $2, 'installed', $3, $4, $5)
            ON CONFLICT (organization_id, integration_id)
            DO UPDATE SET
                status = 'installed',
                configuration = COALESCE($3, organization_integrations.configuration),
                credentials_encrypted = COALESCE($4, organization_integrations.credentials_encrypted),
                updated_at = NOW()
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(req.integration_id)
        .bind(&req.configuration)
        .bind(credentials_encrypted)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        // Update install count
        sqlx::query(
            r#"
            UPDATE marketplace_integrations
            SET install_count = install_count + 1
            WHERE id = $1
            "#,
        )
        .bind(req.integration_id)
        .execute(&self.pool)
        .await
        .ok();

        Ok(installation)
    }

    /// List organization integrations.
    pub async fn list_organization_integrations(
        &self,
        organization_id: Uuid,
    ) -> Result<Vec<OrganizationIntegration>, AppError> {
        let integrations = sqlx::query_as::<_, OrganizationIntegration>(
            r#"
            SELECT * FROM organization_integrations
            WHERE organization_id = $1
            ORDER BY installed_at DESC
            "#,
        )
        .bind(organization_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(integrations)
    }

    /// Uninstall an integration.
    pub async fn uninstall_integration(
        &self,
        organization_id: Uuid,
        integration_id: Uuid,
    ) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            UPDATE organization_integrations
            SET status = 'uninstalled', enabled = FALSE, updated_at = NOW()
            WHERE organization_id = $1 AND integration_id = $2
            "#,
        )
        .bind(organization_id)
        .bind(integration_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    // ============================================
    // Story 150.2: Connector Framework
    // ============================================

    /// List connectors for an integration.
    pub async fn list_connectors(&self, integration_id: Uuid) -> Result<Vec<Connector>, AppError> {
        let connectors = sqlx::query_as::<_, Connector>(
            r#"
            SELECT * FROM connectors
            WHERE integration_id = $1
            ORDER BY name
            "#,
        )
        .bind(integration_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(connectors)
    }

    /// Get a connector by ID.
    pub async fn get_connector(&self, id: Uuid) -> Result<Option<Connector>, AppError> {
        let connector = sqlx::query_as::<_, Connector>(
            r#"
            SELECT * FROM connectors WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(connector)
    }

    /// Create a new connector.
    pub async fn create_connector(&self, req: &CreateConnector) -> Result<Connector, AppError> {
        let connector = sqlx::query_as::<_, Connector>(
            r#"
            INSERT INTO connectors (
                integration_id, name, description, auth_type, auth_config, base_url,
                rate_limit_requests, rate_limit_window_seconds, retry_max_attempts,
                retry_initial_delay_ms, retry_max_delay_ms, timeout_ms, headers,
                supported_actions, error_mapping, data_transformations
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            RETURNING *
            "#,
        )
        .bind(req.integration_id)
        .bind(&req.name)
        .bind(&req.description)
        .bind(&req.auth_type)
        .bind(&req.auth_config)
        .bind(&req.base_url)
        .bind(req.rate_limit_requests)
        .bind(req.rate_limit_window_seconds)
        .bind(req.retry_max_attempts.unwrap_or(3))
        .bind(req.retry_initial_delay_ms.unwrap_or(1000))
        .bind(req.retry_max_delay_ms.unwrap_or(30000))
        .bind(req.timeout_ms.unwrap_or(30000))
        .bind(&req.headers)
        .bind(&req.supported_actions)
        .bind(&req.error_mapping)
        .bind(&req.data_transformations)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(connector)
    }

    /// Update a connector.
    pub async fn update_connector(
        &self,
        id: Uuid,
        req: &UpdateConnector,
    ) -> Result<Option<Connector>, AppError> {
        let connector = sqlx::query_as::<_, Connector>(
            r#"
            UPDATE connectors SET
                name = COALESCE($2, name),
                description = COALESCE($3, description),
                auth_config = COALESCE($4, auth_config),
                base_url = COALESCE($5, base_url),
                rate_limit_requests = COALESCE($6, rate_limit_requests),
                rate_limit_window_seconds = COALESCE($7, rate_limit_window_seconds),
                retry_max_attempts = COALESCE($8, retry_max_attempts),
                retry_initial_delay_ms = COALESCE($9, retry_initial_delay_ms),
                retry_max_delay_ms = COALESCE($10, retry_max_delay_ms),
                timeout_ms = COALESCE($11, timeout_ms),
                headers = COALESCE($12, headers),
                supported_actions = COALESCE($13, supported_actions),
                error_mapping = COALESCE($14, error_mapping),
                data_transformations = COALESCE($15, data_transformations)
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&req.name)
        .bind(&req.description)
        .bind(&req.auth_config)
        .bind(&req.base_url)
        .bind(req.rate_limit_requests)
        .bind(req.rate_limit_window_seconds)
        .bind(req.retry_max_attempts)
        .bind(req.retry_initial_delay_ms)
        .bind(req.retry_max_delay_ms)
        .bind(req.timeout_ms)
        .bind(&req.headers)
        .bind(&req.supported_actions)
        .bind(&req.error_mapping)
        .bind(&req.data_transformations)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(connector)
    }

    /// Delete a connector.
    pub async fn delete_connector(&self, id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM connectors WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    /// List connector actions.
    pub async fn list_connector_actions(
        &self,
        connector_id: Uuid,
    ) -> Result<Vec<ConnectorAction>, AppError> {
        let actions = sqlx::query_as::<_, ConnectorAction>(
            r#"
            SELECT * FROM connector_actions
            WHERE connector_id = $1
            ORDER BY name
            "#,
        )
        .bind(connector_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(actions)
    }

    // ============================================
    // Story 150.3: Enhanced Webhooks
    // ============================================

    /// List enhanced webhook subscriptions for an organization.
    pub async fn list_enhanced_webhooks(
        &self,
        organization_id: Uuid,
    ) -> Result<Vec<EnhancedWebhookSubscription>, AppError> {
        let subscriptions = sqlx::query_as::<_, EnhancedWebhookSubscription>(
            r#"
            SELECT * FROM webhook_subscriptions
            WHERE organization_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(organization_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(subscriptions)
    }

    /// Get a webhook subscription by ID.
    pub async fn get_enhanced_webhook(
        &self,
        id: Uuid,
    ) -> Result<Option<EnhancedWebhookSubscription>, AppError> {
        let subscription = sqlx::query_as::<_, EnhancedWebhookSubscription>(
            r#"
            SELECT * FROM webhook_subscriptions WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(subscription)
    }

    /// Delete a webhook subscription.
    pub async fn delete_enhanced_webhook(&self, id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM webhook_subscriptions WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    // ============================================
    // Story 150.4: Developer Portal
    // ============================================

    /// Get a developer registration by ID.
    pub async fn get_developer_registration(
        &self,
        id: Uuid,
    ) -> Result<Option<DeveloperRegistration>, AppError> {
        let registration = sqlx::query_as::<_, DeveloperRegistration>(
            r#"
            SELECT * FROM developer_accounts WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(registration)
    }

    /// List API keys for a developer.
    pub async fn list_developer_api_keys(
        &self,
        developer_id: Uuid,
    ) -> Result<Vec<DeveloperApiKey>, AppError> {
        let keys = sqlx::query_as::<_, DeveloperApiKey>(
            r#"
            SELECT * FROM developer_api_keys
            WHERE developer_id = $1 AND revoked_at IS NULL
            ORDER BY created_at DESC
            "#,
        )
        .bind(developer_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(keys)
    }

    /// Revoke an API key.
    pub async fn revoke_api_key(&self, key_id: Uuid, revoked_by: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            UPDATE developer_api_keys SET
                is_active = FALSE,
                revoked_at = NOW(),
                revoked_by = $2
            WHERE id = $1 AND revoked_at IS NULL
            "#,
        )
        .bind(key_id)
        .bind(revoked_by)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }
}

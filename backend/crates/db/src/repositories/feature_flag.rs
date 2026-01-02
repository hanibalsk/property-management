//! Feature Flag repository (Epic 10B, Story 10B.2).
//! Extended with Epic 107: Feature Descriptors & Catalog.
//!
//! Repository for feature flag management with override support,
//! descriptors, categories, and user type access matrix.

use crate::models::platform_admin::{
    CatalogPagination, CategoryWithCount, FeatureAccessState, FeatureCatalogItem,
    FeatureCatalogQuery, FeatureCatalogResponse, FeatureCategory, FeatureCategorySummary,
    FeatureDescriptor, FeatureDescriptorDisplay, FeatureFlag, FeatureFlagOverride,
    FeatureFlagScope, FeatureState, FeatureUserTypeAccess, UserFeaturePreference,
};
use crate::DbPool;
use sqlx::Error as SqlxError;
use uuid::Uuid;

/// Repository for feature flag operations.
#[derive(Clone)]
pub struct FeatureFlagRepository {
    pool: DbPool,
}

impl FeatureFlagRepository {
    /// Create a new FeatureFlagRepository.
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// List all feature flags with override counts.
    pub async fn list_all(&self) -> Result<Vec<FeatureFlagWithCount>, SqlxError> {
        let flags = sqlx::query_as::<_, FeatureFlagWithCount>(
            r#"
            SELECT
                f.id, f.key, f.name, f.description, f.is_enabled,
                f.created_at, f.updated_at,
                COUNT(o.id) as override_count
            FROM feature_flags f
            LEFT JOIN feature_flag_overrides o ON o.flag_id = f.id
            GROUP BY f.id, f.key, f.name, f.description, f.is_enabled, f.created_at, f.updated_at
            ORDER BY f.key
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(flags)
    }

    /// Get a feature flag by ID with all its overrides.
    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<FeatureFlagWithOverrides>, SqlxError> {
        let flag = sqlx::query_as::<_, FeatureFlag>(
            r#"
            SELECT id, key, name, description, is_enabled, created_at, updated_at
            FROM feature_flags
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        let flag = match flag {
            Some(f) => f,
            None => return Ok(None),
        };

        let overrides = sqlx::query_as::<_, FeatureFlagOverride>(
            r#"
            SELECT id, flag_id, scope_type, scope_id, is_enabled, created_at
            FROM feature_flag_overrides
            WHERE flag_id = $1
            ORDER BY scope_type, created_at
            "#,
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await?;

        Ok(Some(FeatureFlagWithOverrides { flag, overrides }))
    }

    /// Get a feature flag by key.
    pub async fn get_by_key(&self, key: &str) -> Result<Option<FeatureFlag>, SqlxError> {
        let flag = sqlx::query_as::<_, FeatureFlag>(
            r#"
            SELECT id, key, name, description, is_enabled, created_at, updated_at
            FROM feature_flags
            WHERE key = $1
            "#,
        )
        .bind(key)
        .fetch_optional(&self.pool)
        .await?;

        Ok(flag)
    }

    /// Create a new feature flag.
    pub async fn create(
        &self,
        key: &str,
        name: &str,
        description: Option<&str>,
        is_enabled: bool,
    ) -> Result<FeatureFlag, SqlxError> {
        let flag = sqlx::query_as::<_, FeatureFlag>(
            r#"
            INSERT INTO feature_flags (key, name, description, is_enabled)
            VALUES ($1, $2, $3, $4)
            RETURNING id, key, name, description, is_enabled, created_at, updated_at
            "#,
        )
        .bind(key)
        .bind(name)
        .bind(description)
        .bind(is_enabled)
        .fetch_one(&self.pool)
        .await?;

        Ok(flag)
    }

    /// Update a feature flag.
    pub async fn update(
        &self,
        id: Uuid,
        name: Option<&str>,
        description: Option<&str>,
        is_enabled: Option<bool>,
    ) -> Result<Option<FeatureFlag>, SqlxError> {
        let flag = sqlx::query_as::<_, FeatureFlag>(
            r#"
            UPDATE feature_flags
            SET
                name = COALESCE($2, name),
                description = COALESCE($3, description),
                is_enabled = COALESCE($4, is_enabled),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, key, name, description, is_enabled, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(name)
        .bind(description)
        .bind(is_enabled)
        .fetch_optional(&self.pool)
        .await?;

        Ok(flag)
    }

    /// Toggle a feature flag's global enabled state.
    pub async fn toggle(&self, id: Uuid) -> Result<Option<FeatureFlag>, SqlxError> {
        let flag = sqlx::query_as::<_, FeatureFlag>(
            r#"
            UPDATE feature_flags
            SET is_enabled = NOT is_enabled, updated_at = NOW()
            WHERE id = $1
            RETURNING id, key, name, description, is_enabled, created_at, updated_at
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(flag)
    }

    /// Delete a feature flag and all its overrides.
    pub async fn delete(&self, id: Uuid) -> Result<bool, SqlxError> {
        let result = sqlx::query("DELETE FROM feature_flags WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Create an override for a feature flag.
    pub async fn create_override(
        &self,
        flag_id: Uuid,
        scope_type: FeatureFlagScope,
        scope_id: Uuid,
        is_enabled: bool,
    ) -> Result<FeatureFlagOverride, SqlxError> {
        let override_record = sqlx::query_as::<_, FeatureFlagOverride>(
            r#"
            INSERT INTO feature_flag_overrides (flag_id, scope_type, scope_id, is_enabled)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (flag_id, scope_type, scope_id)
            DO UPDATE SET is_enabled = $4
            RETURNING id, flag_id, scope_type, scope_id, is_enabled, created_at
            "#,
        )
        .bind(flag_id)
        .bind(scope_type.as_str())
        .bind(scope_id)
        .bind(is_enabled)
        .fetch_one(&self.pool)
        .await?;

        Ok(override_record)
    }

    /// Delete an override.
    pub async fn delete_override(&self, override_id: Uuid) -> Result<bool, SqlxError> {
        let result = sqlx::query("DELETE FROM feature_flag_overrides WHERE id = $1")
            .bind(override_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Resolve whether a feature is enabled for a given context.
    /// Resolution order: user override → org override → role override → global default
    pub async fn is_enabled_for_context(
        &self,
        flag_key: &str,
        user_id: Option<Uuid>,
        org_id: Option<Uuid>,
        role_id: Option<Uuid>,
    ) -> Result<Option<bool>, SqlxError> {
        // First get the flag
        let flag = match self.get_by_key(flag_key).await? {
            Some(f) => f,
            None => return Ok(None),
        };

        // Check user override first (highest priority)
        if let Some(uid) = user_id {
            if let Some(override_value) = self.get_override_value(flag.id, "user", uid).await? {
                return Ok(Some(override_value));
            }
        }

        // Check org override
        if let Some(oid) = org_id {
            if let Some(override_value) = self
                .get_override_value(flag.id, "organization", oid)
                .await?
            {
                return Ok(Some(override_value));
            }
        }

        // Check role override
        if let Some(rid) = role_id {
            if let Some(override_value) = self.get_override_value(flag.id, "role", rid).await? {
                return Ok(Some(override_value));
            }
        }

        // Return global default
        Ok(Some(flag.is_enabled))
    }

    /// Get override value for a specific scope.
    async fn get_override_value(
        &self,
        flag_id: Uuid,
        scope_type: &str,
        scope_id: Uuid,
    ) -> Result<Option<bool>, SqlxError> {
        let result = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT is_enabled FROM feature_flag_overrides
            WHERE flag_id = $1 AND scope_type = $2 AND scope_id = $3
            "#,
        )
        .bind(flag_id)
        .bind(scope_type)
        .bind(scope_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    /// Get all flags resolved for a user context.
    pub async fn resolve_all_for_context(
        &self,
        user_id: Option<Uuid>,
        org_id: Option<Uuid>,
        role_id: Option<Uuid>,
    ) -> Result<Vec<ResolvedFeatureFlag>, SqlxError> {
        let flags = self.list_all().await?;
        let mut resolved = Vec::new();

        for flag in flags {
            let is_enabled = self
                .is_enabled_for_context(&flag.key, user_id, org_id, role_id)
                .await?
                .unwrap_or(flag.is_enabled);

            resolved.push(ResolvedFeatureFlag {
                key: flag.key,
                is_enabled,
            });
        }

        Ok(resolved)
    }

    // ==================== Epic 107: Feature Categories ====================

    /// List all feature categories.
    pub async fn list_categories(&self) -> Result<Vec<FeatureCategory>, SqlxError> {
        let categories = sqlx::query_as::<_, FeatureCategory>(
            r#"
            SELECT id, key, name, description, icon, color, parent_id,
                   display_order, translations, metadata, created_at, updated_at
            FROM feature_categories
            ORDER BY display_order, name
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(categories)
    }

    /// Get a category by ID.
    pub async fn get_category(&self, id: Uuid) -> Result<Option<FeatureCategory>, SqlxError> {
        let category = sqlx::query_as::<_, FeatureCategory>(
            r#"
            SELECT id, key, name, description, icon, color, parent_id,
                   display_order, translations, metadata, created_at, updated_at
            FROM feature_categories
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(category)
    }

    /// Get a category by key.
    pub async fn get_category_by_key(
        &self,
        key: &str,
    ) -> Result<Option<FeatureCategory>, SqlxError> {
        let category = sqlx::query_as::<_, FeatureCategory>(
            r#"
            SELECT id, key, name, description, icon, color, parent_id,
                   display_order, translations, metadata, created_at, updated_at
            FROM feature_categories
            WHERE key = $1
            "#,
        )
        .bind(key)
        .fetch_optional(&self.pool)
        .await?;

        Ok(category)
    }

    /// Create a new category.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_category(
        &self,
        key: &str,
        name: &str,
        description: Option<&str>,
        icon: Option<&str>,
        color: Option<&str>,
        parent_id: Option<Uuid>,
        display_order: i32,
        translations: serde_json::Value,
    ) -> Result<FeatureCategory, SqlxError> {
        let category = sqlx::query_as::<_, FeatureCategory>(
            r#"
            INSERT INTO feature_categories (key, name, description, icon, color, parent_id, display_order, translations)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, key, name, description, icon, color, parent_id,
                      display_order, translations, metadata, created_at, updated_at
            "#,
        )
        .bind(key)
        .bind(name)
        .bind(description)
        .bind(icon)
        .bind(color)
        .bind(parent_id)
        .bind(display_order)
        .bind(translations)
        .fetch_one(&self.pool)
        .await?;

        Ok(category)
    }

    /// Update a category.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_category(
        &self,
        id: Uuid,
        name: Option<&str>,
        description: Option<&str>,
        icon: Option<&str>,
        color: Option<&str>,
        parent_id: Option<Uuid>,
        display_order: Option<i32>,
        translations: Option<serde_json::Value>,
    ) -> Result<Option<FeatureCategory>, SqlxError> {
        let category = sqlx::query_as::<_, FeatureCategory>(
            r#"
            UPDATE feature_categories
            SET name = COALESCE($2, name),
                description = COALESCE($3, description),
                icon = COALESCE($4, icon),
                color = COALESCE($5, color),
                parent_id = COALESCE($6, parent_id),
                display_order = COALESCE($7, display_order),
                translations = COALESCE($8, translations),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, key, name, description, icon, color, parent_id,
                      display_order, translations, metadata, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(name)
        .bind(description)
        .bind(icon)
        .bind(color)
        .bind(parent_id)
        .bind(display_order)
        .bind(translations)
        .fetch_optional(&self.pool)
        .await?;

        Ok(category)
    }

    /// Delete a category.
    pub async fn delete_category(&self, id: Uuid) -> Result<bool, SqlxError> {
        let result = sqlx::query("DELETE FROM feature_categories WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    // ==================== Epic 107: Feature Descriptors ====================

    /// Get descriptor for a feature flag.
    pub async fn get_descriptor(
        &self,
        flag_id: Uuid,
    ) -> Result<Option<FeatureDescriptor>, SqlxError> {
        let descriptor = sqlx::query_as::<_, FeatureDescriptor>(
            r#"
            SELECT id, feature_flag_id, display_name, short_description, full_description,
                   icon, preview_image_url, category_id, subcategory, tags, translations,
                   benefits, use_cases, api_scopes, depends_on, conflicts_with,
                   display_order, is_highlighted, badge_text, show_teaser_when_disabled,
                   created_at, updated_at
            FROM feature_descriptors
            WHERE feature_flag_id = $1
            "#,
        )
        .bind(flag_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(descriptor)
    }

    /// Create or update a feature descriptor.
    #[allow(clippy::too_many_arguments)]
    pub async fn upsert_descriptor(
        &self,
        flag_id: Uuid,
        display_name: &str,
        short_description: Option<&str>,
        full_description: Option<&str>,
        icon: Option<&str>,
        preview_image_url: Option<&str>,
        category_id: Option<Uuid>,
        subcategory: Option<&str>,
        tags: serde_json::Value,
        translations: serde_json::Value,
        benefits: serde_json::Value,
        use_cases: serde_json::Value,
        api_scopes: serde_json::Value,
        depends_on: serde_json::Value,
        conflicts_with: serde_json::Value,
        display_order: i32,
        is_highlighted: bool,
        badge_text: Option<&str>,
        show_teaser_when_disabled: bool,
    ) -> Result<FeatureDescriptor, SqlxError> {
        let descriptor = sqlx::query_as::<_, FeatureDescriptor>(
            r#"
            INSERT INTO feature_descriptors (
                feature_flag_id, display_name, short_description, full_description,
                icon, preview_image_url, category_id, subcategory, tags, translations,
                benefits, use_cases, api_scopes, depends_on, conflicts_with,
                display_order, is_highlighted, badge_text, show_teaser_when_disabled
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)
            ON CONFLICT (feature_flag_id)
            DO UPDATE SET
                display_name = $2,
                short_description = $3,
                full_description = $4,
                icon = $5,
                preview_image_url = $6,
                category_id = $7,
                subcategory = $8,
                tags = $9,
                translations = $10,
                benefits = $11,
                use_cases = $12,
                api_scopes = $13,
                depends_on = $14,
                conflicts_with = $15,
                display_order = $16,
                is_highlighted = $17,
                badge_text = $18,
                show_teaser_when_disabled = $19,
                updated_at = NOW()
            RETURNING id, feature_flag_id, display_name, short_description, full_description,
                      icon, preview_image_url, category_id, subcategory, tags, translations,
                      benefits, use_cases, api_scopes, depends_on, conflicts_with,
                      display_order, is_highlighted, badge_text, show_teaser_when_disabled,
                      created_at, updated_at
            "#,
        )
        .bind(flag_id)
        .bind(display_name)
        .bind(short_description)
        .bind(full_description)
        .bind(icon)
        .bind(preview_image_url)
        .bind(category_id)
        .bind(subcategory)
        .bind(tags)
        .bind(translations)
        .bind(benefits)
        .bind(use_cases)
        .bind(api_scopes)
        .bind(depends_on)
        .bind(conflicts_with)
        .bind(display_order)
        .bind(is_highlighted)
        .bind(badge_text)
        .bind(show_teaser_when_disabled)
        .fetch_one(&self.pool)
        .await?;

        Ok(descriptor)
    }

    // ==================== Epic 107: User Type Access Matrix ====================

    /// Get user type access for a feature.
    pub async fn get_user_type_access(
        &self,
        flag_id: Uuid,
        user_type: &str,
    ) -> Result<Option<FeatureUserTypeAccess>, SqlxError> {
        let access = sqlx::query_as::<_, FeatureUserTypeAccess>(
            r#"
            SELECT id, feature_flag_id, user_type, access_state, can_override,
                   default_enabled, created_at, updated_at
            FROM feature_user_type_access
            WHERE feature_flag_id = $1 AND user_type = $2
            "#,
        )
        .bind(flag_id)
        .bind(user_type)
        .fetch_optional(&self.pool)
        .await?;

        Ok(access)
    }

    /// List all user type access rules for a feature.
    pub async fn list_user_type_access(
        &self,
        flag_id: Uuid,
    ) -> Result<Vec<FeatureUserTypeAccess>, SqlxError> {
        let access_list = sqlx::query_as::<_, FeatureUserTypeAccess>(
            r#"
            SELECT id, feature_flag_id, user_type, access_state, can_override,
                   default_enabled, created_at, updated_at
            FROM feature_user_type_access
            WHERE feature_flag_id = $1
            ORDER BY user_type
            "#,
        )
        .bind(flag_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(access_list)
    }

    /// Set user type access for a feature.
    pub async fn set_user_type_access(
        &self,
        flag_id: Uuid,
        user_type: &str,
        access_state: FeatureAccessState,
        can_override: bool,
        default_enabled: bool,
    ) -> Result<FeatureUserTypeAccess, SqlxError> {
        let access = sqlx::query_as::<_, FeatureUserTypeAccess>(
            r#"
            INSERT INTO feature_user_type_access (feature_flag_id, user_type, access_state, can_override, default_enabled)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (feature_flag_id, user_type)
            DO UPDATE SET
                access_state = $3,
                can_override = $4,
                default_enabled = $5,
                updated_at = NOW()
            RETURNING id, feature_flag_id, user_type, access_state, can_override,
                      default_enabled, created_at, updated_at
            "#,
        )
        .bind(flag_id)
        .bind(user_type)
        .bind(access_state)
        .bind(can_override)
        .bind(default_enabled)
        .fetch_one(&self.pool)
        .await?;

        Ok(access)
    }

    /// Delete user type access rule.
    pub async fn delete_user_type_access(
        &self,
        flag_id: Uuid,
        user_type: &str,
    ) -> Result<bool, SqlxError> {
        let result = sqlx::query(
            "DELETE FROM feature_user_type_access WHERE feature_flag_id = $1 AND user_type = $2",
        )
        .bind(flag_id)
        .bind(user_type)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    // ==================== Epic 107: User Feature Preferences ====================

    /// Get user preference for a feature.
    pub async fn get_user_preference(
        &self,
        user_id: Uuid,
        flag_id: Uuid,
    ) -> Result<Option<UserFeaturePreference>, SqlxError> {
        let pref = sqlx::query_as::<_, UserFeaturePreference>(
            r#"
            SELECT id, user_id, feature_flag_id, is_enabled, created_at, updated_at
            FROM user_feature_preferences
            WHERE user_id = $1 AND feature_flag_id = $2
            "#,
        )
        .bind(user_id)
        .bind(flag_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(pref)
    }

    /// List all preferences for a user.
    pub async fn list_user_preferences(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<UserFeaturePreference>, SqlxError> {
        let prefs = sqlx::query_as::<_, UserFeaturePreference>(
            r#"
            SELECT id, user_id, feature_flag_id, is_enabled, created_at, updated_at
            FROM user_feature_preferences
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(prefs)
    }

    /// Set user preference for a feature.
    pub async fn set_user_preference(
        &self,
        user_id: Uuid,
        flag_id: Uuid,
        is_enabled: bool,
    ) -> Result<UserFeaturePreference, SqlxError> {
        let pref = sqlx::query_as::<_, UserFeaturePreference>(
            r#"
            INSERT INTO user_feature_preferences (user_id, feature_flag_id, is_enabled)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id, feature_flag_id)
            DO UPDATE SET is_enabled = $3, updated_at = NOW()
            RETURNING id, user_id, feature_flag_id, is_enabled, created_at, updated_at
            "#,
        )
        .bind(user_id)
        .bind(flag_id)
        .bind(is_enabled)
        .fetch_one(&self.pool)
        .await?;

        Ok(pref)
    }

    /// Delete user preference.
    pub async fn delete_user_preference(
        &self,
        user_id: Uuid,
        flag_id: Uuid,
    ) -> Result<bool, SqlxError> {
        let result = sqlx::query(
            "DELETE FROM user_feature_preferences WHERE user_id = $1 AND feature_flag_id = $2",
        )
        .bind(user_id)
        .bind(flag_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    // ==================== Epic 107: Feature Catalog API ====================

    /// Get the feature catalog with filtering and pagination.
    pub async fn get_catalog(
        &self,
        query: &FeatureCatalogQuery,
        user_id: Option<Uuid>,
        user_type: Option<&str>,
    ) -> Result<FeatureCatalogResponse, SqlxError> {
        let page = query.page.unwrap_or(1).max(1);
        let per_page = query.per_page.unwrap_or(20).clamp(1, 100);
        let offset = (page - 1) * per_page;

        // Build base query
        let mut sql = String::from(
            r#"
            SELECT f.id, f.key, f.is_enabled,
                   fd.display_name, fd.short_description, fd.full_description,
                   fd.icon, fd.preview_image_url, fd.tags, fd.benefits,
                   fd.badge_text, fd.is_highlighted, fd.depends_on, fd.conflicts_with,
                   fc.key as cat_key, fc.name as cat_name, fc.icon as cat_icon, fc.color as cat_color
            FROM feature_flags f
            LEFT JOIN feature_descriptors fd ON fd.feature_flag_id = f.id
            LEFT JOIN feature_categories fc ON fc.id = fd.category_id
            WHERE 1=1
            "#,
        );

        // Apply filters
        let mut bind_idx = 1;
        let mut params: Vec<String> = Vec::new();

        if let Some(ref cat) = query.category {
            sql.push_str(&format!(" AND fc.key = ${}", bind_idx));
            params.push(cat.clone());
            bind_idx += 1;
        }

        if let Some(ref search) = query.search {
            sql.push_str(&format!(
                " AND (f.key ILIKE ${} OR fd.display_name ILIKE ${} OR fd.short_description ILIKE ${})",
                bind_idx, bind_idx, bind_idx
            ));
            params.push(format!("%{}%", search));
            // bind_idx would be incremented here if more filters were added
        }

        sql.push_str(" ORDER BY fd.display_order, fd.display_name");
        sql.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

        // For simplicity, we'll build catalog items manually
        // In production, this would be optimized with proper bound parameters

        let flags = self.list_all().await?;
        let categories = self.list_categories().await?;

        let mut items: Vec<FeatureCatalogItem> = Vec::new();

        for flag in &flags {
            // Apply category filter
            let descriptor = self.get_descriptor(flag.id).await?;

            if let Some(ref cat_key) = query.category {
                if let Some(ref desc) = descriptor {
                    if let Some(cat_id) = desc.category_id {
                        let cat = categories.iter().find(|c| c.id == cat_id);
                        if cat.map(|c| &c.key) != Some(cat_key) {
                            continue;
                        }
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
            }

            // Apply search filter
            if let Some(ref search) = query.search {
                let search_lower = search.to_lowercase();
                let matches = flag.key.to_lowercase().contains(&search_lower)
                    || flag.name.to_lowercase().contains(&search_lower)
                    || descriptor
                        .as_ref()
                        .map(|d| d.display_name.to_lowercase().contains(&search_lower))
                        .unwrap_or(false);

                if !matches {
                    continue;
                }
            }

            // Build category summary
            let category_summary = if let Some(ref desc) = descriptor {
                if let Some(cat_id) = desc.category_id {
                    categories
                        .iter()
                        .find(|c| c.id == cat_id)
                        .map(|c| FeatureCategorySummary {
                            key: c.key.clone(),
                            name: c.name.clone(),
                            icon: c.icon.clone(),
                            color: c.color.clone(),
                        })
                } else {
                    None
                }
            } else {
                None
            };

            // Build descriptor display
            let descriptor_display = if let Some(desc) = descriptor {
                let tags: Vec<String> =
                    serde_json::from_value(desc.tags.clone()).unwrap_or_default();
                let benefits: Vec<String> =
                    serde_json::from_value(desc.benefits.clone()).unwrap_or_default();

                FeatureDescriptorDisplay {
                    display_name: desc.display_name,
                    short_description: desc.short_description,
                    full_description: desc.full_description,
                    icon: desc.icon,
                    preview_image_url: desc.preview_image_url,
                    category: category_summary,
                    tags,
                    benefits,
                    badge_text: desc.badge_text,
                    is_highlighted: desc.is_highlighted,
                }
            } else {
                FeatureDescriptorDisplay {
                    display_name: flag.name.clone(),
                    short_description: flag.description.clone(),
                    full_description: None,
                    icon: None,
                    preview_image_url: None,
                    category: None,
                    tags: vec![],
                    benefits: vec![],
                    badge_text: None,
                    is_highlighted: false,
                }
            };

            // Determine feature state for user
            let mut is_enabled = flag.is_enabled;
            let mut access_state = FeatureAccessState::Included;
            let mut can_toggle = false;

            if let Some(ut) = user_type {
                if let Some(access) = self.get_user_type_access(flag.id, ut).await? {
                    access_state = access.access_state.clone();
                    can_toggle = access.can_override;

                    match access.access_state {
                        FeatureAccessState::Excluded => {
                            is_enabled = false;
                        }
                        FeatureAccessState::Optional => {
                            if let Some(uid) = user_id {
                                if let Some(pref) = self.get_user_preference(uid, flag.id).await? {
                                    is_enabled = pref.is_enabled;
                                } else {
                                    is_enabled = access.default_enabled;
                                }
                            }
                        }
                        FeatureAccessState::Included => {
                            // Use flag's default
                        }
                    }
                }
            }

            // Get dependencies and conflicts from descriptor
            let deps: Vec<String> = self
                .get_descriptor(flag.id)
                .await?
                .map(|d| serde_json::from_value(d.depends_on).unwrap_or_default())
                .unwrap_or_default();

            let conflicts: Vec<String> = self
                .get_descriptor(flag.id)
                .await?
                .map(|d| serde_json::from_value(d.conflicts_with).unwrap_or_default())
                .unwrap_or_default();

            items.push(FeatureCatalogItem {
                key: flag.key.clone(),
                descriptor: descriptor_display,
                state: FeatureState {
                    is_enabled,
                    access_state,
                    can_toggle,
                },
                dependencies: deps,
                conflicts_with: conflicts,
            });
        }

        // Apply pagination
        let total = items.len() as i64;
        let start = offset as usize;
        let end = (start + per_page as usize).min(items.len());
        let paginated_items = if start < items.len() {
            items[start..end].to_vec()
        } else {
            vec![]
        };

        // Build category counts
        let category_counts: Vec<CategoryWithCount> = categories
            .iter()
            .map(|c| {
                let count = items
                    .iter()
                    .filter(|i| {
                        i.descriptor
                            .category
                            .as_ref()
                            .map(|cat| cat.key == c.key)
                            .unwrap_or(false)
                    })
                    .count() as i64;

                CategoryWithCount {
                    key: c.key.clone(),
                    name: c.name.clone(),
                    icon: c.icon.clone(),
                    color: c.color.clone(),
                    count,
                }
            })
            .filter(|c| c.count > 0)
            .collect();

        Ok(FeatureCatalogResponse {
            features: paginated_items,
            categories: category_counts,
            pagination: CatalogPagination {
                total,
                page,
                per_page,
            },
        })
    }
}

/// Feature flag with override count for list view.
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct FeatureFlagWithCount {
    pub id: Uuid,
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub is_enabled: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub override_count: i64,
}

impl From<FeatureFlag> for FeatureFlagWithCount {
    fn from(flag: FeatureFlag) -> Self {
        Self {
            id: flag.id,
            key: flag.key,
            name: flag.name,
            description: flag.description,
            is_enabled: flag.is_enabled,
            created_at: flag.created_at,
            updated_at: flag.updated_at,
            override_count: 0,
        }
    }
}

/// Feature flag with all its overrides.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct FeatureFlagWithOverrides {
    pub flag: FeatureFlag,
    pub overrides: Vec<FeatureFlagOverride>,
}

/// Resolved feature flag for client consumption.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ResolvedFeatureFlag {
    pub key: String,
    pub is_enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolved_feature_flag() {
        let flag = ResolvedFeatureFlag {
            key: "test_feature".to_string(),
            is_enabled: true,
        };

        assert_eq!(flag.key, "test_feature");
        assert!(flag.is_enabled);
    }

    #[test]
    fn test_feature_flag_scope_as_str() {
        assert_eq!(FeatureFlagScope::Organization.as_str(), "organization");
        assert_eq!(FeatureFlagScope::User.as_str(), "user");
        assert_eq!(FeatureFlagScope::Role.as_str(), "role");
    }
}

//! Feature resolution service (Epic 109, Story 109.1).
//!
//! Service for resolving feature flags based on user type, organization packages,
//! and user preferences.

use db::models::feature_analytics::{
    FeatureAccessState, FeatureDescriptorSummary, FeatureEventType, ResolvedFeature,
};
use db::repositories::{FeatureAnalyticsRepository, FeatureFlagRepository, FeatureFlagWithCount};
use db::DbPool;
use sqlx::Error as SqlxError;
use uuid::Uuid;

/// Source of feature resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionSource {
    /// Feature is from organization package
    Package,
    /// Feature has a user/org/role override
    Override,
    /// Feature uses default from user type access matrix
    Default,
    /// Feature uses global default
    Global,
}

impl ResolutionSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Package => "package",
            Self::Override => "override",
            Self::Default => "default",
            Self::Global => "global",
        }
    }
}

/// Service for feature resolution and analytics.
pub struct FeatureService {
    flag_repo: FeatureFlagRepository,
    analytics_repo: FeatureAnalyticsRepository,
}

impl FeatureService {
    /// Create a new FeatureService.
    pub fn new(pool: DbPool) -> Self {
        Self {
            flag_repo: FeatureFlagRepository::new(pool.clone()),
            analytics_repo: FeatureAnalyticsRepository::new(pool),
        }
    }

    /// Create from existing repositories.
    pub fn from_repos(
        flag_repo: FeatureFlagRepository,
        analytics_repo: FeatureAnalyticsRepository,
    ) -> Self {
        Self {
            flag_repo,
            analytics_repo,
        }
    }

    /// Resolve all features for a user context.
    ///
    /// Resolution order:
    /// 1. Check if excluded by user type -> not shown
    /// 2. Check user/org/role overrides -> use override value
    /// 3. Check if included in organization packages -> enabled
    /// 4. Check user preferences for optional features
    /// 5. Use user type default_enabled setting
    /// 6. Fall back to global feature flag default
    pub async fn resolve_features_for_user(
        &self,
        user_id: Uuid,
        org_id: Uuid,
        user_type: &str,
        role_id: Option<Uuid>,
        category: Option<&str>,
        enabled_only: bool,
    ) -> Result<Vec<ResolvedFeature>, SqlxError> {
        // 1. Get all feature flags
        let flags = self.flag_repo.list_all().await?;

        // 2. Get organization's active package features
        let org_package_features = self.analytics_repo.get_org_package_features(org_id).await?;

        // 3. Get user preferences
        let user_prefs = self.analytics_repo.get_user_preferences(user_id).await?;

        let mut resolved = Vec::new();

        for flag in flags {
            // Get user type access for this feature
            let access = self
                .analytics_repo
                .get_user_type_access(flag.id, user_type)
                .await?;

            // Get descriptor for UI metadata
            let descriptor = self.analytics_repo.get_descriptor(flag.id).await?;

            // Filter by category if specified
            if let Some(cat) = category {
                if let Some(ref desc) = descriptor {
                    if desc.category.as_deref() != Some(cat) {
                        continue;
                    }
                } else {
                    continue; // No descriptor means no category
                }
            }

            // Determine access state
            let access_state = access
                .as_ref()
                .map(|a| a.access_state.clone())
                .unwrap_or(FeatureAccessState::Excluded);

            // Skip excluded features
            if access_state == FeatureAccessState::Excluded {
                continue;
            }

            // Determine if feature can be toggled (only optional features)
            let can_toggle = access_state == FeatureAccessState::Optional;

            // Resolve enabled state
            let (is_enabled, source) = self
                .resolve_feature_state(
                    &flag,
                    user_id,
                    org_id,
                    role_id,
                    user_type,
                    &access_state,
                    access.as_ref().map(|a| a.default_enabled).unwrap_or(true),
                    &org_package_features,
                    &user_prefs,
                )
                .await?;

            // Filter if enabled_only is set
            if enabled_only && !is_enabled {
                continue;
            }

            let descriptor_summary = descriptor.map(|d| FeatureDescriptorSummary {
                display_name: d.display_name,
                short_description: d.short_description,
                icon: d.icon,
                badge_text: d.badge_text,
            });

            resolved.push(ResolvedFeature {
                key: flag.key,
                is_enabled,
                access_state: access_state.as_str().to_string(),
                can_toggle,
                source: source.as_str().to_string(),
                descriptor: descriptor_summary,
            });
        }

        Ok(resolved)
    }

    /// Resolve the enabled state for a single feature.
    #[allow(clippy::too_many_arguments)]
    async fn resolve_feature_state(
        &self,
        flag: &FeatureFlagWithCount,
        user_id: Uuid,
        org_id: Uuid,
        role_id: Option<Uuid>,
        _user_type: &str,
        access_state: &FeatureAccessState,
        default_enabled: bool,
        org_package_features: &[Uuid],
        user_prefs: &[db::models::feature_analytics::UserFeaturePreference],
    ) -> Result<(bool, ResolutionSource), SqlxError> {
        // 1. Check for user/org/role overrides (highest priority)
        if let Some(override_value) = self
            .flag_repo
            .is_enabled_for_context(&flag.key, Some(user_id), Some(org_id), role_id)
            .await?
        {
            // Check if there's actually an override (not just the default)
            if let Some(true) = self
                .has_specific_override(flag.id, user_id, org_id, role_id)
                .await?
            {
                return Ok((override_value, ResolutionSource::Override));
            }
        }

        // 2. Check if feature is in organization packages
        if org_package_features.contains(&flag.id) {
            return Ok((true, ResolutionSource::Package));
        }

        // 3. For optional features, check user preference
        if *access_state == FeatureAccessState::Optional {
            if let Some(pref) = user_prefs.iter().find(|p| p.feature_flag_id == flag.id) {
                return Ok((pref.is_enabled, ResolutionSource::Default));
            }
            // Fall back to default_enabled from access matrix
            return Ok((default_enabled, ResolutionSource::Default));
        }

        // 4. For included features, use user type default
        if *access_state == FeatureAccessState::Included {
            return Ok((default_enabled, ResolutionSource::Default));
        }

        // 5. Fall back to global flag default
        Ok((flag.is_enabled, ResolutionSource::Global))
    }

    /// Check if there's a specific override for the user/org/role.
    async fn has_specific_override(
        &self,
        flag_id: Uuid,
        user_id: Uuid,
        org_id: Uuid,
        role_id: Option<Uuid>,
    ) -> Result<Option<bool>, SqlxError> {
        // Check user override
        let user_override = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM feature_flag_overrides
                WHERE flag_id = $1 AND scope_type = 'user' AND scope_id = $2
            )
            "#,
        )
        .bind(flag_id)
        .bind(user_id)
        .fetch_one(self.get_pool())
        .await?;

        if user_override {
            return Ok(Some(true));
        }

        // Check org override
        let org_override = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM feature_flag_overrides
                WHERE flag_id = $1 AND scope_type = 'organization' AND scope_id = $2
            )
            "#,
        )
        .bind(flag_id)
        .bind(org_id)
        .fetch_one(self.get_pool())
        .await?;

        if org_override {
            return Ok(Some(true));
        }

        // Check role override if role_id provided
        if let Some(rid) = role_id {
            let role_override = sqlx::query_scalar::<_, bool>(
                r#"
                SELECT EXISTS(
                    SELECT 1 FROM feature_flag_overrides
                    WHERE flag_id = $1 AND scope_type = 'role' AND scope_id = $2
                )
                "#,
            )
            .bind(flag_id)
            .bind(rid)
            .fetch_one(self.get_pool())
            .await?;

            if role_override {
                return Ok(Some(true));
            }
        }

        Ok(None)
    }

    /// Get pool reference for raw queries.
    fn get_pool(&self) -> &db::DbPool {
        &self.analytics_repo.pool
    }

    /// Check if a specific feature is enabled for user.
    pub async fn is_feature_enabled(
        &self,
        feature_key: &str,
        user_id: Uuid,
        org_id: Uuid,
        user_type: &str,
        role_id: Option<Uuid>,
    ) -> Result<bool, SqlxError> {
        // Get the flag and convert to FeatureFlagWithCount
        let flag: FeatureFlagWithCount = match self.flag_repo.get_by_key(feature_key).await? {
            Some(f) => f.into(),
            None => return Ok(false), // Unknown feature is disabled
        };

        // Check user type access
        let access = self
            .analytics_repo
            .get_user_type_access(flag.id, user_type)
            .await?;

        let access_state = access
            .as_ref()
            .map(|a| a.access_state.clone())
            .unwrap_or(FeatureAccessState::Excluded);

        // Excluded features are always disabled
        if access_state == FeatureAccessState::Excluded {
            return Ok(false);
        }

        // Get org package features
        let org_package_features = self.analytics_repo.get_org_package_features(org_id).await?;

        // Get user preferences
        let user_prefs = self.analytics_repo.get_user_preferences(user_id).await?;

        let default_enabled = access.as_ref().map(|a| a.default_enabled).unwrap_or(true);

        let (is_enabled, _) = self
            .resolve_feature_state(
                &flag,
                user_id,
                org_id,
                role_id,
                user_type,
                &access_state,
                default_enabled,
                &org_package_features,
                &user_prefs,
            )
            .await?;

        Ok(is_enabled)
    }

    /// Log a feature usage event.
    pub async fn log_feature_event(
        &self,
        feature_key: &str,
        user_id: Option<Uuid>,
        org_id: Option<Uuid>,
        event_type: FeatureEventType,
        user_type: Option<&str>,
        metadata: serde_json::Value,
    ) -> Result<(), SqlxError> {
        // Get the flag
        let flag = match self.flag_repo.get_by_key(feature_key).await? {
            Some(f) => f,
            None => return Ok(()), // Silently ignore unknown features
        };

        self.analytics_repo
            .log_event(flag.id, user_id, org_id, event_type, user_type, metadata)
            .await?;

        Ok(())
    }

    /// Get upgrade options for a feature.
    pub async fn get_upgrade_options(
        &self,
        feature_key: &str,
    ) -> Result<Vec<db::models::feature_analytics::FeaturePackage>, SqlxError> {
        // Get the flag
        let flag = match self.flag_repo.get_by_key(feature_key).await? {
            Some(f) => f,
            None => return Ok(vec![]),
        };

        self.analytics_repo.get_packages_with_feature(flag.id).await
    }

    /// Toggle a user's preference for an optional feature.
    pub async fn toggle_user_feature(
        &self,
        user_id: Uuid,
        feature_key: &str,
        is_enabled: bool,
    ) -> Result<bool, SqlxError> {
        // Get the flag
        let flag = match self.flag_repo.get_by_key(feature_key).await? {
            Some(f) => f,
            None => return Ok(false),
        };

        self.analytics_repo
            .set_user_preference(user_id, flag.id, is_enabled)
            .await?;

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolution_source_as_str() {
        assert_eq!(ResolutionSource::Package.as_str(), "package");
        assert_eq!(ResolutionSource::Override.as_str(), "override");
        assert_eq!(ResolutionSource::Default.as_str(), "default");
        assert_eq!(ResolutionSource::Global.as_str(), "global");
    }
}

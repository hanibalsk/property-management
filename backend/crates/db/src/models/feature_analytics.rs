//! Feature analytics models (Epic 109, Story 109.4).
//!
//! Models for feature usage analytics, descriptors, packages, and user type access.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

// ==================== Feature Access State ====================

/// Access state for a feature by user type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "feature_access_state", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum FeatureAccessState {
    /// Feature is included and enabled by default for this user type
    Included,
    /// Feature is available but user can toggle it on/off
    Optional,
    /// Feature is not available for this user type
    Excluded,
}

impl FeatureAccessState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Included => "included",
            Self::Optional => "optional",
            Self::Excluded => "excluded",
        }
    }
}

impl std::fmt::Display for FeatureAccessState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ==================== Feature Event Type ====================

/// Type of feature usage event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "feature_event_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum FeatureEventType {
    /// User accessed/used the feature
    Access,
    /// User was blocked from using the feature
    Blocked,
    /// User was shown an upgrade prompt
    UpgradePrompt,
    /// User clicked on upgrade option
    UpgradeClicked,
    /// User toggled the feature on
    ToggledOn,
    /// User toggled the feature off
    ToggledOff,
}

impl FeatureEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Access => "access",
            Self::Blocked => "blocked",
            Self::UpgradePrompt => "upgrade_prompt",
            Self::UpgradeClicked => "upgrade_clicked",
            Self::ToggledOn => "toggled_on",
            Self::ToggledOff => "toggled_off",
        }
    }
}

impl std::fmt::Display for FeatureEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ==================== Feature Descriptor ====================

/// UI display metadata for a feature flag.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct FeatureDescriptor {
    pub id: Uuid,
    pub feature_flag_id: Uuid,
    pub display_name: String,
    pub short_description: Option<String>,
    pub long_description: Option<String>,
    pub icon: Option<String>,
    pub badge_text: Option<String>,
    pub help_url: Option<String>,
    pub category: Option<String>,
    pub sort_order: i32,
    pub is_premium: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Summary of feature descriptor for API responses.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FeatureDescriptorSummary {
    pub display_name: String,
    pub short_description: Option<String>,
    pub icon: Option<String>,
    pub badge_text: Option<String>,
}

impl From<FeatureDescriptor> for FeatureDescriptorSummary {
    fn from(d: FeatureDescriptor) -> Self {
        Self {
            display_name: d.display_name,
            short_description: d.short_description,
            icon: d.icon,
            badge_text: d.badge_text,
        }
    }
}

/// Request to create or update a feature descriptor.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpsertFeatureDescriptor {
    pub display_name: String,
    pub short_description: Option<String>,
    pub long_description: Option<String>,
    pub icon: Option<String>,
    pub badge_text: Option<String>,
    pub help_url: Option<String>,
    pub category: Option<String>,
    pub sort_order: Option<i32>,
    pub is_premium: Option<bool>,
}

// ==================== User Type Feature Access ====================

/// Access control entry for a feature by user type.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct UserTypeFeatureAccess {
    pub id: Uuid,
    pub feature_flag_id: Uuid,
    pub user_type: String,
    pub access_state: FeatureAccessState,
    pub default_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request to set user type feature access.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct SetUserTypeAccess {
    pub user_type: String,
    pub access_state: FeatureAccessState,
    #[serde(default = "default_true")]
    pub default_enabled: bool,
}

fn default_true() -> bool {
    true
}

// ==================== Feature Package ====================

/// Feature package for bundling features.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct FeaturePackage {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub price_monthly_cents: Option<i32>,
    pub price_yearly_cents: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request to create a feature package.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateFeaturePackage {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub price_monthly_cents: Option<i32>,
    pub price_yearly_cents: Option<i32>,
}

/// Request to update a feature package.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateFeaturePackage {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
    pub price_monthly_cents: Option<i32>,
    pub price_yearly_cents: Option<i32>,
}

/// Feature package item (links feature to package).
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct FeaturePackageItem {
    pub id: Uuid,
    pub package_id: Uuid,
    pub feature_flag_id: Uuid,
    pub created_at: DateTime<Utc>,
}

/// Package with its included features.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FeaturePackageWithFeatures {
    pub package: FeaturePackage,
    pub features: Vec<FeaturePackageFeature>,
}

/// Feature info within a package.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct FeaturePackageFeature {
    pub feature_flag_id: Uuid,
    pub key: String,
    pub name: String,
    pub description: Option<String>,
}

// ==================== Organization Feature Package ====================

/// Organization subscription to a feature package.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct OrganizationFeaturePackage {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub package_id: Uuid,
    pub is_active: bool,
    pub started_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Request to subscribe an organization to a package.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct SubscribeToPackage {
    pub package_id: Uuid,
    pub expires_at: Option<DateTime<Utc>>,
}

// ==================== User Feature Preferences ====================

/// User preference for an optional feature.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct UserFeaturePreference {
    pub id: Uuid,
    pub user_id: Uuid,
    pub feature_flag_id: Uuid,
    pub is_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request to set user feature preference.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct SetFeaturePreference {
    pub is_enabled: bool,
}

// ==================== Feature Usage Event ====================

/// Feature usage analytics event.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct FeatureUsageEvent {
    pub id: Uuid,
    pub feature_flag_id: Uuid,
    pub user_id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
    pub event_type: FeatureEventType,
    pub user_type: Option<String>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

/// Request to log a feature usage event.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct LogFeatureEvent {
    pub feature_key: String,
    pub event_type: FeatureEventType,
    #[serde(default)]
    pub metadata: serde_json::Value,
}

/// Feature usage statistics.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FeatureUsageStats {
    pub feature_flag_id: Uuid,
    pub feature_key: String,
    pub total_events: i64,
    pub access_count: i64,
    pub blocked_count: i64,
    pub upgrade_prompt_count: i64,
    pub upgrade_clicked_count: i64,
    pub toggled_on_count: i64,
    pub toggled_off_count: i64,
    pub unique_users: i64,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
}

/// Feature usage stats grouped by user type.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct FeatureStatsByUserType {
    pub user_type: Option<String>,
    pub total_events: i64,
    pub access_count: i64,
    pub blocked_count: i64,
    pub upgrade_prompt_count: i64,
    pub upgrade_clicked_count: i64,
}

// ==================== Resolved Feature ====================

/// Resolved feature state for a user (Story 109.1).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ResolvedFeature {
    /// Feature flag key
    pub key: String,
    /// Whether the feature is enabled for this user
    pub is_enabled: bool,
    /// Access state for user's type (included, optional, excluded)
    pub access_state: String,
    /// Whether the user can toggle this feature
    pub can_toggle: bool,
    /// Source of the resolution (package, override, default)
    pub source: String,
    /// UI display metadata
    pub descriptor: Option<FeatureDescriptorSummary>,
}

/// Query parameters for feature stats.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct FeatureStatsQuery {
    /// Start of the period (defaults to 30 days ago)
    pub start_date: Option<DateTime<Utc>>,
    /// End of the period (defaults to now)
    pub end_date: Option<DateTime<Utc>>,
}

/// Query parameters for resolved features.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ResolvedFeaturesQuery {
    /// Filter by category
    pub category: Option<String>,
    /// Include only enabled features
    pub enabled_only: Option<bool>,
}

/// Response for upgrade options.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpgradeOptionsResponse {
    pub feature_key: String,
    pub packages: Vec<FeaturePackage>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_access_state_as_str() {
        assert_eq!(FeatureAccessState::Included.as_str(), "included");
        assert_eq!(FeatureAccessState::Optional.as_str(), "optional");
        assert_eq!(FeatureAccessState::Excluded.as_str(), "excluded");
    }

    #[test]
    fn test_feature_event_type_as_str() {
        assert_eq!(FeatureEventType::Access.as_str(), "access");
        assert_eq!(FeatureEventType::Blocked.as_str(), "blocked");
        assert_eq!(FeatureEventType::UpgradePrompt.as_str(), "upgrade_prompt");
        assert_eq!(FeatureEventType::UpgradeClicked.as_str(), "upgrade_clicked");
        assert_eq!(FeatureEventType::ToggledOn.as_str(), "toggled_on");
        assert_eq!(FeatureEventType::ToggledOff.as_str(), "toggled_off");
    }

    #[test]
    fn test_descriptor_summary_from() {
        let descriptor = FeatureDescriptor {
            id: Uuid::new_v4(),
            feature_flag_id: Uuid::new_v4(),
            display_name: "Test Feature".to_string(),
            short_description: Some("A test".to_string()),
            long_description: Some("A longer description".to_string()),
            icon: Some("icon-test".to_string()),
            badge_text: Some("New".to_string()),
            help_url: Some("https://help.example.com".to_string()),
            category: Some("testing".to_string()),
            sort_order: 1,
            is_premium: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let summary: FeatureDescriptorSummary = descriptor.into();
        assert_eq!(summary.display_name, "Test Feature");
        assert_eq!(summary.short_description, Some("A test".to_string()));
        assert_eq!(summary.icon, Some("icon-test".to_string()));
        assert_eq!(summary.badge_text, Some("New".to_string()));
    }
}

//! Feature package models (Epic 108).
//!
//! Models for feature packages that bundle multiple features for subscription plans
//! or standalone purchases.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Package type enum.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "package_type", rename_all = "snake_case")]
pub enum PackageType {
    /// Base package (standalone plan)
    #[default]
    Base,
    /// Add-on package (requires base)
    Addon,
    /// Trial package (time-limited)
    Trial,
}

/// Package source constants (how the package was acquired).
pub mod package_source {
    pub const SUBSCRIPTION: &str = "subscription";
    pub const PURCHASE: &str = "purchase";
    pub const PROMOTION: &str = "promotion";
    pub const TRIAL: &str = "trial";
    pub const MANUAL: &str = "manual";
    pub const ALL: &[&str] = &[SUBSCRIPTION, PURCHASE, PROMOTION, TRIAL, MANUAL];
}

// ==================== Feature Package ====================

/// Feature package entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct FeaturePackage {
    pub id: Uuid,
    pub key: String,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub short_description: Option<String>,
    pub icon: Option<String>,
    pub package_type: PackageType,
    pub parent_package_id: Option<Uuid>,
    pub linked_plan_id: Option<Uuid>,
    pub standalone_monthly_price: Option<Decimal>,
    pub standalone_annual_price: Option<Decimal>,
    pub currency: Option<String>,
    pub max_users: Option<i32>,
    pub max_buildings: Option<i32>,
    pub max_units: Option<i32>,
    pub display_order: Option<i32>,
    pub is_highlighted: Option<bool>,
    pub highlight_text: Option<String>,
    pub color: Option<String>,
    pub is_active: Option<bool>,
    pub is_public: Option<bool>,
    pub version: Option<i32>,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_until: Option<DateTime<Utc>>,
    pub translations: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Create feature package request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateFeaturePackage {
    pub key: String,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub short_description: Option<String>,
    pub icon: Option<String>,
    pub package_type: Option<PackageType>,
    pub parent_package_id: Option<Uuid>,
    pub linked_plan_id: Option<Uuid>,
    pub standalone_monthly_price: Option<Decimal>,
    pub standalone_annual_price: Option<Decimal>,
    pub currency: Option<String>,
    pub max_users: Option<i32>,
    pub max_buildings: Option<i32>,
    pub max_units: Option<i32>,
    pub display_order: Option<i32>,
    pub is_highlighted: Option<bool>,
    pub highlight_text: Option<String>,
    pub color: Option<String>,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_until: Option<DateTime<Utc>>,
    pub translations: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

/// Update feature package request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateFeaturePackage {
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub short_description: Option<String>,
    pub icon: Option<String>,
    pub package_type: Option<PackageType>,
    pub parent_package_id: Option<Uuid>,
    pub linked_plan_id: Option<Uuid>,
    pub standalone_monthly_price: Option<Decimal>,
    pub standalone_annual_price: Option<Decimal>,
    pub currency: Option<String>,
    pub max_users: Option<i32>,
    pub max_buildings: Option<i32>,
    pub max_units: Option<i32>,
    pub display_order: Option<i32>,
    pub is_highlighted: Option<bool>,
    pub highlight_text: Option<String>,
    pub color: Option<String>,
    pub is_active: Option<bool>,
    pub is_public: Option<bool>,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_until: Option<DateTime<Utc>>,
    pub translations: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

// ==================== Feature Package Item ====================

/// Feature package item entity (feature included in a package).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct FeaturePackageItem {
    pub id: Uuid,
    pub package_id: Uuid,
    pub feature_flag_id: Uuid,
    pub custom_description: Option<String>,
    pub usage_limit: Option<i32>,
    pub usage_unit: Option<String>,
    pub display_order: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
}

/// Create feature package item request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateFeaturePackageItem {
    pub feature_flag_id: Uuid,
    pub custom_description: Option<String>,
    pub usage_limit: Option<i32>,
    pub usage_unit: Option<String>,
    pub display_order: Option<i32>,
}

/// Batch add features request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct BatchAddFeatures {
    pub features: Vec<CreateFeaturePackageItem>,
}

// ==================== Organization Package ====================

/// Organization package entity (package assigned to an organization).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct OrganizationPackage {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub package_id: Uuid,
    pub source: String,
    pub subscription_id: Option<Uuid>,
    pub is_active: Option<bool>,
    pub activated_at: Option<DateTime<Utc>>,
    pub deactivated_at: Option<DateTime<Utc>>,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_until: Option<DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Create organization package request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateOrganizationPackage {
    pub organization_id: Uuid,
    pub package_id: Uuid,
    pub source: String,
    pub subscription_id: Option<Uuid>,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_until: Option<DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>,
}

/// Update organization package request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateOrganizationPackage {
    pub is_active: Option<bool>,
    pub valid_until: Option<DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>,
}

// ==================== View Types ====================

/// Feature package with included features count.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct FeaturePackageSummary {
    pub id: Uuid,
    pub key: String,
    pub name: String,
    pub display_name: String,
    pub short_description: Option<String>,
    pub icon: Option<String>,
    pub package_type: PackageType,
    pub standalone_monthly_price: Option<Decimal>,
    pub standalone_annual_price: Option<Decimal>,
    pub currency: Option<String>,
    pub display_order: Option<i32>,
    pub is_highlighted: Option<bool>,
    pub highlight_text: Option<String>,
    pub color: Option<String>,
    pub feature_count: i64,
}

/// Feature package with full details including features.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FeaturePackageWithFeatures {
    pub package: FeaturePackage,
    pub features: Vec<FeaturePackageItemWithDetails>,
}

/// Feature package item with feature flag details.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct FeaturePackageItemWithDetails {
    pub id: Uuid,
    pub package_id: Uuid,
    pub feature_flag_id: Uuid,
    pub custom_description: Option<String>,
    pub usage_limit: Option<i32>,
    pub usage_unit: Option<String>,
    pub display_order: Option<i32>,
    pub feature_key: String,
    pub feature_name: String,
    pub feature_description: Option<String>,
    pub feature_is_enabled: bool,
}

/// Organization package with package details.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct OrganizationPackageWithDetails {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub package_id: Uuid,
    pub source: String,
    pub subscription_id: Option<Uuid>,
    pub is_active: Option<bool>,
    pub activated_at: Option<DateTime<Utc>>,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_until: Option<DateTime<Utc>>,
    pub package_key: String,
    pub package_name: String,
    pub package_display_name: String,
    pub package_type: PackageType,
}

/// Package comparison entry for comparing packages side-by-side.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PackageComparison {
    pub packages: Vec<FeaturePackageSummary>,
    pub features: Vec<FeatureComparisonRow>,
}

/// Feature comparison row showing feature availability across packages.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FeatureComparisonRow {
    pub feature_key: String,
    pub feature_name: String,
    pub feature_description: Option<String>,
    /// Map of package_id -> inclusion details (null if not included)
    pub packages: serde_json::Value,
}

/// Query parameters for listing feature packages.
#[derive(Debug, Default, Deserialize, ToSchema)]
pub struct FeaturePackageQuery {
    pub package_type: Option<String>,
    pub is_active: Option<bool>,
    pub is_public: Option<bool>,
    pub linked_plan_id: Option<Uuid>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// Public package listing for marketing pages.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PublicPackage {
    pub id: Uuid,
    pub key: String,
    pub display_name: String,
    pub short_description: Option<String>,
    pub icon: Option<String>,
    pub package_type: PackageType,
    pub standalone_monthly_price: Option<Decimal>,
    pub standalone_annual_price: Option<Decimal>,
    pub currency: Option<String>,
    pub max_users: Option<i32>,
    pub max_buildings: Option<i32>,
    pub max_units: Option<i32>,
    pub display_order: Option<i32>,
    pub is_highlighted: Option<bool>,
    pub highlight_text: Option<String>,
    pub color: Option<String>,
    pub feature_count: i64,
}

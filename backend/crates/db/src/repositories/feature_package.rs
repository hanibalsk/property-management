//! Feature package repository (Epic 108).
//!
//! Repository for feature package management including CRUD operations,
//! feature assignments, and organization package management.

use crate::models::feature_package::{
    BatchAddFeatures, CreateFeaturePackage, CreateFeaturePackageItem, CreateOrganizationPackage,
    FeaturePackage, FeaturePackageItem, FeaturePackageItemWithDetails, FeaturePackageQuery,
    FeaturePackageSummary, FeaturePackageWithFeatures, OrganizationPackage,
    OrganizationPackageWithDetails, PublicPackage, UpdateFeaturePackage, UpdateOrganizationPackage,
};
use sqlx::PgPool;
use uuid::Uuid;

/// Repository for feature package operations.
#[derive(Clone)]
pub struct FeaturePackageRepository {
    pool: PgPool,
}

impl FeaturePackageRepository {
    /// Create a new FeaturePackageRepository.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // ==================== Feature Package CRUD ====================

    /// Create a new feature package.
    pub async fn create_package(
        &self,
        data: CreateFeaturePackage,
    ) -> Result<FeaturePackage, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO feature_packages
                (key, name, display_name, description, short_description, icon,
                 package_type, parent_package_id, linked_plan_id,
                 standalone_monthly_price, standalone_annual_price, currency,
                 max_users, max_buildings, max_units, display_order,
                 is_highlighted, highlight_text, color,
                 valid_from, valid_until, translations, metadata)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23)
            RETURNING *
            "#,
        )
        .bind(&data.key)
        .bind(&data.name)
        .bind(&data.display_name)
        .bind(&data.description)
        .bind(&data.short_description)
        .bind(&data.icon)
        .bind(data.package_type.unwrap_or_default())
        .bind(data.parent_package_id)
        .bind(data.linked_plan_id)
        .bind(data.standalone_monthly_price)
        .bind(data.standalone_annual_price)
        .bind(data.currency.unwrap_or_else(|| "EUR".to_string()))
        .bind(data.max_users)
        .bind(data.max_buildings)
        .bind(data.max_units)
        .bind(data.display_order.unwrap_or(0))
        .bind(data.is_highlighted.unwrap_or(false))
        .bind(&data.highlight_text)
        .bind(&data.color)
        .bind(data.valid_from)
        .bind(data.valid_until)
        .bind(&data.translations)
        .bind(&data.metadata)
        .fetch_one(&self.pool)
        .await
    }

    /// Find a feature package by ID.
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<FeaturePackage>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM feature_packages WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    /// Find a feature package by key.
    pub async fn find_by_key(&self, key: &str) -> Result<Option<FeaturePackage>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM feature_packages WHERE key = $1")
            .bind(key)
            .fetch_optional(&self.pool)
            .await
    }

    /// List all feature packages with feature counts.
    pub async fn list_packages(
        &self,
        query: FeaturePackageQuery,
    ) -> Result<Vec<FeaturePackageSummary>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT
                fp.id, fp.key, fp.name, fp.display_name, fp.short_description,
                fp.icon, fp.package_type, fp.standalone_monthly_price,
                fp.standalone_annual_price, fp.currency, fp.display_order,
                fp.is_highlighted, fp.highlight_text, fp.color,
                COUNT(fpi.id) as feature_count
            FROM feature_packages fp
            LEFT JOIN feature_package_items fpi ON fpi.package_id = fp.id
            WHERE ($1::text IS NULL OR fp.package_type::text = $1)
            AND ($2::bool IS NULL OR fp.is_active = $2)
            AND ($3::bool IS NULL OR fp.is_public = $3)
            AND ($4::uuid IS NULL OR fp.linked_plan_id = $4)
            GROUP BY fp.id
            ORDER BY fp.display_order, fp.name
            LIMIT $5 OFFSET $6
            "#,
        )
        .bind(&query.package_type)
        .bind(query.is_active)
        .bind(query.is_public)
        .bind(query.linked_plan_id)
        .bind(query.limit.unwrap_or(50))
        .bind(query.offset.unwrap_or(0))
        .fetch_all(&self.pool)
        .await
    }

    /// Get a package with all its features.
    pub async fn get_package_with_features(
        &self,
        id: Uuid,
    ) -> Result<Option<FeaturePackageWithFeatures>, sqlx::Error> {
        let package = match self.find_by_id(id).await? {
            Some(p) => p,
            None => return Ok(None),
        };

        let features = self.list_package_features(id).await?;

        Ok(Some(FeaturePackageWithFeatures { package, features }))
    }

    /// Update a feature package.
    pub async fn update_package(
        &self,
        id: Uuid,
        data: UpdateFeaturePackage,
    ) -> Result<FeaturePackage, sqlx::Error> {
        sqlx::query_as(
            r#"
            UPDATE feature_packages SET
                name = COALESCE($2, name),
                display_name = COALESCE($3, display_name),
                description = COALESCE($4, description),
                short_description = COALESCE($5, short_description),
                icon = COALESCE($6, icon),
                package_type = COALESCE($7, package_type),
                parent_package_id = COALESCE($8, parent_package_id),
                linked_plan_id = COALESCE($9, linked_plan_id),
                standalone_monthly_price = COALESCE($10, standalone_monthly_price),
                standalone_annual_price = COALESCE($11, standalone_annual_price),
                currency = COALESCE($12, currency),
                max_users = COALESCE($13, max_users),
                max_buildings = COALESCE($14, max_buildings),
                max_units = COALESCE($15, max_units),
                display_order = COALESCE($16, display_order),
                is_highlighted = COALESCE($17, is_highlighted),
                highlight_text = COALESCE($18, highlight_text),
                color = COALESCE($19, color),
                is_active = COALESCE($20, is_active),
                is_public = COALESCE($21, is_public),
                valid_from = COALESCE($22, valid_from),
                valid_until = COALESCE($23, valid_until),
                translations = COALESCE($24, translations),
                metadata = COALESCE($25, metadata),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&data.name)
        .bind(&data.display_name)
        .bind(&data.description)
        .bind(&data.short_description)
        .bind(&data.icon)
        .bind(&data.package_type)
        .bind(data.parent_package_id)
        .bind(data.linked_plan_id)
        .bind(data.standalone_monthly_price)
        .bind(data.standalone_annual_price)
        .bind(&data.currency)
        .bind(data.max_users)
        .bind(data.max_buildings)
        .bind(data.max_units)
        .bind(data.display_order)
        .bind(data.is_highlighted)
        .bind(&data.highlight_text)
        .bind(&data.color)
        .bind(data.is_active)
        .bind(data.is_public)
        .bind(data.valid_from)
        .bind(data.valid_until)
        .bind(&data.translations)
        .bind(&data.metadata)
        .fetch_one(&self.pool)
        .await
    }

    /// Soft delete a feature package (set is_active = false).
    pub async fn soft_delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            "UPDATE feature_packages SET is_active = false, updated_at = NOW() WHERE id = $1",
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    /// Hard delete a feature package.
    pub async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM feature_packages WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    // ==================== Feature Package Items ====================

    /// Add a feature to a package.
    pub async fn add_feature(
        &self,
        package_id: Uuid,
        data: CreateFeaturePackageItem,
    ) -> Result<FeaturePackageItem, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO feature_package_items
                (package_id, feature_flag_id, custom_description, usage_limit, usage_unit, display_order)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (package_id, feature_flag_id) DO UPDATE SET
                custom_description = EXCLUDED.custom_description,
                usage_limit = EXCLUDED.usage_limit,
                usage_unit = EXCLUDED.usage_unit,
                display_order = EXCLUDED.display_order
            RETURNING *
            "#,
        )
        .bind(package_id)
        .bind(data.feature_flag_id)
        .bind(&data.custom_description)
        .bind(data.usage_limit)
        .bind(&data.usage_unit)
        .bind(data.display_order.unwrap_or(0))
        .fetch_one(&self.pool)
        .await
    }

    /// Add multiple features to a package.
    pub async fn add_features_batch(
        &self,
        package_id: Uuid,
        data: BatchAddFeatures,
    ) -> Result<Vec<FeaturePackageItem>, sqlx::Error> {
        let mut items = Vec::new();
        for feature in data.features {
            let item = self.add_feature(package_id, feature).await?;
            items.push(item);
        }
        Ok(items)
    }

    /// Remove a feature from a package.
    pub async fn remove_feature(
        &self,
        package_id: Uuid,
        feature_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM feature_package_items WHERE package_id = $1 AND feature_flag_id = $2",
        )
        .bind(package_id)
        .bind(feature_id)
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    /// List features in a package with details.
    pub async fn list_package_features(
        &self,
        package_id: Uuid,
    ) -> Result<Vec<FeaturePackageItemWithDetails>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT
                fpi.id, fpi.package_id, fpi.feature_flag_id,
                fpi.custom_description, fpi.usage_limit, fpi.usage_unit, fpi.display_order,
                ff.key as feature_key, ff.name as feature_name,
                ff.description as feature_description, ff.is_enabled as feature_is_enabled
            FROM feature_package_items fpi
            JOIN feature_flags ff ON ff.id = fpi.feature_flag_id
            WHERE fpi.package_id = $1
            ORDER BY fpi.display_order, ff.name
            "#,
        )
        .bind(package_id)
        .fetch_all(&self.pool)
        .await
    }

    // ==================== Organization Packages ====================

    /// Assign a package to an organization.
    pub async fn assign_to_organization(
        &self,
        data: CreateOrganizationPackage,
    ) -> Result<OrganizationPackage, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO organization_packages
                (organization_id, package_id, source, subscription_id, valid_from, valid_until, metadata)
            VALUES ($1, $2, $3, $4, COALESCE($5, NOW()), $6, $7)
            ON CONFLICT (organization_id, package_id) DO UPDATE SET
                source = EXCLUDED.source,
                subscription_id = EXCLUDED.subscription_id,
                is_active = true,
                activated_at = NOW(),
                deactivated_at = NULL,
                valid_from = EXCLUDED.valid_from,
                valid_until = EXCLUDED.valid_until,
                metadata = EXCLUDED.metadata,
                updated_at = NOW()
            RETURNING *
            "#,
        )
        .bind(data.organization_id)
        .bind(data.package_id)
        .bind(&data.source)
        .bind(data.subscription_id)
        .bind(data.valid_from)
        .bind(data.valid_until)
        .bind(&data.metadata)
        .fetch_one(&self.pool)
        .await
    }

    /// Update an organization package.
    pub async fn update_organization_package(
        &self,
        id: Uuid,
        data: UpdateOrganizationPackage,
    ) -> Result<OrganizationPackage, sqlx::Error> {
        sqlx::query_as(
            r#"
            UPDATE organization_packages SET
                is_active = COALESCE($2, is_active),
                valid_until = COALESCE($3, valid_until),
                metadata = COALESCE($4, metadata),
                deactivated_at = CASE WHEN $2 = false THEN NOW() ELSE deactivated_at END,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(data.is_active)
        .bind(data.valid_until)
        .bind(&data.metadata)
        .fetch_one(&self.pool)
        .await
    }

    /// Deactivate an organization package.
    pub async fn deactivate_organization_package(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            "UPDATE organization_packages SET is_active = false, deactivated_at = NOW(), updated_at = NOW() WHERE id = $1",
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    /// Get active packages for an organization.
    pub async fn get_organization_packages(
        &self,
        org_id: Uuid,
    ) -> Result<Vec<OrganizationPackageWithDetails>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT
                op.id, op.organization_id, op.package_id, op.source,
                op.subscription_id, op.is_active, op.activated_at,
                op.valid_from, op.valid_until,
                fp.key as package_key, fp.name as package_name,
                fp.display_name as package_display_name, fp.package_type
            FROM organization_packages op
            JOIN feature_packages fp ON fp.id = op.package_id
            WHERE op.organization_id = $1
            AND op.is_active = true
            AND (op.valid_until IS NULL OR op.valid_until > NOW())
            ORDER BY fp.display_order, fp.name
            "#,
        )
        .bind(org_id)
        .fetch_all(&self.pool)
        .await
    }

    /// Check if an organization has a specific package.
    pub async fn has_package(&self, org_id: Uuid, package_id: Uuid) -> Result<bool, sqlx::Error> {
        let result: Option<(bool,)> = sqlx::query_as(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM organization_packages
                WHERE organization_id = $1 AND package_id = $2
                AND is_active = true
                AND (valid_until IS NULL OR valid_until > NOW())
            )
            "#,
        )
        .bind(org_id)
        .bind(package_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|(exists,)| exists).unwrap_or(false))
    }

    /// Get packages for a subscription plan.
    pub async fn get_packages_for_plan(
        &self,
        plan_id: Uuid,
    ) -> Result<Vec<FeaturePackageSummary>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT
                fp.id, fp.key, fp.name, fp.display_name, fp.short_description,
                fp.icon, fp.package_type, fp.standalone_monthly_price,
                fp.standalone_annual_price, fp.currency, fp.display_order,
                fp.is_highlighted, fp.highlight_text, fp.color,
                COUNT(fpi.id) as feature_count
            FROM feature_packages fp
            LEFT JOIN feature_package_items fpi ON fpi.package_id = fp.id
            WHERE fp.linked_plan_id = $1
            AND fp.is_active = true
            GROUP BY fp.id
            ORDER BY fp.display_order, fp.name
            "#,
        )
        .bind(plan_id)
        .fetch_all(&self.pool)
        .await
    }

    // ==================== Public API ====================

    /// List public packages for marketing pages.
    pub async fn list_public_packages(&self) -> Result<Vec<PublicPackage>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT
                fp.id, fp.key, fp.display_name, fp.short_description,
                fp.icon, fp.package_type, fp.standalone_monthly_price,
                fp.standalone_annual_price, fp.currency,
                fp.max_users, fp.max_buildings, fp.max_units,
                fp.display_order, fp.is_highlighted, fp.highlight_text, fp.color,
                COUNT(fpi.id) as feature_count
            FROM feature_packages fp
            LEFT JOIN feature_package_items fpi ON fpi.package_id = fp.id
            WHERE fp.is_active = true
            AND fp.is_public = true
            AND (fp.valid_from IS NULL OR fp.valid_from <= NOW())
            AND (fp.valid_until IS NULL OR fp.valid_until > NOW())
            GROUP BY fp.id
            ORDER BY fp.display_order, fp.name
            "#,
        )
        .fetch_all(&self.pool)
        .await
    }

    /// Get public package with features for comparison.
    pub async fn get_public_package_with_features(
        &self,
        id: Uuid,
    ) -> Result<Option<FeaturePackageWithFeatures>, sqlx::Error> {
        let package: Option<FeaturePackage> = sqlx::query_as(
            r#"
            SELECT * FROM feature_packages
            WHERE id = $1 AND is_active = true AND is_public = true
            AND (valid_from IS NULL OR valid_from <= NOW())
            AND (valid_until IS NULL OR valid_until > NOW())
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        let package = match package {
            Some(p) => p,
            None => return Ok(None),
        };

        let features = self.list_package_features(id).await?;

        Ok(Some(FeaturePackageWithFeatures { package, features }))
    }

    /// Compare multiple packages.
    pub async fn compare_packages(
        &self,
        package_ids: Vec<Uuid>,
    ) -> Result<
        (
            Vec<FeaturePackageSummary>,
            Vec<FeaturePackageItemWithDetails>,
        ),
        sqlx::Error,
    > {
        let packages: Vec<FeaturePackageSummary> = sqlx::query_as(
            r#"
            SELECT
                fp.id, fp.key, fp.name, fp.display_name, fp.short_description,
                fp.icon, fp.package_type, fp.standalone_monthly_price,
                fp.standalone_annual_price, fp.currency, fp.display_order,
                fp.is_highlighted, fp.highlight_text, fp.color,
                COUNT(fpi.id) as feature_count
            FROM feature_packages fp
            LEFT JOIN feature_package_items fpi ON fpi.package_id = fp.id
            WHERE fp.id = ANY($1)
            AND fp.is_active = true
            GROUP BY fp.id
            ORDER BY fp.display_order, fp.name
            "#,
        )
        .bind(&package_ids)
        .fetch_all(&self.pool)
        .await?;

        let features: Vec<FeaturePackageItemWithDetails> = sqlx::query_as(
            r#"
            SELECT
                fpi.id, fpi.package_id, fpi.feature_flag_id,
                fpi.custom_description, fpi.usage_limit, fpi.usage_unit, fpi.display_order,
                ff.key as feature_key, ff.name as feature_name,
                ff.description as feature_description, ff.is_enabled as feature_is_enabled
            FROM feature_package_items fpi
            JOIN feature_flags ff ON ff.id = fpi.feature_flag_id
            WHERE fpi.package_id = ANY($1)
            ORDER BY ff.name, fpi.display_order
            "#,
        )
        .bind(&package_ids)
        .fetch_all(&self.pool)
        .await?;

        Ok((packages, features))
    }

    /// Assign packages to organization based on subscription plan.
    /// This is called when an organization subscribes to a plan.
    pub async fn assign_plan_packages(
        &self,
        org_id: Uuid,
        plan_id: Uuid,
        subscription_id: Uuid,
    ) -> Result<Vec<OrganizationPackage>, sqlx::Error> {
        let packages = self.get_packages_for_plan(plan_id).await?;
        let mut assigned = Vec::new();

        for package in packages {
            let org_package = self
                .assign_to_organization(CreateOrganizationPackage {
                    organization_id: org_id,
                    package_id: package.id,
                    source: "subscription".to_string(),
                    subscription_id: Some(subscription_id),
                    valid_from: None,
                    valid_until: None,
                    metadata: None,
                })
                .await?;
            assigned.push(org_package);
        }

        Ok(assigned)
    }

    /// Deactivate all packages for a subscription (when cancelled).
    pub async fn deactivate_subscription_packages(
        &self,
        subscription_id: Uuid,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            r#"
            UPDATE organization_packages
            SET is_active = false, deactivated_at = NOW(), updated_at = NOW()
            WHERE subscription_id = $1 AND is_active = true
            "#,
        )
        .bind(subscription_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}

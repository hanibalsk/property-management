//! Entity factory functions for seeding.
//!
//! Provides factory methods for creating users, organizations, buildings,
//! units, and assignments in the database.

use crate::DbPool;
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use chrono::{Datelike, NaiveDate, Utc};
use rand::rngs::OsRng;
use rust_decimal::Decimal;
use sqlx::Row;
use uuid::Uuid;

use super::data::{SeedBuilding, SeedUnit};

/// Factory for creating seed entities in the database.
pub struct SeedFactories<'a> {
    pool: &'a DbPool,
}

impl<'a> SeedFactories<'a> {
    /// Create a new factory with the given pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Hash a password using Argon2id.
    ///
    /// This matches the password hashing used in AuthService.
    pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2.hash_password(password.as_bytes(), &salt)?;
        Ok(hash.to_string())
    }

    /// Create a user with a hashed password.
    ///
    /// Returns the user's UUID.
    pub async fn create_user(
        &self,
        email: &str,
        name: &str,
        password_hash: &str,
        phone: Option<&str>,
        is_super_admin: bool,
    ) -> Result<Uuid, sqlx::Error> {
        let row = sqlx::query(
            r#"
            INSERT INTO users (
                email, password_hash, name, phone, status,
                email_verified_at, is_super_admin, locale
            )
            VALUES ($1, $2, $3, $4, 'active', NOW(), $5, 'en')
            RETURNING id
            "#,
        )
        .bind(email)
        .bind(password_hash)
        .bind(name)
        .bind(phone)
        .bind(is_super_admin)
        .fetch_one(self.pool)
        .await?;

        Ok(row.get("id"))
    }

    /// Create an organization.
    ///
    /// This triggers automatic creation of default roles.
    /// Returns the organization's UUID.
    pub async fn create_organization(
        &self,
        name: &str,
        slug: &str,
        contact_email: &str,
    ) -> Result<Uuid, sqlx::Error> {
        let row = sqlx::query(
            r#"
            INSERT INTO organizations (name, slug, contact_email, status, settings)
            VALUES ($1, $2, $3, 'active', '{}'::jsonb)
            RETURNING id
            "#,
        )
        .bind(name)
        .bind(slug)
        .bind(contact_email)
        .fetch_one(self.pool)
        .await?;

        Ok(row.get("id"))
    }

    /// Add a user to an organization with a specific role.
    ///
    /// The role_type should match one of the system roles created
    /// when the organization was created.
    ///
    /// Returns the membership UUID.
    pub async fn add_organization_member(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        role_type: &str,
    ) -> Result<Uuid, sqlx::Error> {
        // Get the role_id for this role type in the organization
        let role_row = sqlx::query(
            r#"
            SELECT id FROM roles
            WHERE organization_id = $1 AND name = $2
            "#,
        )
        .bind(org_id)
        .bind(role_type)
        .fetch_optional(self.pool)
        .await?;

        let role_id: Option<Uuid> = role_row.map(|r| r.get("id"));

        let row = sqlx::query(
            r#"
            INSERT INTO organization_members (
                organization_id, user_id, role_id, role_type,
                status, joined_at
            )
            VALUES ($1, $2, $3, $4, 'active', NOW())
            RETURNING id
            "#,
        )
        .bind(org_id)
        .bind(user_id)
        .bind(role_id)
        .bind(role_type)
        .fetch_one(self.pool)
        .await?;

        Ok(row.get("id"))
    }

    /// Create a building in an organization.
    ///
    /// Returns the building's UUID.
    pub async fn create_building(
        &self,
        org_id: Uuid,
        building: &SeedBuilding,
    ) -> Result<Uuid, sqlx::Error> {
        let row = sqlx::query(
            r#"
            INSERT INTO buildings (
                organization_id, name, street, city, postal_code, country,
                total_floors, year_built, total_entrances, status,
                amenities, contacts, settings
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 1, 'active', '[]'::jsonb, '[]'::jsonb, '{}'::jsonb)
            RETURNING id
            "#,
        )
        .bind(org_id)
        .bind(building.name)
        .bind(building.street)
        .bind(building.city)
        .bind(building.postal_code)
        .bind(building.country)
        .bind(building.total_floors)
        .bind(building.year_built)
        .fetch_one(self.pool)
        .await?;

        Ok(row.get("id"))
    }

    /// Create a unit in a building.
    ///
    /// Returns the unit's UUID.
    pub async fn create_unit(
        &self,
        building_id: Uuid,
        unit: &SeedUnit,
    ) -> Result<Uuid, sqlx::Error> {
        let size_sqm = unit.size_sqm.map(Decimal::from);

        let row = sqlx::query(
            r#"
            INSERT INTO units (
                building_id, designation, floor, unit_type,
                size_sqm, rooms, ownership_share, occupancy_status, status, settings
            )
            VALUES ($1, $2, $3, $4, $5, $6, 100.00, 'unknown', 'active', '{}'::jsonb)
            RETURNING id
            "#,
        )
        .bind(building_id)
        .bind(unit.designation)
        .bind(unit.floor)
        .bind(unit.unit_type)
        .bind(size_sqm)
        .bind(unit.rooms)
        .fetch_one(self.pool)
        .await?;

        Ok(row.get("id"))
    }

    /// Assign a user as a resident of a unit.
    ///
    /// Returns the unit_resident UUID.
    pub async fn assign_unit_resident(
        &self,
        unit_id: Uuid,
        user_id: Uuid,
        resident_type: &str,
        is_primary: bool,
    ) -> Result<Uuid, sqlx::Error> {
        let today = Utc::now().date_naive();
        // from_ymd_opt should never return None for the current year; if it does, fail loudly
        let start_date = NaiveDate::from_ymd_opt(today.year(), 1, 1)
            .expect("current year must be a valid NaiveDate");

        let row = sqlx::query(
            r#"
            INSERT INTO unit_residents (
                unit_id, user_id, resident_type, is_primary,
                start_date, receives_notifications, receives_mail
            )
            VALUES ($1, $2, $3::resident_type, $4, $5, true, true)
            RETURNING id
            "#,
        )
        .bind(unit_id)
        .bind(user_id)
        .bind(resident_type)
        .bind(is_primary)
        .bind(start_date)
        .fetch_one(self.pool)
        .await?;

        Ok(row.get("id"))
    }

    /// Update the occupancy status of a unit.
    pub async fn update_unit_occupancy(
        &self,
        unit_id: Uuid,
        occupancy_status: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE units SET occupancy_status = $1, updated_at = NOW()
            WHERE id = $2
            "#,
        )
        .bind(occupancy_status)
        .bind(unit_id)
        .execute(self.pool)
        .await?;

        Ok(())
    }

    /// Delete seed data by email domain pattern.
    ///
    /// This is used for cleanup before re-seeding.
    /// All operations run within a single transaction for atomicity.
    ///
    /// # Security
    /// The email_domain is validated to prevent SQL pattern injection.
    /// Only alphanumeric characters, dots, and hyphens are allowed.
    pub async fn cleanup_seed_data(&self, email_domain: &str) -> Result<CleanupStats, sqlx::Error> {
        // Validate email_domain to prevent SQL pattern injection
        // Only allow alphanumeric, dots, and hyphens (valid domain characters)
        if !email_domain
            .chars()
            .all(|c| c.is_alphanumeric() || c == '.' || c == '-')
        {
            return Err(sqlx::Error::Protocol(
                "Invalid email domain: only alphanumeric characters, dots, and hyphens allowed"
                    .to_string(),
            ));
        }

        let pattern = format!("%@{}", email_domain);

        // Use a transaction for atomicity - if any operation fails,
        // all changes are rolled back to maintain consistency
        let mut tx = self.pool.begin().await?;

        // Delete in dependency order to avoid foreign key violations

        // 1. Delete unit_residents for seed users
        let residents_result = sqlx::query(
            r#"
            DELETE FROM unit_residents
            WHERE user_id IN (SELECT id FROM users WHERE email LIKE $1)
            "#,
        )
        .bind(&pattern)
        .execute(&mut *tx)
        .await?;

        // 2. Delete organization_members for seed users
        let members_result = sqlx::query(
            r#"
            DELETE FROM organization_members
            WHERE user_id IN (SELECT id FROM users WHERE email LIKE $1)
            "#,
        )
        .bind(&pattern)
        .execute(&mut *tx)
        .await?;

        // 3. Find organizations created by seed (by contact email pattern)
        let org_ids: Vec<Uuid> = sqlx::query(
            r#"
            SELECT id FROM organizations WHERE contact_email LIKE $1
            "#,
        )
        .bind(&pattern)
        .fetch_all(&mut *tx)
        .await?
        .into_iter()
        .map(|r| r.get("id"))
        .collect();

        // 4-6. Delete units, buildings, and roles for each organization
        let mut units_deleted = 0u64;
        let mut buildings_deleted = 0u64;
        for org_id in &org_ids {
            // Delete units in this org's buildings
            let units_result = sqlx::query(
                r#"
                DELETE FROM units
                WHERE building_id IN (SELECT id FROM buildings WHERE organization_id = $1)
                "#,
            )
            .bind(org_id)
            .execute(&mut *tx)
            .await?;
            units_deleted += units_result.rows_affected();

            // Delete buildings in this org
            let buildings_result = sqlx::query(
                r#"
                DELETE FROM buildings WHERE organization_id = $1
                "#,
            )
            .bind(org_id)
            .execute(&mut *tx)
            .await?;
            buildings_deleted += buildings_result.rows_affected();

            // Delete roles in this org
            sqlx::query(
                r#"
                DELETE FROM roles WHERE organization_id = $1
                "#,
            )
            .bind(org_id)
            .execute(&mut *tx)
            .await?;
        }

        // 7. Delete seed organizations
        let orgs_result = sqlx::query(
            r#"
            DELETE FROM organizations WHERE contact_email LIKE $1
            "#,
        )
        .bind(&pattern)
        .execute(&mut *tx)
        .await?;

        // 8. Delete seed users
        let users_result = sqlx::query(
            r#"
            DELETE FROM users WHERE email LIKE $1
            "#,
        )
        .bind(&pattern)
        .execute(&mut *tx)
        .await?;

        // Commit the transaction - all deletions succeed or none do
        tx.commit().await?;

        Ok(CleanupStats {
            users_deleted: users_result.rows_affected(),
            organizations_deleted: orgs_result.rows_affected(),
            buildings_deleted,
            units_deleted,
            members_deleted: members_result.rows_affected(),
            residents_deleted: residents_result.rows_affected(),
        })
    }

    /// Check if seed data already exists.
    pub async fn seed_exists(&self, email_domain: &str) -> Result<bool, sqlx::Error> {
        let pattern = format!("%@{}", email_domain);
        let result: Option<i64> = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM users WHERE email LIKE $1
            "#,
        )
        .bind(&pattern)
        .fetch_one(self.pool)
        .await?;

        Ok(result.unwrap_or(0) > 0)
    }
}

/// Statistics from cleanup operation.
#[derive(Debug, Clone, Default)]
pub struct CleanupStats {
    pub users_deleted: u64,
    pub organizations_deleted: u64,
    pub buildings_deleted: u64,
    pub units_deleted: u64,
    pub members_deleted: u64,
    pub residents_deleted: u64,
}

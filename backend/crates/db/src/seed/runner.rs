//! Main seed execution logic.
//!
//! Handles the complete seeding workflow including RLS context,
//! data creation, and cleanup.

use std::collections::HashMap;

use uuid::Uuid;

use crate::{clear_request_context, set_request_context, DbPool};

use super::data::SeedData;
use super::factories::{CleanupStats, SeedFactories};

/// Configuration for the seed runner.
#[derive(Debug, Clone)]
pub struct SeedConfig {
    /// Admin email address
    pub admin_email: String,
    /// Admin password (plaintext, will be hashed)
    pub admin_password: String,
    /// Whether to include sample data (buildings, units, non-admin users)
    pub include_sample_data: bool,
    /// Whether to force re-seed (drops existing seed data first)
    pub force: bool,
}

/// Result of a successful seed operation.
#[derive(Debug, Clone)]
pub struct SeedResult {
    /// Number of organizations created
    pub organizations_created: usize,
    /// Number of users created
    pub users_created: usize,
    /// Number of buildings created
    pub buildings_created: usize,
    /// Number of units created
    pub units_created: usize,
    /// Number of unit residents assigned
    pub residents_assigned: usize,
    /// Admin user ID
    pub admin_user_id: Uuid,
    /// Organization ID
    pub organization_id: Uuid,
    /// Cleanup stats (if force was used)
    pub cleanup_stats: Option<CleanupStats>,
}

/// Seed runner that orchestrates the complete seeding process.
pub struct SeedRunner {
    pool: DbPool,
    config: SeedConfig,
}

impl SeedRunner {
    /// Create a new seed runner.
    pub fn new(pool: DbPool, config: SeedConfig) -> Self {
        Self { pool, config }
    }

    /// Execute the seeding process.
    ///
    /// This will:
    /// 1. Set super admin context to bypass RLS
    /// 2. Optionally cleanup existing seed data
    /// 3. Create organization (triggers role creation)
    /// 4. Create admin user with provided credentials
    /// 5. Create sample users, buildings, units (if include_sample_data)
    /// 6. Assign users to units
    /// 7. Clear RLS context
    pub async fn run(&self) -> Result<SeedResult, SeedError> {
        let factories = SeedFactories::new(&self.pool);
        let seed_data = SeedData::default();
        let email_domain = "demo-property.test";

        // 1. Set super admin context to bypass RLS
        set_request_context(&self.pool, None, None, true)
            .await
            .map_err(|e| SeedError::Database(e.to_string()))?;

        // 2. Optionally cleanup existing seed data
        let cleanup_stats = if self.config.force {
            let stats = factories
                .cleanup_seed_data(email_domain)
                .await
                .map_err(|e| SeedError::Database(e.to_string()))?;
            Some(stats)
        } else {
            // Check if seed data already exists
            if factories
                .seed_exists(email_domain)
                .await
                .map_err(|e| SeedError::Database(e.to_string()))?
            {
                clear_request_context(&self.pool)
                    .await
                    .map_err(|e| SeedError::Database(e.to_string()))?;
                return Err(SeedError::AlreadySeeded);
            }
            None
        };

        // 3. Hash passwords
        let admin_hash = SeedFactories::hash_password(&self.config.admin_password)
            .map_err(|e| SeedError::PasswordHash(e.to_string()))?;

        let default_hash = SeedFactories::hash_password(seed_data.default_password)
            .map_err(|e| SeedError::PasswordHash(e.to_string()))?;

        // 4. Create organization (triggers role creation)
        let org_id = factories
            .create_organization(
                seed_data.organization.name,
                seed_data.organization.slug,
                seed_data.organization.contact_email,
            )
            .await
            .map_err(|e| SeedError::Database(e.to_string()))?;

        // 5. Create admin user with provided credentials
        let admin_id = factories
            .create_user(
                &self.config.admin_email,
                "System Administrator",
                &admin_hash,
                None,
                true, // is_super_admin
            )
            .await
            .map_err(|e| SeedError::Database(e.to_string()))?;

        // 6. Add admin to organization with Super Admin role
        factories
            .add_organization_member(org_id, admin_id, "Super Admin")
            .await
            .map_err(|e| SeedError::Database(e.to_string()))?;

        let mut users_created = 1; // admin
        let mut buildings_created = 0;
        let mut units_created = 0;
        let mut residents_assigned = 0;

        // 7. Create sample data if requested
        if self.config.include_sample_data {
            // Track user IDs by email for unit assignments
            let mut user_ids: HashMap<String, Uuid> = HashMap::new();

            // Create sample users
            for user in &seed_data.users {
                let user_id = factories
                    .create_user(user.email, user.name, &default_hash, user.phone, false)
                    .await
                    .map_err(|e| SeedError::Database(e.to_string()))?;

                // Add user to organization with their role
                factories
                    .add_organization_member(org_id, user_id, user.role_type)
                    .await
                    .map_err(|e| SeedError::Database(e.to_string()))?;

                user_ids.insert(user.email.to_string(), user_id);
                users_created += 1;
            }

            // Create buildings and units
            let mut building_ids: Vec<Uuid> = Vec::new();
            let mut unit_ids: Vec<Vec<Uuid>> = Vec::new();

            for building in &seed_data.buildings {
                let building_id = factories
                    .create_building(org_id, building)
                    .await
                    .map_err(|e| SeedError::Database(e.to_string()))?;

                building_ids.push(building_id);
                buildings_created += 1;

                let mut building_unit_ids: Vec<Uuid> = Vec::new();
                for unit in &building.units {
                    let unit_id = factories
                        .create_unit(building_id, unit)
                        .await
                        .map_err(|e| SeedError::Database(e.to_string()))?;

                    building_unit_ids.push(unit_id);
                    units_created += 1;
                }
                unit_ids.push(building_unit_ids);
            }

            // Assign users to units and update occupancy status
            for user in &seed_data.users {
                let user_id = user_ids
                    .get(user.email)
                    .ok_or_else(|| SeedError::Internal("User ID not found".to_string()))?;

                for assignment in &user.unit_assignments {
                    let unit_id = unit_ids
                        .get(assignment.building_index)
                        .and_then(|units| units.get(assignment.unit_index))
                        .ok_or_else(|| SeedError::Internal("Unit ID not found".to_string()))?;

                    factories
                        .assign_unit_resident(
                            *unit_id,
                            *user_id,
                            assignment.resident_type,
                            assignment.is_primary,
                        )
                        .await
                        .map_err(|e| SeedError::Database(e.to_string()))?;

                    // Update unit occupancy status based on resident type
                    let occupancy = match assignment.resident_type {
                        "owner" => "owner_occupied",
                        "tenant" | "subtenant" => "rented",
                        _ => "unknown",
                    };

                    // Only update if this is the primary resident
                    if assignment.is_primary {
                        factories
                            .update_unit_occupancy(*unit_id, occupancy)
                            .await
                            .map_err(|e| SeedError::Database(e.to_string()))?;
                    }

                    residents_assigned += 1;
                }
            }
        }

        // 8. Clear super admin context
        clear_request_context(&self.pool)
            .await
            .map_err(|e| SeedError::Database(e.to_string()))?;

        Ok(SeedResult {
            organizations_created: 1,
            users_created,
            buildings_created,
            units_created,
            residents_assigned,
            admin_user_id: admin_id,
            organization_id: org_id,
            cleanup_stats,
        })
    }
}

/// Errors that can occur during seeding.
#[derive(Debug, thiserror::Error)]
pub enum SeedError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Password hashing error: {0}")]
    PasswordHash(String),

    #[error("Seed data already exists. Use --force to re-seed.")]
    AlreadySeeded,

    #[error("Internal error: {0}")]
    Internal(String),
}

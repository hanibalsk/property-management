//! Building handlers (UC-15).
//!
//! Implements building and unit management including CRUD operations,
//! unit assignments, owner management, and statistics.

use crate::state::AppState;
use common::errors::ErrorResponse;
use db::models::{
    AssignUnitOwner, Building, BuildingStatistics, BuildingSummary, CreateBuilding, CreateUnit,
    Unit, UnitOwnerInfo, UnitSummary, UnitWithOwners, UpdateBuilding, UpdateUnit,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;
use uuid::Uuid;

// ============================================================================
// Error Types
// ============================================================================

/// Building handler errors.
#[derive(Debug, Error)]
pub enum BuildingHandlerError {
    #[error("Building not found")]
    BuildingNotFound,

    #[error("Unit not found")]
    UnitNotFound,

    #[error("Unit not found in this building")]
    UnitNotInBuilding,

    #[error("Owner not found for this unit")]
    OwnerNotFound,

    #[error("User not found")]
    UserNotFound,

    #[error("Not authorized to access this organization")]
    NotAuthorized,

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Duplicate designation: {0}")]
    DuplicateDesignation(String),

    #[error("Invalid unit type: {0}")]
    InvalidUnitType(String),

    #[error("Invalid occupancy status: {0}")]
    InvalidOccupancyStatus(String),

    #[error("Ownership percentage exceeds 100%")]
    OwnershipExceeds100,

    #[error("User is already an owner of this unit")]
    AlreadyOwner,

    #[error("Import too large: max {0} buildings per request")]
    ImportTooLarge(usize),

    #[error("Empty import: no buildings provided")]
    EmptyImport,

    #[error("Database error: {0}")]
    Database(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<BuildingHandlerError> for ErrorResponse {
    fn from(err: BuildingHandlerError) -> Self {
        match err {
            BuildingHandlerError::BuildingNotFound => {
                ErrorResponse::new("NOT_FOUND", "Building not found")
            }
            BuildingHandlerError::UnitNotFound => ErrorResponse::new("NOT_FOUND", "Unit not found"),
            BuildingHandlerError::UnitNotInBuilding => {
                ErrorResponse::new("NOT_FOUND", "Unit not found in this building")
            }
            BuildingHandlerError::OwnerNotFound => {
                ErrorResponse::new("NOT_FOUND", "Owner not found for this unit")
            }
            BuildingHandlerError::UserNotFound => {
                ErrorResponse::new("USER_NOT_FOUND", "User not found")
            }
            BuildingHandlerError::NotAuthorized => ErrorResponse::new(
                "NOT_AUTHORIZED",
                "You are not a member of this organization",
            ),
            BuildingHandlerError::InvalidInput(msg) => ErrorResponse::new("INVALID_INPUT", msg),
            BuildingHandlerError::DuplicateDesignation(msg) => {
                ErrorResponse::new("DUPLICATE_DESIGNATION", msg)
            }
            BuildingHandlerError::InvalidUnitType(msg) => {
                ErrorResponse::new("INVALID_UNIT_TYPE", msg)
            }
            BuildingHandlerError::InvalidOccupancyStatus(msg) => {
                ErrorResponse::new("INVALID_OCCUPANCY_STATUS", msg)
            }
            BuildingHandlerError::OwnershipExceeds100 => {
                ErrorResponse::new("EXCEEDS_100_PERCENT", "Total ownership would exceed 100%")
            }
            BuildingHandlerError::AlreadyOwner => ErrorResponse::new(
                "ALREADY_OWNER",
                "This user is already an owner of this unit",
            ),
            BuildingHandlerError::ImportTooLarge(max) => ErrorResponse::new(
                "IMPORT_TOO_LARGE",
                format!("Maximum {} buildings per import request", max),
            ),
            BuildingHandlerError::EmptyImport => {
                ErrorResponse::new("EMPTY_IMPORT", "No buildings provided for import")
            }
            BuildingHandlerError::Database(msg) => ErrorResponse::new("DB_ERROR", msg),
            BuildingHandlerError::Internal(msg) => ErrorResponse::new("INTERNAL_ERROR", msg),
        }
    }
}

// ============================================================================
// Request/Response Types
// ============================================================================

/// Create building request data.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateBuildingData {
    pub organization_id: Uuid,
    pub street: String,
    pub city: String,
    pub postal_code: String,
    #[serde(default = "default_country")]
    pub country: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub year_built: Option<i32>,
    #[serde(default = "default_one")]
    pub total_floors: i32,
    #[serde(default = "default_one")]
    pub total_entrances: i32,
    #[serde(default)]
    pub amenities: Vec<String>,
}

fn default_country() -> String {
    "Slovakia".to_string()
}

fn default_one() -> i32 {
    1
}

/// Update building request data.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateBuildingData {
    pub street: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub year_built: Option<i32>,
    pub total_floors: Option<i32>,
    pub total_entrances: Option<i32>,
    pub amenities: Option<Vec<String>>,
}

/// List buildings query parameters.
#[derive(Debug, Clone, Deserialize)]
pub struct ListBuildingsParams {
    pub organization_id: Uuid,
    pub offset: i64,
    pub limit: i64,
    pub include_archived: bool,
    pub search: Option<String>,
}

/// Create unit request data.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateUnitData {
    pub designation: String,
    pub entrance: Option<String>,
    #[serde(default)]
    pub floor: i32,
    #[serde(default = "default_unit_type")]
    pub unit_type: String,
    pub size_sqm: Option<Decimal>,
    pub rooms: Option<i32>,
    #[serde(default = "default_ownership_share")]
    pub ownership_share: Decimal,
    pub description: Option<String>,
}

fn default_unit_type() -> String {
    "apartment".to_string()
}

fn default_ownership_share() -> Decimal {
    Decimal::new(10000, 2) // 100.00
}

/// Update unit request data.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateUnitData {
    pub entrance: Option<String>,
    pub designation: Option<String>,
    pub floor: Option<i32>,
    pub unit_type: Option<String>,
    pub size_sqm: Option<Decimal>,
    pub rooms: Option<i32>,
    pub ownership_share: Option<Decimal>,
    pub occupancy_status: Option<String>,
    pub description: Option<String>,
    pub notes: Option<String>,
}

/// List units query parameters.
#[derive(Debug, Clone, Deserialize)]
pub struct ListUnitsParams {
    pub offset: i64,
    pub limit: i64,
    pub include_archived: bool,
    pub unit_type: Option<String>,
    pub floor: Option<i32>,
}

/// Assign unit owner request data.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct AssignOwnerData {
    pub user_id: Uuid,
    #[serde(default = "default_ownership_share")]
    pub ownership_percentage: Decimal,
    #[serde(default = "default_true")]
    pub is_primary: bool,
    pub valid_from: Option<chrono::NaiveDate>,
}

fn default_true() -> bool {
    true
}

/// Update owner request data.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateOwnerData {
    pub ownership_percentage: Option<Decimal>,
    pub is_primary: Option<bool>,
}

/// Bulk import building entry.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct BulkBuildingEntry {
    pub street: String,
    pub city: String,
    pub postal_code: String,
    #[serde(default = "default_country")]
    pub country: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub year_built: Option<i32>,
    #[serde(default = "default_one")]
    pub total_floors: i32,
    #[serde(default = "default_one")]
    pub total_entrances: i32,
    #[serde(default)]
    pub amenities: Vec<String>,
}

/// Bulk import result for a single building.
#[derive(Debug, Serialize, ToSchema)]
pub struct BulkImportResult {
    pub index: usize,
    pub success: bool,
    pub building_id: Option<Uuid>,
    pub error: Option<String>,
}

/// Bulk import buildings response.
#[derive(Debug, Serialize, ToSchema)]
pub struct BulkImportResponse {
    pub total: usize,
    pub successful: usize,
    pub failed: usize,
    pub results: Vec<BulkImportResult>,
}

/// Paginated buildings response.
#[derive(Debug, Serialize, ToSchema)]
pub struct BuildingsListResult {
    pub buildings: Vec<BuildingSummary>,
    pub total: i64,
    pub offset: i64,
    pub limit: i64,
}

/// Paginated units response.
#[derive(Debug, Serialize, ToSchema)]
pub struct UnitsListResult {
    pub units: Vec<UnitSummary>,
    pub total: i64,
    pub offset: i64,
    pub limit: i64,
}

// ============================================================================
// Constants
// ============================================================================

const MAX_BULK_IMPORT_SIZE: usize = 100;
const VALID_UNIT_TYPES: [&str; 5] = ["apartment", "commercial", "parking", "storage", "other"];
const VALID_OCCUPANCY_STATUSES: [&str; 4] = ["owner_occupied", "rented", "vacant", "unknown"];

// ============================================================================
// Handler Implementation
// ============================================================================

/// Building handler providing business logic for building and unit operations.
pub struct BuildingHandler;

impl BuildingHandler {
    // ========================================================================
    // Authorization Helpers
    // ========================================================================

    /// Check if user is a member of the organization.
    async fn check_org_membership(
        state: &AppState,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), BuildingHandlerError> {
        let is_member = state
            .org_member_repo
            .is_member(org_id, user_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to check org membership");
                BuildingHandlerError::Database("Database error".into())
            })?;

        if !is_member {
            return Err(BuildingHandlerError::NotAuthorized);
        }

        Ok(())
    }

    /// Get building and verify user has access.
    async fn get_building_with_auth(
        state: &AppState,
        building_id: Uuid,
        user_id: Uuid,
    ) -> Result<Building, BuildingHandlerError> {
        let building = state
            .building_repo
            .find_by_id(building_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to get building");
                BuildingHandlerError::Database("Database error".into())
            })?
            .ok_or(BuildingHandlerError::BuildingNotFound)?;

        Self::check_org_membership(state, building.organization_id, user_id).await?;

        Ok(building)
    }

    // ========================================================================
    // Building Operations
    // ========================================================================

    /// Create a new building (UC-15.1).
    pub async fn create_building(
        state: &AppState,
        user_id: Uuid,
        data: CreateBuildingData,
    ) -> Result<Building, BuildingHandlerError> {
        // Verify user has access to organization
        Self::check_org_membership(state, data.organization_id, user_id).await?;

        // Validate required fields
        if data.street.trim().is_empty() {
            return Err(BuildingHandlerError::InvalidInput(
                "Street is required".into(),
            ));
        }

        if data.city.trim().is_empty() {
            return Err(BuildingHandlerError::InvalidInput(
                "City is required".into(),
            ));
        }

        // Create building
        let create_data = CreateBuilding {
            organization_id: data.organization_id,
            street: data.street,
            city: data.city,
            postal_code: data.postal_code,
            country: data.country,
            name: data.name,
            description: data.description,
            year_built: data.year_built,
            total_floors: data.total_floors,
            total_entrances: data.total_entrances,
            amenities: data.amenities,
        };

        let building = state.building_repo.create(create_data).await.map_err(|e| {
            tracing::error!(error = %e, "Failed to create building");
            BuildingHandlerError::Database("Failed to create building".into())
        })?;

        tracing::info!(
            building_id = %building.id,
            org_id = %building.organization_id,
            user_id = %user_id,
            "Building created"
        );

        Ok(building)
    }

    /// List buildings with pagination.
    pub async fn list_buildings(
        state: &AppState,
        user_id: Uuid,
        params: ListBuildingsParams,
    ) -> Result<BuildingsListResult, BuildingHandlerError> {
        // Verify user has access to organization
        Self::check_org_membership(state, params.organization_id, user_id).await?;

        let (buildings, total) = state
            .building_repo
            .list_by_organization(
                params.organization_id,
                params.offset,
                params.limit,
                params.include_archived,
                params.search.as_deref(),
            )
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to list buildings");
                BuildingHandlerError::Database("Failed to list buildings".into())
            })?;

        Ok(BuildingsListResult {
            buildings,
            total,
            offset: params.offset,
            limit: params.limit,
        })
    }

    /// Get building by ID (UC-15.2).
    pub async fn get_building(
        state: &AppState,
        user_id: Uuid,
        building_id: Uuid,
    ) -> Result<Building, BuildingHandlerError> {
        Self::get_building_with_auth(state, building_id, user_id).await
    }

    /// Update building (UC-15.3).
    pub async fn update_building(
        state: &AppState,
        user_id: Uuid,
        building_id: Uuid,
        data: UpdateBuildingData,
    ) -> Result<Building, BuildingHandlerError> {
        // Verify access
        let _ = Self::get_building_with_auth(state, building_id, user_id).await?;

        let update_data = UpdateBuilding {
            street: data.street,
            city: data.city,
            postal_code: data.postal_code,
            country: data.country,
            name: data.name,
            description: data.description,
            year_built: data.year_built,
            total_floors: data.total_floors,
            total_entrances: data.total_entrances,
            amenities: data.amenities,
            contacts: None,
            settings: None,
        };

        let building = state
            .building_repo
            .update(building_id, update_data)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to update building");
                BuildingHandlerError::Database("Failed to update building".into())
            })?
            .ok_or(BuildingHandlerError::BuildingNotFound)?;

        tracing::info!(building_id = %building_id, user_id = %user_id, "Building updated");

        Ok(building)
    }

    /// Archive building (soft delete) (UC-15.10).
    pub async fn archive_building(
        state: &AppState,
        user_id: Uuid,
        building_id: Uuid,
    ) -> Result<Building, BuildingHandlerError> {
        // Verify access
        let _ = Self::get_building_with_auth(state, building_id, user_id).await?;

        let building = state
            .building_repo
            .archive(building_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to archive building");
                BuildingHandlerError::Database("Failed to archive building".into())
            })?
            .ok_or_else(|| {
                BuildingHandlerError::InvalidInput("Building not found or already archived".into())
            })?;

        tracing::info!(building_id = %building_id, user_id = %user_id, "Building archived");

        Ok(building)
    }

    /// Restore archived building.
    pub async fn restore_building(
        state: &AppState,
        user_id: Uuid,
        building_id: Uuid,
    ) -> Result<Building, BuildingHandlerError> {
        // Get building including archived ones
        let existing = state
            .building_repo
            .find_by_id_any_status(building_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to get building");
                BuildingHandlerError::Database("Database error".into())
            })?
            .ok_or(BuildingHandlerError::BuildingNotFound)?;

        // Verify access
        Self::check_org_membership(state, existing.organization_id, user_id).await?;

        let building = state
            .building_repo
            .restore(building_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to restore building");
                BuildingHandlerError::Database("Failed to restore building".into())
            })?
            .ok_or_else(|| {
                BuildingHandlerError::InvalidInput("Building not found or not archived".into())
            })?;

        tracing::info!(building_id = %building_id, user_id = %user_id, "Building restored");

        Ok(building)
    }

    /// Get building statistics (UC-15.7).
    pub async fn get_statistics(
        state: &AppState,
        user_id: Uuid,
        building_id: Uuid,
    ) -> Result<BuildingStatistics, BuildingHandlerError> {
        // Verify access
        let _ = Self::get_building_with_auth(state, building_id, user_id).await?;

        let stats = state
            .building_repo
            .get_statistics(building_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to get building statistics");
                BuildingHandlerError::Database("Failed to get building statistics".into())
            })?;

        Ok(stats)
    }

    /// Bulk import buildings (UC-15.8).
    pub async fn bulk_import(
        state: &AppState,
        user_id: Uuid,
        organization_id: Uuid,
        buildings: Vec<BulkBuildingEntry>,
    ) -> Result<BulkImportResponse, BuildingHandlerError> {
        // Verify access
        Self::check_org_membership(state, organization_id, user_id).await?;

        // Validate import size
        if buildings.is_empty() {
            return Err(BuildingHandlerError::EmptyImport);
        }

        if buildings.len() > MAX_BULK_IMPORT_SIZE {
            return Err(BuildingHandlerError::ImportTooLarge(MAX_BULK_IMPORT_SIZE));
        }

        let mut results = Vec::with_capacity(buildings.len());
        let mut successful = 0;
        let mut failed = 0;

        for (index, entry) in buildings.into_iter().enumerate() {
            // Validate required fields
            if entry.street.trim().is_empty() {
                results.push(BulkImportResult {
                    index,
                    success: false,
                    building_id: None,
                    error: Some("Street is required".to_string()),
                });
                failed += 1;
                continue;
            }

            if entry.city.trim().is_empty() {
                results.push(BulkImportResult {
                    index,
                    success: false,
                    building_id: None,
                    error: Some("City is required".to_string()),
                });
                failed += 1;
                continue;
            }

            // Create building
            let create_data = CreateBuilding {
                organization_id,
                street: entry.street,
                city: entry.city,
                postal_code: entry.postal_code,
                country: entry.country,
                name: entry.name,
                description: entry.description,
                year_built: entry.year_built,
                total_floors: entry.total_floors,
                total_entrances: entry.total_entrances,
                amenities: entry.amenities,
            };

            match state.building_repo.create(create_data).await {
                Ok(building) => {
                    results.push(BulkImportResult {
                        index,
                        success: true,
                        building_id: Some(building.id),
                        error: None,
                    });
                    successful += 1;
                }
                Err(e) => {
                    tracing::warn!(index = index, error = %e, "Failed to import building");
                    results.push(BulkImportResult {
                        index,
                        success: false,
                        building_id: None,
                        error: Some(format!("Database error: {}", e)),
                    });
                    failed += 1;
                }
            }
        }

        tracing::info!(
            org_id = %organization_id,
            user_id = %user_id,
            total = results.len(),
            successful = successful,
            failed = failed,
            "Bulk import completed"
        );

        Ok(BulkImportResponse {
            total: results.len(),
            successful,
            failed,
            results,
        })
    }

    // ========================================================================
    // Unit Operations
    // ========================================================================

    /// List units in a building.
    pub async fn list_units(
        state: &AppState,
        user_id: Uuid,
        building_id: Uuid,
        params: ListUnitsParams,
    ) -> Result<UnitsListResult, BuildingHandlerError> {
        // Verify access
        let _ = Self::get_building_with_auth(state, building_id, user_id).await?;

        let (units, total) = state
            .unit_repo
            .list_by_building(
                building_id,
                params.offset,
                params.limit,
                params.include_archived,
                params.unit_type.as_deref(),
                params.floor,
            )
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to list units");
                BuildingHandlerError::Database("Failed to list units".into())
            })?;

        Ok(UnitsListResult {
            units,
            total,
            offset: params.offset,
            limit: params.limit,
        })
    }

    /// Create a new unit (UC-15.4).
    pub async fn create_unit(
        state: &AppState,
        user_id: Uuid,
        building_id: Uuid,
        data: CreateUnitData,
    ) -> Result<Unit, BuildingHandlerError> {
        // Verify access
        let _ = Self::get_building_with_auth(state, building_id, user_id).await?;

        // Validate required fields
        if data.designation.trim().is_empty() {
            return Err(BuildingHandlerError::InvalidInput(
                "Unit designation is required".into(),
            ));
        }

        // Check for duplicate designation
        let exists = state
            .unit_repo
            .designation_exists(building_id, &data.designation, None)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to check designation");
                BuildingHandlerError::Database("Database error".into())
            })?;

        if exists {
            return Err(BuildingHandlerError::DuplicateDesignation(
                "A unit with this designation already exists in this building".into(),
            ));
        }

        // Validate unit type
        if !VALID_UNIT_TYPES.contains(&data.unit_type.as_str()) {
            return Err(BuildingHandlerError::InvalidUnitType(
                "Invalid unit type. Must be: apartment, commercial, parking, storage, or other"
                    .into(),
            ));
        }

        // Create unit
        let create_data = CreateUnit {
            building_id,
            entrance: data.entrance,
            designation: data.designation,
            floor: data.floor,
            unit_type: data.unit_type,
            size_sqm: data.size_sqm,
            rooms: data.rooms,
            ownership_share: data.ownership_share,
            description: data.description,
        };

        let unit = state.unit_repo.create(create_data).await.map_err(|e| {
            tracing::error!(error = %e, "Failed to create unit");
            BuildingHandlerError::Database("Failed to create unit".into())
        })?;

        tracing::info!(
            unit_id = %unit.id,
            building_id = %building_id,
            user_id = %user_id,
            "Unit created"
        );

        Ok(unit)
    }

    /// Get unit by ID (UC-15.5).
    pub async fn get_unit(
        state: &AppState,
        user_id: Uuid,
        building_id: Uuid,
        unit_id: Uuid,
    ) -> Result<UnitWithOwners, BuildingHandlerError> {
        // Verify access
        let _ = Self::get_building_with_auth(state, building_id, user_id).await?;

        // Get unit with owners
        let unit_with_owners = state
            .unit_repo
            .find_by_id_with_owners(unit_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to get unit");
                BuildingHandlerError::Database("Database error".into())
            })?
            .ok_or(BuildingHandlerError::UnitNotFound)?;

        // Verify unit belongs to building
        if unit_with_owners.unit.building_id != building_id {
            return Err(BuildingHandlerError::UnitNotInBuilding);
        }

        Ok(unit_with_owners)
    }

    /// Update unit.
    pub async fn update_unit(
        state: &AppState,
        user_id: Uuid,
        building_id: Uuid,
        unit_id: Uuid,
        data: UpdateUnitData,
    ) -> Result<Unit, BuildingHandlerError> {
        // Verify access
        let _ = Self::get_building_with_auth(state, building_id, user_id).await?;

        // Verify unit exists and belongs to building
        let existing = state
            .unit_repo
            .find_by_id(unit_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to get unit");
                BuildingHandlerError::Database("Database error".into())
            })?
            .ok_or(BuildingHandlerError::UnitNotFound)?;

        if existing.building_id != building_id {
            return Err(BuildingHandlerError::UnitNotInBuilding);
        }

        // Check for duplicate designation if being updated
        if let Some(ref designation) = data.designation {
            let exists = state
                .unit_repo
                .designation_exists(building_id, designation, Some(unit_id))
                .await
                .map_err(|e| {
                    tracing::error!(error = %e, "Failed to check designation");
                    BuildingHandlerError::Database("Database error".into())
                })?;

            if exists {
                return Err(BuildingHandlerError::DuplicateDesignation(
                    "A unit with this designation already exists in this building".into(),
                ));
            }
        }

        // Validate unit type if provided
        if let Some(ref unit_type) = data.unit_type {
            if !VALID_UNIT_TYPES.contains(&unit_type.as_str()) {
                return Err(BuildingHandlerError::InvalidUnitType(
                    "Invalid unit type. Must be: apartment, commercial, parking, storage, or other"
                        .into(),
                ));
            }
        }

        // Validate occupancy status if provided
        if let Some(ref status) = data.occupancy_status {
            if !VALID_OCCUPANCY_STATUSES.contains(&status.as_str()) {
                return Err(BuildingHandlerError::InvalidOccupancyStatus(
                    "Invalid occupancy status. Must be: owner_occupied, rented, vacant, or unknown"
                        .into(),
                ));
            }
        }

        let update_data = UpdateUnit {
            entrance: data.entrance,
            designation: data.designation,
            floor: data.floor,
            unit_type: data.unit_type,
            size_sqm: data.size_sqm,
            rooms: data.rooms,
            ownership_share: data.ownership_share,
            occupancy_status: data.occupancy_status,
            description: data.description,
            notes: data.notes,
            settings: None,
        };

        let unit = state
            .unit_repo
            .update(unit_id, update_data)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to update unit");
                BuildingHandlerError::Database("Failed to update unit".into())
            })?
            .ok_or(BuildingHandlerError::UnitNotFound)?;

        tracing::info!(unit_id = %unit_id, user_id = %user_id, "Unit updated");

        Ok(unit)
    }

    /// Archive unit (soft delete).
    pub async fn archive_unit(
        state: &AppState,
        user_id: Uuid,
        building_id: Uuid,
        unit_id: Uuid,
    ) -> Result<Unit, BuildingHandlerError> {
        // Verify access
        let _ = Self::get_building_with_auth(state, building_id, user_id).await?;

        // Verify unit belongs to building
        let existing = state.unit_repo.find_by_id(unit_id).await.map_err(|e| {
            tracing::error!(error = %e, "Failed to get unit");
            BuildingHandlerError::Database("Database error".into())
        })?;

        if let Some(u) = existing {
            if u.building_id != building_id {
                return Err(BuildingHandlerError::UnitNotInBuilding);
            }
        }

        let unit = state
            .unit_repo
            .archive(unit_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to archive unit");
                BuildingHandlerError::Database("Failed to archive unit".into())
            })?
            .ok_or_else(|| {
                BuildingHandlerError::InvalidInput("Unit not found or already archived".into())
            })?;

        tracing::info!(unit_id = %unit_id, user_id = %user_id, "Unit archived");

        Ok(unit)
    }

    /// Restore archived unit.
    pub async fn restore_unit(
        state: &AppState,
        user_id: Uuid,
        building_id: Uuid,
        unit_id: Uuid,
    ) -> Result<Unit, BuildingHandlerError> {
        // Verify access
        let _ = Self::get_building_with_auth(state, building_id, user_id).await?;

        // Verify unit belongs to building
        let belongs = state
            .unit_repo
            .belongs_to_building(unit_id, building_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to check unit building");
                BuildingHandlerError::Database("Database error".into())
            })?;

        if !belongs {
            return Err(BuildingHandlerError::UnitNotInBuilding);
        }

        let unit = state
            .unit_repo
            .restore(unit_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to restore unit");
                BuildingHandlerError::Database("Failed to restore unit".into())
            })?
            .ok_or_else(|| {
                BuildingHandlerError::InvalidInput("Unit not found or not archived".into())
            })?;

        tracing::info!(unit_id = %unit_id, user_id = %user_id, "Unit restored");

        Ok(unit)
    }

    // ========================================================================
    // Unit Owner Operations
    // ========================================================================

    /// List owners for a unit.
    pub async fn list_unit_owners(
        state: &AppState,
        user_id: Uuid,
        building_id: Uuid,
        unit_id: Uuid,
    ) -> Result<Vec<UnitOwnerInfo>, BuildingHandlerError> {
        // Verify access
        let _ = Self::get_building_with_auth(state, building_id, user_id).await?;

        // Verify unit exists and belongs to building
        let unit = state
            .unit_repo
            .find_by_id(unit_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to get unit");
                BuildingHandlerError::Database("Database error".into())
            })?
            .ok_or(BuildingHandlerError::UnitNotFound)?;

        if unit.building_id != building_id {
            return Err(BuildingHandlerError::UnitNotInBuilding);
        }

        let owners = state.unit_repo.get_owners(unit_id).await.map_err(|e| {
            tracing::error!(error = %e, "Failed to get unit owners");
            BuildingHandlerError::Database("Failed to get unit owners".into())
        })?;

        Ok(owners)
    }

    /// Assign owner to unit (UC-15.6).
    pub async fn assign_owner(
        state: &AppState,
        user_id: Uuid,
        building_id: Uuid,
        unit_id: Uuid,
        data: AssignOwnerData,
    ) -> Result<UnitOwnerInfo, BuildingHandlerError> {
        // Verify access
        let _ = Self::get_building_with_auth(state, building_id, user_id).await?;

        // Verify unit exists and belongs to building
        let unit = state
            .unit_repo
            .find_by_id(unit_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to get unit");
                BuildingHandlerError::Database("Database error".into())
            })?
            .ok_or(BuildingHandlerError::UnitNotFound)?;

        if unit.building_id != building_id {
            return Err(BuildingHandlerError::UnitNotInBuilding);
        }

        // Validate ownership percentage
        if data.ownership_percentage <= Decimal::ZERO
            || data.ownership_percentage > Decimal::new(10000, 2)
        {
            return Err(BuildingHandlerError::InvalidInput(
                "Ownership percentage must be between 0 and 100".into(),
            ));
        }

        // Check that total ownership won't exceed 100%
        let current_total = state
            .unit_repo
            .get_total_ownership(unit_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to get total ownership");
                BuildingHandlerError::Database("Database error".into())
            })?;

        if current_total + data.ownership_percentage > Decimal::new(10000, 2) {
            return Err(BuildingHandlerError::OwnershipExceeds100);
        }

        // Verify target user exists
        let target_user = state
            .user_repo
            .find_by_id(data.user_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to check user");
                BuildingHandlerError::Database("Database error".into())
            })?
            .ok_or(BuildingHandlerError::UserNotFound)?;

        // Assign owner
        let assign_data = AssignUnitOwner {
            unit_id,
            user_id: data.user_id,
            ownership_percentage: data.ownership_percentage,
            is_primary: data.is_primary,
            valid_from: data.valid_from,
        };

        let owner = state
            .unit_repo
            .assign_owner(assign_data)
            .await
            .map_err(|e| {
                // Check for duplicate key violation
                if e.to_string().contains("unique_owner_per_unit") {
                    return BuildingHandlerError::AlreadyOwner;
                }
                tracing::error!(error = %e, "Failed to assign owner");
                BuildingHandlerError::Database("Failed to assign owner".into())
            })?;

        tracing::info!(
            unit_id = %unit_id,
            owner_user_id = %data.user_id,
            by_user_id = %user_id,
            "Unit owner assigned"
        );

        let owner_info = UnitOwnerInfo {
            user_id: owner.user_id,
            user_name: target_user.name,
            user_email: target_user.email,
            ownership_percentage: owner.ownership_percentage,
            is_primary: owner.is_primary,
        };

        Ok(owner_info)
    }

    /// Update unit owner.
    pub async fn update_owner(
        state: &AppState,
        user_id: Uuid,
        building_id: Uuid,
        unit_id: Uuid,
        owner_user_id: Uuid,
        data: UpdateOwnerData,
    ) -> Result<UnitOwnerInfo, BuildingHandlerError> {
        // Verify access
        let _ = Self::get_building_with_auth(state, building_id, user_id).await?;

        // Verify unit belongs to building
        let belongs = state
            .unit_repo
            .belongs_to_building(unit_id, building_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to check unit building");
                BuildingHandlerError::Database("Database error".into())
            })?;

        if !belongs {
            return Err(BuildingHandlerError::UnitNotInBuilding);
        }

        // Validate ownership percentage if provided
        if let Some(pct) = data.ownership_percentage {
            if pct <= Decimal::ZERO || pct > Decimal::new(10000, 2) {
                return Err(BuildingHandlerError::InvalidInput(
                    "Ownership percentage must be between 0 and 100".into(),
                ));
            }
        }

        let owner = state
            .unit_repo
            .update_owner(
                unit_id,
                owner_user_id,
                data.ownership_percentage,
                data.is_primary,
            )
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to update owner");
                BuildingHandlerError::Database("Failed to update owner".into())
            })?
            .ok_or(BuildingHandlerError::OwnerNotFound)?;

        tracing::info!(
            unit_id = %unit_id,
            owner_user_id = %owner_user_id,
            by_user_id = %user_id,
            "Unit owner updated"
        );

        // Get user info
        let target_user = state
            .user_repo
            .find_by_id(owner_user_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to get user");
                BuildingHandlerError::Database("Database error".into())
            })?
            .ok_or(BuildingHandlerError::UserNotFound)?;

        let owner_info = UnitOwnerInfo {
            user_id: owner.user_id,
            user_name: target_user.name,
            user_email: target_user.email,
            ownership_percentage: owner.ownership_percentage,
            is_primary: owner.is_primary,
        };

        Ok(owner_info)
    }

    /// Remove owner from unit.
    pub async fn remove_owner(
        state: &AppState,
        user_id: Uuid,
        building_id: Uuid,
        unit_id: Uuid,
        owner_user_id: Uuid,
    ) -> Result<(), BuildingHandlerError> {
        // Verify access
        let _ = Self::get_building_with_auth(state, building_id, user_id).await?;

        // Verify unit belongs to building
        let belongs = state
            .unit_repo
            .belongs_to_building(unit_id, building_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to check unit building");
                BuildingHandlerError::Database("Database error".into())
            })?;

        if !belongs {
            return Err(BuildingHandlerError::UnitNotInBuilding);
        }

        let removed = state
            .unit_repo
            .remove_owner(unit_id, owner_user_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to remove owner");
                BuildingHandlerError::Database("Failed to remove owner".into())
            })?;

        if removed {
            tracing::info!(
                unit_id = %unit_id,
                owner_user_id = %owner_user_id,
                by_user_id = %user_id,
                "Unit owner removed"
            );
            Ok(())
        } else {
            Err(BuildingHandlerError::OwnerNotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_conversion() {
        let err = BuildingHandlerError::BuildingNotFound;
        let response: ErrorResponse = err.into();
        assert_eq!(response.code, "NOT_FOUND");
    }

    #[test]
    fn test_valid_unit_types() {
        assert!(VALID_UNIT_TYPES.contains(&"apartment"));
        assert!(VALID_UNIT_TYPES.contains(&"commercial"));
        assert!(!VALID_UNIT_TYPES.contains(&"invalid"));
    }

    #[test]
    fn test_valid_occupancy_statuses() {
        assert!(VALID_OCCUPANCY_STATUSES.contains(&"owner_occupied"));
        assert!(VALID_OCCUPANCY_STATUSES.contains(&"rented"));
        assert!(!VALID_OCCUPANCY_STATUSES.contains(&"invalid"));
    }
}

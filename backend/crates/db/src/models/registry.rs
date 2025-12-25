//! Building registry models for Epic 57.
//!
//! Supports pet and vehicle registrations, parking spots, and registry rules.

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

// ============================================================================
// Status Constants
// ============================================================================

/// Registry status values.
pub mod registry_status {
    pub const PENDING: &str = "pending";
    pub const APPROVED: &str = "approved";
    pub const REJECTED: &str = "rejected";
    pub const EXPIRED: &str = "expired";
}

/// Pet type values.
pub mod pet_type {
    pub const DOG: &str = "dog";
    pub const CAT: &str = "cat";
    pub const BIRD: &str = "bird";
    pub const FISH: &str = "fish";
    pub const REPTILE: &str = "reptile";
    pub const RODENT: &str = "rodent";
    pub const OTHER: &str = "other";
}

/// Pet size values.
pub mod pet_size {
    pub const SMALL: &str = "small";
    pub const MEDIUM: &str = "medium";
    pub const LARGE: &str = "large";
    pub const EXTRA_LARGE: &str = "extra_large";
}

/// Vehicle type values.
pub mod vehicle_type {
    pub const CAR: &str = "car";
    pub const MOTORCYCLE: &str = "motorcycle";
    pub const BICYCLE: &str = "bicycle";
    pub const SCOOTER: &str = "scooter";
    pub const OTHER: &str = "other";
}

// ============================================================================
// Pet Registration Entity
// ============================================================================

/// A pet registration record.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct PetRegistration {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub unit_id: Uuid,
    pub owner_id: Uuid,
    pub pet_name: String,
    pub pet_type: String,
    pub breed: Option<String>,
    pub pet_size: String,
    pub weight_kg: Option<Decimal>,
    pub age_years: Option<i32>,
    pub color: Option<String>,
    pub microchip_id: Option<String>,
    pub vaccination_date: Option<NaiveDate>,
    pub vaccination_document_id: Option<Uuid>,
    pub special_needs: Option<String>,
    pub status: String,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub rejection_reason: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Summary view of a pet registration for list responses.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct PetRegistrationSummary {
    pub id: Uuid,
    pub unit_id: Uuid,
    pub unit_number: Option<String>,
    pub owner_id: Uuid,
    pub owner_name: Option<String>,
    pub pet_name: String,
    pub pet_type: String,
    pub breed: Option<String>,
    pub pet_size: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

/// Pet registration with all related details.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PetRegistrationWithDetails {
    #[serde(flatten)]
    pub registration: PetRegistration,
    pub unit_number: Option<String>,
    pub building_name: Option<String>,
    pub owner_name: Option<String>,
    pub reviewed_by_name: Option<String>,
}

// ============================================================================
// Vehicle Registration Entity
// ============================================================================

/// A vehicle registration record.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct VehicleRegistration {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub unit_id: Uuid,
    pub owner_id: Uuid,
    pub vehicle_type: String,
    pub make: String,
    pub model: String,
    pub year: Option<i32>,
    pub color: Option<String>,
    pub license_plate: String,
    pub registration_document_id: Option<Uuid>,
    pub insurance_document_id: Option<Uuid>,
    pub parking_spot_id: Option<Uuid>,
    pub status: String,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub rejection_reason: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Summary view of a vehicle registration for list responses.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct VehicleRegistrationSummary {
    pub id: Uuid,
    pub unit_id: Uuid,
    pub unit_number: Option<String>,
    pub owner_id: Uuid,
    pub owner_name: Option<String>,
    pub vehicle_type: String,
    pub make: String,
    pub model: String,
    pub license_plate: String,
    pub status: String,
    pub parking_spot_number: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Vehicle registration with all related details.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct VehicleRegistrationWithDetails {
    #[serde(flatten)]
    pub registration: VehicleRegistration,
    pub unit_number: Option<String>,
    pub building_name: Option<String>,
    pub owner_name: Option<String>,
    pub reviewed_by_name: Option<String>,
    pub parking_spot_number: Option<String>,
}

// ============================================================================
// Parking Spot Entity
// ============================================================================

/// A parking spot record.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct ParkingSpot {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub building_id: Uuid,
    pub spot_number: String,
    pub spot_type: String,
    pub floor_level: Option<String>,
    pub is_covered: bool,
    pub is_reserved: bool,
    pub assigned_to_unit_id: Option<Uuid>,
    pub monthly_fee: Option<Decimal>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Parking spot with assignment details (flattened for SQL query).
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct ParkingSpotWithDetails {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub building_id: Uuid,
    pub spot_number: String,
    pub spot_type: String,
    pub floor_level: Option<String>,
    pub is_covered: bool,
    pub is_reserved: bool,
    pub assigned_to_unit_id: Option<Uuid>,
    pub monthly_fee: Option<Decimal>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub assigned_unit_number: Option<String>,
    pub building_name: Option<String>,
}

// ============================================================================
// Building Registry Rules Entity
// ============================================================================

/// Registry rules for a building.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct BuildingRegistryRules {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub building_id: Uuid,
    pub pets_allowed: bool,
    pub pets_require_approval: bool,
    pub max_pets_per_unit: Option<i32>,
    pub allowed_pet_types: Option<Vec<String>>,
    pub banned_pet_breeds: Option<Vec<String>>,
    pub max_pet_weight: Option<Decimal>,
    pub vehicles_require_approval: bool,
    pub max_vehicles_per_unit: Option<i32>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ============================================================================
// Request/Response Types
// ============================================================================

/// Request to create a pet registration.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreatePetRegistration {
    pub unit_id: Uuid,
    pub pet_name: String,
    pub pet_type: String,
    pub breed: Option<String>,
    pub pet_size: String,
    pub weight_kg: Option<Decimal>,
    pub age_years: Option<i32>,
    pub color: Option<String>,
    pub microchip_id: Option<String>,
    pub vaccination_date: Option<NaiveDate>,
    pub vaccination_document_id: Option<Uuid>,
    pub special_needs: Option<String>,
    pub notes: Option<String>,
}

/// Request to update a pet registration.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdatePetRegistration {
    pub pet_name: Option<String>,
    pub breed: Option<String>,
    pub pet_size: Option<String>,
    pub weight_kg: Option<Decimal>,
    pub age_years: Option<i32>,
    pub color: Option<String>,
    pub microchip_id: Option<String>,
    pub vaccination_date: Option<NaiveDate>,
    pub vaccination_document_id: Option<Uuid>,
    pub special_needs: Option<String>,
    pub notes: Option<String>,
}

/// Request to create a vehicle registration.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateVehicleRegistration {
    pub unit_id: Uuid,
    pub vehicle_type: String,
    pub make: String,
    pub model: String,
    pub year: Option<i32>,
    pub color: Option<String>,
    pub license_plate: String,
    pub registration_document_id: Option<Uuid>,
    pub insurance_document_id: Option<Uuid>,
    pub notes: Option<String>,
}

/// Request to update a vehicle registration.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateVehicleRegistration {
    pub make: Option<String>,
    pub model: Option<String>,
    pub year: Option<i32>,
    pub color: Option<String>,
    pub license_plate: Option<String>,
    pub registration_document_id: Option<Uuid>,
    pub insurance_document_id: Option<Uuid>,
    pub parking_spot_id: Option<Uuid>,
    pub notes: Option<String>,
}

/// Request to approve/reject a registration.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReviewRegistration {
    pub approve: bool,
    pub rejection_reason: Option<String>,
}

/// Request to create a parking spot.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateParkingSpot {
    pub building_id: Uuid,
    pub spot_number: String,
    pub spot_type: Option<String>,
    pub floor_level: Option<String>,
    pub is_covered: Option<bool>,
    pub is_reserved: Option<bool>,
    pub assigned_to_unit_id: Option<Uuid>,
    pub monthly_fee: Option<Decimal>,
    pub notes: Option<String>,
}

/// Request to update a parking spot.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateParkingSpot {
    pub spot_number: Option<String>,
    pub spot_type: Option<String>,
    pub floor_level: Option<String>,
    pub is_covered: Option<bool>,
    pub is_reserved: Option<bool>,
    pub assigned_to_unit_id: Option<Uuid>,
    pub monthly_fee: Option<Decimal>,
    pub notes: Option<String>,
}

/// Request to update building registry rules.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateRegistryRules {
    pub pets_allowed: Option<bool>,
    pub pets_require_approval: Option<bool>,
    pub max_pets_per_unit: Option<i32>,
    pub allowed_pet_types: Option<Vec<String>>,
    pub banned_pet_breeds: Option<Vec<String>>,
    pub max_pet_weight: Option<Decimal>,
    pub vehicles_require_approval: Option<bool>,
    pub max_vehicles_per_unit: Option<i32>,
    pub notes: Option<String>,
}

/// Query parameters for listing pet registrations.
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct PetRegistrationQuery {
    pub building_id: Option<Uuid>,
    pub unit_id: Option<Uuid>,
    pub owner_id: Option<Uuid>,
    pub status: Option<String>,
    pub pet_type: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Query parameters for listing vehicle registrations.
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct VehicleRegistrationQuery {
    pub building_id: Option<Uuid>,
    pub unit_id: Option<Uuid>,
    pub owner_id: Option<Uuid>,
    pub status: Option<String>,
    pub vehicle_type: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Query parameters for listing parking spots.
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct ParkingSpotQuery {
    pub building_id: Option<Uuid>,
    pub is_available: Option<bool>,
    pub is_covered: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Registry statistics for a building.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RegistryStatistics {
    pub total_pets: i64,
    pub pending_pets: i64,
    pub approved_pets: i64,
    pub total_vehicles: i64,
    pub pending_vehicles: i64,
    pub approved_vehicles: i64,
    pub total_parking_spots: i64,
    pub available_parking_spots: i64,
}

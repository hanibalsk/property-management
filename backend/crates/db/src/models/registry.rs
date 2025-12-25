//! Building Registries models (Epic 57: Pets & Vehicles).
//!
//! Types for pet and vehicle registrations.

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

// =============================================================================
// ENUMS
// =============================================================================

/// Pet type categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "pet_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PetType {
    Dog,
    Cat,
    Bird,
    Fish,
    Rabbit,
    Hamster,
    Reptile,
    Other,
}

/// Pet size categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "pet_size", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PetSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

/// Vehicle type categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "vehicle_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum VehicleType {
    Car,
    Motorcycle,
    Bicycle,
    ElectricScooter,
    Truck,
    Van,
    Other,
}

/// Registry entry status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "registry_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum RegistryStatus {
    Pending,
    Approved,
    Rejected,
    Expired,
    Inactive,
}

// =============================================================================
// PET REGISTRATION
// =============================================================================

/// Pet registration entity.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PetRegistration {
    pub id: Uuid,
    pub building_id: Uuid,
    pub unit_id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub pet_type: PetType,
    pub breed: Option<String>,
    pub pet_size: Option<PetSize>,
    pub weight_kg: Option<Decimal>,
    pub color: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub microchip_number: Option<String>,
    pub status: RegistryStatus,
    pub registration_number: Option<String>,
    pub registered_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub photo_url: Option<String>,
    pub vaccination_document_url: Option<String>,
    pub vaccination_expiry: Option<NaiveDate>,
    pub license_document_url: Option<String>,
    pub insurance_document_url: Option<String>,
    pub special_needs: Option<String>,
    pub notes: Option<String>,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub rejection_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Pet registration with additional details.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PetRegistrationWithDetails {
    #[serde(flatten)]
    pub registration: PetRegistration,
    pub owner_name: Option<String>,
    pub unit_number: Option<String>,
}

/// Request to create a pet registration.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreatePetRegistrationRequest {
    pub unit_id: Uuid,
    pub name: String,
    pub pet_type: PetType,
    pub breed: Option<String>,
    pub pet_size: Option<PetSize>,
    pub weight_kg: Option<Decimal>,
    pub color: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub microchip_number: Option<String>,
    pub photo_url: Option<String>,
    pub vaccination_document_url: Option<String>,
    pub vaccination_expiry: Option<NaiveDate>,
    pub license_document_url: Option<String>,
    pub insurance_document_url: Option<String>,
    pub special_needs: Option<String>,
    pub notes: Option<String>,
}

/// Request to update a pet registration.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdatePetRegistrationRequest {
    pub name: Option<String>,
    pub breed: Option<String>,
    pub pet_size: Option<PetSize>,
    pub weight_kg: Option<Decimal>,
    pub color: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub microchip_number: Option<String>,
    pub photo_url: Option<String>,
    pub vaccination_document_url: Option<String>,
    pub vaccination_expiry: Option<NaiveDate>,
    pub license_document_url: Option<String>,
    pub insurance_document_url: Option<String>,
    pub special_needs: Option<String>,
    pub notes: Option<String>,
}

/// Pet registration list response.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PetRegistrationListResponse {
    pub items: Vec<PetRegistrationWithDetails>,
    pub total: i64,
    pub page: i32,
    pub page_size: i32,
}

// =============================================================================
// VEHICLE REGISTRATION
// =============================================================================

/// Vehicle registration entity.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct VehicleRegistration {
    pub id: Uuid,
    pub building_id: Uuid,
    pub unit_id: Uuid,
    pub owner_id: Uuid,
    pub vehicle_type: VehicleType,
    pub make: String,
    pub model: String,
    pub year: Option<i32>,
    pub color: Option<String>,
    pub license_plate: String,
    pub vin: Option<String>,
    pub status: RegistryStatus,
    pub registration_number: Option<String>,
    pub registered_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub parking_spot_id: Option<Uuid>,
    pub parking_permit_number: Option<String>,
    pub photo_url: Option<String>,
    pub registration_document_url: Option<String>,
    pub insurance_document_url: Option<String>,
    pub insurance_expiry: Option<NaiveDate>,
    pub notes: Option<String>,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub rejection_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Vehicle registration with additional details.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct VehicleRegistrationWithDetails {
    #[serde(flatten)]
    pub registration: VehicleRegistration,
    pub owner_name: Option<String>,
    pub unit_number: Option<String>,
    pub parking_spot_number: Option<String>,
}

/// Request to create a vehicle registration.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateVehicleRegistrationRequest {
    pub unit_id: Uuid,
    pub vehicle_type: VehicleType,
    pub make: String,
    pub model: String,
    pub year: Option<i32>,
    pub color: Option<String>,
    pub license_plate: String,
    pub vin: Option<String>,
    pub parking_spot_id: Option<Uuid>,
    pub photo_url: Option<String>,
    pub registration_document_url: Option<String>,
    pub insurance_document_url: Option<String>,
    pub insurance_expiry: Option<NaiveDate>,
    pub notes: Option<String>,
}

/// Request to update a vehicle registration.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateVehicleRegistrationRequest {
    pub make: Option<String>,
    pub model: Option<String>,
    pub year: Option<i32>,
    pub color: Option<String>,
    pub license_plate: Option<String>,
    pub vin: Option<String>,
    pub parking_spot_id: Option<Uuid>,
    pub photo_url: Option<String>,
    pub registration_document_url: Option<String>,
    pub insurance_document_url: Option<String>,
    pub insurance_expiry: Option<NaiveDate>,
    pub notes: Option<String>,
}

/// Vehicle registration list response.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct VehicleRegistrationListResponse {
    pub items: Vec<VehicleRegistrationWithDetails>,
    pub total: i64,
    pub page: i32,
    pub page_size: i32,
}

// =============================================================================
// PARKING SPOTS
// =============================================================================

/// Parking spot entity.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ParkingSpot {
    pub id: Uuid,
    pub building_id: Uuid,
    pub spot_number: String,
    pub floor: Option<String>,
    pub section: Option<String>,
    pub spot_type: String,
    pub assigned_unit_id: Option<Uuid>,
    pub assigned_vehicle_id: Option<Uuid>,
    pub is_available: bool,
    pub has_electric_charging: bool,
    pub is_covered: bool,
    pub width_meters: Option<Decimal>,
    pub length_meters: Option<Decimal>,
    pub monthly_fee: Option<Decimal>,
    pub fee_currency: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request to create a parking spot.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateParkingSpotRequest {
    pub spot_number: String,
    pub floor: Option<String>,
    pub section: Option<String>,
    pub spot_type: Option<String>,
    pub has_electric_charging: Option<bool>,
    pub is_covered: Option<bool>,
    pub width_meters: Option<Decimal>,
    pub length_meters: Option<Decimal>,
    pub monthly_fee: Option<Decimal>,
    pub fee_currency: Option<String>,
    pub notes: Option<String>,
}

/// Parking spot list response.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ParkingSpotListResponse {
    pub items: Vec<ParkingSpot>,
    pub total: i64,
}

// =============================================================================
// REGISTRY RULES
// =============================================================================

/// Building registry rules entity.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BuildingRegistryRules {
    pub id: Uuid,
    pub building_id: Uuid,
    // Pet rules
    pub pets_allowed: bool,
    pub max_pets_per_unit: Option<i32>,
    pub allowed_pet_types: Option<Vec<PetType>>,
    pub max_pet_weight_kg: Option<Decimal>,
    pub requires_pet_approval: bool,
    pub requires_pet_vaccination: bool,
    pub requires_pet_insurance: bool,
    pub pet_deposit_amount: Option<Decimal>,
    pub pet_monthly_fee: Option<Decimal>,
    pub restricted_breeds: Option<Vec<String>>,
    // Vehicle rules
    pub vehicles_allowed: bool,
    pub max_vehicles_per_unit: Option<i32>,
    pub allowed_vehicle_types: Option<Vec<VehicleType>>,
    pub requires_vehicle_approval: bool,
    pub requires_vehicle_insurance: bool,
    pub parking_fee_included: bool,
    pub guest_parking_allowed: bool,
    pub guest_parking_max_hours: Option<i32>,
    // General settings
    pub registration_validity_months: Option<i32>,
    pub renewal_reminder_days: Option<i32>,
    pub additional_rules: Option<String>,
    // Audit
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request to update registry rules.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateRegistryRulesRequest {
    // Pet rules
    pub pets_allowed: Option<bool>,
    pub max_pets_per_unit: Option<i32>,
    pub allowed_pet_types: Option<Vec<PetType>>,
    pub max_pet_weight_kg: Option<Decimal>,
    pub requires_pet_approval: Option<bool>,
    pub requires_pet_vaccination: Option<bool>,
    pub requires_pet_insurance: Option<bool>,
    pub pet_deposit_amount: Option<Decimal>,
    pub pet_monthly_fee: Option<Decimal>,
    pub restricted_breeds: Option<Vec<String>>,
    // Vehicle rules
    pub vehicles_allowed: Option<bool>,
    pub max_vehicles_per_unit: Option<i32>,
    pub allowed_vehicle_types: Option<Vec<VehicleType>>,
    pub requires_vehicle_approval: Option<bool>,
    pub requires_vehicle_insurance: Option<bool>,
    pub parking_fee_included: Option<bool>,
    pub guest_parking_allowed: Option<bool>,
    pub guest_parking_max_hours: Option<i32>,
    // General settings
    pub registration_validity_months: Option<i32>,
    pub renewal_reminder_days: Option<i32>,
    pub additional_rules: Option<String>,
}

// =============================================================================
// REVIEW REQUEST
// =============================================================================

/// Request to approve or reject a registration.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ReviewRegistrationRequest {
    pub approve: bool,
    pub rejection_reason: Option<String>,
}

// =============================================================================
// QUERY PARAMETERS
// =============================================================================

/// Query parameters for listing registrations.
#[derive(Debug, Clone, Deserialize, IntoParams)]
pub struct ListRegistrationsQuery {
    pub status: Option<RegistryStatus>,
    pub unit_id: Option<Uuid>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

/// Query parameters for listing parking spots.
#[derive(Debug, Clone, Deserialize, IntoParams)]
pub struct ListParkingSpotsQuery {
    pub available_only: Option<bool>,
    pub floor: Option<String>,
    pub spot_type: Option<String>,
}

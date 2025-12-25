//! Building Registries repository (Epic 57: Pets & Vehicles).
//!
//! Database operations for pet and vehicle registrations.

use sqlx::PgPool;
use uuid::Uuid;

use crate::models::registry::{
    BuildingRegistryRules, CreateParkingSpotRequest, CreatePetRegistrationRequest,
    CreateVehicleRegistrationRequest, ListParkingSpotsQuery, ListRegistrationsQuery, ParkingSpot,
    PetRegistration, PetRegistrationWithDetails, RegistryStatus, UpdatePetRegistrationRequest,
    UpdateRegistryRulesRequest, UpdateVehicleRegistrationRequest, VehicleRegistration,
    VehicleRegistrationWithDetails,
};

// =============================================================================
// PET REGISTRATIONS
// =============================================================================

/// Create a new pet registration.
pub async fn create_pet_registration(
    pool: &PgPool,
    building_id: Uuid,
    owner_id: Uuid,
    req: &CreatePetRegistrationRequest,
) -> Result<PetRegistration, sqlx::Error> {
    sqlx::query_as!(
        PetRegistration,
        r#"
        INSERT INTO pet_registrations (
            building_id, unit_id, owner_id, name, pet_type, breed, pet_size,
            weight_kg, color, date_of_birth, microchip_number, photo_url,
            vaccination_document_url, vaccination_expiry, license_document_url,
            insurance_document_url, special_needs, notes, status
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, 'pending')
        RETURNING
            id, building_id, unit_id, owner_id, name,
            pet_type as "pet_type: _", breed, pet_size as "pet_size: _",
            weight_kg, color, date_of_birth, microchip_number,
            status as "status: _", registration_number, registered_at, expires_at,
            photo_url, vaccination_document_url, vaccination_expiry,
            license_document_url, insurance_document_url, special_needs, notes,
            reviewed_by, reviewed_at, rejection_reason, created_at, updated_at
        "#,
        building_id,
        req.unit_id,
        owner_id,
        req.name,
        req.pet_type as _,
        req.breed,
        req.pet_size as _,
        req.weight_kg,
        req.color,
        req.date_of_birth,
        req.microchip_number,
        req.photo_url,
        req.vaccination_document_url,
        req.vaccination_expiry,
        req.license_document_url,
        req.insurance_document_url,
        req.special_needs,
        req.notes,
    )
    .fetch_one(pool)
    .await
}

/// Get a pet registration by ID.
pub async fn get_pet_registration(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<PetRegistration>, sqlx::Error> {
    sqlx::query_as!(
        PetRegistration,
        r#"
        SELECT
            id, building_id, unit_id, owner_id, name,
            pet_type as "pet_type: _", breed, pet_size as "pet_size: _",
            weight_kg, color, date_of_birth, microchip_number,
            status as "status: _", registration_number, registered_at, expires_at,
            photo_url, vaccination_document_url, vaccination_expiry,
            license_document_url, insurance_document_url, special_needs, notes,
            reviewed_by, reviewed_at, rejection_reason, created_at, updated_at
        FROM pet_registrations
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

/// List pet registrations for a building.
pub async fn list_pet_registrations(
    pool: &PgPool,
    building_id: Uuid,
    query: &ListRegistrationsQuery,
) -> Result<(Vec<PetRegistrationWithDetails>, i64), sqlx::Error> {
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * page_size;

    // Get total count
    let total: i64 = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) as "count!"
        FROM pet_registrations
        WHERE building_id = $1
        AND ($2::registry_status IS NULL OR status = $2)
        AND ($3::uuid IS NULL OR unit_id = $3)
        "#,
        building_id,
        query.status as _,
        query.unit_id,
    )
    .fetch_one(pool)
    .await?;

    // Get items with details
    let rows = sqlx::query!(
        r#"
        SELECT
            p.id, p.building_id, p.unit_id, p.owner_id, p.name,
            p.pet_type as "pet_type: _", p.breed, p.pet_size as "pet_size: _",
            p.weight_kg, p.color, p.date_of_birth, p.microchip_number,
            p.status as "status: _", p.registration_number, p.registered_at, p.expires_at,
            p.photo_url, p.vaccination_document_url, p.vaccination_expiry,
            p.license_document_url, p.insurance_document_url, p.special_needs, p.notes,
            p.reviewed_by, p.reviewed_at, p.rejection_reason, p.created_at, p.updated_at,
            u.name as owner_name,
            un.unit_number
        FROM pet_registrations p
        LEFT JOIN users u ON u.id = p.owner_id
        LEFT JOIN units un ON un.id = p.unit_id
        WHERE p.building_id = $1
        AND ($2::registry_status IS NULL OR p.status = $2)
        AND ($3::uuid IS NULL OR p.unit_id = $3)
        ORDER BY p.created_at DESC
        LIMIT $4 OFFSET $5
        "#,
        building_id,
        query.status as _,
        query.unit_id,
        page_size as i64,
        offset as i64,
    )
    .fetch_all(pool)
    .await?;

    let items: Vec<PetRegistrationWithDetails> = rows
        .into_iter()
        .map(|row| PetRegistrationWithDetails {
            registration: PetRegistration {
                id: row.id,
                building_id: row.building_id,
                unit_id: row.unit_id,
                owner_id: row.owner_id,
                name: row.name,
                pet_type: row.pet_type,
                breed: row.breed,
                pet_size: row.pet_size,
                weight_kg: row.weight_kg,
                color: row.color,
                date_of_birth: row.date_of_birth,
                microchip_number: row.microchip_number,
                status: row.status,
                registration_number: row.registration_number,
                registered_at: row.registered_at,
                expires_at: row.expires_at,
                photo_url: row.photo_url,
                vaccination_document_url: row.vaccination_document_url,
                vaccination_expiry: row.vaccination_expiry,
                license_document_url: row.license_document_url,
                insurance_document_url: row.insurance_document_url,
                special_needs: row.special_needs,
                notes: row.notes,
                reviewed_by: row.reviewed_by,
                reviewed_at: row.reviewed_at,
                rejection_reason: row.rejection_reason,
                created_at: row.created_at,
                updated_at: row.updated_at,
            },
            owner_name: row.owner_name,
            unit_number: row.unit_number,
        })
        .collect();

    Ok((items, total))
}

/// Update a pet registration.
pub async fn update_pet_registration(
    pool: &PgPool,
    id: Uuid,
    req: &UpdatePetRegistrationRequest,
) -> Result<PetRegistration, sqlx::Error> {
    sqlx::query_as!(
        PetRegistration,
        r#"
        UPDATE pet_registrations SET
            name = COALESCE($2, name),
            breed = COALESCE($3, breed),
            pet_size = COALESCE($4, pet_size),
            weight_kg = COALESCE($5, weight_kg),
            color = COALESCE($6, color),
            date_of_birth = COALESCE($7, date_of_birth),
            microchip_number = COALESCE($8, microchip_number),
            photo_url = COALESCE($9, photo_url),
            vaccination_document_url = COALESCE($10, vaccination_document_url),
            vaccination_expiry = COALESCE($11, vaccination_expiry),
            license_document_url = COALESCE($12, license_document_url),
            insurance_document_url = COALESCE($13, insurance_document_url),
            special_needs = COALESCE($14, special_needs),
            notes = COALESCE($15, notes),
            updated_at = NOW()
        WHERE id = $1
        RETURNING
            id, building_id, unit_id, owner_id, name,
            pet_type as "pet_type: _", breed, pet_size as "pet_size: _",
            weight_kg, color, date_of_birth, microchip_number,
            status as "status: _", registration_number, registered_at, expires_at,
            photo_url, vaccination_document_url, vaccination_expiry,
            license_document_url, insurance_document_url, special_needs, notes,
            reviewed_by, reviewed_at, rejection_reason, created_at, updated_at
        "#,
        id,
        req.name,
        req.breed,
        req.pet_size as _,
        req.weight_kg,
        req.color,
        req.date_of_birth,
        req.microchip_number,
        req.photo_url,
        req.vaccination_document_url,
        req.vaccination_expiry,
        req.license_document_url,
        req.insurance_document_url,
        req.special_needs,
        req.notes,
    )
    .fetch_one(pool)
    .await
}

/// Approve or reject a pet registration.
pub async fn review_pet_registration(
    pool: &PgPool,
    id: Uuid,
    reviewer_id: Uuid,
    approve: bool,
    rejection_reason: Option<String>,
) -> Result<PetRegistration, sqlx::Error> {
    let status = if approve {
        RegistryStatus::Approved
    } else {
        RegistryStatus::Rejected
    };

    sqlx::query_as!(
        PetRegistration,
        r#"
        UPDATE pet_registrations SET
            status = $2,
            reviewed_by = $3,
            reviewed_at = NOW(),
            rejection_reason = $4,
            updated_at = NOW()
        WHERE id = $1
        RETURNING
            id, building_id, unit_id, owner_id, name,
            pet_type as "pet_type: _", breed, pet_size as "pet_size: _",
            weight_kg, color, date_of_birth, microchip_number,
            status as "status: _", registration_number, registered_at, expires_at,
            photo_url, vaccination_document_url, vaccination_expiry,
            license_document_url, insurance_document_url, special_needs, notes,
            reviewed_by, reviewed_at, rejection_reason, created_at, updated_at
        "#,
        id,
        status as _,
        reviewer_id,
        rejection_reason,
    )
    .fetch_one(pool)
    .await
}

/// Delete a pet registration.
pub async fn delete_pet_registration(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM pet_registrations WHERE id = $1", id)
        .execute(pool)
        .await?;
    Ok(())
}

// =============================================================================
// VEHICLE REGISTRATIONS
// =============================================================================

/// Create a new vehicle registration.
pub async fn create_vehicle_registration(
    pool: &PgPool,
    building_id: Uuid,
    owner_id: Uuid,
    req: &CreateVehicleRegistrationRequest,
) -> Result<VehicleRegistration, sqlx::Error> {
    sqlx::query_as!(
        VehicleRegistration,
        r#"
        INSERT INTO vehicle_registrations (
            building_id, unit_id, owner_id, vehicle_type, make, model,
            year, color, license_plate, vin, parking_spot_id, photo_url,
            registration_document_url, insurance_document_url, insurance_expiry,
            notes, status
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, 'pending')
        RETURNING
            id, building_id, unit_id, owner_id,
            vehicle_type as "vehicle_type: _", make, model, year, color,
            license_plate, vin, status as "status: _", registration_number,
            registered_at, expires_at, parking_spot_id, parking_permit_number,
            photo_url, registration_document_url, insurance_document_url,
            insurance_expiry, notes, reviewed_by, reviewed_at, rejection_reason,
            created_at, updated_at
        "#,
        building_id,
        req.unit_id,
        owner_id,
        req.vehicle_type as _,
        req.make,
        req.model,
        req.year,
        req.color,
        req.license_plate,
        req.vin,
        req.parking_spot_id,
        req.photo_url,
        req.registration_document_url,
        req.insurance_document_url,
        req.insurance_expiry,
        req.notes,
    )
    .fetch_one(pool)
    .await
}

/// Get a vehicle registration by ID.
pub async fn get_vehicle_registration(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<VehicleRegistration>, sqlx::Error> {
    sqlx::query_as!(
        VehicleRegistration,
        r#"
        SELECT
            id, building_id, unit_id, owner_id,
            vehicle_type as "vehicle_type: _", make, model, year, color,
            license_plate, vin, status as "status: _", registration_number,
            registered_at, expires_at, parking_spot_id, parking_permit_number,
            photo_url, registration_document_url, insurance_document_url,
            insurance_expiry, notes, reviewed_by, reviewed_at, rejection_reason,
            created_at, updated_at
        FROM vehicle_registrations
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

/// List vehicle registrations for a building.
pub async fn list_vehicle_registrations(
    pool: &PgPool,
    building_id: Uuid,
    query: &ListRegistrationsQuery,
) -> Result<(Vec<VehicleRegistrationWithDetails>, i64), sqlx::Error> {
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * page_size;

    // Get total count
    let total: i64 = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) as "count!"
        FROM vehicle_registrations
        WHERE building_id = $1
        AND ($2::registry_status IS NULL OR status = $2)
        AND ($3::uuid IS NULL OR unit_id = $3)
        "#,
        building_id,
        query.status as _,
        query.unit_id,
    )
    .fetch_one(pool)
    .await?;

    // Get items with details
    let rows = sqlx::query!(
        r#"
        SELECT
            v.id, v.building_id, v.unit_id, v.owner_id,
            v.vehicle_type as "vehicle_type: _", v.make, v.model, v.year, v.color,
            v.license_plate, v.vin, v.status as "status: _", v.registration_number,
            v.registered_at, v.expires_at, v.parking_spot_id, v.parking_permit_number,
            v.photo_url, v.registration_document_url, v.insurance_document_url,
            v.insurance_expiry, v.notes, v.reviewed_by, v.reviewed_at, v.rejection_reason,
            v.created_at, v.updated_at,
            u.name as owner_name,
            un.unit_number,
            ps.spot_number as parking_spot_number
        FROM vehicle_registrations v
        LEFT JOIN users u ON u.id = v.owner_id
        LEFT JOIN units un ON un.id = v.unit_id
        LEFT JOIN parking_spots ps ON ps.id = v.parking_spot_id
        WHERE v.building_id = $1
        AND ($2::registry_status IS NULL OR v.status = $2)
        AND ($3::uuid IS NULL OR v.unit_id = $3)
        ORDER BY v.created_at DESC
        LIMIT $4 OFFSET $5
        "#,
        building_id,
        query.status as _,
        query.unit_id,
        page_size as i64,
        offset as i64,
    )
    .fetch_all(pool)
    .await?;

    let items: Vec<VehicleRegistrationWithDetails> = rows
        .into_iter()
        .map(|row| VehicleRegistrationWithDetails {
            registration: VehicleRegistration {
                id: row.id,
                building_id: row.building_id,
                unit_id: row.unit_id,
                owner_id: row.owner_id,
                vehicle_type: row.vehicle_type,
                make: row.make,
                model: row.model,
                year: row.year,
                color: row.color,
                license_plate: row.license_plate,
                vin: row.vin,
                status: row.status,
                registration_number: row.registration_number,
                registered_at: row.registered_at,
                expires_at: row.expires_at,
                parking_spot_id: row.parking_spot_id,
                parking_permit_number: row.parking_permit_number,
                photo_url: row.photo_url,
                registration_document_url: row.registration_document_url,
                insurance_document_url: row.insurance_document_url,
                insurance_expiry: row.insurance_expiry,
                notes: row.notes,
                reviewed_by: row.reviewed_by,
                reviewed_at: row.reviewed_at,
                rejection_reason: row.rejection_reason,
                created_at: row.created_at,
                updated_at: row.updated_at,
            },
            owner_name: row.owner_name,
            unit_number: row.unit_number,
            parking_spot_number: row.parking_spot_number,
        })
        .collect();

    Ok((items, total))
}

/// Update a vehicle registration.
pub async fn update_vehicle_registration(
    pool: &PgPool,
    id: Uuid,
    req: &UpdateVehicleRegistrationRequest,
) -> Result<VehicleRegistration, sqlx::Error> {
    sqlx::query_as!(
        VehicleRegistration,
        r#"
        UPDATE vehicle_registrations SET
            make = COALESCE($2, make),
            model = COALESCE($3, model),
            year = COALESCE($4, year),
            color = COALESCE($5, color),
            license_plate = COALESCE($6, license_plate),
            vin = COALESCE($7, vin),
            parking_spot_id = COALESCE($8, parking_spot_id),
            photo_url = COALESCE($9, photo_url),
            registration_document_url = COALESCE($10, registration_document_url),
            insurance_document_url = COALESCE($11, insurance_document_url),
            insurance_expiry = COALESCE($12, insurance_expiry),
            notes = COALESCE($13, notes),
            updated_at = NOW()
        WHERE id = $1
        RETURNING
            id, building_id, unit_id, owner_id,
            vehicle_type as "vehicle_type: _", make, model, year, color,
            license_plate, vin, status as "status: _", registration_number,
            registered_at, expires_at, parking_spot_id, parking_permit_number,
            photo_url, registration_document_url, insurance_document_url,
            insurance_expiry, notes, reviewed_by, reviewed_at, rejection_reason,
            created_at, updated_at
        "#,
        id,
        req.make,
        req.model,
        req.year,
        req.color,
        req.license_plate,
        req.vin,
        req.parking_spot_id,
        req.photo_url,
        req.registration_document_url,
        req.insurance_document_url,
        req.insurance_expiry,
        req.notes,
    )
    .fetch_one(pool)
    .await
}

/// Approve or reject a vehicle registration.
pub async fn review_vehicle_registration(
    pool: &PgPool,
    id: Uuid,
    reviewer_id: Uuid,
    approve: bool,
    rejection_reason: Option<String>,
) -> Result<VehicleRegistration, sqlx::Error> {
    let status = if approve {
        RegistryStatus::Approved
    } else {
        RegistryStatus::Rejected
    };

    sqlx::query_as!(
        VehicleRegistration,
        r#"
        UPDATE vehicle_registrations SET
            status = $2,
            reviewed_by = $3,
            reviewed_at = NOW(),
            rejection_reason = $4,
            updated_at = NOW()
        WHERE id = $1
        RETURNING
            id, building_id, unit_id, owner_id,
            vehicle_type as "vehicle_type: _", make, model, year, color,
            license_plate, vin, status as "status: _", registration_number,
            registered_at, expires_at, parking_spot_id, parking_permit_number,
            photo_url, registration_document_url, insurance_document_url,
            insurance_expiry, notes, reviewed_by, reviewed_at, rejection_reason,
            created_at, updated_at
        "#,
        id,
        status as _,
        reviewer_id,
        rejection_reason,
    )
    .fetch_one(pool)
    .await
}

/// Delete a vehicle registration.
pub async fn delete_vehicle_registration(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM vehicle_registrations WHERE id = $1", id)
        .execute(pool)
        .await?;
    Ok(())
}

// =============================================================================
// PARKING SPOTS
// =============================================================================

/// Create a parking spot.
pub async fn create_parking_spot(
    pool: &PgPool,
    building_id: Uuid,
    req: &CreateParkingSpotRequest,
) -> Result<ParkingSpot, sqlx::Error> {
    sqlx::query_as!(
        ParkingSpot,
        r#"
        INSERT INTO parking_spots (
            building_id, spot_number, floor, section, spot_type,
            has_electric_charging, is_covered, width_meters, length_meters,
            monthly_fee, fee_currency, notes
        )
        VALUES ($1, $2, $3, $4, COALESCE($5, 'standard'), COALESCE($6, false),
                COALESCE($7, false), $8, $9, $10, $11, $12)
        RETURNING *
        "#,
        building_id,
        req.spot_number,
        req.floor,
        req.section,
        req.spot_type,
        req.has_electric_charging,
        req.is_covered,
        req.width_meters,
        req.length_meters,
        req.monthly_fee,
        req.fee_currency,
        req.notes,
    )
    .fetch_one(pool)
    .await
}

/// List parking spots for a building.
pub async fn list_parking_spots(
    pool: &PgPool,
    building_id: Uuid,
    query: &ListParkingSpotsQuery,
) -> Result<Vec<ParkingSpot>, sqlx::Error> {
    sqlx::query_as!(
        ParkingSpot,
        r#"
        SELECT *
        FROM parking_spots
        WHERE building_id = $1
        AND ($2::boolean IS NULL OR is_available = $2)
        AND ($3::text IS NULL OR floor = $3)
        AND ($4::text IS NULL OR spot_type = $4)
        ORDER BY spot_number
        "#,
        building_id,
        query.available_only,
        query.floor,
        query.spot_type,
    )
    .fetch_all(pool)
    .await
}

/// Delete a parking spot.
pub async fn delete_parking_spot(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM parking_spots WHERE id = $1", id)
        .execute(pool)
        .await?;
    Ok(())
}

// =============================================================================
// REGISTRY RULES
// =============================================================================

/// Get registry rules for a building.
pub async fn get_registry_rules(
    pool: &PgPool,
    building_id: Uuid,
) -> Result<Option<BuildingRegistryRules>, sqlx::Error> {
    sqlx::query_as!(
        BuildingRegistryRules,
        r#"
        SELECT
            id, building_id, pets_allowed, max_pets_per_unit,
            allowed_pet_types as "allowed_pet_types: Vec<_>",
            max_pet_weight_kg, requires_pet_approval, requires_pet_vaccination,
            requires_pet_insurance, pet_deposit_amount, pet_monthly_fee,
            restricted_breeds, vehicles_allowed, max_vehicles_per_unit,
            allowed_vehicle_types as "allowed_vehicle_types: Vec<_>",
            requires_vehicle_approval, requires_vehicle_insurance,
            parking_fee_included, guest_parking_allowed, guest_parking_max_hours,
            registration_validity_months, renewal_reminder_days, additional_rules,
            created_at, updated_at
        FROM building_registry_rules
        WHERE building_id = $1
        "#,
        building_id
    )
    .fetch_optional(pool)
    .await
}

/// Create or update registry rules for a building.
pub async fn upsert_registry_rules(
    pool: &PgPool,
    building_id: Uuid,
    req: &UpdateRegistryRulesRequest,
) -> Result<BuildingRegistryRules, sqlx::Error> {
    sqlx::query_as!(
        BuildingRegistryRules,
        r#"
        INSERT INTO building_registry_rules (
            building_id, pets_allowed, max_pets_per_unit, allowed_pet_types,
            max_pet_weight_kg, requires_pet_approval, requires_pet_vaccination,
            requires_pet_insurance, pet_deposit_amount, pet_monthly_fee,
            restricted_breeds, vehicles_allowed, max_vehicles_per_unit,
            allowed_vehicle_types, requires_vehicle_approval, requires_vehicle_insurance,
            parking_fee_included, guest_parking_allowed, guest_parking_max_hours,
            registration_validity_months, renewal_reminder_days, additional_rules
        )
        VALUES (
            $1, COALESCE($2, true), $3, $4, $5, COALESCE($6, true), COALESCE($7, true),
            COALESCE($8, false), $9, $10, $11, COALESCE($12, true), $13, $14,
            COALESCE($15, false), COALESCE($16, true), COALESCE($17, false),
            COALESCE($18, true), $19, $20, $21, $22
        )
        ON CONFLICT (building_id) DO UPDATE SET
            pets_allowed = COALESCE($2, building_registry_rules.pets_allowed),
            max_pets_per_unit = COALESCE($3, building_registry_rules.max_pets_per_unit),
            allowed_pet_types = COALESCE($4, building_registry_rules.allowed_pet_types),
            max_pet_weight_kg = COALESCE($5, building_registry_rules.max_pet_weight_kg),
            requires_pet_approval = COALESCE($6, building_registry_rules.requires_pet_approval),
            requires_pet_vaccination = COALESCE($7, building_registry_rules.requires_pet_vaccination),
            requires_pet_insurance = COALESCE($8, building_registry_rules.requires_pet_insurance),
            pet_deposit_amount = COALESCE($9, building_registry_rules.pet_deposit_amount),
            pet_monthly_fee = COALESCE($10, building_registry_rules.pet_monthly_fee),
            restricted_breeds = COALESCE($11, building_registry_rules.restricted_breeds),
            vehicles_allowed = COALESCE($12, building_registry_rules.vehicles_allowed),
            max_vehicles_per_unit = COALESCE($13, building_registry_rules.max_vehicles_per_unit),
            allowed_vehicle_types = COALESCE($14, building_registry_rules.allowed_vehicle_types),
            requires_vehicle_approval = COALESCE($15, building_registry_rules.requires_vehicle_approval),
            requires_vehicle_insurance = COALESCE($16, building_registry_rules.requires_vehicle_insurance),
            parking_fee_included = COALESCE($17, building_registry_rules.parking_fee_included),
            guest_parking_allowed = COALESCE($18, building_registry_rules.guest_parking_allowed),
            guest_parking_max_hours = COALESCE($19, building_registry_rules.guest_parking_max_hours),
            registration_validity_months = COALESCE($20, building_registry_rules.registration_validity_months),
            renewal_reminder_days = COALESCE($21, building_registry_rules.renewal_reminder_days),
            additional_rules = COALESCE($22, building_registry_rules.additional_rules),
            updated_at = NOW()
        RETURNING
            id, building_id, pets_allowed, max_pets_per_unit,
            allowed_pet_types as "allowed_pet_types: Vec<_>",
            max_pet_weight_kg, requires_pet_approval, requires_pet_vaccination,
            requires_pet_insurance, pet_deposit_amount, pet_monthly_fee,
            restricted_breeds, vehicles_allowed, max_vehicles_per_unit,
            allowed_vehicle_types as "allowed_vehicle_types: Vec<_>",
            requires_vehicle_approval, requires_vehicle_insurance,
            parking_fee_included, guest_parking_allowed, guest_parking_max_hours,
            registration_validity_months, renewal_reminder_days, additional_rules,
            created_at, updated_at
        "#,
        building_id,
        req.pets_allowed,
        req.max_pets_per_unit,
        req.allowed_pet_types.as_ref().map(|v| v.as_slice()) as _,
        req.max_pet_weight_kg,
        req.requires_pet_approval,
        req.requires_pet_vaccination,
        req.requires_pet_insurance,
        req.pet_deposit_amount,
        req.pet_monthly_fee,
        req.restricted_breeds.as_ref().map(|v| v.as_slice()),
        req.vehicles_allowed,
        req.max_vehicles_per_unit,
        req.allowed_vehicle_types.as_ref().map(|v| v.as_slice()) as _,
        req.requires_vehicle_approval,
        req.requires_vehicle_insurance,
        req.parking_fee_included,
        req.guest_parking_allowed,
        req.guest_parking_max_hours,
        req.registration_validity_months,
        req.renewal_reminder_days,
        req.additional_rules,
    )
    .fetch_one(pool)
    .await
}

//! Building Registries routes (Epic 57: Pets & Vehicles).
//!
//! API endpoints for pet and vehicle registrations.

use crate::state::AppState;
use api_core::AuthUser;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use common::errors::ErrorResponse;
use db::models::registry::{
    BuildingRegistryRules, CreateParkingSpotRequest, CreatePetRegistrationRequest,
    CreateVehicleRegistrationRequest, ListParkingSpotsQuery, ListRegistrationsQuery, ParkingSpot,
    ParkingSpotListResponse, PetRegistration, PetRegistrationListResponse,
    PetRegistrationWithDetails, ReviewRegistrationRequest, UpdatePetRegistrationRequest,
    UpdateRegistryRulesRequest, UpdateVehicleRegistrationRequest, VehicleRegistration,
    VehicleRegistrationListResponse, VehicleRegistrationWithDetails,
};
use db::repositories::registry;
use uuid::Uuid;

/// Create registry routes.
pub fn routes() -> Router<AppState> {
    Router::new()
        // Pet registrations
        .route(
            "/buildings/:building_id/pets",
            get(list_pet_registrations).post(create_pet_registration),
        )
        .route(
            "/buildings/:building_id/pets/:id",
            get(get_pet_registration)
                .put(update_pet_registration)
                .delete(delete_pet_registration),
        )
        .route(
            "/buildings/:building_id/pets/:id/review",
            post(review_pet_registration),
        )
        // Vehicle registrations
        .route(
            "/buildings/:building_id/vehicles",
            get(list_vehicle_registrations).post(create_vehicle_registration),
        )
        .route(
            "/buildings/:building_id/vehicles/:id",
            get(get_vehicle_registration)
                .put(update_vehicle_registration)
                .delete(delete_vehicle_registration),
        )
        .route(
            "/buildings/:building_id/vehicles/:id/review",
            post(review_vehicle_registration),
        )
        // Parking spots
        .route(
            "/buildings/:building_id/parking-spots",
            get(list_parking_spots).post(create_parking_spot),
        )
        .route(
            "/buildings/:building_id/parking-spots/:id",
            delete(delete_parking_spot),
        )
        // Registry rules
        .route(
            "/buildings/:building_id/registry-rules",
            get(get_registry_rules).put(update_registry_rules),
        )
        // My registrations
        .route("/my/pets", get(list_my_pets))
        .route("/my/vehicles", get(list_my_vehicles))
}

// =============================================================================
// PET REGISTRATION HANDLERS
// =============================================================================

/// List pet registrations for a building.
#[utoipa::path(
    get,
    path = "/api/v1/buildings/{building_id}/pets",
    params(
        ("building_id" = Uuid, Path, description = "Building ID"),
        ListRegistrationsQuery
    ),
    responses(
        (status = 200, description = "List of pet registrations", body = PetRegistrationListResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
async fn list_pet_registrations(
    State(state): State<AppState>,
    Path(building_id): Path<Uuid>,
    Query(query): Query<ListRegistrationsQuery>,
    _auth: AuthUser,
) -> Result<Json<PetRegistrationListResponse>, ErrorResponse> {
    let (items, total) = registry::list_pet_registrations(&state.db, building_id, &query).await?;
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);

    Ok(Json(PetRegistrationListResponse {
        items,
        total,
        page,
        page_size,
    }))
}

/// Create a pet registration.
#[utoipa::path(
    post,
    path = "/api/v1/buildings/{building_id}/pets",
    params(("building_id" = Uuid, Path, description = "Building ID")),
    request_body = CreatePetRegistrationRequest,
    responses(
        (status = 201, description = "Pet registration created", body = PetRegistration),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
async fn create_pet_registration(
    State(state): State<AppState>,
    Path(building_id): Path<Uuid>,
    auth: AuthUser,
    Json(req): Json<CreatePetRegistrationRequest>,
) -> Result<(StatusCode, Json<PetRegistration>), ErrorResponse> {
    let registration =
        registry::create_pet_registration(&state.db, building_id, auth.user_id, &req).await?;

    Ok((StatusCode::CREATED, Json(registration)))
}

/// Get a pet registration by ID.
#[utoipa::path(
    get,
    path = "/api/v1/buildings/{building_id}/pets/{id}",
    params(
        ("building_id" = Uuid, Path, description = "Building ID"),
        ("id" = Uuid, Path, description = "Pet registration ID")
    ),
    responses(
        (status = 200, description = "Pet registration details", body = PetRegistration),
        (status = 404, description = "Not found")
    ),
    security(("bearer_auth" = []))
)]
async fn get_pet_registration(
    State(state): State<AppState>,
    Path((_building_id, id)): Path<(Uuid, Uuid)>,
    _auth: AuthUser,
) -> Result<Json<PetRegistration>, ErrorResponse> {
    let registration = registry::get_pet_registration(&state.db, id)
        .await?
        .ok_or_else(|| ErrorResponse::not_found("Pet registration not found"))?;

    Ok(Json(registration))
}

/// Update a pet registration.
#[utoipa::path(
    put,
    path = "/api/v1/buildings/{building_id}/pets/{id}",
    params(
        ("building_id" = Uuid, Path, description = "Building ID"),
        ("id" = Uuid, Path, description = "Pet registration ID")
    ),
    request_body = UpdatePetRegistrationRequest,
    responses(
        (status = 200, description = "Pet registration updated", body = PetRegistration),
        (status = 404, description = "Not found")
    ),
    security(("bearer_auth" = []))
)]
async fn update_pet_registration(
    State(state): State<AppState>,
    Path((_building_id, id)): Path<(Uuid, Uuid)>,
    _auth: AuthUser,
    Json(req): Json<UpdatePetRegistrationRequest>,
) -> Result<Json<PetRegistration>, ErrorResponse> {
    let registration = registry::update_pet_registration(&state.db, id, &req).await?;
    Ok(Json(registration))
}

/// Delete a pet registration.
#[utoipa::path(
    delete,
    path = "/api/v1/buildings/{building_id}/pets/{id}",
    params(
        ("building_id" = Uuid, Path, description = "Building ID"),
        ("id" = Uuid, Path, description = "Pet registration ID")
    ),
    responses(
        (status = 204, description = "Pet registration deleted"),
        (status = 404, description = "Not found")
    ),
    security(("bearer_auth" = []))
)]
async fn delete_pet_registration(
    State(state): State<AppState>,
    Path((_building_id, id)): Path<(Uuid, Uuid)>,
    _auth: AuthUser,
) -> Result<StatusCode, ErrorResponse> {
    registry::delete_pet_registration(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Approve or reject a pet registration.
#[utoipa::path(
    post,
    path = "/api/v1/buildings/{building_id}/pets/{id}/review",
    params(
        ("building_id" = Uuid, Path, description = "Building ID"),
        ("id" = Uuid, Path, description = "Pet registration ID")
    ),
    request_body = ReviewRegistrationRequest,
    responses(
        (status = 200, description = "Pet registration reviewed", body = PetRegistration),
        (status = 404, description = "Not found")
    ),
    security(("bearer_auth" = []))
)]
async fn review_pet_registration(
    State(state): State<AppState>,
    Path((_building_id, id)): Path<(Uuid, Uuid)>,
    auth: AuthUser,
    Json(req): Json<ReviewRegistrationRequest>,
) -> Result<Json<PetRegistration>, ErrorResponse> {
    let registration = registry::review_pet_registration(
        &state.db,
        id,
        auth.user_id,
        req.approve,
        req.rejection_reason,
    )
    .await?;

    Ok(Json(registration))
}

// =============================================================================
// VEHICLE REGISTRATION HANDLERS
// =============================================================================

/// List vehicle registrations for a building.
#[utoipa::path(
    get,
    path = "/api/v1/buildings/{building_id}/vehicles",
    params(
        ("building_id" = Uuid, Path, description = "Building ID"),
        ListRegistrationsQuery
    ),
    responses(
        (status = 200, description = "List of vehicle registrations", body = VehicleRegistrationListResponse),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
async fn list_vehicle_registrations(
    State(state): State<AppState>,
    Path(building_id): Path<Uuid>,
    Query(query): Query<ListRegistrationsQuery>,
    _auth: AuthUser,
) -> Result<Json<VehicleRegistrationListResponse>, ErrorResponse> {
    let (items, total) =
        registry::list_vehicle_registrations(&state.db, building_id, &query).await?;
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);

    Ok(Json(VehicleRegistrationListResponse {
        items,
        total,
        page,
        page_size,
    }))
}

/// Create a vehicle registration.
#[utoipa::path(
    post,
    path = "/api/v1/buildings/{building_id}/vehicles",
    params(("building_id" = Uuid, Path, description = "Building ID")),
    request_body = CreateVehicleRegistrationRequest,
    responses(
        (status = 201, description = "Vehicle registration created", body = VehicleRegistration),
        (status = 400, description = "Invalid request")
    ),
    security(("bearer_auth" = []))
)]
async fn create_vehicle_registration(
    State(state): State<AppState>,
    Path(building_id): Path<Uuid>,
    auth: AuthUser,
    Json(req): Json<CreateVehicleRegistrationRequest>,
) -> Result<(StatusCode, Json<VehicleRegistration>), ErrorResponse> {
    let registration =
        registry::create_vehicle_registration(&state.db, building_id, auth.user_id, &req).await?;

    Ok((StatusCode::CREATED, Json(registration)))
}

/// Get a vehicle registration by ID.
#[utoipa::path(
    get,
    path = "/api/v1/buildings/{building_id}/vehicles/{id}",
    params(
        ("building_id" = Uuid, Path, description = "Building ID"),
        ("id" = Uuid, Path, description = "Vehicle registration ID")
    ),
    responses(
        (status = 200, description = "Vehicle registration details", body = VehicleRegistration),
        (status = 404, description = "Not found")
    ),
    security(("bearer_auth" = []))
)]
async fn get_vehicle_registration(
    State(state): State<AppState>,
    Path((_building_id, id)): Path<(Uuid, Uuid)>,
    _auth: AuthUser,
) -> Result<Json<VehicleRegistration>, ErrorResponse> {
    let registration = registry::get_vehicle_registration(&state.db, id)
        .await?
        .ok_or_else(|| ErrorResponse::not_found("Vehicle registration not found"))?;

    Ok(Json(registration))
}

/// Update a vehicle registration.
#[utoipa::path(
    put,
    path = "/api/v1/buildings/{building_id}/vehicles/{id}",
    params(
        ("building_id" = Uuid, Path, description = "Building ID"),
        ("id" = Uuid, Path, description = "Vehicle registration ID")
    ),
    request_body = UpdateVehicleRegistrationRequest,
    responses(
        (status = 200, description = "Vehicle registration updated", body = VehicleRegistration),
        (status = 404, description = "Not found")
    ),
    security(("bearer_auth" = []))
)]
async fn update_vehicle_registration(
    State(state): State<AppState>,
    Path((_building_id, id)): Path<(Uuid, Uuid)>,
    _auth: AuthUser,
    Json(req): Json<UpdateVehicleRegistrationRequest>,
) -> Result<Json<VehicleRegistration>, ErrorResponse> {
    let registration = registry::update_vehicle_registration(&state.db, id, &req).await?;
    Ok(Json(registration))
}

/// Delete a vehicle registration.
#[utoipa::path(
    delete,
    path = "/api/v1/buildings/{building_id}/vehicles/{id}",
    params(
        ("building_id" = Uuid, Path, description = "Building ID"),
        ("id" = Uuid, Path, description = "Vehicle registration ID")
    ),
    responses(
        (status = 204, description = "Vehicle registration deleted"),
        (status = 404, description = "Not found")
    ),
    security(("bearer_auth" = []))
)]
async fn delete_vehicle_registration(
    State(state): State<AppState>,
    Path((_building_id, id)): Path<(Uuid, Uuid)>,
    _auth: AuthUser,
) -> Result<StatusCode, ErrorResponse> {
    registry::delete_vehicle_registration(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Approve or reject a vehicle registration.
#[utoipa::path(
    post,
    path = "/api/v1/buildings/{building_id}/vehicles/{id}/review",
    params(
        ("building_id" = Uuid, Path, description = "Building ID"),
        ("id" = Uuid, Path, description = "Vehicle registration ID")
    ),
    request_body = ReviewRegistrationRequest,
    responses(
        (status = 200, description = "Vehicle registration reviewed", body = VehicleRegistration),
        (status = 404, description = "Not found")
    ),
    security(("bearer_auth" = []))
)]
async fn review_vehicle_registration(
    State(state): State<AppState>,
    Path((_building_id, id)): Path<(Uuid, Uuid)>,
    auth: AuthUser,
    Json(req): Json<ReviewRegistrationRequest>,
) -> Result<Json<VehicleRegistration>, ErrorResponse> {
    let registration = registry::review_vehicle_registration(
        &state.db,
        id,
        auth.user_id,
        req.approve,
        req.rejection_reason,
    )
    .await?;

    Ok(Json(registration))
}

// =============================================================================
// PARKING SPOT HANDLERS
// =============================================================================

/// List parking spots for a building.
#[utoipa::path(
    get,
    path = "/api/v1/buildings/{building_id}/parking-spots",
    params(
        ("building_id" = Uuid, Path, description = "Building ID"),
        ListParkingSpotsQuery
    ),
    responses(
        (status = 200, description = "List of parking spots", body = ParkingSpotListResponse),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
async fn list_parking_spots(
    State(state): State<AppState>,
    Path(building_id): Path<Uuid>,
    Query(query): Query<ListParkingSpotsQuery>,
    _auth: AuthUser,
) -> Result<Json<ParkingSpotListResponse>, ErrorResponse> {
    let items = registry::list_parking_spots(&state.db, building_id, &query).await?;
    let total = items.len() as i64;

    Ok(Json(ParkingSpotListResponse { items, total }))
}

/// Create a parking spot.
#[utoipa::path(
    post,
    path = "/api/v1/buildings/{building_id}/parking-spots",
    params(("building_id" = Uuid, Path, description = "Building ID")),
    request_body = CreateParkingSpotRequest,
    responses(
        (status = 201, description = "Parking spot created", body = ParkingSpot),
        (status = 400, description = "Invalid request")
    ),
    security(("bearer_auth" = []))
)]
async fn create_parking_spot(
    State(state): State<AppState>,
    Path(building_id): Path<Uuid>,
    _auth: AuthUser,
    Json(req): Json<CreateParkingSpotRequest>,
) -> Result<(StatusCode, Json<ParkingSpot>), ErrorResponse> {
    let spot = registry::create_parking_spot(&state.db, building_id, &req).await?;
    Ok((StatusCode::CREATED, Json(spot)))
}

/// Delete a parking spot.
#[utoipa::path(
    delete,
    path = "/api/v1/buildings/{building_id}/parking-spots/{id}",
    params(
        ("building_id" = Uuid, Path, description = "Building ID"),
        ("id" = Uuid, Path, description = "Parking spot ID")
    ),
    responses(
        (status = 204, description = "Parking spot deleted"),
        (status = 404, description = "Not found")
    ),
    security(("bearer_auth" = []))
)]
async fn delete_parking_spot(
    State(state): State<AppState>,
    Path((_building_id, id)): Path<(Uuid, Uuid)>,
    _auth: AuthUser,
) -> Result<StatusCode, ErrorResponse> {
    registry::delete_parking_spot(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// =============================================================================
// REGISTRY RULES HANDLERS
// =============================================================================

/// Get registry rules for a building.
#[utoipa::path(
    get,
    path = "/api/v1/buildings/{building_id}/registry-rules",
    params(("building_id" = Uuid, Path, description = "Building ID")),
    responses(
        (status = 200, description = "Registry rules", body = BuildingRegistryRules),
        (status = 404, description = "No rules configured")
    ),
    security(("bearer_auth" = []))
)]
async fn get_registry_rules(
    State(state): State<AppState>,
    Path(building_id): Path<Uuid>,
    _auth: AuthUser,
) -> Result<Json<BuildingRegistryRules>, ErrorResponse> {
    let rules = registry::get_registry_rules(&state.db, building_id)
        .await?
        .ok_or_else(|| ErrorResponse::not_found("Registry rules not configured"))?;

    Ok(Json(rules))
}

/// Update registry rules for a building.
#[utoipa::path(
    put,
    path = "/api/v1/buildings/{building_id}/registry-rules",
    params(("building_id" = Uuid, Path, description = "Building ID")),
    request_body = UpdateRegistryRulesRequest,
    responses(
        (status = 200, description = "Registry rules updated", body = BuildingRegistryRules),
        (status = 400, description = "Invalid request")
    ),
    security(("bearer_auth" = []))
)]
async fn update_registry_rules(
    State(state): State<AppState>,
    Path(building_id): Path<Uuid>,
    _auth: AuthUser,
    Json(req): Json<UpdateRegistryRulesRequest>,
) -> Result<Json<BuildingRegistryRules>, ErrorResponse> {
    let rules = registry::upsert_registry_rules(&state.db, building_id, &req).await?;
    Ok(Json(rules))
}

// =============================================================================
// MY REGISTRATIONS HANDLERS
// =============================================================================

/// List current user's pet registrations.
#[utoipa::path(
    get,
    path = "/api/v1/my/pets",
    responses(
        (status = 200, description = "List of user's pet registrations", body = Vec<PetRegistrationWithDetails>),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
async fn list_my_pets(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<Vec<PetRegistrationWithDetails>>, ErrorResponse> {
    // Get all buildings the user has access to and fetch their pets
    let rows = sqlx::query!(
        r#"
        SELECT
            p.id, p.building_id, p.unit_id, p.owner_id, p.name,
            p.pet_type as "pet_type: db::models::registry::PetType",
            p.breed, p.pet_size as "pet_size: db::models::registry::PetSize",
            p.weight_kg, p.color, p.date_of_birth, p.microchip_number,
            p.status as "status: db::models::registry::RegistryStatus",
            p.registration_number, p.registered_at, p.expires_at,
            p.photo_url, p.vaccination_document_url, p.vaccination_expiry,
            p.license_document_url, p.insurance_document_url, p.special_needs, p.notes,
            p.reviewed_by, p.reviewed_at, p.rejection_reason, p.created_at, p.updated_at,
            u.name as owner_name,
            un.unit_number
        FROM pet_registrations p
        LEFT JOIN users u ON u.id = p.owner_id
        LEFT JOIN units un ON un.id = p.unit_id
        WHERE p.owner_id = $1
        ORDER BY p.created_at DESC
        "#,
        auth.user_id
    )
    .fetch_all(&*state.db)
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

    Ok(Json(items))
}

/// List current user's vehicle registrations.
#[utoipa::path(
    get,
    path = "/api/v1/my/vehicles",
    responses(
        (status = 200, description = "List of user's vehicle registrations", body = Vec<VehicleRegistrationWithDetails>),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
async fn list_my_vehicles(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<Vec<VehicleRegistrationWithDetails>>, ErrorResponse> {
    let rows = sqlx::query!(
        r#"
        SELECT
            v.id, v.building_id, v.unit_id, v.owner_id,
            v.vehicle_type as "vehicle_type: db::models::registry::VehicleType",
            v.make, v.model, v.year, v.color,
            v.license_plate, v.vin,
            v.status as "status: db::models::registry::RegistryStatus",
            v.registration_number,
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
        WHERE v.owner_id = $1
        ORDER BY v.created_at DESC
        "#,
        auth.user_id
    )
    .fetch_all(&*state.db)
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

    Ok(Json(items))
}

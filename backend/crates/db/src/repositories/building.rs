//! Building repository (Epic 2B, Story 2B.3).

use crate::models::building::{
    Building, BuildingStatistics, BuildingSummary, CreateBuilding, UpdateBuilding,
};
use crate::DbPool;
use sqlx::Error as SqlxError;
use uuid::Uuid;

/// Repository for building operations.
#[derive(Clone)]
pub struct BuildingRepository {
    pool: DbPool,
}

impl BuildingRepository {
    /// Create a new BuildingRepository.
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Create a new building (UC-15.1).
    pub async fn create(&self, data: CreateBuilding) -> Result<Building, SqlxError> {
        let amenities = serde_json::to_value(&data.amenities).unwrap_or_default();

        let building = sqlx::query_as::<_, Building>(
            r#"
            INSERT INTO buildings (
                organization_id, street, city, postal_code, country,
                name, description, year_built, total_floors, total_entrances, amenities
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING *
            "#,
        )
        .bind(data.organization_id)
        .bind(&data.street)
        .bind(&data.city)
        .bind(&data.postal_code)
        .bind(&data.country)
        .bind(&data.name)
        .bind(&data.description)
        .bind(data.year_built)
        .bind(data.total_floors)
        .bind(data.total_entrances)
        .bind(&amenities)
        .fetch_one(&self.pool)
        .await?;

        Ok(building)
    }

    /// Find building by ID (UC-15.2).
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Building>, SqlxError> {
        let building = sqlx::query_as::<_, Building>(
            r#"
            SELECT * FROM buildings WHERE id = $1 AND status != 'archived'
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(building)
    }

    /// Find building by ID including archived.
    pub async fn find_by_id_any_status(&self, id: Uuid) -> Result<Option<Building>, SqlxError> {
        let building = sqlx::query_as::<_, Building>(
            r#"
            SELECT * FROM buildings WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(building)
    }

    /// Update building (UC-15.3).
    pub async fn update(
        &self,
        id: Uuid,
        data: UpdateBuilding,
    ) -> Result<Option<Building>, SqlxError> {
        // Build dynamic update query
        let mut updates = vec!["updated_at = NOW()".to_string()];
        let mut param_idx = 1;

        if data.street.is_some() {
            param_idx += 1;
            updates.push(format!("street = ${}", param_idx));
        }
        if data.city.is_some() {
            param_idx += 1;
            updates.push(format!("city = ${}", param_idx));
        }
        if data.postal_code.is_some() {
            param_idx += 1;
            updates.push(format!("postal_code = ${}", param_idx));
        }
        if data.country.is_some() {
            param_idx += 1;
            updates.push(format!("country = ${}", param_idx));
        }
        if data.name.is_some() {
            param_idx += 1;
            updates.push(format!("name = ${}", param_idx));
        }
        if data.description.is_some() {
            param_idx += 1;
            updates.push(format!("description = ${}", param_idx));
        }
        if data.year_built.is_some() {
            param_idx += 1;
            updates.push(format!("year_built = ${}", param_idx));
        }
        if data.total_floors.is_some() {
            param_idx += 1;
            updates.push(format!("total_floors = ${}", param_idx));
        }
        if data.total_entrances.is_some() {
            param_idx += 1;
            updates.push(format!("total_entrances = ${}", param_idx));
        }
        if data.amenities.is_some() {
            param_idx += 1;
            updates.push(format!("amenities = ${}", param_idx));
        }
        if data.contacts.is_some() {
            param_idx += 1;
            updates.push(format!("contacts = ${}", param_idx));
        }
        if data.settings.is_some() {
            param_idx += 1;
            updates.push(format!("settings = ${}", param_idx));
        }

        let query = format!(
            "UPDATE buildings SET {} WHERE id = $1 AND status != 'archived' RETURNING *",
            updates.join(", ")
        );

        let mut q = sqlx::query_as::<_, Building>(&query).bind(id);

        if let Some(street) = &data.street {
            q = q.bind(street);
        }
        if let Some(city) = &data.city {
            q = q.bind(city);
        }
        if let Some(postal_code) = &data.postal_code {
            q = q.bind(postal_code);
        }
        if let Some(country) = &data.country {
            q = q.bind(country);
        }
        if let Some(name) = &data.name {
            q = q.bind(name);
        }
        if let Some(description) = &data.description {
            q = q.bind(description);
        }
        if let Some(year_built) = &data.year_built {
            q = q.bind(year_built);
        }
        if let Some(total_floors) = &data.total_floors {
            q = q.bind(total_floors);
        }
        if let Some(total_entrances) = &data.total_entrances {
            q = q.bind(total_entrances);
        }
        if let Some(amenities) = &data.amenities {
            let amenities_json = serde_json::to_value(amenities).unwrap_or_default();
            q = q.bind(amenities_json);
        }
        if let Some(contacts) = &data.contacts {
            q = q.bind(contacts);
        }
        if let Some(settings) = &data.settings {
            q = q.bind(settings);
        }

        let building = q.fetch_optional(&self.pool).await?;
        Ok(building)
    }

    /// Archive building (soft delete) (UC-15.10).
    pub async fn archive(&self, id: Uuid) -> Result<Option<Building>, SqlxError> {
        let building = sqlx::query_as::<_, Building>(
            r#"
            UPDATE buildings
            SET status = 'archived', updated_at = NOW()
            WHERE id = $1 AND status = 'active'
            RETURNING *
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(building)
    }

    /// Restore archived building.
    pub async fn restore(&self, id: Uuid) -> Result<Option<Building>, SqlxError> {
        let building = sqlx::query_as::<_, Building>(
            r#"
            UPDATE buildings
            SET status = 'active', updated_at = NOW()
            WHERE id = $1 AND status = 'archived'
            RETURNING *
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(building)
    }

    /// List buildings for an organization.
    pub async fn list_by_organization(
        &self,
        organization_id: Uuid,
        offset: i64,
        limit: i64,
        include_archived: bool,
        search: Option<&str>,
    ) -> Result<(Vec<BuildingSummary>, i64), SqlxError> {
        let mut conditions = vec!["organization_id = $1".to_string()];

        if !include_archived {
            conditions.push("status = 'active'".to_string());
        }

        if search.is_some() {
            conditions.push(
                "(LOWER(street) LIKE '%' || LOWER($4::text) || '%' OR \
                 LOWER(city) LIKE '%' || LOWER($4::text) || '%' OR \
                 LOWER(name) LIKE '%' || LOWER($4::text) || '%')"
                    .to_string(),
            );
        }

        let where_clause = conditions.join(" AND ");

        let count_query = format!("SELECT COUNT(*) FROM buildings WHERE {}", where_clause);
        let data_query = format!(
            r#"
            SELECT b.id, b.name, b.street, b.city, b.postal_code, b.total_floors, b.status,
                   (SELECT COUNT(*) FROM units u WHERE u.building_id = b.id AND u.status = 'active') as unit_count
            FROM buildings b
            WHERE {}
            ORDER BY b.created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            where_clause
        );

        // Execute count query
        let mut count_q = sqlx::query_scalar::<_, i64>(&count_query).bind(organization_id);
        if let Some(s) = search {
            count_q = count_q.bind(s);
        }
        let total = count_q.fetch_one(&self.pool).await?;

        // Execute data query
        let mut data_q = sqlx::query_as::<_, BuildingSummary>(&data_query)
            .bind(organization_id)
            .bind(limit)
            .bind(offset);
        if let Some(s) = search {
            data_q = data_q.bind(s);
        }
        let buildings = data_q.fetch_all(&self.pool).await?;

        Ok((buildings, total))
    }

    /// Get building statistics (UC-15.7).
    pub async fn get_statistics(&self, building_id: Uuid) -> Result<BuildingStatistics, SqlxError> {
        let stats = sqlx::query_as::<_, BuildingStatisticsRow>(
            r#"
            SELECT
                $1::uuid as building_id,
                COUNT(DISTINCT u.id) FILTER (WHERE u.status = 'active') as total_units,
                COUNT(DISTINCT u.id) FILTER (WHERE u.status = 'active' AND u.occupancy_status IN ('owner_occupied', 'rented')) as occupied_units,
                COUNT(DISTINCT u.id) FILTER (WHERE u.status = 'active' AND u.occupancy_status = 'vacant') as vacant_units,
                COUNT(DISTINCT uo.user_id) FILTER (WHERE uo.status = 'active') as total_owners
            FROM units u
            LEFT JOIN unit_owners uo ON uo.unit_id = u.id
            WHERE u.building_id = $1
            "#,
        )
        .bind(building_id)
        .fetch_one(&self.pool)
        .await?;

        let ownership_coverage = if stats.total_units > 0 {
            // Count units with at least one active owner
            let units_with_owners: i64 = sqlx::query_scalar(
                r#"
                SELECT COUNT(DISTINCT u.id)
                FROM units u
                INNER JOIN unit_owners uo ON uo.unit_id = u.id AND uo.status = 'active'
                WHERE u.building_id = $1 AND u.status = 'active'
                "#,
            )
            .bind(building_id)
            .fetch_one(&self.pool)
            .await?;

            (units_with_owners as f64 / stats.total_units as f64) * 100.0
        } else {
            0.0
        };

        Ok(BuildingStatistics {
            building_id: stats.building_id,
            total_units: stats.total_units,
            occupied_units: stats.occupied_units,
            vacant_units: stats.vacant_units,
            total_owners: stats.total_owners,
            ownership_coverage,
        })
    }

    /// Check if building belongs to organization.
    pub async fn belongs_to_organization(
        &self,
        building_id: Uuid,
        organization_id: Uuid,
    ) -> Result<bool, SqlxError> {
        let count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM buildings
            WHERE id = $1 AND organization_id = $2
            "#,
        )
        .bind(building_id)
        .bind(organization_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count > 0)
    }
}

/// Internal struct for fetching building statistics.
#[derive(sqlx::FromRow)]
struct BuildingStatisticsRow {
    building_id: Uuid,
    total_units: i64,
    occupied_units: i64,
    vacant_units: i64,
    total_owners: i64,
}

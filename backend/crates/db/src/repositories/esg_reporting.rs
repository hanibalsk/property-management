// Epic 136: ESG Reporting Dashboard
// Repository for ESG (Environmental, Social, Governance) reporting and compliance

use chrono::{Datelike, Utc};
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::esg_reporting::*;
use crate::DbPool;

/// Repository for ESG reporting operations.
#[derive(Clone)]
pub struct EsgReportingRepository {
    pool: DbPool,
}

impl EsgReportingRepository {
    /// Create a new repository instance.
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Get the underlying pool reference.
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    // =========================================================================
    // ESG Configuration
    // =========================================================================

    /// Get ESG configuration for an organization.
    pub async fn get_configuration(
        &self,
        organization_id: Uuid,
    ) -> Result<Option<EsgConfiguration>, sqlx::Error> {
        sqlx::query_as::<_, EsgConfiguration>(
            r#"
            SELECT id, organization_id, reporting_currency, default_unit_system,
                   fiscal_year_start_month, enabled_frameworks, grid_emission_factor,
                   natural_gas_emission_factor, carbon_reduction_target_pct,
                   target_year, baseline_year, created_at, updated_at
            FROM esg_configurations
            WHERE organization_id = $1
            "#,
        )
        .bind(organization_id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Create or update ESG configuration.
    pub async fn upsert_configuration(
        &self,
        organization_id: Uuid,
        input: CreateEsgConfiguration,
    ) -> Result<EsgConfiguration, sqlx::Error> {
        sqlx::query_as::<_, EsgConfiguration>(
            r#"
            INSERT INTO esg_configurations (
                id, organization_id, reporting_currency, default_unit_system,
                fiscal_year_start_month, enabled_frameworks, grid_emission_factor,
                natural_gas_emission_factor, carbon_reduction_target_pct,
                target_year, baseline_year, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, NOW(), NOW())
            ON CONFLICT (organization_id) DO UPDATE SET
                reporting_currency = COALESCE(EXCLUDED.reporting_currency, esg_configurations.reporting_currency),
                default_unit_system = COALESCE(EXCLUDED.default_unit_system, esg_configurations.default_unit_system),
                fiscal_year_start_month = COALESCE(EXCLUDED.fiscal_year_start_month, esg_configurations.fiscal_year_start_month),
                enabled_frameworks = COALESCE(EXCLUDED.enabled_frameworks, esg_configurations.enabled_frameworks),
                grid_emission_factor = COALESCE(EXCLUDED.grid_emission_factor, esg_configurations.grid_emission_factor),
                natural_gas_emission_factor = COALESCE(EXCLUDED.natural_gas_emission_factor, esg_configurations.natural_gas_emission_factor),
                carbon_reduction_target_pct = COALESCE(EXCLUDED.carbon_reduction_target_pct, esg_configurations.carbon_reduction_target_pct),
                target_year = COALESCE(EXCLUDED.target_year, esg_configurations.target_year),
                baseline_year = COALESCE(EXCLUDED.baseline_year, esg_configurations.baseline_year),
                updated_at = NOW()
            RETURNING id, organization_id, reporting_currency, default_unit_system,
                      fiscal_year_start_month, enabled_frameworks, grid_emission_factor,
                      natural_gas_emission_factor, carbon_reduction_target_pct,
                      target_year, baseline_year, created_at, updated_at
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(organization_id)
        .bind(input.reporting_currency.unwrap_or_else(|| "EUR".to_string()))
        .bind(input.default_unit_system.unwrap_or_else(|| "metric".to_string()))
        .bind(input.fiscal_year_start_month.unwrap_or(1))
        .bind(serde_json::to_value(input.enabled_frameworks.unwrap_or_default()).unwrap_or(serde_json::Value::Array(vec![])))
        .bind(input.grid_emission_factor)
        .bind(input.natural_gas_emission_factor)
        .bind(input.carbon_reduction_target_pct)
        .bind(input.target_year)
        .bind(input.baseline_year)
        .fetch_one(&self.pool)
        .await
    }

    // =========================================================================
    // ESG Metrics
    // =========================================================================

    /// Create an ESG metric.
    pub async fn create_metric(
        &self,
        organization_id: Uuid,
        user_id: Uuid,
        input: CreateEsgMetric,
    ) -> Result<EsgMetric, sqlx::Error> {
        sqlx::query_as::<_, EsgMetric>(
            r#"
            INSERT INTO esg_metrics (
                id, organization_id, building_id, period_start, period_end,
                category, metric_type, metric_name, value, unit, normalized_value,
                data_source, confidence_level, notes, created_at, updated_at, created_by
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, NOW(), NOW(), $15)
            RETURNING id, organization_id, building_id, period_start, period_end,
                      category, metric_type, metric_name, value, unit, normalized_value,
                      data_source, confidence_level, verification_status, verified_by,
                      verified_at, notes, supporting_documents, created_at, updated_at, created_by
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(organization_id)
        .bind(input.building_id)
        .bind(input.period_start)
        .bind(input.period_end)
        .bind(input.category)
        .bind(&input.metric_type)
        .bind(&input.metric_name)
        .bind(input.value)
        .bind(&input.unit)
        .bind(input.normalized_value)
        .bind(input.data_source)
        .bind(input.confidence_level)
        .bind(input.notes.as_deref())
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
    }

    /// Get ESG metric by ID.
    pub async fn get_metric(&self, id: Uuid) -> Result<Option<EsgMetric>, sqlx::Error> {
        sqlx::query_as::<_, EsgMetric>(
            r#"
            SELECT id, organization_id, building_id, period_start, period_end,
                   category, metric_type, metric_name, value, unit, normalized_value,
                   data_source, confidence_level, verification_status, verified_by,
                   verified_at, notes, supporting_documents, created_at, updated_at, created_by
            FROM esg_metrics
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    /// List ESG metrics with filters.
    pub async fn list_metrics(
        &self,
        organization_id: Uuid,
        query: EsgMetricsQuery,
    ) -> Result<Vec<EsgMetric>, sqlx::Error> {
        let limit = query.limit.unwrap_or(50).min(100);
        let offset = query.offset.unwrap_or(0);

        sqlx::query_as::<_, EsgMetric>(
            r#"
            SELECT id, organization_id, building_id, period_start, period_end,
                   category, metric_type, metric_name, value, unit, normalized_value,
                   data_source, confidence_level, verification_status, verified_by,
                   verified_at, notes, supporting_documents, created_at, updated_at, created_by
            FROM esg_metrics
            WHERE organization_id = $1
              AND ($2::uuid IS NULL OR building_id = $2)
              AND ($3::esg_metric_category IS NULL OR category = $3)
              AND ($4::text IS NULL OR metric_type = $4)
              AND ($5::date IS NULL OR period_start >= $5)
              AND ($6::date IS NULL OR period_end <= $6)
            ORDER BY period_end DESC, created_at DESC
            LIMIT $7 OFFSET $8
            "#,
        )
        .bind(organization_id)
        .bind(query.building_id)
        .bind(query.category)
        .bind(query.metric_type.as_deref())
        .bind(query.period_start)
        .bind(query.period_end)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
    }

    /// Update an ESG metric.
    pub async fn update_metric(
        &self,
        id: Uuid,
        input: UpdateEsgMetric,
    ) -> Result<Option<EsgMetric>, sqlx::Error> {
        sqlx::query_as::<_, EsgMetric>(
            r#"
            UPDATE esg_metrics
            SET value = COALESCE($2, value),
                unit = COALESCE($3, unit),
                normalized_value = COALESCE($4, normalized_value),
                confidence_level = COALESCE($5, confidence_level),
                notes = COALESCE($6, notes),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, organization_id, building_id, period_start, period_end,
                      category, metric_type, metric_name, value, unit, normalized_value,
                      data_source, confidence_level, verification_status, verified_by,
                      verified_at, notes, supporting_documents, created_at, updated_at, created_by
            "#,
        )
        .bind(id)
        .bind(input.value)
        .bind(input.unit.as_deref())
        .bind(input.normalized_value)
        .bind(input.confidence_level)
        .bind(input.notes.as_deref())
        .fetch_optional(&self.pool)
        .await
    }

    /// Verify an ESG metric.
    pub async fn verify_metric(
        &self,
        id: Uuid,
        verifier_id: Uuid,
        status: &str,
    ) -> Result<Option<EsgMetric>, sqlx::Error> {
        sqlx::query_as::<_, EsgMetric>(
            r#"
            UPDATE esg_metrics
            SET verification_status = $2,
                verified_by = $3,
                verified_at = NOW(),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, organization_id, building_id, period_start, period_end,
                      category, metric_type, metric_name, value, unit, normalized_value,
                      data_source, confidence_level, verification_status, verified_by,
                      verified_at, notes, supporting_documents, created_at, updated_at, created_by
            "#,
        )
        .bind(id)
        .bind(status)
        .bind(verifier_id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Delete an ESG metric.
    pub async fn delete_metric(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM esg_metrics WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    // =========================================================================
    // Carbon Footprint
    // =========================================================================

    /// Create a carbon footprint record.
    pub async fn create_carbon_footprint(
        &self,
        organization_id: Uuid,
        input: CreateCarbonFootprint,
    ) -> Result<CarbonFootprint, sqlx::Error> {
        // Calculate CO2 equivalent
        let co2_equivalent_kg = input.consumption_value * input.emission_factor;
        let co2_per_sqm = input
            .area_sqm
            .as_ref()
            .filter(|a| *a > &Decimal::ZERO)
            .map(|a| co2_equivalent_kg / *a);
        let co2_per_unit = input
            .num_units
            .filter(|n| *n > 0)
            .map(|n| co2_equivalent_kg / Decimal::from(n));

        sqlx::query_as::<_, CarbonFootprint>(
            r#"
            INSERT INTO carbon_footprints (
                id, organization_id, building_id, year, month, source_type,
                energy_source, consumption_value, consumption_unit, emission_factor,
                co2_equivalent_kg, area_sqm, co2_per_sqm, num_units, co2_per_unit,
                created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, NOW(), NOW())
            RETURNING id, organization_id, building_id, year, month, source_type,
                      energy_source, consumption_value, consumption_unit, emission_factor,
                      co2_equivalent_kg, area_sqm, co2_per_sqm, num_units, co2_per_unit,
                      calculation_methodology, created_at, updated_at
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(organization_id)
        .bind(input.building_id)
        .bind(input.year)
        .bind(input.month)
        .bind(input.source_type)
        .bind(input.energy_source)
        .bind(input.consumption_value)
        .bind(&input.consumption_unit)
        .bind(input.emission_factor)
        .bind(co2_equivalent_kg)
        .bind(input.area_sqm)
        .bind(co2_per_sqm)
        .bind(input.num_units)
        .bind(co2_per_unit)
        .fetch_one(&self.pool)
        .await
    }

    /// Get carbon footprint by ID.
    pub async fn get_carbon_footprint(
        &self,
        id: Uuid,
    ) -> Result<Option<CarbonFootprint>, sqlx::Error> {
        sqlx::query_as::<_, CarbonFootprint>(
            r#"
            SELECT id, organization_id, building_id, year, month, source_type,
                   energy_source, consumption_value, consumption_unit, emission_factor,
                   co2_equivalent_kg, area_sqm, co2_per_sqm, num_units, co2_per_unit,
                   calculation_methodology, created_at, updated_at
            FROM carbon_footprints
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    /// List carbon footprints with filters.
    pub async fn list_carbon_footprints(
        &self,
        organization_id: Uuid,
        query: CarbonFootprintQuery,
    ) -> Result<Vec<CarbonFootprint>, sqlx::Error> {
        sqlx::query_as::<_, CarbonFootprint>(
            r#"
            SELECT id, organization_id, building_id, year, month, source_type,
                   energy_source, consumption_value, consumption_unit, emission_factor,
                   co2_equivalent_kg, area_sqm, co2_per_sqm, num_units, co2_per_unit,
                   calculation_methodology, created_at, updated_at
            FROM carbon_footprints
            WHERE organization_id = $1
              AND ($2::uuid IS NULL OR building_id = $2)
              AND ($3::int IS NULL OR year = $3)
              AND ($4::esg_emission_scope IS NULL OR source_type = $4)
            ORDER BY year DESC, month DESC NULLS LAST, created_at DESC
            LIMIT 100
            "#,
        )
        .bind(organization_id)
        .bind(query.building_id)
        .bind(query.year)
        .bind(query.source_type)
        .fetch_all(&self.pool)
        .await
    }

    /// Get carbon footprint summary by year.
    pub async fn get_carbon_summary(
        &self,
        organization_id: Uuid,
        year: i32,
    ) -> Result<Option<CarbonFootprintSummary>, sqlx::Error> {
        sqlx::query_as::<_, CarbonFootprintSummary>(
            r#"
            SELECT
                year,
                COALESCE(SUM(co2_equivalent_kg), 0) as total_co2_kg,
                SUM(CASE WHEN source_type = 'scope_1_direct' THEN co2_equivalent_kg ELSE 0 END) as scope_1_kg,
                SUM(CASE WHEN source_type = 'scope_2_indirect' THEN co2_equivalent_kg ELSE 0 END) as scope_2_kg,
                SUM(CASE WHEN source_type = 'scope_3_value_chain' THEN co2_equivalent_kg ELSE 0 END) as scope_3_kg,
                CASE
                    WHEN SUM(area_sqm) > 0 THEN SUM(co2_equivalent_kg) / SUM(area_sqm)
                    ELSE NULL
                END as avg_co2_per_sqm
            FROM carbon_footprints
            WHERE organization_id = $1 AND year = $2
            GROUP BY year
            "#,
        )
        .bind(organization_id)
        .bind(year)
        .fetch_optional(&self.pool)
        .await
    }

    /// Delete a carbon footprint record.
    pub async fn delete_carbon_footprint(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM carbon_footprints WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    // =========================================================================
    // ESG Benchmarks
    // =========================================================================

    /// Create an ESG benchmark.
    pub async fn create_benchmark(
        &self,
        organization_id: Uuid,
        input: CreateEsgBenchmark,
    ) -> Result<EsgBenchmark, sqlx::Error> {
        sqlx::query_as::<_, EsgBenchmark>(
            r#"
            INSERT INTO esg_benchmarks (
                id, organization_id, name, category, metric_type, benchmark_value,
                unit, region, property_type, source, effective_date, expiry_date, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, NOW())
            RETURNING id, organization_id, name, category, metric_type, benchmark_value,
                      unit, region, property_type, source, effective_date, expiry_date, created_at
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(organization_id)
        .bind(&input.name)
        .bind(input.category)
        .bind(&input.metric_type)
        .bind(input.benchmark_value)
        .bind(&input.unit)
        .bind(input.region.as_deref())
        .bind(input.property_type.as_deref())
        .bind(input.source.as_deref())
        .bind(input.effective_date)
        .bind(input.expiry_date)
        .fetch_one(&self.pool)
        .await
    }

    /// List ESG benchmarks.
    pub async fn list_benchmarks(
        &self,
        organization_id: Uuid,
        category: Option<EsgBenchmarkCategory>,
    ) -> Result<Vec<EsgBenchmark>, sqlx::Error> {
        sqlx::query_as::<_, EsgBenchmark>(
            r#"
            SELECT id, organization_id, name, category, metric_type, benchmark_value,
                   unit, region, property_type, source, effective_date, expiry_date, created_at
            FROM esg_benchmarks
            WHERE organization_id = $1
              AND ($2::esg_benchmark_category IS NULL OR category = $2)
              AND (expiry_date IS NULL OR expiry_date >= CURRENT_DATE)
            ORDER BY name ASC
            "#,
        )
        .bind(organization_id)
        .bind(category)
        .fetch_all(&self.pool)
        .await
    }

    /// Delete an ESG benchmark.
    pub async fn delete_benchmark(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM esg_benchmarks WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    // =========================================================================
    // ESG Targets
    // =========================================================================

    /// Create an ESG target.
    pub async fn create_target(
        &self,
        organization_id: Uuid,
        input: CreateEsgTarget,
    ) -> Result<EsgTarget, sqlx::Error> {
        sqlx::query_as::<_, EsgTarget>(
            r#"
            INSERT INTO esg_targets (
                id, organization_id, building_id, name, category, metric_type,
                target_value, unit, target_date, baseline_value, baseline_date,
                created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, NOW(), NOW())
            RETURNING id, organization_id, building_id, name, category, metric_type,
                      target_value, unit, target_date, baseline_value, baseline_date,
                      current_value, progress_pct, status, created_at, updated_at
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(organization_id)
        .bind(input.building_id)
        .bind(&input.name)
        .bind(input.category)
        .bind(&input.metric_type)
        .bind(input.target_value)
        .bind(&input.unit)
        .bind(input.target_date)
        .bind(input.baseline_value)
        .bind(input.baseline_date)
        .fetch_one(&self.pool)
        .await
    }

    /// Get ESG target by ID.
    pub async fn get_target(&self, id: Uuid) -> Result<Option<EsgTarget>, sqlx::Error> {
        sqlx::query_as::<_, EsgTarget>(
            r#"
            SELECT id, organization_id, building_id, name, category, metric_type,
                   target_value, unit, target_date, baseline_value, baseline_date,
                   current_value, progress_pct, status, created_at, updated_at
            FROM esg_targets
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    /// List ESG targets.
    pub async fn list_targets(
        &self,
        organization_id: Uuid,
        building_id: Option<Uuid>,
    ) -> Result<Vec<EsgTarget>, sqlx::Error> {
        sqlx::query_as::<_, EsgTarget>(
            r#"
            SELECT id, organization_id, building_id, name, category, metric_type,
                   target_value, unit, target_date, baseline_value, baseline_date,
                   current_value, progress_pct, status, created_at, updated_at
            FROM esg_targets
            WHERE organization_id = $1
              AND ($2::uuid IS NULL OR building_id = $2)
            ORDER BY target_date ASC
            "#,
        )
        .bind(organization_id)
        .bind(building_id)
        .fetch_all(&self.pool)
        .await
    }

    /// Update an ESG target.
    pub async fn update_target(
        &self,
        id: Uuid,
        input: UpdateEsgTarget,
    ) -> Result<Option<EsgTarget>, sqlx::Error> {
        // Calculate progress if current_value and baseline are available
        sqlx::query_as::<_, EsgTarget>(
            r#"
            UPDATE esg_targets
            SET target_value = COALESCE($2, target_value),
                target_date = COALESCE($3, target_date),
                current_value = COALESCE($4, current_value),
                progress_pct = CASE
                    WHEN baseline_value IS NOT NULL AND COALESCE($4, current_value) IS NOT NULL
                         AND baseline_value != target_value
                    THEN ((baseline_value - COALESCE($4, current_value)) / (baseline_value - target_value)) * 100
                    ELSE progress_pct
                END,
                status = COALESCE($5, status),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, organization_id, building_id, name, category, metric_type,
                      target_value, unit, target_date, baseline_value, baseline_date,
                      current_value, progress_pct, status, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(input.target_value)
        .bind(input.target_date)
        .bind(input.current_value)
        .bind(input.status.as_deref())
        .fetch_optional(&self.pool)
        .await
    }

    /// Delete an ESG target.
    pub async fn delete_target(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM esg_targets WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    // =========================================================================
    // ESG Reports
    // =========================================================================

    /// Create an ESG report.
    pub async fn create_report(
        &self,
        organization_id: Uuid,
        user_id: Uuid,
        input: CreateEsgReport,
    ) -> Result<EsgReport, sqlx::Error> {
        sqlx::query_as::<_, EsgReport>(
            r#"
            INSERT INTO esg_reports (
                id, organization_id, report_type, title, description,
                period_start, period_end, frameworks, status, created_at, updated_at, created_by
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'draft', NOW(), NOW(), $9)
            RETURNING id, organization_id, report_type, title, description,
                      period_start, period_end, frameworks, status, submitted_at,
                      approved_by, approved_at, report_data, summary_scores,
                      pdf_url, xml_url, created_at, updated_at, created_by
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(organization_id)
        .bind(&input.report_type)
        .bind(&input.title)
        .bind(input.description.as_deref())
        .bind(input.period_start)
        .bind(input.period_end)
        .bind(serde_json::to_value(&input.frameworks).unwrap_or(serde_json::Value::Array(vec![])))
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
    }

    /// Get ESG report by ID.
    pub async fn get_report(&self, id: Uuid) -> Result<Option<EsgReport>, sqlx::Error> {
        sqlx::query_as::<_, EsgReport>(
            r#"
            SELECT id, organization_id, report_type, title, description,
                   period_start, period_end, frameworks, status, submitted_at,
                   approved_by, approved_at, report_data, summary_scores,
                   pdf_url, xml_url, created_at, updated_at, created_by
            FROM esg_reports
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    /// List ESG reports.
    pub async fn list_reports(
        &self,
        organization_id: Uuid,
        status: Option<EsgReportStatus>,
    ) -> Result<Vec<EsgReport>, sqlx::Error> {
        sqlx::query_as::<_, EsgReport>(
            r#"
            SELECT id, organization_id, report_type, title, description,
                   period_start, period_end, frameworks, status, submitted_at,
                   approved_by, approved_at, report_data, summary_scores,
                   pdf_url, xml_url, created_at, updated_at, created_by
            FROM esg_reports
            WHERE organization_id = $1
              AND ($2::esg_report_status IS NULL OR status = $2)
            ORDER BY created_at DESC
            LIMIT 50
            "#,
        )
        .bind(organization_id)
        .bind(status)
        .fetch_all(&self.pool)
        .await
    }

    /// Update an ESG report.
    pub async fn update_report(
        &self,
        id: Uuid,
        input: UpdateEsgReport,
    ) -> Result<Option<EsgReport>, sqlx::Error> {
        sqlx::query_as::<_, EsgReport>(
            r#"
            UPDATE esg_reports
            SET title = COALESCE($2, title),
                description = COALESCE($3, description),
                status = COALESCE($4, status),
                report_data = COALESCE($5, report_data),
                summary_scores = COALESCE($6, summary_scores),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, organization_id, report_type, title, description,
                      period_start, period_end, frameworks, status, submitted_at,
                      approved_by, approved_at, report_data, summary_scores,
                      pdf_url, xml_url, created_at, updated_at, created_by
            "#,
        )
        .bind(id)
        .bind(input.title.as_deref())
        .bind(input.description.as_deref())
        .bind(input.status)
        .bind(&input.report_data)
        .bind(&input.summary_scores)
        .fetch_optional(&self.pool)
        .await
    }

    /// Submit an ESG report for review.
    pub async fn submit_report(&self, id: Uuid) -> Result<Option<EsgReport>, sqlx::Error> {
        sqlx::query_as::<_, EsgReport>(
            r#"
            UPDATE esg_reports
            SET status = 'pending_review',
                submitted_at = NOW(),
                updated_at = NOW()
            WHERE id = $1 AND status = 'draft'
            RETURNING id, organization_id, report_type, title, description,
                      period_start, period_end, frameworks, status, submitted_at,
                      approved_by, approved_at, report_data, summary_scores,
                      pdf_url, xml_url, created_at, updated_at, created_by
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Approve an ESG report.
    pub async fn approve_report(
        &self,
        id: Uuid,
        approver_id: Uuid,
    ) -> Result<Option<EsgReport>, sqlx::Error> {
        sqlx::query_as::<_, EsgReport>(
            r#"
            UPDATE esg_reports
            SET status = 'approved',
                approved_by = $2,
                approved_at = NOW(),
                updated_at = NOW()
            WHERE id = $1 AND status = 'pending_review'
            RETURNING id, organization_id, report_type, title, description,
                      period_start, period_end, frameworks, status, submitted_at,
                      approved_by, approved_at, report_data, summary_scores,
                      pdf_url, xml_url, created_at, updated_at, created_by
            "#,
        )
        .bind(id)
        .bind(approver_id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Delete an ESG report.
    pub async fn delete_report(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM esg_reports WHERE id = $1 AND status = 'draft'")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    // =========================================================================
    // EU Taxonomy Assessment
    // =========================================================================

    /// Create an EU Taxonomy assessment.
    pub async fn create_eu_taxonomy_assessment(
        &self,
        organization_id: Uuid,
        input: CreateEuTaxonomyAssessment,
    ) -> Result<EuTaxonomyAssessment, sqlx::Error> {
        sqlx::query_as::<_, EuTaxonomyAssessment>(
            r#"
            INSERT INTO eu_taxonomy_assessments (
                id, organization_id, building_id, year, climate_mitigation_eligible,
                climate_mitigation_aligned, climate_adaptation_eligible, climate_adaptation_aligned,
                energy_performance_class, primary_energy_demand, meets_nzeb_standard,
                notes, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, NOW(), NOW())
            RETURNING id, organization_id, building_id, year, climate_mitigation_eligible,
                      climate_mitigation_aligned, climate_mitigation_revenue_pct,
                      climate_adaptation_eligible, climate_adaptation_aligned,
                      climate_adaptation_revenue_pct, energy_performance_class,
                      primary_energy_demand, meets_nzeb_standard, dnsh_water,
                      dnsh_circular_economy, dnsh_pollution, dnsh_biodiversity,
                      oecd_guidelines_compliance, un_guiding_principles,
                      overall_alignment_pct, notes, created_at, updated_at
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(organization_id)
        .bind(input.building_id)
        .bind(input.year)
        .bind(input.climate_mitigation_eligible)
        .bind(input.climate_mitigation_aligned)
        .bind(input.climate_adaptation_eligible)
        .bind(input.climate_adaptation_aligned)
        .bind(input.energy_performance_class.as_deref())
        .bind(input.primary_energy_demand)
        .bind(input.meets_nzeb_standard)
        .bind(input.notes.as_deref())
        .fetch_one(&self.pool)
        .await
    }

    /// Get EU Taxonomy assessment by ID.
    pub async fn get_eu_taxonomy_assessment(
        &self,
        id: Uuid,
    ) -> Result<Option<EuTaxonomyAssessment>, sqlx::Error> {
        sqlx::query_as::<_, EuTaxonomyAssessment>(
            r#"
            SELECT id, organization_id, building_id, year, climate_mitigation_eligible,
                   climate_mitigation_aligned, climate_mitigation_revenue_pct,
                   climate_adaptation_eligible, climate_adaptation_aligned,
                   climate_adaptation_revenue_pct, energy_performance_class,
                   primary_energy_demand, meets_nzeb_standard, dnsh_water,
                   dnsh_circular_economy, dnsh_pollution, dnsh_biodiversity,
                   oecd_guidelines_compliance, un_guiding_principles,
                   overall_alignment_pct, notes, created_at, updated_at
            FROM eu_taxonomy_assessments
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    /// List EU Taxonomy assessments.
    pub async fn list_eu_taxonomy_assessments(
        &self,
        organization_id: Uuid,
        year: Option<i32>,
    ) -> Result<Vec<EuTaxonomyAssessment>, sqlx::Error> {
        sqlx::query_as::<_, EuTaxonomyAssessment>(
            r#"
            SELECT id, organization_id, building_id, year, climate_mitigation_eligible,
                   climate_mitigation_aligned, climate_mitigation_revenue_pct,
                   climate_adaptation_eligible, climate_adaptation_aligned,
                   climate_adaptation_revenue_pct, energy_performance_class,
                   primary_energy_demand, meets_nzeb_standard, dnsh_water,
                   dnsh_circular_economy, dnsh_pollution, dnsh_biodiversity,
                   oecd_guidelines_compliance, un_guiding_principles,
                   overall_alignment_pct, notes, created_at, updated_at
            FROM eu_taxonomy_assessments
            WHERE organization_id = $1
              AND ($2::int IS NULL OR year = $2)
            ORDER BY year DESC, created_at DESC
            "#,
        )
        .bind(organization_id)
        .bind(year)
        .fetch_all(&self.pool)
        .await
    }

    /// Update EU Taxonomy assessment.
    pub async fn update_eu_taxonomy_assessment(
        &self,
        id: Uuid,
        input: UpdateEuTaxonomyAssessment,
    ) -> Result<Option<EuTaxonomyAssessment>, sqlx::Error> {
        // Calculate overall alignment based on DNSH criteria
        sqlx::query_as::<_, EuTaxonomyAssessment>(
            r#"
            UPDATE eu_taxonomy_assessments
            SET climate_mitigation_eligible = COALESCE($2, climate_mitigation_eligible),
                climate_mitigation_aligned = COALESCE($3, climate_mitigation_aligned),
                climate_adaptation_eligible = COALESCE($4, climate_adaptation_eligible),
                climate_adaptation_aligned = COALESCE($5, climate_adaptation_aligned),
                energy_performance_class = COALESCE($6, energy_performance_class),
                primary_energy_demand = COALESCE($7, primary_energy_demand),
                meets_nzeb_standard = COALESCE($8, meets_nzeb_standard),
                dnsh_water = COALESCE($9, dnsh_water),
                dnsh_circular_economy = COALESCE($10, dnsh_circular_economy),
                dnsh_pollution = COALESCE($11, dnsh_pollution),
                dnsh_biodiversity = COALESCE($12, dnsh_biodiversity),
                oecd_guidelines_compliance = COALESCE($13, oecd_guidelines_compliance),
                un_guiding_principles = COALESCE($14, un_guiding_principles),
                notes = COALESCE($15, notes),
                overall_alignment_pct = CASE
                    WHEN COALESCE($3, climate_mitigation_aligned) = true
                         AND COALESCE($9, dnsh_water) = true
                         AND COALESCE($10, dnsh_circular_economy) = true
                         AND COALESCE($11, dnsh_pollution) = true
                         AND COALESCE($12, dnsh_biodiversity) = true
                         AND COALESCE($13, oecd_guidelines_compliance) = true
                         AND COALESCE($14, un_guiding_principles) = true
                    THEN 100.0
                    ELSE (
                        (CASE WHEN COALESCE($3, climate_mitigation_aligned) THEN 1 ELSE 0 END +
                         CASE WHEN COALESCE($9, dnsh_water) THEN 1 ELSE 0 END +
                         CASE WHEN COALESCE($10, dnsh_circular_economy) THEN 1 ELSE 0 END +
                         CASE WHEN COALESCE($11, dnsh_pollution) THEN 1 ELSE 0 END +
                         CASE WHEN COALESCE($12, dnsh_biodiversity) THEN 1 ELSE 0 END +
                         CASE WHEN COALESCE($13, oecd_guidelines_compliance) THEN 1 ELSE 0 END +
                         CASE WHEN COALESCE($14, un_guiding_principles) THEN 1 ELSE 0 END
                        )::decimal / 7.0 * 100.0
                    )
                END,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, organization_id, building_id, year, climate_mitigation_eligible,
                      climate_mitigation_aligned, climate_mitigation_revenue_pct,
                      climate_adaptation_eligible, climate_adaptation_aligned,
                      climate_adaptation_revenue_pct, energy_performance_class,
                      primary_energy_demand, meets_nzeb_standard, dnsh_water,
                      dnsh_circular_economy, dnsh_pollution, dnsh_biodiversity,
                      oecd_guidelines_compliance, un_guiding_principles,
                      overall_alignment_pct, notes, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(input.climate_mitigation_eligible)
        .bind(input.climate_mitigation_aligned)
        .bind(input.climate_adaptation_eligible)
        .bind(input.climate_adaptation_aligned)
        .bind(input.energy_performance_class.as_deref())
        .bind(input.primary_energy_demand)
        .bind(input.meets_nzeb_standard)
        .bind(input.dnsh_water)
        .bind(input.dnsh_circular_economy)
        .bind(input.dnsh_pollution)
        .bind(input.dnsh_biodiversity)
        .bind(input.oecd_guidelines_compliance)
        .bind(input.un_guiding_principles)
        .bind(input.notes.as_deref())
        .fetch_optional(&self.pool)
        .await
    }

    // =========================================================================
    // ESG Dashboard
    // =========================================================================

    /// Get ESG dashboard metrics.
    pub async fn get_dashboard_metrics(
        &self,
        organization_id: Uuid,
        year: i32,
        building_id: Option<Uuid>,
    ) -> Result<Option<EsgDashboardMetrics>, sqlx::Error> {
        sqlx::query_as::<_, EsgDashboardMetrics>(
            r#"
            SELECT id, organization_id, building_id, year, month,
                   environmental_score, social_score, governance_score, overall_esg_score,
                   total_co2_kg, co2_per_sqm, energy_intensity, water_intensity,
                   waste_diversion_rate, renewable_energy_pct, yoy_co2_change_pct,
                   benchmark_comparison, compliance_alerts, calculated_at
            FROM esg_dashboard_metrics
            WHERE organization_id = $1
              AND year = $2
              AND (($3::uuid IS NULL AND building_id IS NULL) OR building_id = $3)
              AND month IS NULL
            ORDER BY calculated_at DESC
            LIMIT 1
            "#,
        )
        .bind(organization_id)
        .bind(year)
        .bind(building_id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Refresh ESG dashboard metrics (recalculate from source data).
    pub async fn refresh_dashboard_metrics(
        &self,
        organization_id: Uuid,
        year: i32,
        building_id: Option<Uuid>,
    ) -> Result<EsgDashboardMetrics, sqlx::Error> {
        // Get carbon footprint data
        let carbon = self.get_carbon_summary(organization_id, year).await?;

        // Get previous year for YoY comparison
        let prev_carbon = self.get_carbon_summary(organization_id, year - 1).await?;

        let yoy_change = match (&carbon, &prev_carbon) {
            (Some(c), Some(p)) if p.total_co2_kg > Decimal::ZERO => {
                Some(((c.total_co2_kg - p.total_co2_kg) / p.total_co2_kg) * Decimal::from(100))
            }
            _ => None,
        };

        // Insert or update dashboard metrics
        sqlx::query_as::<_, EsgDashboardMetrics>(
            r#"
            INSERT INTO esg_dashboard_metrics (
                id, organization_id, building_id, year, month,
                total_co2_kg, co2_per_sqm, yoy_co2_change_pct, calculated_at
            ) VALUES ($1, $2, $3, $4, NULL, $5, $6, $7, NOW())
            ON CONFLICT (organization_id, building_id, year, month)
            WHERE month IS NULL
            DO UPDATE SET
                total_co2_kg = EXCLUDED.total_co2_kg,
                co2_per_sqm = EXCLUDED.co2_per_sqm,
                yoy_co2_change_pct = EXCLUDED.yoy_co2_change_pct,
                calculated_at = NOW()
            RETURNING id, organization_id, building_id, year, month,
                      environmental_score, social_score, governance_score, overall_esg_score,
                      total_co2_kg, co2_per_sqm, energy_intensity, water_intensity,
                      waste_diversion_rate, renewable_energy_pct, yoy_co2_change_pct,
                      benchmark_comparison, compliance_alerts, calculated_at
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(organization_id)
        .bind(building_id)
        .bind(year)
        .bind(carbon.as_ref().map(|c| &c.total_co2_kg))
        .bind(carbon.as_ref().and_then(|c| c.avg_co2_per_sqm.as_ref()))
        .bind(yoy_change)
        .fetch_one(&self.pool)
        .await
    }

    // =========================================================================
    // Import Jobs
    // =========================================================================

    /// Create an import job.
    pub async fn create_import_job(
        &self,
        organization_id: Uuid,
        user_id: Uuid,
        input: CreateEsgImportJob,
    ) -> Result<EsgImportJob, sqlx::Error> {
        sqlx::query_as::<_, EsgImportJob>(
            r#"
            INSERT INTO esg_import_jobs (
                id, organization_id, file_name, file_url, data_type,
                status, rows_total, created_at, created_by
            ) VALUES ($1, $2, $3, $4, $5, 'pending', $6, NOW(), $7)
            RETURNING id, organization_id, file_name, file_url, data_type, status,
                      rows_total, rows_processed, rows_failed, error_log,
                      started_at, completed_at, created_at, created_by
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(organization_id)
        .bind(&input.file_name)
        .bind(input.file_url.as_deref())
        .bind(&input.data_type)
        .bind(input.rows_total)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
    }

    /// Get import job by ID.
    pub async fn get_import_job(&self, id: Uuid) -> Result<Option<EsgImportJob>, sqlx::Error> {
        sqlx::query_as::<_, EsgImportJob>(
            r#"
            SELECT id, organization_id, file_name, file_url, data_type, status,
                   rows_total, rows_processed, rows_failed, error_log,
                   started_at, completed_at, created_at, created_by
            FROM esg_import_jobs
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    /// List import jobs.
    pub async fn list_import_jobs(
        &self,
        organization_id: Uuid,
    ) -> Result<Vec<EsgImportJob>, sqlx::Error> {
        sqlx::query_as::<_, EsgImportJob>(
            r#"
            SELECT id, organization_id, file_name, file_url, data_type, status,
                   rows_total, rows_processed, rows_failed, error_log,
                   started_at, completed_at, created_at, created_by
            FROM esg_import_jobs
            WHERE organization_id = $1
            ORDER BY created_at DESC
            LIMIT 50
            "#,
        )
        .bind(organization_id)
        .fetch_all(&self.pool)
        .await
    }

    /// Update import job status.
    pub async fn update_import_job_status(
        &self,
        id: Uuid,
        status: &str,
        rows_processed: Option<i32>,
        rows_failed: Option<i32>,
        error_log: Option<serde_json::Value>,
    ) -> Result<Option<EsgImportJob>, sqlx::Error> {
        let now = Utc::now();
        let started_at = if status == "processing" {
            Some(now)
        } else {
            None
        };
        let completed_at = if status == "completed" || status == "failed" {
            Some(now)
        } else {
            None
        };

        sqlx::query_as::<_, EsgImportJob>(
            r#"
            UPDATE esg_import_jobs
            SET status = $2,
                rows_processed = COALESCE($3, rows_processed),
                rows_failed = COALESCE($4, rows_failed),
                error_log = COALESCE($5, error_log),
                started_at = COALESCE($6, started_at),
                completed_at = COALESCE($7, completed_at)
            WHERE id = $1
            RETURNING id, organization_id, file_name, file_url, data_type, status,
                      rows_total, rows_processed, rows_failed, error_log,
                      started_at, completed_at, created_at, created_by
            "#,
        )
        .bind(id)
        .bind(status)
        .bind(rows_processed)
        .bind(rows_failed)
        .bind(&error_log)
        .bind(started_at)
        .bind(completed_at)
        .fetch_optional(&self.pool)
        .await
    }

    // =========================================================================
    // Statistics
    // =========================================================================

    /// Get ESG statistics for an organization.
    pub async fn get_statistics(
        &self,
        organization_id: Uuid,
    ) -> Result<EsgStatistics, sqlx::Error> {
        let current_year = Utc::now().naive_utc().date().year();

        sqlx::query_as::<_, EsgStatistics>(
            r#"
            SELECT
                (SELECT COUNT(*) FROM esg_metrics WHERE organization_id = $1) as total_metrics,
                (SELECT COUNT(DISTINCT building_id) FROM esg_metrics WHERE organization_id = $1 AND building_id IS NOT NULL) as total_buildings_tracked,
                (SELECT overall_esg_score FROM esg_dashboard_metrics WHERE organization_id = $1 AND year = $2 AND building_id IS NULL ORDER BY calculated_at DESC LIMIT 1) as latest_esg_score,
                (SELECT SUM(co2_equivalent_kg) FROM carbon_footprints WHERE organization_id = $1 AND year = $2) as total_co2_current_year,
                (SELECT
                    CASE WHEN prev.total > 0 THEN ((curr.total - prev.total) / prev.total) * 100 ELSE NULL END
                 FROM
                    (SELECT COALESCE(SUM(co2_equivalent_kg), 0) as total FROM carbon_footprints WHERE organization_id = $1 AND year = $2) curr,
                    (SELECT COALESCE(SUM(co2_equivalent_kg), 0) as total FROM carbon_footprints WHERE organization_id = $1 AND year = $2 - 1) prev
                ) as yoy_co2_change,
                (SELECT COUNT(*) FROM esg_reports WHERE organization_id = $1 AND status = 'published') as reports_published
            "#,
        )
        .bind(organization_id)
        .bind(current_year)
        .fetch_one(&self.pool)
        .await
    }
}

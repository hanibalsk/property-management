//! Portfolio Analytics repository for Epic 140: Multi-Property Portfolio Analytics.
//!
//! Provides database operations for portfolio benchmarks, performance metrics,
//! comparisons, trends, and alerts.

use chrono::NaiveDate;
use rust_decimal::Decimal;
use sqlx::{Error as SqlxError, Row};
use uuid::Uuid;

use crate::models::portfolio_analytics::*;
use crate::DbPool;

/// Repository for portfolio analytics operations.
#[derive(Clone)]
pub struct PortfolioAnalyticsRepository {
    pool: DbPool,
}

impl PortfolioAnalyticsRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    // =========================================================================
    // PORTFOLIO BENCHMARKS
    // =========================================================================

    /// Create a new benchmark.
    pub async fn create_benchmark(
        &self,
        org_id: Uuid,
        data: &CreatePortfolioBenchmark,
        created_by: Uuid,
    ) -> Result<PortfolioBenchmark, SqlxError> {
        sqlx::query_as::<_, PortfolioBenchmark>(
            r#"
            INSERT INTO portfolio_benchmarks (
                organization_id, name, description, category, comparison_scope,
                target_value, warning_threshold, critical_threshold,
                is_higher_better, created_by
            )
            VALUES ($1, $2, $3, $4::benchmark_category, $5::comparison_scope, $6, $7, $8, $9, $10)
            RETURNING *
            "#,
        )
        .bind(org_id)
        .bind(&data.name)
        .bind(&data.description)
        .bind(&data.category)
        .bind(
            data.comparison_scope
                .as_ref()
                .unwrap_or(&ComparisonScope::Portfolio),
        )
        .bind(data.target_value)
        .bind(data.warning_threshold)
        .bind(data.critical_threshold)
        .bind(data.is_higher_better.unwrap_or(true))
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
    }

    /// Get a benchmark by ID.
    pub async fn get_benchmark(
        &self,
        id: Uuid,
        org_id: Uuid,
    ) -> Result<Option<PortfolioBenchmark>, SqlxError> {
        sqlx::query_as::<_, PortfolioBenchmark>(
            "SELECT * FROM portfolio_benchmarks WHERE id = $1 AND organization_id = $2",
        )
        .bind(id)
        .bind(org_id)
        .fetch_optional(&self.pool)
        .await
    }

    /// List all benchmarks for an organization.
    pub async fn list_benchmarks(
        &self,
        org_id: Uuid,
        category: Option<BenchmarkCategory>,
    ) -> Result<Vec<PortfolioBenchmark>, SqlxError> {
        if let Some(cat) = category {
            sqlx::query_as::<_, PortfolioBenchmark>(
                r#"
                SELECT * FROM portfolio_benchmarks
                WHERE organization_id = $1 AND category = $2::benchmark_category AND is_active = true
                ORDER BY name
                "#,
            )
            .bind(org_id)
            .bind(cat)
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query_as::<_, PortfolioBenchmark>(
                r#"
                SELECT * FROM portfolio_benchmarks
                WHERE organization_id = $1 AND is_active = true
                ORDER BY category, name
                "#,
            )
            .bind(org_id)
            .fetch_all(&self.pool)
            .await
        }
    }

    /// Update a benchmark.
    pub async fn update_benchmark(
        &self,
        id: Uuid,
        org_id: Uuid,
        data: &UpdatePortfolioBenchmark,
    ) -> Result<Option<PortfolioBenchmark>, SqlxError> {
        sqlx::query_as::<_, PortfolioBenchmark>(
            r#"
            UPDATE portfolio_benchmarks SET
                name = COALESCE($3, name),
                description = COALESCE($4, description),
                target_value = COALESCE($5, target_value),
                warning_threshold = COALESCE($6, warning_threshold),
                critical_threshold = COALESCE($7, critical_threshold),
                is_higher_better = COALESCE($8, is_higher_better),
                is_active = COALESCE($9, is_active),
                updated_at = NOW()
            WHERE id = $1 AND organization_id = $2
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(org_id)
        .bind(&data.name)
        .bind(&data.description)
        .bind(data.target_value)
        .bind(data.warning_threshold)
        .bind(data.critical_threshold)
        .bind(data.is_higher_better)
        .bind(data.is_active)
        .fetch_optional(&self.pool)
        .await
    }

    /// Delete a benchmark.
    pub async fn delete_benchmark(&self, id: Uuid, org_id: Uuid) -> Result<bool, SqlxError> {
        let result =
            sqlx::query("DELETE FROM portfolio_benchmarks WHERE id = $1 AND organization_id = $2")
                .bind(id)
                .bind(org_id)
                .execute(&self.pool)
                .await?;

        Ok(result.rows_affected() > 0)
    }

    // =========================================================================
    // PROPERTY PERFORMANCE METRICS
    // =========================================================================

    /// Create or update property performance metrics.
    pub async fn upsert_property_metrics(
        &self,
        org_id: Uuid,
        data: &CreatePropertyMetrics,
    ) -> Result<PropertyPerformanceMetrics, SqlxError> {
        // Calculate derived fields
        let vacant_units = data
            .total_units
            .map(|t| t - data.occupied_units.unwrap_or(0));
        let rent_collection_rate = match (data.collected_rent, data.gross_potential_rent) {
            (Some(c), Some(g)) if g > Decimal::ZERO => Some(c / g * Decimal::from(100)),
            _ => None,
        };
        let noi = match (data.gross_revenue, data.operating_expenses) {
            (Some(r), Some(e)) => Some(r - e),
            _ => None,
        };
        let noi_margin = match (noi, data.gross_revenue) {
            (Some(n), Some(r)) if r > Decimal::ZERO => Some(n / r * Decimal::from(100)),
            _ => None,
        };
        let turnover_rate = match (data.move_outs, data.total_units) {
            (Some(m), Some(t)) if t > 0 => {
                Some(Decimal::from(m) / Decimal::from(t) * Decimal::from(100))
            }
            _ => None,
        };

        sqlx::query_as::<_, PropertyPerformanceMetrics>(
            r#"
            INSERT INTO property_performance_metrics (
                organization_id, building_id, period_type, period_start, period_end,
                occupancy_rate, total_units, occupied_units, vacant_units,
                gross_potential_rent, collected_rent, rent_collection_rate,
                gross_revenue, operating_expenses, net_operating_income, noi_margin,
                maintenance_requests, completed_requests, avg_resolution_time_hours, maintenance_cost,
                new_leases, renewals, move_outs, tenant_turnover_rate,
                energy_consumption_kwh, water_consumption_liters, energy_cost,
                property_value
            )
            VALUES ($1, $2, $3::aggregation_period, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28)
            ON CONFLICT (building_id, period_type, period_start)
            DO UPDATE SET
                occupancy_rate = EXCLUDED.occupancy_rate,
                total_units = EXCLUDED.total_units,
                occupied_units = EXCLUDED.occupied_units,
                vacant_units = EXCLUDED.vacant_units,
                gross_potential_rent = EXCLUDED.gross_potential_rent,
                collected_rent = EXCLUDED.collected_rent,
                rent_collection_rate = EXCLUDED.rent_collection_rate,
                gross_revenue = EXCLUDED.gross_revenue,
                operating_expenses = EXCLUDED.operating_expenses,
                net_operating_income = EXCLUDED.net_operating_income,
                noi_margin = EXCLUDED.noi_margin,
                maintenance_requests = EXCLUDED.maintenance_requests,
                completed_requests = EXCLUDED.completed_requests,
                avg_resolution_time_hours = EXCLUDED.avg_resolution_time_hours,
                maintenance_cost = EXCLUDED.maintenance_cost,
                new_leases = EXCLUDED.new_leases,
                renewals = EXCLUDED.renewals,
                move_outs = EXCLUDED.move_outs,
                tenant_turnover_rate = EXCLUDED.tenant_turnover_rate,
                energy_consumption_kwh = EXCLUDED.energy_consumption_kwh,
                water_consumption_liters = EXCLUDED.water_consumption_liters,
                energy_cost = EXCLUDED.energy_cost,
                property_value = EXCLUDED.property_value,
                calculated_at = NOW(),
                updated_at = NOW()
            RETURNING *
            "#,
        )
        .bind(org_id)
        .bind(data.building_id)
        .bind(&data.period_type)
        .bind(data.period_start)
        .bind(data.period_end)
        .bind(data.occupancy_rate)
        .bind(data.total_units)
        .bind(data.occupied_units)
        .bind(vacant_units)
        .bind(data.gross_potential_rent)
        .bind(data.collected_rent)
        .bind(rent_collection_rate)
        .bind(data.gross_revenue)
        .bind(data.operating_expenses)
        .bind(noi)
        .bind(noi_margin)
        .bind(data.maintenance_requests)
        .bind(data.completed_requests)
        .bind(data.avg_resolution_time_hours)
        .bind(data.maintenance_cost)
        .bind(data.new_leases)
        .bind(data.renewals)
        .bind(data.move_outs)
        .bind(turnover_rate)
        .bind(data.energy_consumption_kwh)
        .bind(data.water_consumption_liters)
        .bind(data.energy_cost)
        .bind(data.property_value)
        .fetch_one(&self.pool)
        .await
    }

    /// Get property metrics for a building.
    pub async fn get_property_metrics(
        &self,
        org_id: Uuid,
        building_id: Uuid,
        period_type: &AggregationPeriod,
        period_start: NaiveDate,
    ) -> Result<Option<PropertyPerformanceMetrics>, SqlxError> {
        sqlx::query_as::<_, PropertyPerformanceMetrics>(
            r#"
            SELECT * FROM property_performance_metrics
            WHERE organization_id = $1 AND building_id = $2
              AND period_type = $3::aggregation_period AND period_start = $4
            "#,
        )
        .bind(org_id)
        .bind(building_id)
        .bind(period_type)
        .bind(period_start)
        .fetch_optional(&self.pool)
        .await
    }

    /// List property metrics history.
    pub async fn list_property_metrics_history(
        &self,
        org_id: Uuid,
        building_id: Uuid,
        period_type: &AggregationPeriod,
        limit: i64,
    ) -> Result<Vec<PropertyPerformanceMetrics>, SqlxError> {
        sqlx::query_as::<_, PropertyPerformanceMetrics>(
            r#"
            SELECT * FROM property_performance_metrics
            WHERE organization_id = $1 AND building_id = $2 AND period_type = $3::aggregation_period
            ORDER BY period_start DESC
            LIMIT $4
            "#,
        )
        .bind(org_id)
        .bind(building_id)
        .bind(period_type)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
    }

    /// List all property metrics for a period.
    pub async fn list_all_property_metrics(
        &self,
        org_id: Uuid,
        period_type: &AggregationPeriod,
        period_start: NaiveDate,
    ) -> Result<Vec<PropertyPerformanceMetrics>, SqlxError> {
        sqlx::query_as::<_, PropertyPerformanceMetrics>(
            r#"
            SELECT * FROM property_performance_metrics
            WHERE organization_id = $1 AND period_type = $2::aggregation_period AND period_start = $3
            ORDER BY building_id
            "#,
        )
        .bind(org_id)
        .bind(period_type)
        .bind(period_start)
        .fetch_all(&self.pool)
        .await
    }

    // =========================================================================
    // PORTFOLIO AGGREGATED METRICS
    // =========================================================================

    /// Calculate and store portfolio aggregated metrics.
    pub async fn calculate_portfolio_metrics(
        &self,
        org_id: Uuid,
        period_type: &AggregationPeriod,
        period_start: NaiveDate,
        period_end: NaiveDate,
    ) -> Result<PortfolioAggregatedMetrics, SqlxError> {
        // Aggregate from property metrics
        let row = sqlx::query(
            r#"
            SELECT
                COUNT(DISTINCT building_id) as total_properties,
                SUM(total_units) as total_units,
                AVG(occupancy_rate) as avg_occupancy_rate,
                AVG(rent_collection_rate) as avg_rent_collection_rate,
                AVG(noi_margin) as avg_noi_margin,
                AVG(cap_rate) as avg_cap_rate,
                AVG(tenant_turnover_rate) as avg_tenant_turnover_rate,
                SUM(gross_revenue) as total_gross_revenue,
                SUM(operating_expenses) as total_operating_expenses,
                SUM(net_operating_income) as total_noi,
                SUM(property_value) as total_property_value
            FROM property_performance_metrics
            WHERE organization_id = $1 AND period_type = $2::aggregation_period AND period_start = $3
            "#,
        )
        .bind(org_id)
        .bind(period_type)
        .bind(period_start)
        .fetch_one(&self.pool)
        .await?;

        let total_properties: Option<i64> = row.get("total_properties");
        let total_units: Option<i64> = row.get("total_units");

        // Calculate per-unit metrics
        let total_revenue: Option<Decimal> = row.get("total_gross_revenue");
        let total_expenses: Option<Decimal> = row.get("total_operating_expenses");
        let total_noi: Option<Decimal> = row.get("total_noi");

        let revenue_per_unit = match (total_revenue, total_units) {
            (Some(r), Some(u)) if u > 0 => Some(r / Decimal::from(u)),
            _ => None,
        };
        let expense_per_unit = match (total_expenses, total_units) {
            (Some(e), Some(u)) if u > 0 => Some(e / Decimal::from(u)),
            _ => None,
        };
        let noi_per_unit = match (total_noi, total_units) {
            (Some(n), Some(u)) if u > 0 => Some(n / Decimal::from(u)),
            _ => None,
        };

        sqlx::query_as::<_, PortfolioAggregatedMetrics>(
            r#"
            INSERT INTO portfolio_aggregated_metrics (
                organization_id, period_type, period_start, period_end,
                total_properties, total_units,
                avg_occupancy_rate, avg_rent_collection_rate, avg_noi_margin,
                avg_cap_rate, avg_tenant_turnover_rate,
                total_gross_revenue, total_operating_expenses, total_noi, total_property_value,
                portfolio_revenue_per_unit, portfolio_expense_per_unit, portfolio_noi_per_unit
            )
            VALUES ($1, $2::aggregation_period, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
            ON CONFLICT (organization_id, period_type, period_start)
            DO UPDATE SET
                total_properties = EXCLUDED.total_properties,
                total_units = EXCLUDED.total_units,
                avg_occupancy_rate = EXCLUDED.avg_occupancy_rate,
                avg_rent_collection_rate = EXCLUDED.avg_rent_collection_rate,
                avg_noi_margin = EXCLUDED.avg_noi_margin,
                avg_cap_rate = EXCLUDED.avg_cap_rate,
                avg_tenant_turnover_rate = EXCLUDED.avg_tenant_turnover_rate,
                total_gross_revenue = EXCLUDED.total_gross_revenue,
                total_operating_expenses = EXCLUDED.total_operating_expenses,
                total_noi = EXCLUDED.total_noi,
                total_property_value = EXCLUDED.total_property_value,
                portfolio_revenue_per_unit = EXCLUDED.portfolio_revenue_per_unit,
                portfolio_expense_per_unit = EXCLUDED.portfolio_expense_per_unit,
                portfolio_noi_per_unit = EXCLUDED.portfolio_noi_per_unit,
                calculated_at = NOW(),
                updated_at = NOW()
            RETURNING *
            "#,
        )
        .bind(org_id)
        .bind(period_type)
        .bind(period_start)
        .bind(period_end)
        .bind(total_properties.map(|v| v as i32))
        .bind(total_units.map(|v| v as i32))
        .bind(row.get::<Option<Decimal>, _>("avg_occupancy_rate"))
        .bind(row.get::<Option<Decimal>, _>("avg_rent_collection_rate"))
        .bind(row.get::<Option<Decimal>, _>("avg_noi_margin"))
        .bind(row.get::<Option<Decimal>, _>("avg_cap_rate"))
        .bind(row.get::<Option<Decimal>, _>("avg_tenant_turnover_rate"))
        .bind(total_revenue)
        .bind(total_expenses)
        .bind(total_noi)
        .bind(row.get::<Option<Decimal>, _>("total_property_value"))
        .bind(revenue_per_unit)
        .bind(expense_per_unit)
        .bind(noi_per_unit)
        .fetch_one(&self.pool)
        .await
    }

    /// Get portfolio aggregated metrics.
    pub async fn get_portfolio_metrics(
        &self,
        org_id: Uuid,
        period_type: &AggregationPeriod,
        period_start: NaiveDate,
    ) -> Result<Option<PortfolioAggregatedMetrics>, SqlxError> {
        sqlx::query_as::<_, PortfolioAggregatedMetrics>(
            r#"
            SELECT * FROM portfolio_aggregated_metrics
            WHERE organization_id = $1 AND period_type = $2::aggregation_period AND period_start = $3
            "#,
        )
        .bind(org_id)
        .bind(period_type)
        .bind(period_start)
        .fetch_optional(&self.pool)
        .await
    }

    /// List portfolio metrics history.
    pub async fn list_portfolio_metrics_history(
        &self,
        org_id: Uuid,
        period_type: &AggregationPeriod,
        limit: i64,
    ) -> Result<Vec<PortfolioAggregatedMetrics>, SqlxError> {
        sqlx::query_as::<_, PortfolioAggregatedMetrics>(
            r#"
            SELECT * FROM portfolio_aggregated_metrics
            WHERE organization_id = $1 AND period_type = $2::aggregation_period
            ORDER BY period_start DESC
            LIMIT $3
            "#,
        )
        .bind(org_id)
        .bind(period_type)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
    }

    // =========================================================================
    // ALERT RULES
    // =========================================================================

    /// Create an alert rule.
    pub async fn create_alert_rule(
        &self,
        org_id: Uuid,
        data: &CreateAlertRule,
        created_by: Uuid,
    ) -> Result<PortfolioAlertRule, SqlxError> {
        sqlx::query_as::<_, PortfolioAlertRule>(
            r#"
            INSERT INTO portfolio_alert_rules (
                organization_id, benchmark_id, name, description, metric_name,
                condition_type, threshold_value, threshold_operator,
                severity, notification_channels, recipients, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING *
            "#,
        )
        .bind(org_id)
        .bind(data.benchmark_id)
        .bind(&data.name)
        .bind(&data.description)
        .bind(&data.metric_name)
        .bind(&data.condition_type)
        .bind(data.threshold_value)
        .bind(&data.threshold_operator)
        .bind(data.severity.as_deref().unwrap_or("warning"))
        .bind(&data.notification_channels)
        .bind(&data.recipients)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
    }

    /// List alert rules.
    pub async fn list_alert_rules(
        &self,
        org_id: Uuid,
    ) -> Result<Vec<PortfolioAlertRule>, SqlxError> {
        sqlx::query_as::<_, PortfolioAlertRule>(
            "SELECT * FROM portfolio_alert_rules WHERE organization_id = $1 ORDER BY name",
        )
        .bind(org_id)
        .fetch_all(&self.pool)
        .await
    }

    /// Update an alert rule.
    pub async fn update_alert_rule(
        &self,
        id: Uuid,
        org_id: Uuid,
        data: &UpdateAlertRule,
    ) -> Result<Option<PortfolioAlertRule>, SqlxError> {
        sqlx::query_as::<_, PortfolioAlertRule>(
            r#"
            UPDATE portfolio_alert_rules SET
                name = COALESCE($3, name),
                description = COALESCE($4, description),
                threshold_value = COALESCE($5, threshold_value),
                threshold_operator = COALESCE($6, threshold_operator),
                severity = COALESCE($7, severity),
                notification_channels = COALESCE($8, notification_channels),
                recipients = COALESCE($9, recipients),
                is_active = COALESCE($10, is_active),
                updated_at = NOW()
            WHERE id = $1 AND organization_id = $2
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(org_id)
        .bind(&data.name)
        .bind(&data.description)
        .bind(data.threshold_value)
        .bind(&data.threshold_operator)
        .bind(&data.severity)
        .bind(&data.notification_channels)
        .bind(&data.recipients)
        .bind(data.is_active)
        .fetch_optional(&self.pool)
        .await
    }

    /// Delete an alert rule.
    pub async fn delete_alert_rule(&self, id: Uuid, org_id: Uuid) -> Result<bool, SqlxError> {
        let result =
            sqlx::query("DELETE FROM portfolio_alert_rules WHERE id = $1 AND organization_id = $2")
                .bind(id)
                .bind(org_id)
                .execute(&self.pool)
                .await?;

        Ok(result.rows_affected() > 0)
    }

    // =========================================================================
    // ALERTS
    // =========================================================================

    /// Create an alert.
    pub async fn create_alert(
        &self,
        org_id: Uuid,
        rule_id: Option<Uuid>,
        building_id: Option<Uuid>,
        alert_type: &str,
        severity: &str,
        title: &str,
        message: Option<&str>,
        metric_name: Option<&str>,
        metric_value: Option<Decimal>,
        threshold_value: Option<Decimal>,
    ) -> Result<PortfolioAlert, SqlxError> {
        sqlx::query_as::<_, PortfolioAlert>(
            r#"
            INSERT INTO portfolio_alerts (
                organization_id, rule_id, building_id, alert_type, severity,
                title, message, metric_name, metric_value, threshold_value
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#,
        )
        .bind(org_id)
        .bind(rule_id)
        .bind(building_id)
        .bind(alert_type)
        .bind(severity)
        .bind(title)
        .bind(message)
        .bind(metric_name)
        .bind(metric_value)
        .bind(threshold_value)
        .fetch_one(&self.pool)
        .await
    }

    /// List alerts.
    pub async fn list_alerts(
        &self,
        org_id: Uuid,
        query: &AlertQuery,
    ) -> Result<Vec<PortfolioAlert>, SqlxError> {
        let mut sql = String::from("SELECT * FROM portfolio_alerts WHERE organization_id = $1");
        let mut param_count = 1;

        if query.status.is_some() {
            param_count += 1;
            sql.push_str(&format!(" AND status = ${}", param_count));
        }
        if query.severity.is_some() {
            param_count += 1;
            sql.push_str(&format!(" AND severity = ${}", param_count));
        }
        if query.building_id.is_some() {
            param_count += 1;
            sql.push_str(&format!(" AND building_id = ${}", param_count));
        }

        sql.push_str(" ORDER BY triggered_at DESC LIMIT 100");

        let mut query_builder = sqlx::query_as::<_, PortfolioAlert>(&sql).bind(org_id);

        if let Some(ref status) = query.status {
            query_builder = query_builder.bind(status);
        }
        if let Some(ref severity) = query.severity {
            query_builder = query_builder.bind(severity);
        }
        if let Some(building_id) = query.building_id {
            query_builder = query_builder.bind(building_id);
        }

        query_builder.fetch_all(&self.pool).await
    }

    /// Acknowledge an alert.
    pub async fn acknowledge_alert(
        &self,
        id: Uuid,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<PortfolioAlert>, SqlxError> {
        sqlx::query_as::<_, PortfolioAlert>(
            r#"
            UPDATE portfolio_alerts SET
                status = 'acknowledged',
                acknowledged_at = NOW(),
                acknowledged_by = $3
            WHERE id = $1 AND organization_id = $2
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(org_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Resolve an alert.
    pub async fn resolve_alert(
        &self,
        id: Uuid,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<PortfolioAlert>, SqlxError> {
        sqlx::query_as::<_, PortfolioAlert>(
            r#"
            UPDATE portfolio_alerts SET
                status = 'resolved',
                resolved_at = NOW(),
                resolved_by = $3
            WHERE id = $1 AND organization_id = $2
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(org_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Get active alerts count.
    pub async fn get_active_alerts_count(&self, org_id: Uuid) -> Result<i64, SqlxError> {
        let row = sqlx::query(
            "SELECT COUNT(*) as count FROM portfolio_alerts WHERE organization_id = $1 AND status = 'active'",
        )
        .bind(org_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.get("count"))
    }

    // =========================================================================
    // TRENDS
    // =========================================================================

    /// Get trends for a category.
    pub async fn list_trends(
        &self,
        org_id: Uuid,
        query: &TrendQuery,
    ) -> Result<Vec<PortfolioTrend>, SqlxError> {
        let mut sql = String::from("SELECT * FROM portfolio_trends WHERE organization_id = $1");
        let mut param_count = 1;

        if query.metric_name.is_some() {
            param_count += 1;
            sql.push_str(&format!(" AND metric_name = ${}", param_count));
        }
        if query.category.is_some() {
            param_count += 1;
            sql.push_str(&format!(
                " AND category = ${}::benchmark_category",
                param_count
            ));
        }

        sql.push_str(" ORDER BY calculated_at DESC");

        let mut query_builder = sqlx::query_as::<_, PortfolioTrend>(&sql).bind(org_id);

        if let Some(ref metric_name) = query.metric_name {
            query_builder = query_builder.bind(metric_name);
        }
        if let Some(ref category) = query.category {
            query_builder = query_builder.bind(category);
        }

        query_builder.fetch_all(&self.pool).await
    }
}

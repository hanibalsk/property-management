//! Workflow templates repository (Epic 94, Story 94.4).
//!
//! Provides CRUD operations for workflow templates, including
//! search, import, and rating functionality.

use crate::models::{
    template_scope, CreateTemplateAction, CreateTemplateVariable, CreateWorkflowTemplate,
    ImportTemplateRequest, RateTemplateRequest, TemplateSearchQuery, UpdateWorkflowTemplate,
    WorkflowTemplate, WorkflowTemplateAction, WorkflowTemplateRating, WorkflowTemplateSummary,
    WorkflowTemplateVariable, WorkflowTemplateWithDetails,
};
use sqlx::PgPool;
use uuid::Uuid;

/// Repository for workflow template operations.
#[derive(Clone)]
pub struct WorkflowTemplateRepository {
    pool: PgPool,
}

impl WorkflowTemplateRepository {
    /// Create a new repository instance.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new workflow template.
    pub async fn create(
        &self,
        data: CreateWorkflowTemplate,
    ) -> Result<WorkflowTemplate, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO workflow_templates
                (organization_id, name, description, category, trigger_type, trigger_config,
                 conditions, scope, tags, icon, created_by)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING *
            "#,
        )
        .bind(data.organization_id)
        .bind(&data.name)
        .bind(&data.description)
        .bind(&data.category)
        .bind(&data.trigger_type)
        .bind(sqlx::types::Json(data.trigger_config.unwrap_or_default()))
        .bind(sqlx::types::Json(data.conditions.unwrap_or_default()))
        .bind(
            data.scope
                .unwrap_or_else(|| template_scope::ORGANIZATION.to_string()),
        )
        .bind(data.tags.unwrap_or_default())
        .bind(&data.icon)
        .bind(data.created_by)
        .fetch_one(&self.pool)
        .await
    }

    /// Get template by ID.
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<WorkflowTemplate>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM workflow_templates WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    /// Get template with full details (actions and variables).
    pub async fn find_with_details(
        &self,
        id: Uuid,
    ) -> Result<Option<WorkflowTemplateWithDetails>, sqlx::Error> {
        let template = self.find_by_id(id).await?;
        match template {
            Some(t) => {
                let actions = self.list_actions(id).await?;
                let variables = self.list_variables(id).await?;
                Ok(Some(WorkflowTemplateWithDetails {
                    template: t,
                    actions,
                    variables,
                }))
            }
            None => Ok(None),
        }
    }

    /// Search templates with filters.
    pub async fn search(
        &self,
        org_id: Option<Uuid>,
        query: TemplateSearchQuery,
    ) -> Result<Vec<WorkflowTemplateSummary>, sqlx::Error> {
        let limit = query.limit.unwrap_or(50);
        let offset = query.offset.unwrap_or(0);

        sqlx::query_as(
            r#"
            SELECT
                t.id,
                t.name,
                t.description,
                t.category,
                t.trigger_type,
                t.scope,
                t.use_count,
                t.avg_rating,
                t.tags,
                t.icon,
                t.featured,
                COUNT(a.id) as action_count
            FROM workflow_templates t
            LEFT JOIN workflow_template_actions a ON a.template_id = t.id
            WHERE t.active = TRUE
                AND (
                    t.scope = 'global'
                    OR t.scope = 'platform'
                    OR (t.scope = 'organization' AND t.organization_id = $1)
                )
                AND ($2::text IS NULL OR t.category = $2)
                AND ($3::text IS NULL OR t.trigger_type = $3)
                AND ($4::text IS NULL OR t.name ILIKE '%' || $4 || '%' OR t.description ILIKE '%' || $4 || '%')
                AND ($5::boolean IS NULL OR t.featured = $5)
                AND ($6::text IS NULL OR t.scope = $6)
            GROUP BY t.id
            ORDER BY
                t.featured DESC,
                t.use_count DESC,
                t.avg_rating DESC NULLS LAST,
                t.name
            LIMIT $7 OFFSET $8
            "#,
        )
        .bind(org_id)
        .bind(&query.category)
        .bind(&query.trigger_type)
        .bind(&query.search)
        .bind(query.featured)
        .bind(&query.scope)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
    }

    /// List templates by category.
    pub async fn list_by_category(
        &self,
        category: &str,
        org_id: Option<Uuid>,
    ) -> Result<Vec<WorkflowTemplateSummary>, sqlx::Error> {
        self.search(
            org_id,
            TemplateSearchQuery {
                category: Some(category.to_string()),
                ..Default::default()
            },
        )
        .await
    }

    /// List featured templates.
    pub async fn list_featured(
        &self,
        org_id: Option<Uuid>,
    ) -> Result<Vec<WorkflowTemplateSummary>, sqlx::Error> {
        self.search(
            org_id,
            TemplateSearchQuery {
                featured: Some(true),
                limit: Some(10),
                ..Default::default()
            },
        )
        .await
    }

    /// Update a template.
    pub async fn update(
        &self,
        id: Uuid,
        data: UpdateWorkflowTemplate,
    ) -> Result<WorkflowTemplate, sqlx::Error> {
        sqlx::query_as(
            r#"
            UPDATE workflow_templates SET
                name = COALESCE($2, name),
                description = COALESCE($3, description),
                category = COALESCE($4, category),
                trigger_config = COALESCE($5, trigger_config),
                conditions = COALESCE($6, conditions),
                tags = COALESCE($7, tags),
                icon = COALESCE($8, icon),
                featured = COALESCE($9, featured),
                active = COALESCE($10, active),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&data.name)
        .bind(&data.description)
        .bind(&data.category)
        .bind(data.trigger_config.map(sqlx::types::Json))
        .bind(data.conditions.map(sqlx::types::Json))
        .bind(&data.tags)
        .bind(&data.icon)
        .bind(data.featured)
        .bind(data.active)
        .fetch_one(&self.pool)
        .await
    }

    /// Delete a template.
    pub async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM workflow_templates WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    // --- Actions ---

    /// Add an action to a template.
    pub async fn add_action(
        &self,
        data: CreateTemplateAction,
    ) -> Result<WorkflowTemplateAction, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO workflow_template_actions
                (template_id, action_order, action_type, action_config, description,
                 on_failure, retry_count, retry_delay_seconds)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#,
        )
        .bind(data.template_id)
        .bind(data.action_order)
        .bind(&data.action_type)
        .bind(sqlx::types::Json(&data.action_config))
        .bind(&data.description)
        .bind(data.on_failure.unwrap_or_else(|| "stop".to_string()))
        .bind(data.retry_count.unwrap_or(3))
        .bind(data.retry_delay_seconds.unwrap_or(60))
        .fetch_one(&self.pool)
        .await
    }

    /// List actions for a template.
    pub async fn list_actions(
        &self,
        template_id: Uuid,
    ) -> Result<Vec<WorkflowTemplateAction>, sqlx::Error> {
        sqlx::query_as(
            "SELECT * FROM workflow_template_actions WHERE template_id = $1 ORDER BY action_order",
        )
        .bind(template_id)
        .fetch_all(&self.pool)
        .await
    }

    /// Delete an action.
    pub async fn delete_action(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM workflow_template_actions WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    // --- Variables ---

    /// Add a variable to a template.
    pub async fn add_variable(
        &self,
        data: CreateTemplateVariable,
    ) -> Result<WorkflowTemplateVariable, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO workflow_template_variables
                (template_id, name, label, description, variable_type,
                 default_value, required, options, validation_pattern)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#,
        )
        .bind(data.template_id)
        .bind(&data.name)
        .bind(&data.label)
        .bind(&data.description)
        .bind(&data.variable_type)
        .bind(&data.default_value)
        .bind(data.required.unwrap_or(false))
        .bind(data.options.map(sqlx::types::Json))
        .bind(&data.validation_pattern)
        .fetch_one(&self.pool)
        .await
    }

    /// List variables for a template.
    pub async fn list_variables(
        &self,
        template_id: Uuid,
    ) -> Result<Vec<WorkflowTemplateVariable>, sqlx::Error> {
        sqlx::query_as(
            "SELECT * FROM workflow_template_variables WHERE template_id = $1 ORDER BY name",
        )
        .bind(template_id)
        .fetch_all(&self.pool)
        .await
    }

    /// Delete a variable.
    pub async fn delete_variable(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM workflow_template_variables WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    // --- Import ---

    /// Import a template as a new workflow.
    /// Returns the new workflow ID.
    /// Uses a database transaction to ensure atomicity.
    pub async fn import_template(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        request: ImportTemplateRequest,
    ) -> Result<Uuid, sqlx::Error> {
        // Get the template with details (outside transaction for read)
        let template_details = self
            .find_with_details(request.template_id)
            .await?
            .ok_or_else(|| sqlx::Error::RowNotFound)?;

        let template = &template_details.template;

        // Start transaction for all write operations
        let mut tx = self.pool.begin().await?;

        // Create the workflow
        let workflow_name = request.name.unwrap_or_else(|| template.name.clone());

        let workflow: (Uuid,) = sqlx::query_as(
            r#"
            INSERT INTO workflows
                (organization_id, name, description, trigger_type, trigger_config,
                 conditions, enabled, created_by)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id
            "#,
        )
        .bind(org_id)
        .bind(&workflow_name)
        .bind(&template.description)
        .bind(&template.trigger_type)
        .bind(&template.trigger_config)
        .bind(&template.conditions)
        .bind(request.enabled.unwrap_or(false))
        .bind(user_id)
        .fetch_one(&mut *tx)
        .await?;

        let workflow_id = workflow.0;

        // Copy actions with variable substitution
        for action in &template_details.actions {
            let mut config = action.action_config.0.clone();

            // Substitute variables in the config
            if let Some(vars) = request.variables.as_object() {
                substitute_variables(&mut config, vars);
            }

            sqlx::query(
                r#"
                INSERT INTO workflow_actions
                    (workflow_id, action_order, action_type, action_config,
                     on_failure, retry_count, retry_delay_seconds)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#,
            )
            .bind(workflow_id)
            .bind(action.action_order)
            .bind(&action.action_type)
            .bind(sqlx::types::Json(&config))
            .bind(&action.on_failure)
            .bind(action.retry_count)
            .bind(action.retry_delay_seconds)
            .execute(&mut *tx)
            .await?;
        }

        // Increment use count
        sqlx::query("UPDATE workflow_templates SET use_count = use_count + 1 WHERE id = $1")
            .bind(request.template_id)
            .execute(&mut *tx)
            .await?;

        // Commit transaction
        tx.commit().await?;

        Ok(workflow_id)
    }

    // --- Ratings ---

    /// Rate a template.
    pub async fn rate_template(
        &self,
        template_id: Uuid,
        org_id: Uuid,
        user_id: Uuid,
        request: RateTemplateRequest,
    ) -> Result<WorkflowTemplateRating, sqlx::Error> {
        // Upsert the rating
        let rating: WorkflowTemplateRating = sqlx::query_as(
            r#"
            INSERT INTO workflow_template_ratings (template_id, organization_id, user_id, rating, review)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (template_id, organization_id, user_id) DO UPDATE SET
                rating = EXCLUDED.rating,
                review = EXCLUDED.review,
                updated_at = NOW()
            RETURNING *
            "#,
        )
        .bind(template_id)
        .bind(org_id)
        .bind(user_id)
        .bind(request.rating.clamp(1, 5))
        .bind(&request.review)
        .fetch_one(&self.pool)
        .await?;

        // Update average rating
        sqlx::query(
            r#"
            UPDATE workflow_templates
            SET avg_rating = (
                SELECT AVG(rating)::real
                FROM workflow_template_ratings
                WHERE template_id = $1
            )
            WHERE id = $1
            "#,
        )
        .bind(template_id)
        .execute(&self.pool)
        .await?;

        Ok(rating)
    }

    /// Get ratings for a template.
    pub async fn list_ratings(
        &self,
        template_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<WorkflowTemplateRating>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT * FROM workflow_template_ratings
            WHERE template_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(template_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
    }

    /// Seed built-in templates.
    pub async fn seed_builtin_templates(&self) -> Result<usize, sqlx::Error> {
        let templates = crate::models::get_builtin_templates();
        let mut count = 0;

        for (template_data, actions) in templates {
            // Check if template already exists by name
            let existing: Option<(Uuid,)> = sqlx::query_as(
                "SELECT id FROM workflow_templates WHERE name = $1 AND scope = 'global'",
            )
            .bind(&template_data.name)
            .fetch_optional(&self.pool)
            .await?;

            if existing.is_some() {
                continue;
            }

            // Create template
            let template = self.create(template_data).await?;

            // Add actions
            for mut action in actions {
                action.template_id = template.id;
                self.add_action(action).await?;
            }

            count += 1;
        }

        Ok(count)
    }
}

/// Substitute variables in a JSON value.
fn substitute_variables(
    value: &mut serde_json::Value,
    vars: &serde_json::Map<String, serde_json::Value>,
) {
    match value {
        serde_json::Value::String(s) => {
            for (key, val) in vars {
                let placeholder = format!("{{{{{}}}}}", key);
                if let Some(replacement) = val.as_str() {
                    *s = s.replace(&placeholder, replacement);
                }
            }
        }
        serde_json::Value::Object(map) => {
            for (_, v) in map.iter_mut() {
                substitute_variables(v, vars);
            }
        }
        serde_json::Value::Array(arr) => {
            for v in arr.iter_mut() {
                substitute_variables(v, vars);
            }
        }
        _ => {}
    }
}

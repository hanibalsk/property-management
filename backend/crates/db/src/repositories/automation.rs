//! Workflow Automation repository (Epic 38).
//!
//! Repository for automation rules, templates, and execution logs.

use crate::models::automation::*;
use crate::DbPool;
use chrono::Utc;
use sqlx::Error as SqlxError;
use uuid::Uuid;

/// Repository for workflow automation operations.
#[derive(Clone)]
pub struct AutomationRepository {
    pool: DbPool,
}

impl AutomationRepository {
    /// Create a new AutomationRepository.
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    // ========================================================================
    // Automation Rules (Story 38.1)
    // ========================================================================

    /// Create an automation rule.
    pub async fn create_rule(
        &self,
        organization_id: Uuid,
        created_by: Uuid,
        data: CreateAutomationRule,
    ) -> Result<WorkflowAutomationRule, SqlxError> {
        sqlx::query_as::<_, WorkflowAutomationRule>(
            r#"
            INSERT INTO workflow_automation_rules (
                organization_id, name, description, trigger_type, trigger_config,
                conditions, actions, is_active, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, COALESCE($8, true), $9)
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(&data.name)
        .bind(&data.description)
        .bind(&data.trigger_type)
        .bind(&data.trigger_config)
        .bind(&data.conditions)
        .bind(&data.actions)
        .bind(data.is_active)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
    }

    /// Get automation rule by ID.
    pub async fn get_rule(&self, id: Uuid) -> Result<Option<WorkflowAutomationRule>, SqlxError> {
        sqlx::query_as::<_, WorkflowAutomationRule>(
            "SELECT * FROM workflow_automation_rules WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    /// List automation rules for an organization.
    pub async fn list_rules(
        &self,
        organization_id: Uuid,
    ) -> Result<Vec<WorkflowAutomationRule>, SqlxError> {
        sqlx::query_as::<_, WorkflowAutomationRule>(
            "SELECT * FROM workflow_automation_rules WHERE organization_id = $1 ORDER BY created_at DESC",
        )
        .bind(organization_id)
        .fetch_all(&self.pool)
        .await
    }

    /// Update automation rule.
    pub async fn update_rule(
        &self,
        id: Uuid,
        data: UpdateAutomationRule,
    ) -> Result<WorkflowAutomationRule, SqlxError> {
        sqlx::query_as::<_, WorkflowAutomationRule>(
            r#"
            UPDATE workflow_automation_rules SET
                name = COALESCE($2, name),
                description = COALESCE($3, description),
                trigger_config = COALESCE($4, trigger_config),
                conditions = COALESCE($5, conditions),
                actions = COALESCE($6, actions),
                is_active = COALESCE($7, is_active),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&data.name)
        .bind(&data.description)
        .bind(&data.trigger_config)
        .bind(&data.conditions)
        .bind(&data.actions)
        .bind(data.is_active)
        .fetch_one(&self.pool)
        .await
    }

    /// Delete automation rule.
    pub async fn delete_rule(&self, id: Uuid) -> Result<bool, SqlxError> {
        let result = sqlx::query("DELETE FROM workflow_automation_rules WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Toggle rule active status.
    pub async fn toggle_rule(&self, id: Uuid, is_active: bool) -> Result<(), SqlxError> {
        sqlx::query(
            "UPDATE workflow_automation_rules SET is_active = $2, updated_at = NOW() WHERE id = $1",
        )
        .bind(id)
        .bind(is_active)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    // ========================================================================
    // Automation Logs
    // ========================================================================

    /// Log rule execution.
    pub async fn log_execution(
        &self,
        rule_id: Uuid,
        trigger_data: Option<serde_json::Value>,
    ) -> Result<WorkflowAutomationLog, SqlxError> {
        sqlx::query_as::<_, WorkflowAutomationLog>(
            r#"
            INSERT INTO workflow_automation_logs (rule_id, trigger_data, status)
            VALUES ($1, $2, 'running')
            RETURNING *
            "#,
        )
        .bind(rule_id)
        .bind(&trigger_data)
        .fetch_one(&self.pool)
        .await
    }

    /// Complete log execution.
    pub async fn complete_execution(
        &self,
        log_id: Uuid,
        status: &str,
        actions_executed: serde_json::Value,
        error_message: Option<String>,
    ) -> Result<(), SqlxError> {
        let started_at: chrono::DateTime<Utc> =
            sqlx::query_scalar("SELECT started_at FROM workflow_automation_logs WHERE id = $1")
                .bind(log_id)
                .fetch_one(&self.pool)
                .await?;

        let duration_ms = (Utc::now() - started_at).num_milliseconds() as i32;

        sqlx::query(
            r#"
            UPDATE workflow_automation_logs SET
                status = $2,
                actions_executed = $3,
                error_message = $4,
                completed_at = NOW(),
                duration_ms = $5
            WHERE id = $1
            "#,
        )
        .bind(log_id)
        .bind(status)
        .bind(&actions_executed)
        .bind(&error_message)
        .bind(duration_ms)
        .execute(&self.pool)
        .await?;

        // Update rule stats
        if status == "success" {
            sqlx::query(
                r#"
                UPDATE workflow_automation_rules SET
                    run_count = run_count + 1,
                    last_run_at = NOW()
                WHERE id = (SELECT rule_id FROM workflow_automation_logs WHERE id = $1)
                "#,
            )
            .bind(log_id)
            .execute(&self.pool)
            .await?;
        } else {
            sqlx::query(
                r#"
                UPDATE workflow_automation_rules SET
                    run_count = run_count + 1,
                    error_count = error_count + 1,
                    last_run_at = NOW(),
                    last_error = $2
                WHERE id = (SELECT rule_id FROM workflow_automation_logs WHERE id = $1)
                "#,
            )
            .bind(log_id)
            .bind(&error_message)
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    /// Get execution logs for a rule.
    pub async fn get_rule_logs(
        &self,
        rule_id: Uuid,
        limit: i32,
    ) -> Result<Vec<WorkflowAutomationLog>, SqlxError> {
        sqlx::query_as::<_, WorkflowAutomationLog>(
            r#"
            SELECT * FROM workflow_automation_logs
            WHERE rule_id = $1
            ORDER BY started_at DESC
            LIMIT $2
            "#,
        )
        .bind(rule_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
    }

    // ========================================================================
    // Automation Templates
    // ========================================================================

    /// List all templates.
    pub async fn list_templates(&self) -> Result<Vec<WorkflowAutomationTemplate>, SqlxError> {
        sqlx::query_as::<_, WorkflowAutomationTemplate>(
            "SELECT * FROM workflow_automation_templates ORDER BY category, name",
        )
        .fetch_all(&self.pool)
        .await
    }

    /// Get template by ID.
    pub async fn get_template(
        &self,
        id: Uuid,
    ) -> Result<Option<WorkflowAutomationTemplate>, SqlxError> {
        sqlx::query_as::<_, WorkflowAutomationTemplate>(
            "SELECT * FROM workflow_automation_templates WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Create rule from template.
    pub async fn create_from_template(
        &self,
        organization_id: Uuid,
        created_by: Uuid,
        data: CreateRuleFromTemplate,
    ) -> Result<WorkflowAutomationRule, SqlxError> {
        let template = self
            .get_template(data.template_id)
            .await?
            .ok_or_else(|| SqlxError::RowNotFound)?;

        // Merge overrides with template
        let trigger_config = if let Some(overrides) = data.trigger_config_overrides {
            let mut base = template.trigger_config_template.clone();
            if let (Some(base_obj), Some(override_obj)) =
                (base.as_object_mut(), overrides.as_object())
            {
                for (k, v) in override_obj {
                    base_obj.insert(k.clone(), v.clone());
                }
            }
            base
        } else {
            template.trigger_config_template
        };

        let actions = if let Some(overrides) = data.actions_overrides {
            overrides
        } else {
            template.actions_template
        };

        self.create_rule(
            organization_id,
            created_by,
            CreateAutomationRule {
                name: data.name,
                description: data.description.or(template.description),
                trigger_type: template.trigger_type,
                trigger_config,
                conditions: None,
                actions,
                is_active: Some(true),
            },
        )
        .await
    }

    // ========================================================================
    // Scheduled Execution
    // ========================================================================

    /// Get rules due for scheduled execution.
    pub async fn get_due_rules(&self) -> Result<Vec<WorkflowAutomationRule>, SqlxError> {
        sqlx::query_as::<_, WorkflowAutomationRule>(
            r#"
            SELECT * FROM workflow_automation_rules
            WHERE is_active = true
              AND trigger_type = 'schedule'
              AND (next_run_at IS NULL OR next_run_at <= NOW())
            "#,
        )
        .fetch_all(&self.pool)
        .await
    }

    /// Update next run time for a scheduled rule.
    pub async fn update_next_run(
        &self,
        id: Uuid,
        next_run_at: chrono::DateTime<Utc>,
    ) -> Result<(), SqlxError> {
        sqlx::query("UPDATE workflow_automation_rules SET next_run_at = $2 WHERE id = $1")
            .bind(id)
            .bind(next_run_at)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

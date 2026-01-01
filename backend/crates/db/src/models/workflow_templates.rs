//! Workflow templates model (Epic 94, Story 94.4).
//!
//! Templates provide pre-built workflow configurations that can be
//! imported and customized by property managers.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Workflow template categories.
pub mod template_category {
    pub const ONBOARDING: &str = "onboarding";
    pub const REMINDERS: &str = "reminders";
    pub const ALERTS: &str = "alerts";
    pub const MAINTENANCE: &str = "maintenance";
    pub const FINANCIAL: &str = "financial";
    pub const COMMUNICATION: &str = "communication";
    pub const COMPLIANCE: &str = "compliance";

    pub const ALL: &[&str] = &[
        ONBOARDING,
        REMINDERS,
        ALERTS,
        MAINTENANCE,
        FINANCIAL,
        COMMUNICATION,
        COMPLIANCE,
    ];
}

/// Template scope (who can use the template).
pub mod template_scope {
    pub const GLOBAL: &str = "global"; // Available to all organizations
    pub const ORGANIZATION: &str = "organization"; // Created by and for specific org
    pub const PLATFORM: &str = "platform"; // Created by platform admin

    pub const ALL: &[&str] = &[GLOBAL, ORGANIZATION, PLATFORM];
}

/// A workflow template.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WorkflowTemplate {
    pub id: Uuid,
    /// Organization ID (null for global/platform templates)
    pub organization_id: Option<Uuid>,
    /// Template name
    pub name: String,
    /// Template description
    pub description: Option<String>,
    /// Category for grouping
    pub category: String,
    /// Trigger type this template is for
    pub trigger_type: String,
    /// Default trigger configuration
    pub trigger_config: sqlx::types::Json<serde_json::Value>,
    /// Default conditions
    pub conditions: sqlx::types::Json<Vec<serde_json::Value>>,
    /// Template scope
    pub scope: String,
    /// Number of times this template has been used
    pub use_count: i32,
    /// Average rating (1-5)
    pub avg_rating: Option<f32>,
    /// Tags for searchability
    pub tags: Vec<String>,
    /// Template icon/emoji
    pub icon: Option<String>,
    /// Whether this template is featured
    pub featured: bool,
    /// Whether this template is active
    pub active: bool,
    /// User who created the template
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Action within a workflow template.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WorkflowTemplateAction {
    pub id: Uuid,
    pub template_id: Uuid,
    pub action_order: i32,
    pub action_type: String,
    /// Action configuration (may contain placeholders)
    pub action_config: sqlx::types::Json<serde_json::Value>,
    /// Description of what this action does
    pub description: Option<String>,
    /// Failure handling
    pub on_failure: String,
    pub retry_count: i32,
    pub retry_delay_seconds: i32,
    pub created_at: DateTime<Utc>,
}

/// Configuration variable for a template.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WorkflowTemplateVariable {
    pub id: Uuid,
    pub template_id: Uuid,
    /// Variable name (referenced in config as {{variable_name}})
    pub name: String,
    /// Human-readable label
    pub label: String,
    /// Description of the variable
    pub description: Option<String>,
    /// Variable type (string, number, boolean, select)
    pub variable_type: String,
    /// Default value
    pub default_value: Option<String>,
    /// Whether this variable is required
    pub required: bool,
    /// For select type: available options
    pub options: Option<sqlx::types::Json<Vec<String>>>,
    /// Validation regex pattern
    pub validation_pattern: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Rating for a template.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WorkflowTemplateRating {
    pub id: Uuid,
    pub template_id: Uuid,
    pub organization_id: Uuid,
    pub user_id: Uuid,
    /// Rating value (1-5)
    pub rating: i32,
    /// Optional review text
    pub review: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request to create a workflow template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWorkflowTemplate {
    pub organization_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub trigger_type: String,
    pub trigger_config: Option<serde_json::Value>,
    pub conditions: Option<Vec<serde_json::Value>>,
    pub scope: Option<String>,
    pub tags: Option<Vec<String>>,
    pub icon: Option<String>,
    pub created_by: Option<Uuid>,
}

/// Request to update a workflow template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWorkflowTemplate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub trigger_config: Option<serde_json::Value>,
    pub conditions: Option<Vec<serde_json::Value>>,
    pub tags: Option<Vec<String>>,
    pub icon: Option<String>,
    pub featured: Option<bool>,
    pub active: Option<bool>,
}

/// Request to add an action to a template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTemplateAction {
    pub template_id: Uuid,
    pub action_order: i32,
    pub action_type: String,
    pub action_config: serde_json::Value,
    pub description: Option<String>,
    pub on_failure: Option<String>,
    pub retry_count: Option<i32>,
    pub retry_delay_seconds: Option<i32>,
}

/// Request to add a variable to a template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTemplateVariable {
    pub template_id: Uuid,
    pub name: String,
    pub label: String,
    pub description: Option<String>,
    pub variable_type: String,
    pub default_value: Option<String>,
    pub required: Option<bool>,
    pub options: Option<Vec<String>>,
    pub validation_pattern: Option<String>,
}

/// Request to import a template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportTemplateRequest {
    pub template_id: Uuid,
    /// Custom name for the workflow
    pub name: Option<String>,
    /// Variable values
    pub variables: serde_json::Value,
    /// Whether to enable the workflow immediately
    pub enabled: Option<bool>,
}

/// Request to rate a template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateTemplateRequest {
    pub rating: i32,
    pub review: Option<String>,
}

/// Query parameters for template search.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TemplateSearchQuery {
    pub category: Option<String>,
    pub trigger_type: Option<String>,
    pub search: Option<String>,
    pub tags: Option<Vec<String>>,
    pub featured: Option<bool>,
    pub scope: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Template with full details (actions and variables).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTemplateWithDetails {
    pub template: WorkflowTemplate,
    pub actions: Vec<WorkflowTemplateAction>,
    pub variables: Vec<WorkflowTemplateVariable>,
}

/// Template summary for listing.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WorkflowTemplateSummary {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub trigger_type: String,
    pub scope: String,
    pub use_count: i32,
    pub avg_rating: Option<f32>,
    pub tags: Vec<String>,
    pub icon: Option<String>,
    pub featured: bool,
    pub action_count: i64,
}

/// Pre-defined common workflow templates.
pub fn get_builtin_templates() -> Vec<(CreateWorkflowTemplate, Vec<CreateTemplateAction>)> {
    vec![
        // Payment reminder template
        (
            CreateWorkflowTemplate {
                organization_id: None,
                name: "Payment Reminder".to_string(),
                description: Some("Send reminders for overdue payments".to_string()),
                category: template_category::REMINDERS.to_string(),
                trigger_type: "payment_overdue".to_string(),
                trigger_config: Some(serde_json::json!({
                    "min_amount": 0
                })),
                conditions: Some(vec![]),
                scope: Some(template_scope::GLOBAL.to_string()),
                tags: Some(vec![
                    "payment".to_string(),
                    "reminder".to_string(),
                    "financial".to_string(),
                ]),
                icon: Some("💰".to_string()),
                created_by: None,
            },
            vec![
                CreateTemplateAction {
                    template_id: Uuid::nil(), // Will be replaced
                    action_order: 1,
                    action_type: "send_email".to_string(),
                    action_config: serde_json::json!({
                        "to": "{{trigger.user_email}}",
                        "subject": "Payment Reminder: {{trigger.amount}} due",
                        "template": "payment_reminder",
                        "template_data": {
                            "amount": "{{trigger.amount}}",
                            "due_date": "{{trigger.due_date}}"
                        }
                    }),
                    description: Some("Send initial email reminder".to_string()),
                    on_failure: Some("continue".to_string()),
                    retry_count: Some(2),
                    retry_delay_seconds: Some(60),
                },
                CreateTemplateAction {
                    template_id: Uuid::nil(),
                    action_order: 2,
                    action_type: "send_notification".to_string(),
                    action_config: serde_json::json!({
                        "title": "Payment Reminder",
                        "message": "You have an overdue payment of {{trigger.amount}}",
                        "target": {"dynamic": "trigger.user_id"},
                        "priority": 2
                    }),
                    description: Some("Send in-app notification".to_string()),
                    on_failure: Some("continue".to_string()),
                    retry_count: Some(1),
                    retry_delay_seconds: Some(30),
                },
            ],
        ),
        // Fault escalation template
        (
            CreateWorkflowTemplate {
                organization_id: None,
                name: "Fault Escalation".to_string(),
                description: Some("Escalate unresolved faults after a delay".to_string()),
                category: template_category::ALERTS.to_string(),
                trigger_type: "fault_created".to_string(),
                trigger_config: Some(serde_json::json!({
                    "fault_priorities": ["critical", "high"]
                })),
                conditions: Some(vec![]),
                scope: Some(template_scope::GLOBAL.to_string()),
                tags: Some(vec![
                    "fault".to_string(),
                    "escalation".to_string(),
                    "maintenance".to_string(),
                ]),
                icon: Some("🚨".to_string()),
                created_by: None,
            },
            vec![
                CreateTemplateAction {
                    template_id: Uuid::nil(),
                    action_order: 1,
                    action_type: "delay".to_string(),
                    action_config: serde_json::json!({
                        "duration": 24,
                        "unit": "hours",
                        "reason": "Wait for initial response"
                    }),
                    description: Some("Wait 24 hours for initial response".to_string()),
                    on_failure: None,
                    retry_count: None,
                    retry_delay_seconds: None,
                },
                CreateTemplateAction {
                    template_id: Uuid::nil(),
                    action_order: 2,
                    action_type: "send_notification".to_string(),
                    action_config: serde_json::json!({
                        "title": "Fault Escalation Alert",
                        "message": "Fault '{{trigger.title}}' requires attention ({{trigger.priority}} priority)",
                        "target": {"role": "manager"},
                        "priority": 1,
                        "channel": "all"
                    }),
                    description: Some("Notify managers about unresolved fault".to_string()),
                    on_failure: Some("continue".to_string()),
                    retry_count: Some(2),
                    retry_delay_seconds: Some(60),
                },
                CreateTemplateAction {
                    template_id: Uuid::nil(),
                    action_order: 3,
                    action_type: "send_email".to_string(),
                    action_config: serde_json::json!({
                        "to": "{{escalation_email}}",
                        "subject": "ESCALATION: Unresolved Fault - {{trigger.title}}",
                        "body": "A {{trigger.priority}} priority fault has not been addressed within 24 hours.\n\nFault: {{trigger.title}}\nDescription: {{trigger.description}}\nBuilding: {{trigger.building_name}}\n\nPlease take immediate action.",
                        "priority": "high"
                    }),
                    description: Some("Send escalation email".to_string()),
                    on_failure: Some("stop".to_string()),
                    retry_count: Some(3),
                    retry_delay_seconds: Some(120),
                },
            ],
        ),
        // Lease expiry alert template
        (
            CreateWorkflowTemplate {
                organization_id: None,
                name: "Lease Expiry Alert".to_string(),
                description: Some("Notify about upcoming lease expirations".to_string()),
                category: template_category::REMINDERS.to_string(),
                trigger_type: "schedule".to_string(),
                trigger_config: Some(serde_json::json!({
                    "cron": "0 9 * * 1", // Every Monday at 9 AM
                    "check_type": "lease_expiry",
                    "days_ahead": 60
                })),
                conditions: Some(vec![]),
                scope: Some(template_scope::GLOBAL.to_string()),
                tags: Some(vec![
                    "lease".to_string(),
                    "expiry".to_string(),
                    "reminder".to_string(),
                ]),
                icon: Some("📋".to_string()),
                created_by: None,
            },
            vec![
                CreateTemplateAction {
                    template_id: Uuid::nil(),
                    action_order: 1,
                    action_type: "send_email".to_string(),
                    action_config: serde_json::json!({
                        "to": "{{trigger.tenant_email}}",
                        "subject": "Lease Renewal Notice - {{trigger.unit_name}}",
                        "body": "Dear {{trigger.tenant_name}},\n\nYour lease for {{trigger.unit_name}} is set to expire on {{trigger.expiry_date}}.\n\nPlease contact us to discuss renewal options.\n\nBest regards,\n{{building_name}} Management"
                    }),
                    description: Some("Send lease expiry notice to tenant".to_string()),
                    on_failure: Some("continue".to_string()),
                    retry_count: Some(2),
                    retry_delay_seconds: Some(300),
                },
                CreateTemplateAction {
                    template_id: Uuid::nil(),
                    action_order: 2,
                    action_type: "send_notification".to_string(),
                    action_config: serde_json::json!({
                        "title": "Lease Expiring Soon",
                        "message": "Lease for {{trigger.unit_name}} expires on {{trigger.expiry_date}}",
                        "target": {"role": "manager"},
                        "priority": 2
                    }),
                    description: Some("Notify property manager".to_string()),
                    on_failure: Some("continue".to_string()),
                    retry_count: Some(1),
                    retry_delay_seconds: Some(60),
                },
            ],
        ),
        // New resident welcome template
        (
            CreateWorkflowTemplate {
                organization_id: None,
                name: "New Resident Welcome".to_string(),
                description: Some("Welcome new residents with helpful information".to_string()),
                category: template_category::ONBOARDING.to_string(),
                trigger_type: "document_signed".to_string(),
                trigger_config: Some(serde_json::json!({
                    "document_types": ["lease"]
                })),
                conditions: Some(vec![]),
                scope: Some(template_scope::GLOBAL.to_string()),
                tags: Some(vec![
                    "onboarding".to_string(),
                    "welcome".to_string(),
                    "resident".to_string(),
                ]),
                icon: Some("🏠".to_string()),
                created_by: None,
            },
            vec![
                CreateTemplateAction {
                    template_id: Uuid::nil(),
                    action_order: 1,
                    action_type: "delay".to_string(),
                    action_config: serde_json::json!({
                        "duration": 1,
                        "unit": "hours",
                        "reason": "Allow processing time"
                    }),
                    description: Some("Brief delay for processing".to_string()),
                    on_failure: None,
                    retry_count: None,
                    retry_delay_seconds: None,
                },
                CreateTemplateAction {
                    template_id: Uuid::nil(),
                    action_order: 2,
                    action_type: "send_email".to_string(),
                    action_config: serde_json::json!({
                        "to": "{{trigger.signer_email}}",
                        "subject": "Welcome to {{building_name}}!",
                        "body": "Dear {{trigger.signer_name}},\n\nWelcome to {{building_name}}! We're delighted to have you.\n\nHere are some helpful resources:\n- Building rules and regulations\n- Emergency contacts\n- Amenities schedule\n\nFeel free to reach out with any questions.\n\nBest regards,\n{{building_name}} Management",
                        "html": false
                    }),
                    description: Some("Send welcome email".to_string()),
                    on_failure: Some("continue".to_string()),
                    retry_count: Some(2),
                    retry_delay_seconds: Some(60),
                },
                CreateTemplateAction {
                    template_id: Uuid::nil(),
                    action_order: 3,
                    action_type: "send_notification".to_string(),
                    action_config: serde_json::json!({
                        "title": "Welcome to {{building_name}}!",
                        "message": "Check your email for important move-in information.",
                        "target": {"dynamic": "trigger.signer_id"},
                        "priority": 3,
                        "action_url": "/onboarding"
                    }),
                    description: Some("Send welcome notification".to_string()),
                    on_failure: Some("continue".to_string()),
                    retry_count: Some(1),
                    retry_delay_seconds: Some(30),
                },
            ],
        ),
    ]
}

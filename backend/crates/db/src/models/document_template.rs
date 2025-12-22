//! Document template model (Epic 7B: Story 7B.2 - Document Templates & Generation).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

// ============================================================================
// Template Types
// ============================================================================

/// Template type enum matching database enum.
pub mod template_type {
    pub const LEASE: &str = "lease";
    pub const NOTICE: &str = "notice";
    pub const INVOICE: &str = "invoice";
    pub const REPORT: &str = "report";
    pub const MINUTES: &str = "minutes";
    pub const CONTRACT: &str = "contract";
    pub const CUSTOM: &str = "custom";

    pub const ALL: &[&str] = &[LEASE, NOTICE, INVOICE, REPORT, MINUTES, CONTRACT, CUSTOM];
}

/// Placeholder data type for template variables.
pub mod placeholder_type {
    pub const TEXT: &str = "text";
    pub const DATE: &str = "date";
    pub const NUMBER: &str = "number";
    pub const CURRENCY: &str = "currency";

    pub const ALL: &[&str] = &[TEXT, DATE, NUMBER, CURRENCY];
}

// ============================================================================
// Template Entities
// ============================================================================

/// Document template entity from database.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct DocumentTemplate {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub template_type: String,
    pub content: String,
    pub placeholders: serde_json::Value,
    pub usage_count: i32,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl DocumentTemplate {
    /// Get placeholders as typed vector.
    pub fn get_placeholders(&self) -> Vec<TemplatePlaceholder> {
        serde_json::from_value(self.placeholders.clone()).unwrap_or_default()
    }

    /// Get required placeholders.
    pub fn required_placeholders(&self) -> Vec<TemplatePlaceholder> {
        self.get_placeholders()
            .into_iter()
            .filter(|p| p.required)
            .collect()
    }

    /// Validate that all required placeholders have values.
    pub fn validate_values(&self, values: &std::collections::HashMap<String, String>) -> Result<(), Vec<String>> {
        let missing: Vec<String> = self
            .required_placeholders()
            .iter()
            .filter(|p| !values.contains_key(&p.name))
            .map(|p| p.name.clone())
            .collect();

        if missing.is_empty() {
            Ok(())
        } else {
            Err(missing)
        }
    }

    /// Generate document content by replacing placeholders with values.
    pub fn generate_content(&self, values: &std::collections::HashMap<String, String>) -> String {
        let mut content = self.content.clone();

        // Replace all placeholders with provided values or defaults
        for placeholder in self.get_placeholders() {
            let pattern = format!("{{{{{}}}}}", placeholder.name);
            let value = values
                .get(&placeholder.name)
                .cloned()
                .or(placeholder.default_value)
                .unwrap_or_default();
            content = content.replace(&pattern, &value);
        }

        content
    }
}

/// Template placeholder definition.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TemplatePlaceholder {
    /// Placeholder name (used in {{name}} syntax).
    pub name: String,
    /// Data type: text, date, number, currency.
    #[serde(rename = "type")]
    pub placeholder_type: String,
    /// Whether this placeholder is required.
    pub required: bool,
    /// Default value if not provided.
    pub default_value: Option<String>,
    /// Human-readable description.
    pub description: Option<String>,
}

/// Template summary for list views.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct TemplateSummary {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub template_type: String,
    pub usage_count: i32,
    pub placeholder_count: i64,
    pub created_at: DateTime<Utc>,
}

/// Template with creator details.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TemplateWithDetails {
    #[serde(flatten)]
    pub template: DocumentTemplate,
    pub created_by_name: String,
}

// ============================================================================
// Request/Response Types
// ============================================================================

/// Request to create a new template.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateTemplate {
    pub organization_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub template_type: String,
    pub content: String,
    pub placeholders: Vec<TemplatePlaceholder>,
    pub created_by: Uuid,
}

/// Request to update a template.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateTemplate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub template_type: Option<String>,
    pub content: Option<String>,
    pub placeholders: Option<Vec<TemplatePlaceholder>>,
}

/// Query parameters for listing templates.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
pub struct TemplateListQuery {
    pub template_type: Option<String>,
    pub search: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Request to generate a document from a template.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GenerateDocumentRequest {
    /// Values for template placeholders.
    pub values: std::collections::HashMap<String, String>,
    /// Title for the generated document.
    pub title: String,
    /// Optional description for the generated document.
    pub description: Option<String>,
    /// Category for the generated document.
    pub category: String,
    /// Folder to place the generated document.
    pub folder_id: Option<Uuid>,
}

/// Response after generating a document.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GenerateDocumentResponse {
    pub document_id: Uuid,
    pub message: String,
}

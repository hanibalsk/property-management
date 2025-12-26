# Story 7B.2: Document Templates & Generation

## Story

As a **property manager**,
I want to **create and use document templates**,
So that **I can quickly generate standardized documents with placeholder data**.

## Status

done

## Acceptance Criteria

1. **Given** a user creates a document template
   **When** they define placeholders (e.g., {{tenant_name}}, {{unit_number}})
   **Then** the template is saved with placeholder definitions
   **And** can be used to generate new documents

2. **Given** a user generates a document from a template
   **When** they provide values for placeholders
   **Then** a new document is created with placeholders replaced
   **And** the generated document is linked to the source template

3. **Given** a user views available templates
   **When** they browse the template library
   **Then** they see template name, description, and required placeholders
   **And** can preview template structure

## Tasks/Subtasks

- [x] **Task 1: Database Schema for Templates**
  - [x] Create migration 00036 for document_templates table
  - [x] Add template_content (markdown/HTML storage)
  - [x] Add placeholders JSONB column for placeholder definitions
  - [x] Add template_type enum (lease, notice, invoice, report, custom)
  - [x] Add generated_documents tracking

- [x] **Task 2: Update Models**
  - [x] Create DocumentTemplate model
  - [x] Create TemplatePlaceholder struct for placeholder metadata
  - [x] Create GenerateDocumentRequest for document generation
  - [x] Create template response types

- [x] **Task 3: Repository Methods**
  - [x] Implement create_template()
  - [x] Implement find_template_by_id()
  - [x] Implement list_templates()
  - [x] Implement generate_document() - replace placeholders and create doc

- [x] **Task 4: API Endpoints**
  - [x] POST /api/v1/templates - Create template
  - [x] GET /api/v1/templates - List templates
  - [x] GET /api/v1/templates/:id - Get template details
  - [x] PUT /api/v1/templates/:id - Update template
  - [x] DELETE /api/v1/templates/:id - Delete template
  - [x] POST /api/v1/templates/:id/generate - Generate document from template

## Dev Notes

### Technical Specifications
- Placeholders use Mustache-style syntax: {{placeholder_name}}
- Template content stored as markdown with embedded placeholders
- Generated documents stored as new document records with template_id reference
- Placeholder definitions include: name, type (text, date, number), required flag

## Dev Agent Record

### Implementation Plan
All tasks have been implemented as part of the Epic 7B document templates work.

## File List

### Database Migration
- `backend/crates/db/migrations/00036_add_document_templates.sql` - Template schema

### Models
- `backend/crates/db/src/models/document_template.rs` - DocumentTemplate, TemplatePlaceholder types
- `backend/crates/db/src/models/mod.rs` - Model exports

### Repository
- `backend/crates/db/src/repositories/document_template.rs` - Template repository

### API Routes
- `backend/servers/api-server/src/routes/templates.rs` - Template API endpoints
- `backend/servers/api-server/src/routes/mod.rs` - Route exports
- `backend/servers/api-server/src/main.rs` - Route registration

### State
- `backend/servers/api-server/src/state.rs` - DocumentTemplateRepository in AppState

## Change Log

| Date | Change | Author |
|------|--------|--------|
| 2025-12-22 | Story created | AI Agent |
| 2025-12-26 | Story verified complete - all tasks implemented | AI Agent |

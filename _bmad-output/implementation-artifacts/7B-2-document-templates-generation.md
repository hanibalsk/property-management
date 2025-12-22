# Story 7B.2: Document Templates & Generation

## Story

As a **property manager**,
I want to **create and use document templates**,
So that **I can quickly generate standardized documents with placeholder data**.

## Status

in-progress

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

- [ ] **Task 1: Database Schema for Templates**
  - [ ] Create migration 00036 for document_templates table
  - [ ] Add template_content (markdown/HTML storage)
  - [ ] Add placeholders JSONB column for placeholder definitions
  - [ ] Add template_type enum (lease, notice, invoice, report, custom)
  - [ ] Add generated_documents tracking

- [ ] **Task 2: Update Models**
  - [ ] Create DocumentTemplate model
  - [ ] Create TemplatePlaceholder struct for placeholder metadata
  - [ ] Create GenerateDocumentRequest for document generation
  - [ ] Create template response types

- [ ] **Task 3: Repository Methods**
  - [ ] Implement create_template()
  - [ ] Implement find_template_by_id()
  - [ ] Implement list_templates()
  - [ ] Implement generate_document() - replace placeholders and create doc

- [ ] **Task 4: API Endpoints**
  - [ ] POST /api/v1/templates - Create template
  - [ ] GET /api/v1/templates - List templates
  - [ ] GET /api/v1/templates/:id - Get template details
  - [ ] PUT /api/v1/templates/:id - Update template
  - [ ] DELETE /api/v1/templates/:id - Delete template
  - [ ] POST /api/v1/templates/:id/generate - Generate document from template

## Dev Notes

### Technical Specifications
- Placeholders use Mustache-style syntax: {{placeholder_name}}
- Template content stored as markdown with embedded placeholders
- Generated documents stored as new document records with template_id reference
- Placeholder definitions include: name, type (text, date, number), required flag

## Dev Agent Record

### Implementation Plan
*To be filled during implementation*

## File List

*To be filled during implementation*

## Change Log

| Date | Change | Author |
|------|--------|--------|
| 2025-12-22 | Story created | AI Agent |

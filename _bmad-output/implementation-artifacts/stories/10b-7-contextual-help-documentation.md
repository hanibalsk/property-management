# Story 10B.7: Contextual Help & Documentation

Status: ready-for-dev

## Story

As a **user**,
I want to **access contextual help**,
So that **I can learn features without leaving the app**.

## Acceptance Criteria

1. **AC-1: Contextual Help Panel**
   - Given a user clicks help icon on a screen
   - When help panel opens
   - Then relevant documentation is displayed
   - And links to full docs if needed

2. **AC-2: Help Search**
   - Given a user searches in help
   - When they enter a query
   - Then matching help articles are returned
   - And can be viewed inline

3. **AC-3: Documentation Updates**
   - Given documentation is updated
   - When the change is published
   - Then in-app help reflects updates immediately

## Tasks / Subtasks

- [ ] Task 1: Database Schema & Migrations (AC: 1, 2, 3)
  - [ ] 1.1 Create `help_articles` table: id (UUID), key (VARCHAR unique), title, content (TEXT), route_pattern, feature_key, tags (JSONB), is_published, version, created_at, updated_at
  - [ ] 1.2 Create `help_article_revisions` table: id, article_id (FK), title, content, published_by, published_at
  - [ ] 1.3 Add full-text search index on help_articles (title, content, tags)
  - [ ] 1.4 Seed initial help articles for core features

- [ ] Task 2: Help Article Models (AC: 1, 2, 3)
  - [ ] 2.1 Create HelpArticle model: id, key, title, content, route_pattern, feature_key, tags, is_published, version
  - [ ] 2.2 Create HelpArticleRevision model: id, article_id, title, content, published_by, published_at
  - [ ] 2.3 Create DTOs: HelpArticleResponse, SearchHelpRequest, SearchHelpResponse, CreateHelpArticleRequest

- [ ] Task 3: Help Article Repository (AC: 1, 2, 3)
  - [ ] 3.1 Create HelpArticleRepository
  - [ ] 3.2 Implement get_by_route() matching current page route pattern
  - [ ] 3.3 Implement get_by_feature_key() for feature-specific help
  - [ ] 3.4 Implement search_articles() with full-text search
  - [ ] 3.5 Implement publish_article() creating revision and updating live content
  - [ ] 3.6 Implement get_article_history() for revision tracking

- [ ] Task 4: Help Article Service (AC: 1, 2, 3)
  - [ ] 4.1 Create HelpArticleService for help orchestration
  - [ ] 4.2 Implement get_contextual_help() resolving help by route or feature key
  - [ ] 4.3 Implement search_help() with relevance scoring
  - [ ] 4.4 Implement publish_article() with version increment and revision creation
  - [ ] 4.5 Implement get_related_articles() suggesting similar content

- [ ] Task 5: Help Article API Endpoints (AC: 1, 2, 3)
  - [ ] 5.1 GET /api/v1/help/context - get help for current route/feature (query params: route, feature_key)
  - [ ] 5.2 GET /api/v1/help/search - search help articles (query param: q)
  - [ ] 5.3 GET /api/v1/help/articles/:key - get specific article by key
  - [ ] 5.4 GET /api/v1/help/articles - list all published articles (for help index)

- [ ] Task 6: Admin Help Management (AC: 3)
  - [ ] 6.1 GET /api/v1/platform-admin/help/articles - list all articles (including unpublished)
  - [ ] 6.2 POST /api/v1/platform-admin/help/articles - create new article (draft)
  - [ ] 6.3 PUT /api/v1/platform-admin/help/articles/:id - update article content
  - [ ] 6.4 POST /api/v1/platform-admin/help/articles/:id/publish - publish article
  - [ ] 6.5 DELETE /api/v1/platform-admin/help/articles/:id - unpublish/delete article
  - [ ] 6.6 GET /api/v1/platform-admin/help/articles/:id/revisions - view revision history

- [ ] Task 7: Unit & Integration Tests (AC: 1, 2, 3)
  - [ ] 7.1 Test contextual help resolution by route pattern
  - [ ] 7.2 Test full-text search functionality
  - [ ] 7.3 Test article publish flow with versioning
  - [ ] 7.4 Test revision history tracking
  - [ ] 7.5 Test authorization - public read, admin write

## Dev Notes

### Architecture Requirements
- Help articles linked to routes via pattern matching (e.g., "/buildings/*" matches "/buildings/123")
- feature_key provides alternative lookup for component-level help
- Full-text search using PostgreSQL tsvector/tsquery
- Published articles cached for fast access

### Technical Specifications
- Backend: Rust + Axum following existing patterns
- Content stored as Markdown, rendered on frontend
- Route patterns support wildcards: "/buildings/*", "/faults/*/edit"
- Tags enable categorization and filtering

### Security Considerations
- All users can read published help articles
- Only SuperAdmin can create, edit, or publish articles
- Draft articles not visible to regular users

### Database Patterns
- Follow existing model patterns in crates/db/src/models/
- Full-text search with GIN index on tsvector column
- Revision history for rollback capability

### References
- [Source: _bmad-output/epics.md#Epic-10B-Story-10B.7]
- PostgreSQL full-text search: https://www.postgresql.org/docs/current/textsearch.html

## Dev Agent Record

### Agent Model Used

TBD

### Debug Log References

N/A

### Completion Notes List

(To be filled during implementation)

### File List

(To be filled during implementation)

## Change Log

| Date | Change |
|------|--------|
| 2025-12-21 | Story created |

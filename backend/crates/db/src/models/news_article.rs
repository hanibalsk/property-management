//! News article model (Epic 59: News & Media Management).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Article status enum values.
pub mod article_status {
    pub const DRAFT: &str = "draft";
    pub const PUBLISHED: &str = "published";
    pub const ARCHIVED: &str = "archived";

    pub const ALL: &[&str] = &[DRAFT, PUBLISHED, ARCHIVED];
}

/// Reaction type enum values.
pub mod reaction_type {
    pub const LIKE: &str = "like";
    pub const LOVE: &str = "love";
    pub const SURPRISED: &str = "surprised";
    pub const SAD: &str = "sad";
    pub const ANGRY: &str = "angry";

    pub const ALL: &[&str] = &[LIKE, LOVE, SURPRISED, SAD, ANGRY];
}

// ============================================================================
// News Article
// ============================================================================

/// News article entity from database.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct NewsArticle {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub cover_image_url: Option<String>,
    pub building_ids: serde_json::Value,
    pub status: String,
    pub published_at: Option<DateTime<Utc>>,
    pub archived_at: Option<DateTime<Utc>>,
    pub pinned: bool,
    pub pinned_at: Option<DateTime<Utc>>,
    pub pinned_by: Option<Uuid>,
    pub comments_enabled: bool,
    pub reactions_enabled: bool,
    pub view_count: i32,
    pub reaction_count: i32,
    pub comment_count: i32,
    pub share_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl NewsArticle {
    /// Check if article is in draft status.
    pub fn is_draft(&self) -> bool {
        self.status == article_status::DRAFT
    }

    /// Check if article is published.
    pub fn is_published(&self) -> bool {
        self.status == article_status::PUBLISHED
    }

    /// Check if article is archived.
    pub fn is_archived(&self) -> bool {
        self.status == article_status::ARCHIVED
    }

    /// Check if article can be edited.
    pub fn can_edit(&self) -> bool {
        self.status == article_status::DRAFT
    }

    /// Check if article is currently visible to users.
    pub fn is_visible(&self) -> bool {
        self.status == article_status::PUBLISHED
    }
}

/// Article summary view.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct ArticleSummary {
    pub id: Uuid,
    pub title: String,
    pub excerpt: Option<String>,
    pub cover_image_url: Option<String>,
    pub author_id: Uuid,
    pub status: String,
    pub published_at: Option<DateTime<Utc>>,
    pub pinned: bool,
    pub view_count: i32,
    pub reaction_count: i32,
    pub comment_count: i32,
    pub created_at: DateTime<Utc>,
}

/// Article with author details.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ArticleWithDetails {
    #[serde(flatten)]
    pub article: NewsArticle,
    pub author_name: String,
    pub author_avatar_url: Option<String>,
}

// ============================================================================
// Article Media
// ============================================================================

/// Article media/attachment entity.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct ArticleMedia {
    pub id: Uuid,
    pub article_id: Uuid,
    pub media_type: String,
    pub file_key: Option<String>,
    pub file_name: Option<String>,
    pub file_size: Option<i64>,
    pub mime_type: Option<String>,
    pub embed_url: Option<String>,
    pub embed_html: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub alt_text: Option<String>,
    pub caption: Option<String>,
    pub display_order: i32,
    pub created_at: DateTime<Utc>,
}

// ============================================================================
// Article Reactions
// ============================================================================

/// Article reaction entity.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct ArticleReaction {
    pub id: Uuid,
    pub article_id: Uuid,
    pub user_id: Uuid,
    pub reaction: String,
    pub created_at: DateTime<Utc>,
}

/// Reaction counts by type.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReactionCounts {
    pub like: i32,
    pub love: i32,
    pub surprised: i32,
    pub sad: i32,
    pub angry: i32,
    pub total: i32,
}

impl Default for ReactionCounts {
    fn default() -> Self {
        Self {
            like: 0,
            love: 0,
            surprised: 0,
            sad: 0,
            angry: 0,
            total: 0,
        }
    }
}

// ============================================================================
// Article Comments
// ============================================================================

/// Article comment entity.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct ArticleComment {
    pub id: Uuid,
    pub article_id: Uuid,
    pub user_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub content: String,
    pub is_moderated: bool,
    pub moderated_at: Option<DateTime<Utc>>,
    pub moderated_by: Option<Uuid>,
    pub moderation_reason: Option<String>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
    pub like_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ArticleComment {
    /// Check if comment is deleted.
    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    /// Check if comment is moderated.
    pub fn is_moderated_deleted(&self) -> bool {
        self.is_moderated && self.deleted_at.is_some()
    }
}

/// Comment with author details.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CommentWithAuthor {
    #[serde(flatten)]
    pub comment: ArticleComment,
    pub author_name: String,
    pub author_avatar_url: Option<String>,
    pub reply_count: i32,
}

/// Database row for comment with author join.
#[derive(Debug, Clone, FromRow)]
pub struct CommentWithAuthorRow {
    // Comment fields
    pub id: Uuid,
    pub article_id: Uuid,
    pub user_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub content: String,
    pub is_moderated: bool,
    pub moderated_at: Option<DateTime<Utc>>,
    pub moderated_by: Option<Uuid>,
    pub moderation_reason: Option<String>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
    pub like_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Author fields
    pub author_name: String,
    pub author_avatar_url: Option<String>,
    pub reply_count: i64,
}

impl From<CommentWithAuthorRow> for CommentWithAuthor {
    fn from(row: CommentWithAuthorRow) -> Self {
        Self {
            comment: ArticleComment {
                id: row.id,
                article_id: row.article_id,
                user_id: row.user_id,
                parent_id: row.parent_id,
                content: row.content,
                is_moderated: row.is_moderated,
                moderated_at: row.moderated_at,
                moderated_by: row.moderated_by,
                moderation_reason: row.moderation_reason,
                deleted_at: row.deleted_at,
                deleted_by: row.deleted_by,
                like_count: row.like_count,
                created_at: row.created_at,
                updated_at: row.updated_at,
            },
            author_name: row.author_name,
            author_avatar_url: row.author_avatar_url,
            reply_count: row.reply_count as i32,
        }
    }
}

// ============================================================================
// Article Views
// ============================================================================

/// Article view/read tracking entity.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct ArticleView {
    pub id: Uuid,
    pub article_id: Uuid,
    pub user_id: Option<Uuid>,
    pub viewed_at: DateTime<Utc>,
    pub duration_seconds: Option<i32>,
}

// ============================================================================
// Request/Response DTOs
// ============================================================================

/// Create article request (Story 59.1).
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateArticle {
    pub title: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub cover_image_url: Option<String>,
    #[serde(default)]
    pub building_ids: Vec<Uuid>,
    #[serde(default)]
    pub status: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
    #[serde(default = "default_true")]
    pub comments_enabled: bool,
    #[serde(default = "default_true")]
    pub reactions_enabled: bool,
}

fn default_true() -> bool {
    true
}

/// Update article request.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateArticle {
    pub title: Option<String>,
    pub content: Option<String>,
    pub excerpt: Option<String>,
    pub cover_image_url: Option<String>,
    pub building_ids: Option<Vec<Uuid>>,
    pub status: Option<String>,
    pub comments_enabled: Option<bool>,
    pub reactions_enabled: Option<bool>,
}

/// Publish article request.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct PublishArticle {
    pub published_at: Option<DateTime<Utc>>,
}

/// Archive article request (Story 59.4).
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ArchiveArticle {
    pub reason: Option<String>,
}

/// Pin article request.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct PinArticle {
    pub pinned: bool,
}

/// Create media attachment request.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateArticleMedia {
    pub media_type: String,
    pub file_key: Option<String>,
    pub file_name: Option<String>,
    pub file_size: Option<i64>,
    pub mime_type: Option<String>,
    pub embed_url: Option<String>,
    pub embed_html: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub alt_text: Option<String>,
    pub caption: Option<String>,
    pub display_order: Option<i32>,
}

/// Toggle reaction request (Story 59.2).
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ToggleReaction {
    pub reaction: String,
}

/// Create comment request (Story 59.3).
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateArticleComment {
    pub content: String,
    pub parent_id: Option<Uuid>,
}

/// Update comment request.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateArticleComment {
    pub content: String,
}

/// Moderate comment request.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ModerateComment {
    pub delete: bool,
    pub reason: Option<String>,
}

/// Article list query parameters.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ArticleListQuery {
    pub status: Option<String>,
    pub building_id: Option<Uuid>,
    pub pinned_only: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Article statistics.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct ArticleStatistics {
    pub total_articles: i64,
    pub published_articles: i64,
    pub draft_articles: i64,
    pub archived_articles: i64,
    pub total_views: i64,
    pub total_reactions: i64,
    pub total_comments: i64,
}

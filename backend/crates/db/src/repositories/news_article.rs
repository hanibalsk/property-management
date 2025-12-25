//! News article repository (Epic 59: News & Media Management).

use crate::models::news_article::{
    article_status, ArticleComment, ArticleListQuery, ArticleMedia, ArticleStatistics,
    ArticleSummary, ArticleView, ArticleWithDetails, CommentWithAuthor, CreateArticle,
    CreateArticleComment, CreateArticleMedia, NewsArticle, ReactionCounts, UpdateArticle,
};
use crate::DbPool;
use chrono::{DateTime, Utc};
use sqlx::Error as SqlxError;
use uuid::Uuid;

/// Repository for news article operations.
#[derive(Clone)]
pub struct NewsArticleRepository {
    pool: DbPool,
}

impl NewsArticleRepository {
    /// Create a new NewsArticleRepository.
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        organization_id: Uuid,
        author_id: Uuid,
        data: CreateArticle,
    ) -> Result<NewsArticle, SqlxError> {
        let building_ids_json = serde_json::to_value(&data.building_ids).unwrap_or_default();
        let status = data.status.as_deref().unwrap_or(article_status::DRAFT);

        let article = sqlx::query_as::<_, NewsArticle>(
            r#"
            INSERT INTO news_articles (
                organization_id, author_id, title, content, excerpt,
                cover_image_url, building_ids, status, published_at,
                comments_enabled, reactions_enabled
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(author_id)
        .bind(&data.title)
        .bind(&data.content)
        .bind(&data.excerpt)
        .bind(&data.cover_image_url)
        .bind(&building_ids_json)
        .bind(status)
        .bind(data.published_at)
        .bind(data.comments_enabled)
        .bind(data.reactions_enabled)
        .fetch_one(&self.pool)
        .await?;

        Ok(article)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<NewsArticle>, SqlxError> {
        sqlx::query_as::<_, NewsArticle>("SELECT * FROM news_articles WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn find_by_id_with_details(
        &self,
        _id: Uuid,
    ) -> Result<Option<ArticleWithDetails>, SqlxError> {
        // Simplified version - to be fully implemented
        Ok(None)
    }

    pub async fn list(&self, _query: &ArticleListQuery) -> Result<Vec<ArticleSummary>, SqlxError> {
        // Simplified version - returns empty for now
        Ok(vec![])
    }

    pub async fn update(
        &self,
        id: Uuid,
        _data: UpdateArticle,
    ) -> Result<Option<NewsArticle>, SqlxError> {
        self.find_by_id(id).await
    }

    pub async fn publish(
        &self,
        id: Uuid,
        published_at: Option<DateTime<Utc>>,
    ) -> Result<Option<NewsArticle>, SqlxError> {
        sqlx::query_as::<_, NewsArticle>(
            "UPDATE news_articles SET status = $2, published_at = COALESCE($3, NOW()) WHERE id = $1 RETURNING *",
        )
        .bind(id)
        .bind(article_status::PUBLISHED)
        .bind(published_at)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn archive(&self, id: Uuid) -> Result<Option<NewsArticle>, SqlxError> {
        sqlx::query_as::<_, NewsArticle>(
            "UPDATE news_articles SET status = $2, archived_at = NOW() WHERE id = $1 RETURNING *",
        )
        .bind(id)
        .bind(article_status::ARCHIVED)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn restore(&self, id: Uuid) -> Result<Option<NewsArticle>, SqlxError> {
        sqlx::query_as::<_, NewsArticle>(
            "UPDATE news_articles SET status = $2, archived_at = NULL WHERE id = $1 RETURNING *",
        )
        .bind(id)
        .bind(article_status::DRAFT)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn delete(&self, id: Uuid) -> Result<bool, SqlxError> {
        let result = sqlx::query("DELETE FROM news_articles WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn set_pinned(
        &self,
        id: Uuid,
        pinned: bool,
        pinned_by: Option<Uuid>,
    ) -> Result<Option<NewsArticle>, SqlxError> {
        if pinned {
            sqlx::query_as::<_, NewsArticle>(
                "UPDATE news_articles SET pinned = TRUE, pinned_at = NOW(), pinned_by = $2 WHERE id = $1 RETURNING *",
            )
            .bind(id)
            .bind(pinned_by)
            .fetch_optional(&self.pool)
            .await
        } else {
            sqlx::query_as::<_, NewsArticle>(
                "UPDATE news_articles SET pinned = FALSE, pinned_at = NULL, pinned_by = NULL WHERE id = $1 RETURNING *",
            )
            .bind(id)
            .fetch_optional(&self.pool)
            .await
        }
    }

    pub async fn add_media(
        &self,
        article_id: Uuid,
        data: CreateArticleMedia,
    ) -> Result<ArticleMedia, SqlxError> {
        let display_order = data.display_order.unwrap_or(0);
        sqlx::query_as::<_, ArticleMedia>(
            r#"
            INSERT INTO article_media (
                article_id, media_type, file_key, file_name, file_size,
                mime_type, embed_url, embed_html, width, height,
                alt_text, caption, display_order
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING *
            "#,
        )
        .bind(article_id)
        .bind(&data.media_type)
        .bind(&data.file_key)
        .bind(&data.file_name)
        .bind(data.file_size)
        .bind(&data.mime_type)
        .bind(&data.embed_url)
        .bind(&data.embed_html)
        .bind(data.width)
        .bind(data.height)
        .bind(&data.alt_text)
        .bind(&data.caption)
        .bind(display_order)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn list_media(&self, article_id: Uuid) -> Result<Vec<ArticleMedia>, SqlxError> {
        sqlx::query_as::<_, ArticleMedia>(
            "SELECT * FROM article_media WHERE article_id = $1 ORDER BY display_order",
        )
        .bind(article_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn delete_media(&self, media_id: Uuid) -> Result<bool, SqlxError> {
        let result = sqlx::query("DELETE FROM article_media WHERE id = $1")
            .bind(media_id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn toggle_reaction(
        &self,
        article_id: Uuid,
        user_id: Uuid,
        reaction: &str,
    ) -> Result<bool, SqlxError> {
        let existing: Option<(Uuid,)> = sqlx::query_as(
            "SELECT id FROM article_reactions WHERE article_id = $1 AND user_id = $2",
        )
        .bind(article_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        if existing.is_some() {
            sqlx::query("DELETE FROM article_reactions WHERE article_id = $1 AND user_id = $2")
                .bind(article_id)
                .bind(user_id)
                .execute(&self.pool)
                .await?;
            Ok(false)
        } else {
            sqlx::query(
                "INSERT INTO article_reactions (article_id, user_id, reaction) VALUES ($1, $2, $3) ON CONFLICT (article_id, user_id) DO UPDATE SET reaction = $3",
            )
            .bind(article_id)
            .bind(user_id)
            .bind(reaction)
            .execute(&self.pool)
            .await?;
            Ok(true)
        }
    }

    pub async fn get_reaction_counts(
        &self,
        _article_id: Uuid,
    ) -> Result<ReactionCounts, SqlxError> {
        Ok(ReactionCounts::default())
    }

    pub async fn get_user_reaction(
        &self,
        article_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<String>, SqlxError> {
        let result: Option<(String,)> = sqlx::query_as(
            "SELECT reaction FROM article_reactions WHERE article_id = $1 AND user_id = $2",
        )
        .bind(article_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(result.map(|(r,)| r))
    }

    pub async fn add_comment(
        &self,
        article_id: Uuid,
        user_id: Uuid,
        data: CreateArticleComment,
    ) -> Result<ArticleComment, SqlxError> {
        sqlx::query_as::<_, ArticleComment>(
            "INSERT INTO article_comments (article_id, user_id, parent_id, content) VALUES ($1, $2, $3, $4) RETURNING *",
        )
        .bind(article_id)
        .bind(user_id)
        .bind(data.parent_id)
        .bind(&data.content)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn list_comments(
        &self,
        article_id: Uuid,
        parent_id: Option<Uuid>,
    ) -> Result<Vec<CommentWithAuthor>, SqlxError> {
        // Simplified - returns empty for now
        Ok(vec![])
    }

    pub async fn update_comment(
        &self,
        comment_id: Uuid,
        user_id: Uuid,
        content: &str,
    ) -> Result<Option<ArticleComment>, SqlxError> {
        sqlx::query_as::<_, ArticleComment>(
            "UPDATE article_comments SET content = $3 WHERE id = $1 AND user_id = $2 RETURNING *",
        )
        .bind(comment_id)
        .bind(user_id)
        .bind(content)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn delete_comment(&self, comment_id: Uuid, user_id: Uuid) -> Result<bool, SqlxError> {
        let result = sqlx::query(
            "UPDATE article_comments SET deleted_at = NOW(), deleted_by = $2 WHERE id = $1 AND user_id = $2",
        )
        .bind(comment_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn moderate_comment(
        &self,
        comment_id: Uuid,
        moderator_id: Uuid,
        delete: bool,
        reason: Option<String>,
    ) -> Result<Option<ArticleComment>, SqlxError> {
        if delete {
            sqlx::query_as::<_, ArticleComment>(
                "UPDATE article_comments SET is_moderated = TRUE, moderated_by = $2, moderation_reason = $3, deleted_at = NOW() WHERE id = $1 RETURNING *",
            )
            .bind(comment_id)
            .bind(moderator_id)
            .bind(&reason)
            .fetch_optional(&self.pool)
            .await
        } else {
            sqlx::query_as::<_, ArticleComment>(
                "UPDATE article_comments SET is_moderated = TRUE, moderated_by = $2, moderation_reason = $3 WHERE id = $1 RETURNING *",
            )
            .bind(comment_id)
            .bind(moderator_id)
            .bind(&reason)
            .fetch_optional(&self.pool)
            .await
        }
    }

    pub async fn record_view(
        &self,
        article_id: Uuid,
        user_id: Option<Uuid>,
        duration_seconds: Option<i32>,
    ) -> Result<ArticleView, SqlxError> {
        sqlx::query_as::<_, ArticleView>(
            "INSERT INTO article_views (article_id, user_id, duration_seconds) VALUES ($1, $2, $3) ON CONFLICT (article_id, user_id) DO UPDATE SET viewed_at = NOW(), duration_seconds = $3 RETURNING *",
        )
        .bind(article_id)
        .bind(user_id)
        .bind(duration_seconds)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_statistics(&self) -> Result<ArticleStatistics, SqlxError> {
        sqlx::query_as::<_, ArticleStatistics>(
            r#"
            SELECT
                COUNT(*) as total_articles,
                COUNT(*) FILTER (WHERE status = 'published') as published_articles,
                COUNT(*) FILTER (WHERE status = 'draft') as draft_articles,
                COUNT(*) FILTER (WHERE status = 'archived') as archived_articles,
                COALESCE(SUM(view_count), 0) as total_views,
                COALESCE(SUM(reaction_count), 0) as total_reactions,
                COALESCE(SUM(comment_count), 0) as total_comments
            FROM news_articles
            "#,
        )
        .fetch_one(&self.pool)
        .await
    }
}

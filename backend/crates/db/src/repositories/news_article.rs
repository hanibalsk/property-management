//! News article repository (Epic 59: News & Media Management).

use crate::models::news_article::{
    article_status, ArticleComment, ArticleListQuery, ArticleMedia, ArticleStatistics,
    ArticleSummary, ArticleView, ArticleWithDetails, CommentWithAuthor, CommentWithAuthorRow,
    CreateArticle, CreateArticleComment, CreateArticleMedia, NewsArticle, ReactionCounts,
    UpdateArticle,
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

    /// Find an article by ID with full details.
    ///
    /// TODO: Implement full query with JOIN to include author details and related data.
    /// For now, returns `Ok(None)` to behave like a standard "not found" lookup until
    /// the detailed implementation with JOINs is ready.
    pub async fn find_by_id_with_details(
        &self,
        _id: Uuid,
    ) -> Result<Option<ArticleWithDetails>, SqlxError> {
        // TODO: Implement proper query with JOINs for author details and related data.
        // Placeholder implementation: indicate "no result" instead of an unconditional error.
        Ok(None)
    }

    /// List articles matching the query filters.
    ///
    /// Filters articles by organization_id for multi-tenant security.
    /// Additional filters include status, building_id, and pagination.
    pub async fn list(
        &self,
        organization_id: Uuid,
        query: &ArticleListQuery,
    ) -> Result<Vec<ArticleSummary>, SqlxError> {
        let limit = query.limit.unwrap_or(20).min(100);
        let offset = query.offset.unwrap_or(0);

        let articles = sqlx::query_as::<_, ArticleSummary>(
            r#"
            SELECT id, title, excerpt, cover_image_url, author_id, status,
                   published_at, pinned, view_count, reaction_count, comment_count, created_at
            FROM news_articles
            WHERE organization_id = $1
              AND ($2::text IS NULL OR status = $2)
              AND ($3::uuid IS NULL OR building_ids @> to_jsonb($3::uuid))
              AND ($4::bool IS NULL OR $4 = FALSE OR pinned = TRUE)
            ORDER BY pinned DESC, published_at DESC NULLS LAST, created_at DESC
            LIMIT $5 OFFSET $6
            "#,
        )
        .bind(organization_id)
        .bind(query.status.as_deref())
        .bind(query.building_id)
        .bind(query.pinned_only)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(articles)
    }

    /// Count the total number of articles matching the query filters.
    ///
    /// This is used for accurate pagination totals, separate from the paginated list query.
    /// Filters by organization_id for multi-tenant security.
    pub async fn count(
        &self,
        organization_id: Uuid,
        query: &ArticleListQuery,
    ) -> Result<i64, SqlxError> {
        let result: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM news_articles
            WHERE organization_id = $1
              AND ($2::text IS NULL OR status = $2)
              AND ($3::uuid IS NULL OR building_ids @> to_jsonb($3::uuid))
              AND ($4::bool IS NULL OR $4 = FALSE OR pinned = TRUE)
            "#,
        )
        .bind(organization_id)
        .bind(query.status.as_deref())
        .bind(query.building_id)
        .bind(query.pinned_only)
        .fetch_one(&self.pool)
        .await?;
        Ok(result.0)
    }

    /// Update an article with the provided data.
    pub async fn update(
        &self,
        id: Uuid,
        data: UpdateArticle,
    ) -> Result<Option<NewsArticle>, SqlxError> {
        // Build dynamic UPDATE query based on which fields are provided
        let mut query = String::from("UPDATE news_articles SET ");
        let mut updates = Vec::new();
        let mut param_count = 1;

        if data.title.is_some() {
            updates.push(format!("title = ${}", param_count));
            param_count += 1;
        }
        if data.content.is_some() {
            updates.push(format!("content = ${}", param_count));
            param_count += 1;
        }
        if data.excerpt.is_some() {
            updates.push(format!("excerpt = ${}", param_count));
            param_count += 1;
        }
        if data.cover_image_url.is_some() {
            updates.push(format!("cover_image_url = ${}", param_count));
            param_count += 1;
        }
        if data.building_ids.is_some() {
            updates.push(format!("building_ids = ${}", param_count));
            param_count += 1;
        }
        if data.status.is_some() {
            updates.push(format!("status = ${}", param_count));
            param_count += 1;
        }
        if data.comments_enabled.is_some() {
            updates.push(format!("comments_enabled = ${}", param_count));
            param_count += 1;
        }
        if data.reactions_enabled.is_some() {
            updates.push(format!("reactions_enabled = ${}", param_count));
            param_count += 1;
        }

        // If no fields to update, just return the existing article
        if updates.is_empty() {
            return self.find_by_id(id).await;
        }

        updates.push("updated_at = NOW()".to_string());
        query.push_str(&updates.join(", "));
        query.push_str(&format!(" WHERE id = ${} RETURNING *", param_count));

        let mut q = sqlx::query_as::<_, NewsArticle>(&query);

        // Bind parameters in the same order as updates vector
        if let Some(title) = data.title {
            q = q.bind(title);
        }
        if let Some(content) = data.content {
            q = q.bind(content);
        }
        if let Some(excerpt) = data.excerpt {
            q = q.bind(excerpt);
        }
        if let Some(cover_image_url) = data.cover_image_url {
            q = q.bind(cover_image_url);
        }
        if let Some(building_ids) = data.building_ids {
            let building_ids_json = serde_json::to_value(&building_ids).unwrap_or_default();
            q = q.bind(building_ids_json);
        }
        if let Some(status) = data.status {
            q = q.bind(status);
        }
        if let Some(comments_enabled) = data.comments_enabled {
            q = q.bind(comments_enabled);
        }
        if let Some(reactions_enabled) = data.reactions_enabled {
            q = q.bind(reactions_enabled);
        }
        q = q.bind(id);

        q.fetch_optional(&self.pool).await
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

    /// Toggle a reaction on an article.
    ///
    /// Uses UPSERT pattern to handle race conditions:
    /// - If user has no reaction, adds the new reaction
    /// - If user has the same reaction, removes it (toggle off)
    /// - If user has a different reaction, updates to the new reaction
    pub async fn toggle_reaction(
        &self,
        article_id: Uuid,
        user_id: Uuid,
        reaction: &str,
    ) -> Result<bool, SqlxError> {
        // First, check if user already has a reaction on this article
        let existing: Option<(String,)> = sqlx::query_as(
            "SELECT reaction FROM article_reactions WHERE article_id = $1 AND user_id = $2",
        )
        .bind(article_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        match existing {
            Some((existing_reaction,)) if existing_reaction == reaction => {
                // Same reaction - toggle off (remove it)
                sqlx::query("DELETE FROM article_reactions WHERE article_id = $1 AND user_id = $2")
                    .bind(article_id)
                    .bind(user_id)
                    .execute(&self.pool)
                    .await?;
                Ok(false)
            }
            Some(_) => {
                // Different reaction - update to new reaction
                sqlx::query(
                    "UPDATE article_reactions SET reaction = $3 WHERE article_id = $1 AND user_id = $2",
                )
                .bind(article_id)
                .bind(user_id)
                .bind(reaction)
                .execute(&self.pool)
                .await?;
                Ok(true)
            }
            None => {
                // No existing reaction - add new one
                sqlx::query(
                    "INSERT INTO article_reactions (article_id, user_id, reaction) VALUES ($1, $2, $3)",
                )
                .bind(article_id)
                .bind(user_id)
                .bind(reaction)
                .execute(&self.pool)
                .await?;
                Ok(true)
            }
        }
    }

    /// Get reaction counts for an article.
    pub async fn get_reaction_counts(&self, article_id: Uuid) -> Result<ReactionCounts, SqlxError> {
        let counts = sqlx::query_as::<_, (String, i64)>(
            r#"
            SELECT reaction, COUNT(*) as count
            FROM article_reactions
            WHERE article_id = $1
            GROUP BY reaction
            "#,
        )
        .bind(article_id)
        .fetch_all(&self.pool)
        .await?;

        let mut reaction_counts = ReactionCounts::default();

        for (reaction, count) in counts {
            match reaction.as_str() {
                "like" => reaction_counts.like = count as i32,
                "love" => reaction_counts.love = count as i32,
                "surprised" => reaction_counts.surprised = count as i32,
                "sad" => reaction_counts.sad = count as i32,
                "angry" => reaction_counts.angry = count as i32,
                _ => {}
            }
        }

        reaction_counts.total = reaction_counts.like
            + reaction_counts.love
            + reaction_counts.surprised
            + reaction_counts.sad
            + reaction_counts.angry;

        Ok(reaction_counts)
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

    /// List comments for an article with author information.
    pub async fn list_comments(
        &self,
        article_id: Uuid,
        parent_id: Option<Uuid>,
    ) -> Result<Vec<CommentWithAuthor>, SqlxError> {
        let rows = sqlx::query_as::<_, CommentWithAuthorRow>(
            r#"
            SELECT
                c.*,
                u.name as author_name,
                u.avatar_url as author_avatar_url,
                (SELECT COUNT(*) FROM article_comments WHERE parent_id = c.id AND deleted_at IS NULL) as reply_count
            FROM article_comments c
            LEFT JOIN users u ON c.user_id = u.id
            WHERE c.article_id = $1
                AND c.parent_id IS NOT DISTINCT FROM $2
                AND c.deleted_at IS NULL
            ORDER BY c.created_at DESC
            "#,
        )
        .bind(article_id)
        .bind(parent_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
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

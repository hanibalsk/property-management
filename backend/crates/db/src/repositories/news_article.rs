//! News article repository (Epic 59: News & Media Management).

use crate::models::news_article::{
    article_status, ArticleComment, ArticleListQuery, ArticleMedia, ArticleStatistics,
    ArticleSummary, ArticleView, ArticleWithDetails, CommentWithAuthor, CreateArticle,
    CreateArticleComment, CreateArticleMedia, NewsArticle, ReactionCounts, UpdateArticle,
};
use crate::DbPool;
use chrono::{DateTime, Utc};
use sqlx::{Error as SqlxError, FromRow};
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
        id: Uuid,
    ) -> Result<Option<ArticleWithDetails>, SqlxError> {
        // Query article with author details joined from users table
        #[derive(FromRow)]
        struct ArticleWithAuthorRow {
            // Article fields
            id: Uuid,
            organization_id: Uuid,
            author_id: Uuid,
            title: String,
            content: String,
            excerpt: Option<String>,
            cover_image_url: Option<String>,
            building_ids: serde_json::Value,
            status: String,
            published_at: Option<DateTime<Utc>>,
            archived_at: Option<DateTime<Utc>>,
            pinned: bool,
            pinned_at: Option<DateTime<Utc>>,
            pinned_by: Option<Uuid>,
            comments_enabled: bool,
            reactions_enabled: bool,
            view_count: i32,
            reaction_count: i32,
            comment_count: i32,
            share_count: i32,
            created_at: DateTime<Utc>,
            updated_at: DateTime<Utc>,
            // Author fields
            author_name: String,
            author_avatar_url: Option<String>,
        }

        let row = sqlx::query_as::<_, ArticleWithAuthorRow>(
            r#"
            SELECT
                a.id, a.organization_id, a.author_id, a.title, a.content, a.excerpt,
                a.cover_image_url, a.building_ids, a.status, a.published_at, a.archived_at,
                a.pinned, a.pinned_at, a.pinned_by, a.comments_enabled, a.reactions_enabled,
                a.view_count, a.reaction_count, a.comment_count, a.share_count,
                a.created_at, a.updated_at,
                COALESCE(u.first_name || ' ' || u.last_name, 'Unknown') as author_name,
                u.avatar_url as author_avatar_url
            FROM news_articles a
            LEFT JOIN users u ON u.id = a.author_id
            WHERE a.id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| ArticleWithDetails {
            article: NewsArticle {
                id: r.id,
                organization_id: r.organization_id,
                author_id: r.author_id,
                title: r.title,
                content: r.content,
                excerpt: r.excerpt,
                cover_image_url: r.cover_image_url,
                building_ids: r.building_ids,
                status: r.status,
                published_at: r.published_at,
                archived_at: r.archived_at,
                pinned: r.pinned,
                pinned_at: r.pinned_at,
                pinned_by: r.pinned_by,
                comments_enabled: r.comments_enabled,
                reactions_enabled: r.reactions_enabled,
                view_count: r.view_count,
                reaction_count: r.reaction_count,
                comment_count: r.comment_count,
                share_count: r.share_count,
                created_at: r.created_at,
                updated_at: r.updated_at,
            },
            author_name: r.author_name,
            author_avatar_url: r.author_avatar_url,
        }))
    }

    pub async fn list(
        &self,
        organization_id: Uuid,
        query: &ArticleListQuery,
    ) -> Result<Vec<ArticleSummary>, SqlxError> {
        let mut sql = String::from(
            r#"
            SELECT 
                id, title, excerpt, cover_image_url, author_id, status,
                published_at, pinned, view_count, reaction_count, comment_count,
                created_at, updated_at
            FROM news_articles
            WHERE organization_id = $1
            "#,
        );

        let mut param_count = 1;

        // Apply status filter
        if query.status.is_some() {
            param_count += 1;
            sql.push_str(&format!(" AND status = ${}", param_count));
        }

        // Apply building_id filter
        if query.building_id.is_some() {
            param_count += 1;
            sql.push_str(&format!(" AND building_ids @> ${}", param_count));
        }

        // Apply pinned_only filter
        if query.pinned_only == Some(true) {
            sql.push_str(" AND pinned = true");
        }

        // Add ordering - pinned first, then by published_at desc
        sql.push_str(" ORDER BY pinned DESC, published_at DESC NULLS LAST");

        // Apply pagination
        if query.limit.is_some() {
            param_count += 1;
            sql.push_str(&format!(" LIMIT ${}", param_count));
        }

        if query.offset.is_some() {
            param_count += 1;
            sql.push_str(&format!(" OFFSET ${}", param_count));
        }

        // Build and execute query
        let mut query_builder = sqlx::query_as::<_, ArticleSummary>(&sql).bind(organization_id);

        if let Some(ref status) = query.status {
            query_builder = query_builder.bind(status);
        }

        if let Some(building_id) = query.building_id {
            // Wrap UUID in an array for JSON containment check (@> checks if building_ids contains the array)
            query_builder = query_builder.bind(serde_json::json!([building_id.to_string()]));
        }

        if let Some(limit) = query.limit {
            query_builder = query_builder.bind(limit);
        }

        if let Some(offset) = query.offset {
            query_builder = query_builder.bind(offset);
        }

        query_builder.fetch_all(&self.pool).await
    }

    pub async fn count(
        &self,
        organization_id: Uuid,
        query: &ArticleListQuery,
    ) -> Result<i64, SqlxError> {
        let mut sql =
            String::from("SELECT COUNT(*) as count FROM news_articles WHERE organization_id = $1");

        let mut param_count = 1;

        // Apply status filter
        if query.status.is_some() {
            param_count += 1;
            sql.push_str(&format!(" AND status = ${}", param_count));
        }

        // Apply building_id filter
        if query.building_id.is_some() {
            param_count += 1;
            sql.push_str(&format!(" AND building_ids @> ${}", param_count));
        }

        // Apply pinned_only filter
        if query.pinned_only == Some(true) {
            sql.push_str(" AND pinned = true");
        }

        // Build and execute query
        let mut query_builder = sqlx::query_scalar::<_, i64>(&sql).bind(organization_id);

        if let Some(ref status) = query.status {
            query_builder = query_builder.bind(status);
        }

        if let Some(building_id) = query.building_id {
            // building_ids is stored as a JSONB array of UUID strings (e.g., ["uuid-1", "uuid-2"])
            // Use @> containment check with a single-element array
            query_builder = query_builder.bind(serde_json::json!([building_id.to_string()]));
        }

        query_builder.fetch_one(&self.pool).await
    }

    pub async fn update(
        &self,
        id: Uuid,
        data: UpdateArticle,
    ) -> Result<Option<NewsArticle>, SqlxError> {
        // First, fetch the existing article
        let existing = self.find_by_id(id).await?;
        if existing.is_none() {
            return Ok(None);
        }
        let existing = existing.unwrap();

        // Merge updated fields with existing values
        let title = data.title.as_ref().unwrap_or(&existing.title);
        let content = data.content.as_ref().unwrap_or(&existing.content);
        let excerpt = data.excerpt.clone().or(existing.excerpt.clone());
        let cover_image_url = data
            .cover_image_url
            .clone()
            .or(existing.cover_image_url.clone());
        let building_ids_json = if let Some(ref ids) = data.building_ids {
            serde_json::to_value(ids).unwrap_or_default()
        } else {
            existing.building_ids.clone()
        };
        let status = data.status.as_deref().unwrap_or(&existing.status);
        let comments_enabled = data.comments_enabled.unwrap_or(existing.comments_enabled);
        let reactions_enabled = data.reactions_enabled.unwrap_or(existing.reactions_enabled);

        sqlx::query_as::<_, NewsArticle>(
            r#"
            UPDATE news_articles
            SET
                title = $2,
                content = $3,
                excerpt = $4,
                cover_image_url = $5,
                building_ids = $6,
                status = $7,
                comments_enabled = $8,
                reactions_enabled = $9,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(title)
        .bind(content)
        .bind(&excerpt)
        .bind(&cover_image_url)
        .bind(&building_ids_json)
        .bind(status)
        .bind(comments_enabled)
        .bind(reactions_enabled)
        .fetch_optional(&self.pool)
        .await
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
        // Use a single atomic query with INSERT ... ON CONFLICT ... DO UPDATE
        // combined with a CTE to handle all cases atomically and avoid race conditions.
        //
        // This query:
        // 1. Tries to insert a new reaction
        // 2. On conflict, if the reaction type is the same, deletes it (toggle off)
        // 3. On conflict, if the reaction type is different, updates it
        //
        // Returns the action taken: 'inserted', 'updated', or 'deleted'
        let result: Option<(String,)> = sqlx::query_as(
            r#"
            WITH existing AS (
                SELECT id, reaction FROM article_reactions
                WHERE article_id = $1 AND user_id = $2
                FOR UPDATE
            ),
            action AS (
                SELECT CASE
                    WHEN NOT EXISTS (SELECT 1 FROM existing) THEN 'insert'
                    WHEN (SELECT reaction FROM existing) = $3 THEN 'delete'
                    ELSE 'update'
                END as op
            ),
            do_insert AS (
                INSERT INTO article_reactions (article_id, user_id, reaction)
                SELECT $1, $2, $3
                WHERE (SELECT op FROM action) = 'insert'
                RETURNING 'inserted' as result
            ),
            do_update AS (
                UPDATE article_reactions
                SET reaction = $3
                WHERE article_id = $1 AND user_id = $2
                  AND (SELECT op FROM action) = 'update'
                RETURNING 'updated' as result
            ),
            do_delete AS (
                DELETE FROM article_reactions
                WHERE article_id = $1 AND user_id = $2
                  AND (SELECT op FROM action) = 'delete'
                RETURNING 'deleted' as result
            )
            SELECT result FROM do_insert
            UNION ALL SELECT result FROM do_update
            UNION ALL SELECT result FROM do_delete
            "#,
        )
        .bind(article_id)
        .bind(user_id)
        .bind(reaction)
        .fetch_optional(&self.pool)
        .await?;

        // Return true if reaction was added/updated, false if removed
        Ok(result.map(|(r,)| r != "deleted").unwrap_or(false))
    }

    pub async fn get_reaction_counts(&self, article_id: Uuid) -> Result<ReactionCounts, SqlxError> {
        let rows: Vec<(String, i64)> = sqlx::query_as(
            "SELECT reaction, COUNT(*) as count FROM article_reactions WHERE article_id = $1 GROUP BY reaction",
        )
        .bind(article_id)
        .fetch_all(&self.pool)
        .await?;

        let mut counts = ReactionCounts::default();
        for (reaction, count) in rows {
            let count_i32 = count as i32;
            match reaction.as_str() {
                "like" => counts.like = count_i32,
                "love" => counts.love = count_i32,
                "surprised" => counts.surprised = count_i32,
                "sad" => counts.sad = count_i32,
                "angry" => counts.angry = count_i32,
                _ => {} // Ignore unknown reaction types
            }
        }
        counts.total = counts.like + counts.love + counts.surprised + counts.sad + counts.angry;

        Ok(counts)
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
        use crate::models::news_article::CommentWithAuthorRow;

        let rows: Vec<CommentWithAuthorRow> = match parent_id {
            Some(parent) => {
                sqlx::query_as::<_, CommentWithAuthorRow>(
                    r#"
                    SELECT
                        c.id, c.article_id, c.user_id, c.parent_id, c.content,
                        c.is_moderated, c.moderated_at, c.moderated_by, c.moderation_reason,
                        c.deleted_at, c.deleted_by, c.like_count, c.created_at, c.updated_at,
                        COALESCE(u.first_name || ' ' || u.last_name, 'Unknown') as author_name,
                        u.avatar_url as author_avatar_url,
                        (SELECT COUNT(*) FROM article_comments r WHERE r.parent_id = c.id AND r.deleted_at IS NULL) as reply_count
                    FROM article_comments c
                    LEFT JOIN users u ON u.id = c.user_id
                    WHERE c.article_id = $1
                      AND c.parent_id = $2
                      AND c.deleted_at IS NULL
                    ORDER BY c.created_at ASC
                    "#,
                )
                .bind(article_id)
                .bind(parent)
                .fetch_all(&self.pool)
                .await?
            }
            None => {
                sqlx::query_as::<_, CommentWithAuthorRow>(
                    r#"
                    SELECT
                        c.id, c.article_id, c.user_id, c.parent_id, c.content,
                        c.is_moderated, c.moderated_at, c.moderated_by, c.moderation_reason,
                        c.deleted_at, c.deleted_by, c.like_count, c.created_at, c.updated_at,
                        COALESCE(u.first_name || ' ' || u.last_name, 'Unknown') as author_name,
                        u.avatar_url as author_avatar_url,
                        (SELECT COUNT(*) FROM article_comments r WHERE r.parent_id = c.id AND r.deleted_at IS NULL) as reply_count
                    FROM article_comments c
                    LEFT JOIN users u ON u.id = c.user_id
                    WHERE c.article_id = $1
                      AND c.parent_id IS NULL
                      AND c.deleted_at IS NULL
                    ORDER BY c.created_at ASC
                    "#,
                )
                .bind(article_id)
                .fetch_all(&self.pool)
                .await?
            }
        };

        Ok(rows.into_iter().map(CommentWithAuthor::from).collect())
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
                "UPDATE article_comments SET is_moderated = TRUE, moderated_at = NOW(), moderated_by = $2, moderation_reason = $3, deleted_at = NOW() WHERE id = $1 RETURNING *",
            )
            .bind(comment_id)
            .bind(moderator_id)
            .bind(&reason)
            .fetch_optional(&self.pool)
            .await
        } else {
            sqlx::query_as::<_, ArticleComment>(
                "UPDATE article_comments SET is_moderated = TRUE, moderated_at = NOW(), moderated_by = $2, moderation_reason = $3 WHERE id = $1 RETURNING *",
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
        match user_id {
            Some(uid) => {
                // For authenticated users, upsert on (article_id, user_id)
                sqlx::query_as::<_, ArticleView>(
                    "INSERT INTO article_views (article_id, user_id, duration_seconds) \
                     VALUES ($1, $2, $3) \
                     ON CONFLICT (article_id, user_id) \
                     DO UPDATE SET viewed_at = NOW(), duration_seconds = $3 \
                     RETURNING *",
                )
                .bind(article_id)
                .bind(uid)
                .bind(duration_seconds)
                .fetch_one(&self.pool)
                .await
            }
            None => {
                // For anonymous views, always insert a new row since NULL values
                // don't participate in unique constraints
                sqlx::query_as::<_, ArticleView>(
                    "INSERT INTO article_views (article_id, user_id, duration_seconds) \
                     VALUES ($1, NULL, $2) \
                     RETURNING *",
                )
                .bind(article_id)
                .bind(duration_seconds)
                .fetch_one(&self.pool)
                .await
            }
        }
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

//! Contextual Help repository (Epic 10B, Story 10B.7).
//!
//! Repository for help articles, FAQ, and tooltips.

use crate::DbPool;
use chrono::{DateTime, Utc};
use sqlx::Error as SqlxError;
use uuid::Uuid;

/// Repository for help operations.
#[derive(Clone)]
pub struct HelpRepository {
    pool: DbPool,
}

/// Help article.
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct HelpArticle {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub summary: Option<String>,
    pub category: String,
    pub tags: Option<Vec<String>>,
    pub context_keys: Option<Vec<String>>,
    pub is_published: bool,
    pub view_count: i32,
    pub helpful_count: i32,
    pub not_helpful_count: i32,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Help category.
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct HelpCategory {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub display_order: i32,
    pub parent_id: Option<Uuid>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// FAQ entry.
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct FaqEntry {
    pub id: Uuid,
    pub question: String,
    pub answer: String,
    pub category: String,
    pub display_order: i32,
    pub is_published: bool,
    pub view_count: i32,
    pub helpful_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Tooltip.
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct Tooltip {
    pub id: Uuid,
    pub key: String,
    pub title: Option<String>,
    pub content: String,
    pub position: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Article with category info.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ArticleWithCategory {
    #[serde(flatten)]
    pub article: HelpArticle,
    pub category_name: String,
}

impl HelpRepository {
    /// Create a new HelpRepository.
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    // ==================== Articles ====================

    /// Get all published articles.
    pub async fn get_published_articles(&self) -> Result<Vec<HelpArticle>, SqlxError> {
        let articles = sqlx::query_as::<_, HelpArticle>(
            r#"
            SELECT id, slug, title, content, summary, category, tags, context_keys,
                   is_published, view_count, helpful_count, not_helpful_count,
                   created_by, created_at, updated_at
            FROM help_articles
            WHERE is_published = true
            ORDER BY category, title
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(articles)
    }

    /// Get articles by category.
    pub async fn get_articles_by_category(
        &self,
        category: &str,
    ) -> Result<Vec<HelpArticle>, SqlxError> {
        let articles = sqlx::query_as::<_, HelpArticle>(
            r#"
            SELECT id, slug, title, content, summary, category, tags, context_keys,
                   is_published, view_count, helpful_count, not_helpful_count,
                   created_by, created_at, updated_at
            FROM help_articles
            WHERE category = $1 AND is_published = true
            ORDER BY title
            "#,
        )
        .bind(category)
        .fetch_all(&self.pool)
        .await?;

        Ok(articles)
    }

    /// Get articles by context key (for contextual help).
    pub async fn get_articles_by_context(
        &self,
        context_key: &str,
    ) -> Result<Vec<HelpArticle>, SqlxError> {
        let articles = sqlx::query_as::<_, HelpArticle>(
            r#"
            SELECT id, slug, title, content, summary, category, tags, context_keys,
                   is_published, view_count, helpful_count, not_helpful_count,
                   created_by, created_at, updated_at
            FROM help_articles
            WHERE $1 = ANY(context_keys) AND is_published = true
            ORDER BY title
            "#,
        )
        .bind(context_key)
        .fetch_all(&self.pool)
        .await?;

        Ok(articles)
    }

    /// Get article by slug.
    pub async fn get_article_by_slug(&self, slug: &str) -> Result<Option<HelpArticle>, SqlxError> {
        let article = sqlx::query_as::<_, HelpArticle>(
            r#"
            SELECT id, slug, title, content, summary, category, tags, context_keys,
                   is_published, view_count, helpful_count, not_helpful_count,
                   created_by, created_at, updated_at
            FROM help_articles
            WHERE slug = $1 AND is_published = true
            "#,
        )
        .bind(slug)
        .fetch_optional(&self.pool)
        .await?;

        Ok(article)
    }

    /// Increment article view count.
    pub async fn increment_article_view(&self, slug: &str) -> Result<(), SqlxError> {
        sqlx::query("UPDATE help_articles SET view_count = view_count + 1 WHERE slug = $1")
            .bind(slug)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Search articles.
    pub async fn search_articles(&self, query: &str) -> Result<Vec<HelpArticle>, SqlxError> {
        let search_pattern = format!("%{}%", query.to_lowercase());
        let articles = sqlx::query_as::<_, HelpArticle>(
            r#"
            SELECT id, slug, title, content, summary, category, tags, context_keys,
                   is_published, view_count, helpful_count, not_helpful_count,
                   created_by, created_at, updated_at
            FROM help_articles
            WHERE is_published = true
              AND (LOWER(title) LIKE $1 OR LOWER(content) LIKE $1 OR LOWER(summary) LIKE $1)
            ORDER BY
                CASE WHEN LOWER(title) LIKE $1 THEN 1 ELSE 2 END,
                title
            LIMIT 50
            "#,
        )
        .bind(&search_pattern)
        .fetch_all(&self.pool)
        .await?;

        Ok(articles)
    }

    /// Record article feedback.
    pub async fn record_article_feedback(
        &self,
        article_id: Uuid,
        user_id: Uuid,
        is_helpful: bool,
        feedback_text: Option<&str>,
    ) -> Result<(), SqlxError> {
        sqlx::query(
            r#"
            INSERT INTO user_article_feedback (article_id, user_id, is_helpful, feedback_text)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (article_id, user_id) DO UPDATE SET
                is_helpful = $3,
                feedback_text = COALESCE($4, user_article_feedback.feedback_text),
                created_at = NOW()
            "#,
        )
        .bind(article_id)
        .bind(user_id)
        .bind(is_helpful)
        .bind(feedback_text)
        .execute(&self.pool)
        .await?;

        // Update article counts
        if is_helpful {
            sqlx::query("UPDATE help_articles SET helpful_count = helpful_count + 1 WHERE id = $1")
                .bind(article_id)
                .execute(&self.pool)
                .await?;
        } else {
            sqlx::query(
                "UPDATE help_articles SET not_helpful_count = not_helpful_count + 1 WHERE id = $1",
            )
            .bind(article_id)
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    // ==================== Categories ====================

    /// Get all active categories.
    pub async fn get_categories(&self) -> Result<Vec<HelpCategory>, SqlxError> {
        let categories = sqlx::query_as::<_, HelpCategory>(
            r#"
            SELECT id, slug, name, description, icon, display_order, parent_id,
                   is_active, created_at, updated_at
            FROM help_categories
            WHERE is_active = true
            ORDER BY display_order, name
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(categories)
    }

    /// Get category by slug.
    pub async fn get_category_by_slug(
        &self,
        slug: &str,
    ) -> Result<Option<HelpCategory>, SqlxError> {
        let category = sqlx::query_as::<_, HelpCategory>(
            r#"
            SELECT id, slug, name, description, icon, display_order, parent_id,
                   is_active, created_at, updated_at
            FROM help_categories
            WHERE slug = $1 AND is_active = true
            "#,
        )
        .bind(slug)
        .fetch_optional(&self.pool)
        .await?;

        Ok(category)
    }

    // ==================== FAQ ====================

    /// Get all published FAQ.
    pub async fn get_faq(&self) -> Result<Vec<FaqEntry>, SqlxError> {
        let faq = sqlx::query_as::<_, FaqEntry>(
            r#"
            SELECT id, question, answer, category, display_order, is_published,
                   view_count, helpful_count, created_at, updated_at
            FROM faq
            WHERE is_published = true
            ORDER BY category, display_order, question
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(faq)
    }

    /// Get FAQ by category.
    pub async fn get_faq_by_category(&self, category: &str) -> Result<Vec<FaqEntry>, SqlxError> {
        let faq = sqlx::query_as::<_, FaqEntry>(
            r#"
            SELECT id, question, answer, category, display_order, is_published,
                   view_count, helpful_count, created_at, updated_at
            FROM faq
            WHERE category = $1 AND is_published = true
            ORDER BY display_order, question
            "#,
        )
        .bind(category)
        .fetch_all(&self.pool)
        .await?;

        Ok(faq)
    }

    /// Search FAQ.
    pub async fn search_faq(&self, query: &str) -> Result<Vec<FaqEntry>, SqlxError> {
        let search_pattern = format!("%{}%", query.to_lowercase());
        let faq = sqlx::query_as::<_, FaqEntry>(
            r#"
            SELECT id, question, answer, category, display_order, is_published,
                   view_count, helpful_count, created_at, updated_at
            FROM faq
            WHERE is_published = true
              AND (LOWER(question) LIKE $1 OR LOWER(answer) LIKE $1)
            ORDER BY
                CASE WHEN LOWER(question) LIKE $1 THEN 1 ELSE 2 END,
                display_order
            LIMIT 20
            "#,
        )
        .bind(&search_pattern)
        .fetch_all(&self.pool)
        .await?;

        Ok(faq)
    }

    // ==================== Tooltips ====================

    /// Get all active tooltips.
    pub async fn get_tooltips(&self) -> Result<Vec<Tooltip>, SqlxError> {
        let tooltips = sqlx::query_as::<_, Tooltip>(
            r#"
            SELECT id, key, title, content, position, is_active, created_at, updated_at
            FROM tooltips
            WHERE is_active = true
            ORDER BY key
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tooltips)
    }

    /// Get tooltip by key.
    pub async fn get_tooltip(&self, key: &str) -> Result<Option<Tooltip>, SqlxError> {
        let tooltip = sqlx::query_as::<_, Tooltip>(
            r#"
            SELECT id, key, title, content, position, is_active, created_at, updated_at
            FROM tooltips
            WHERE key = $1 AND is_active = true
            "#,
        )
        .bind(key)
        .fetch_optional(&self.pool)
        .await?;

        Ok(tooltip)
    }

    /// Get tooltips by prefix (e.g., "page:dashboard" gets all dashboard tooltips).
    pub async fn get_tooltips_by_prefix(&self, prefix: &str) -> Result<Vec<Tooltip>, SqlxError> {
        let pattern = format!("{}%", prefix);
        let tooltips = sqlx::query_as::<_, Tooltip>(
            r#"
            SELECT id, key, title, content, position, is_active, created_at, updated_at
            FROM tooltips
            WHERE key LIKE $1 AND is_active = true
            ORDER BY key
            "#,
        )
        .bind(&pattern)
        .fetch_all(&self.pool)
        .await?;

        Ok(tooltips)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help_article_struct() {
        let article = HelpArticle {
            id: Uuid::new_v4(),
            slug: "test-article".to_string(),
            title: "Test Article".to_string(),
            content: "Content".to_string(),
            summary: Some("Summary".to_string()),
            category: "test".to_string(),
            tags: Some(vec!["tag1".to_string()]),
            context_keys: Some(vec!["page:test".to_string()]),
            is_published: true,
            view_count: 0,
            helpful_count: 0,
            not_helpful_count: 0,
            created_by: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert!(article.is_published);
        assert_eq!(article.slug, "test-article");
    }
}

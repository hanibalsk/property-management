//! User repository (Epic 1, Story 1.1).

use crate::models::user::{CreateUser, EmailVerificationToken, UpdateUser, User};
use crate::DbPool;
use chrono::{Duration, Utc};
use sqlx::Error as SqlxError;
use uuid::Uuid;

/// Repository for user operations.
#[derive(Clone)]
pub struct UserRepository {
    pool: DbPool,
}

impl UserRepository {
    /// Create a new UserRepository.
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Create a new user.
    pub async fn create(&self, data: CreateUser) -> Result<User, SqlxError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (email, password_hash, name, phone, locale, status)
            VALUES ($1, $2, $3, $4, $5, 'pending')
            RETURNING *
            "#,
        )
        .bind(&data.email)
        .bind(&data.password_hash)
        .bind(&data.name)
        .bind(&data.phone)
        .bind(data.locale.as_str())
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    /// Find user by ID.
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, SqlxError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users WHERE id = $1 AND status != 'deleted'
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// Find user by email.
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, SqlxError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users WHERE LOWER(email) = LOWER($1) AND status != 'deleted'
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// Check if email exists (including deleted accounts within 30 days).
    pub async fn email_exists(&self, email: &str) -> Result<bool, SqlxError> {
        let result = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM users
            WHERE LOWER(email) = LOWER($1)
            AND (status != 'deleted' OR deleted_at > NOW() - INTERVAL '30 days')
            "#,
        )
        .bind(email)
        .fetch_one(&self.pool)
        .await?;

        Ok(result > 0)
    }

    /// Update user.
    pub async fn update(&self, id: Uuid, data: UpdateUser) -> Result<Option<User>, SqlxError> {
        let mut query = String::from("UPDATE users SET updated_at = NOW()");
        let mut param_count = 1;

        if data.name.is_some() {
            param_count += 1;
            query.push_str(&format!(", name = ${}", param_count));
        }
        if data.phone.is_some() {
            param_count += 1;
            query.push_str(&format!(", phone = ${}", param_count));
        }
        if data.locale.is_some() {
            param_count += 1;
            query.push_str(&format!(", locale = ${}", param_count));
        }

        query.push_str(" WHERE id = $1 AND status != 'deleted' RETURNING *");

        let mut q = sqlx::query_as::<_, User>(&query).bind(id);

        if let Some(name) = &data.name {
            q = q.bind(name);
        }
        if let Some(phone) = &data.phone {
            q = q.bind(phone);
        }
        if let Some(locale) = &data.locale {
            q = q.bind(locale.as_str());
        }

        let user = q.fetch_optional(&self.pool).await?;
        Ok(user)
    }

    /// Verify user email.
    pub async fn verify_email(&self, id: Uuid) -> Result<Option<User>, SqlxError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET email_verified_at = NOW(), status = 'active', updated_at = NOW()
            WHERE id = $1 AND status = 'pending'
            RETURNING *
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// Suspend user.
    pub async fn suspend(&self, id: Uuid, by: Uuid) -> Result<Option<User>, SqlxError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET status = 'suspended', suspended_at = NOW(), suspended_by = $2, updated_at = NOW()
            WHERE id = $1 AND status = 'active'
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(by)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// Reactivate suspended user.
    pub async fn reactivate(&self, id: Uuid) -> Result<Option<User>, SqlxError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET status = 'active', suspended_at = NULL, suspended_by = NULL, updated_at = NOW()
            WHERE id = $1 AND status = 'suspended'
            RETURNING *
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// Soft delete user.
    pub async fn soft_delete(&self, id: Uuid, by: Uuid) -> Result<Option<User>, SqlxError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET status = 'deleted', deleted_at = NOW(), deleted_by = $2, updated_at = NOW()
            WHERE id = $1 AND status != 'deleted'
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(by)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// Update password hash.
    pub async fn update_password(&self, id: Uuid, password_hash: &str) -> Result<bool, SqlxError> {
        let result = sqlx::query(
            r#"
            UPDATE users SET password_hash = $2, updated_at = NOW()
            WHERE id = $1 AND status != 'deleted'
            "#,
        )
        .bind(id)
        .bind(password_hash)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    // ==================== Email Verification Tokens ====================

    /// Create email verification token.
    pub async fn create_verification_token(
        &self,
        user_id: Uuid,
        token_hash: &str,
    ) -> Result<EmailVerificationToken, SqlxError> {
        // Token valid for 24 hours
        let expires_at = Utc::now() + Duration::hours(24);

        let token = sqlx::query_as::<_, EmailVerificationToken>(
            r#"
            INSERT INTO email_verification_tokens (user_id, token_hash, expires_at)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(token_hash)
        .bind(expires_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(token)
    }

    /// Find verification token by hash.
    pub async fn find_verification_token(
        &self,
        token_hash: &str,
    ) -> Result<Option<EmailVerificationToken>, SqlxError> {
        let token = sqlx::query_as::<_, EmailVerificationToken>(
            r#"
            SELECT * FROM email_verification_tokens
            WHERE token_hash = $1 AND used_at IS NULL
            "#,
        )
        .bind(token_hash)
        .fetch_optional(&self.pool)
        .await?;

        Ok(token)
    }

    /// Mark verification token as used.
    pub async fn use_verification_token(&self, id: Uuid) -> Result<bool, SqlxError> {
        let result = sqlx::query(
            r#"
            UPDATE email_verification_tokens SET used_at = NOW()
            WHERE id = $1 AND used_at IS NULL
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Delete expired verification tokens.
    pub async fn cleanup_expired_tokens(&self) -> Result<u64, SqlxError> {
        let result = sqlx::query(
            r#"
            DELETE FROM email_verification_tokens
            WHERE expires_at < NOW() OR used_at IS NOT NULL
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    /// Invalidate all verification tokens for a user.
    pub async fn invalidate_user_tokens(&self, user_id: Uuid) -> Result<u64, SqlxError> {
        let result = sqlx::query(
            r#"
            UPDATE email_verification_tokens SET used_at = NOW()
            WHERE user_id = $1 AND used_at IS NULL
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    // ==================== Admin Operations ====================

    /// List users with pagination and optional filters.
    pub async fn list_users(
        &self,
        offset: i64,
        limit: i64,
        status_filter: Option<&str>,
        search: Option<&str>,
    ) -> Result<(Vec<User>, i64), SqlxError> {
        // Build dynamic query based on filters
        let mut conditions = vec!["1=1".to_string()];
        let mut param_idx = 1;

        if status_filter.is_some() {
            param_idx += 1;
            conditions.push(format!("status = ${}", param_idx));
        }

        if search.is_some() {
            param_idx += 1;
            conditions.push(format!(
                "(LOWER(email) LIKE '%' || LOWER(${}::text) || '%' OR LOWER(name) LIKE '%' || LOWER(${}::text) || '%')",
                param_idx, param_idx
            ));
        }

        let where_clause = conditions.join(" AND ");

        // Count query
        let count_query = format!("SELECT COUNT(*) FROM users WHERE {}", where_clause);

        // Data query
        let data_query = format!(
            "SELECT * FROM users WHERE {} ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            where_clause
        );

        // Execute count query
        let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
        if let Some(status) = status_filter {
            count_q = count_q.bind(status);
        }
        if let Some(s) = search {
            count_q = count_q.bind(s);
        }
        let total = count_q.fetch_one(&self.pool).await?;

        // Execute data query
        let mut data_q = sqlx::query_as::<_, User>(&data_query)
            .bind(limit)
            .bind(offset);
        if let Some(status) = status_filter {
            data_q = data_q.bind(status);
        }
        if let Some(s) = search {
            data_q = data_q.bind(s);
        }
        let users = data_q.fetch_all(&self.pool).await?;

        Ok((users, total))
    }

    /// Find user by ID including deleted users (for admin).
    pub async fn find_by_id_admin(&self, id: Uuid) -> Result<Option<User>, SqlxError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }
}

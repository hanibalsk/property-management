//! Messaging repository (Epic 6, Story 6.5).

use crate::models::messaging::{
    BlockWithUserInfo, BlockWithUserInfoRow, CreateBlock, CreateMessage, CreateThread, Message,
    MessageThread, MessageWithSender, MessageWithSenderRow, ThreadWithPreview, ThreadWithPreviewRow,
    UserBlock,
};
use crate::DbPool;
use sqlx::Error as SqlxError;
use uuid::Uuid;

/// Repository for messaging operations.
#[derive(Clone)]
pub struct MessagingRepository {
    pool: DbPool,
}

impl MessagingRepository {
    /// Create a new MessagingRepository.
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    // ========================================================================
    // MESSAGE THREAD OPERATIONS
    // ========================================================================

    /// Get or create a thread between two users.
    ///
    /// If a thread already exists between the two users, return it.
    /// Otherwise, create a new thread.
    pub async fn get_or_create_thread(
        &self,
        data: CreateThread,
    ) -> Result<MessageThread, SqlxError> {
        // Ensure exactly 2 participants
        if data.participant_ids.len() != 2 {
            return Err(SqlxError::Protocol(
                "Thread must have exactly 2 participants".to_string(),
            ));
        }

        // Sort participant IDs for consistent lookup
        let mut sorted_ids = data.participant_ids.clone();
        sorted_ids.sort();

        // Check if thread already exists
        let existing = sqlx::query_as::<_, MessageThread>(
            r#"
            SELECT * FROM message_threads
            WHERE organization_id = $1
              AND participant_ids @> $2::uuid[]
              AND participant_ids <@ $2::uuid[]
            "#,
        )
        .bind(data.organization_id)
        .bind(&sorted_ids)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(thread) = existing {
            return Ok(thread);
        }

        // Create new thread
        let thread = sqlx::query_as::<_, MessageThread>(
            r#"
            INSERT INTO message_threads (organization_id, participant_ids)
            VALUES ($1, $2)
            RETURNING *
            "#,
        )
        .bind(data.organization_id)
        .bind(&sorted_ids)
        .fetch_one(&self.pool)
        .await?;

        Ok(thread)
    }

    /// Get a thread by ID.
    pub async fn get_thread(&self, id: Uuid) -> Result<Option<MessageThread>, SqlxError> {
        let thread = sqlx::query_as::<_, MessageThread>(
            r#"
            SELECT * FROM message_threads WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(thread)
    }

    /// List threads for a user with preview info.
    pub async fn list_threads(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<ThreadWithPreview>, SqlxError> {
        let limit = limit.unwrap_or(20).min(100);
        let offset = offset.unwrap_or(0);

        let rows = sqlx::query_as::<_, ThreadWithPreviewRow>(
            r#"
            WITH thread_messages AS (
                SELECT DISTINCT ON (m.thread_id)
                    m.thread_id,
                    m.id as message_id,
                    m.content as message_content,
                    m.sender_id as message_sender_id,
                    m.created_at as message_created_at
                FROM messages m
                WHERE m.deleted_at IS NULL
                ORDER BY m.thread_id, m.created_at DESC
            ),
            unread_counts AS (
                SELECT thread_id, COUNT(*) as unread
                FROM messages
                WHERE sender_id != $1
                  AND read_at IS NULL
                  AND deleted_at IS NULL
                GROUP BY thread_id
            )
            SELECT
                t.id,
                t.organization_id,
                t.participant_ids,
                t.last_message_at,
                t.created_at,
                t.updated_at,
                -- Other participant (the one that's not the current user)
                u.id as other_user_id,
                u.first_name as other_first_name,
                u.last_name as other_last_name,
                u.email as other_email,
                -- Last message
                tm.message_id as last_message_id,
                tm.message_content as last_message_content,
                tm.message_sender_id as last_message_sender_id,
                tm.message_created_at as last_message_created_at,
                -- Unread count
                COALESCE(uc.unread, 0) as unread_count
            FROM message_threads t
            CROSS JOIN LATERAL (
                SELECT id, first_name, last_name, email
                FROM users
                WHERE id = ANY(t.participant_ids) AND id != $1
                LIMIT 1
            ) u
            LEFT JOIN thread_messages tm ON tm.thread_id = t.id
            LEFT JOIN unread_counts uc ON uc.thread_id = t.id
            WHERE $1 = ANY(t.participant_ids)
              AND t.organization_id = $2
            ORDER BY t.last_message_at DESC NULLS LAST, t.created_at DESC
            LIMIT $3 OFFSET $4
            "#,
        )
        .bind(user_id)
        .bind(organization_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let threads = rows
            .into_iter()
            .map(|row| row.into_thread_with_preview(user_id))
            .collect();

        Ok(threads)
    }

    /// Count threads for a user.
    pub async fn count_threads(&self, user_id: Uuid, organization_id: Uuid) -> Result<i64, SqlxError> {
        let (count,): (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM message_threads
            WHERE $1 = ANY(participant_ids)
              AND organization_id = $2
            "#,
        )
        .bind(user_id)
        .bind(organization_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count)
    }

    /// Check if user is participant in thread.
    pub async fn is_participant(&self, thread_id: Uuid, user_id: Uuid) -> Result<bool, SqlxError> {
        let (is_participant,): (bool,) = sqlx::query_as(
            r#"
            SELECT $2 = ANY(participant_ids)
            FROM message_threads
            WHERE id = $1
            "#,
        )
        .bind(thread_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(is_participant)
    }

    // ========================================================================
    // MESSAGE OPERATIONS
    // ========================================================================

    /// Create a new message.
    pub async fn create_message(&self, data: CreateMessage) -> Result<Message, SqlxError> {
        let message = sqlx::query_as::<_, Message>(
            r#"
            INSERT INTO messages (thread_id, sender_id, content)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(data.thread_id)
        .bind(data.sender_id)
        .bind(&data.content)
        .fetch_one(&self.pool)
        .await?;

        Ok(message)
    }

    /// Get messages for a thread with sender info.
    pub async fn get_thread_messages(
        &self,
        thread_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<MessageWithSender>, SqlxError> {
        let limit = limit.unwrap_or(50).min(100);
        let offset = offset.unwrap_or(0);

        let rows = sqlx::query_as::<_, MessageWithSenderRow>(
            r#"
            SELECT
                m.id,
                m.thread_id,
                m.sender_id,
                m.content,
                m.read_at,
                m.deleted_at,
                m.created_at,
                u.first_name as sender_first_name,
                u.last_name as sender_last_name,
                u.email as sender_email
            FROM messages m
            JOIN users u ON u.id = m.sender_id
            WHERE m.thread_id = $1
            ORDER BY m.created_at ASC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(thread_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let messages = rows.into_iter().map(MessageWithSender::from).collect();

        Ok(messages)
    }

    /// Count messages in a thread.
    pub async fn count_thread_messages(&self, thread_id: Uuid) -> Result<i64, SqlxError> {
        let (count,): (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM messages
            WHERE thread_id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(thread_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count)
    }

    /// Mark all messages in a thread as read for a user.
    pub async fn mark_thread_read(
        &self,
        thread_id: Uuid,
        reader_id: Uuid,
    ) -> Result<i64, SqlxError> {
        let result = sqlx::query(
            r#"
            UPDATE messages
            SET read_at = NOW(), updated_at = NOW()
            WHERE thread_id = $1
              AND sender_id != $2
              AND read_at IS NULL
              AND deleted_at IS NULL
            "#,
        )
        .bind(thread_id)
        .bind(reader_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() as i64)
    }

    /// Count unread messages for a user across all threads.
    pub async fn count_unread(&self, user_id: Uuid, organization_id: Uuid) -> Result<i64, SqlxError> {
        let (count,): (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM messages m
            JOIN message_threads t ON t.id = m.thread_id
            WHERE $1 = ANY(t.participant_ids)
              AND t.organization_id = $2
              AND m.sender_id != $1
              AND m.read_at IS NULL
              AND m.deleted_at IS NULL
            "#,
        )
        .bind(user_id)
        .bind(organization_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count)
    }

    /// Soft delete a message.
    pub async fn delete_message(
        &self,
        message_id: Uuid,
        deleted_by: Uuid,
    ) -> Result<Message, SqlxError> {
        let message = sqlx::query_as::<_, Message>(
            r#"
            UPDATE messages
            SET deleted_at = NOW(),
                deleted_by = $2,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(message_id)
        .bind(deleted_by)
        .fetch_one(&self.pool)
        .await?;

        Ok(message)
    }

    // ========================================================================
    // USER BLOCK OPERATIONS
    // ========================================================================

    /// Block a user.
    pub async fn block_user(&self, data: CreateBlock) -> Result<UserBlock, SqlxError> {
        // Check if already blocked
        let existing = self.get_block(data.blocker_id, data.blocked_id).await?;
        if existing.is_some() {
            return Err(SqlxError::Protocol("User is already blocked".to_string()));
        }

        let block = sqlx::query_as::<_, UserBlock>(
            r#"
            INSERT INTO user_blocks (blocker_id, blocked_id, organization_id)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(data.blocker_id)
        .bind(data.blocked_id)
        .bind(data.organization_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(block)
    }

    /// Unblock a user.
    pub async fn unblock_user(&self, blocker_id: Uuid, blocked_id: Uuid) -> Result<(), SqlxError> {
        sqlx::query(
            r#"
            DELETE FROM user_blocks
            WHERE blocker_id = $1 AND blocked_id = $2
            "#,
        )
        .bind(blocker_id)
        .bind(blocked_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get a specific block.
    pub async fn get_block(
        &self,
        blocker_id: Uuid,
        blocked_id: Uuid,
    ) -> Result<Option<UserBlock>, SqlxError> {
        let block = sqlx::query_as::<_, UserBlock>(
            r#"
            SELECT * FROM user_blocks
            WHERE blocker_id = $1 AND blocked_id = $2
            "#,
        )
        .bind(blocker_id)
        .bind(blocked_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(block)
    }

    /// Check if either user has blocked the other.
    pub async fn is_blocked(&self, user_a: Uuid, user_b: Uuid) -> Result<bool, SqlxError> {
        let (exists,): (bool,) = sqlx::query_as(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM user_blocks
                WHERE (blocker_id = $1 AND blocked_id = $2)
                   OR (blocker_id = $2 AND blocked_id = $1)
            )
            "#,
        )
        .bind(user_a)
        .bind(user_b)
        .fetch_one(&self.pool)
        .await?;

        Ok(exists)
    }

    /// List blocked users with their info.
    pub async fn list_blocked_users(
        &self,
        blocker_id: Uuid,
    ) -> Result<Vec<BlockWithUserInfo>, SqlxError> {
        let rows = sqlx::query_as::<_, BlockWithUserInfoRow>(
            r#"
            SELECT
                b.id,
                b.blocker_id,
                b.blocked_id,
                b.created_at,
                u.first_name as blocked_first_name,
                u.last_name as blocked_last_name,
                u.email as blocked_email
            FROM user_blocks b
            JOIN users u ON u.id = b.blocked_id
            WHERE b.blocker_id = $1
            ORDER BY b.created_at DESC
            "#,
        )
        .bind(blocker_id)
        .fetch_all(&self.pool)
        .await?;

        let blocks = rows.into_iter().map(BlockWithUserInfo::from).collect();

        Ok(blocks)
    }

    /// Count blocked users.
    pub async fn count_blocked_users(&self, blocker_id: Uuid) -> Result<i64, SqlxError> {
        let (count,): (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM user_blocks WHERE blocker_id = $1
            "#,
        )
        .bind(blocker_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count)
    }
}

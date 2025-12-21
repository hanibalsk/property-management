//! User Onboarding repository (Epic 10B, Story 10B.6).
//!
//! Repository for user onboarding tour progress tracking.

use crate::DbPool;
use chrono::{DateTime, Utc};
use sqlx::Error as SqlxError;
use uuid::Uuid;

/// Repository for onboarding operations.
#[derive(Clone)]
pub struct OnboardingRepository {
    pool: DbPool,
}

/// User onboarding progress.
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct UserOnboardingProgress {
    pub id: Uuid,
    pub user_id: Uuid,
    pub tour_id: String,
    pub completed_steps: serde_json::Value,
    pub current_step: Option<String>,
    pub is_completed: bool,
    pub is_skipped: bool,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Onboarding tour definition.
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct OnboardingTour {
    pub id: Uuid,
    pub tour_id: String,
    pub name: String,
    pub description: Option<String>,
    pub steps: serde_json::Value,
    pub target_roles: Option<Vec<String>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Single tour step.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct TourStep {
    pub id: String,
    pub title: String,
    pub content: String,
    pub target: Option<String>,
}

/// Tour with user progress.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct TourWithProgress {
    pub tour: OnboardingTour,
    pub progress: Option<UserOnboardingProgress>,
}

impl OnboardingRepository {
    /// Create a new OnboardingRepository.
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Get user's onboarding progress for a tour.
    pub async fn get_progress(
        &self,
        user_id: Uuid,
        tour_id: &str,
    ) -> Result<Option<UserOnboardingProgress>, SqlxError> {
        let progress = sqlx::query_as::<_, UserOnboardingProgress>(
            r#"
            SELECT id, user_id, tour_id, completed_steps, current_step, is_completed,
                   is_skipped, started_at, completed_at, created_at, updated_at
            FROM user_onboarding_progress
            WHERE user_id = $1 AND tour_id = $2
            "#,
        )
        .bind(user_id)
        .bind(tour_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(progress)
    }

    /// Get all tours for a user with progress.
    pub async fn get_tours_for_user(
        &self,
        user_id: Uuid,
        roles: &[String],
    ) -> Result<Vec<TourWithProgress>, SqlxError> {
        // Get active tours that match user's roles
        let tours = sqlx::query_as::<_, OnboardingTour>(
            r#"
            SELECT id, tour_id, name, description, steps, target_roles, is_active, created_at, updated_at
            FROM onboarding_tours
            WHERE is_active = true
              AND (target_roles IS NULL OR target_roles && $1::varchar[])
            ORDER BY created_at ASC
            "#,
        )
        .bind(roles)
        .fetch_all(&self.pool)
        .await?;

        // Get progress for each tour
        let mut results = Vec::new();
        for tour in tours {
            let progress = self.get_progress(user_id, &tour.tour_id).await?;
            results.push(TourWithProgress { tour, progress });
        }

        Ok(results)
    }

    /// Start or resume a tour for a user.
    pub async fn start_tour(
        &self,
        user_id: Uuid,
        tour_id: &str,
    ) -> Result<UserOnboardingProgress, SqlxError> {
        let progress = sqlx::query_as::<_, UserOnboardingProgress>(
            r#"
            INSERT INTO user_onboarding_progress (user_id, tour_id)
            VALUES ($1, $2)
            ON CONFLICT (user_id, tour_id) DO UPDATE SET
                updated_at = NOW()
            RETURNING id, user_id, tour_id, completed_steps, current_step, is_completed,
                      is_skipped, started_at, completed_at, created_at, updated_at
            "#,
        )
        .bind(user_id)
        .bind(tour_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(progress)
    }

    /// Complete a step in the tour.
    pub async fn complete_step(
        &self,
        user_id: Uuid,
        tour_id: &str,
        step_id: &str,
    ) -> Result<Option<UserOnboardingProgress>, SqlxError> {
        let progress = sqlx::query_as::<_, UserOnboardingProgress>(
            r#"
            UPDATE user_onboarding_progress
            SET
                completed_steps = CASE
                    WHEN NOT (completed_steps @> $3::jsonb)
                    THEN completed_steps || $3::jsonb
                    ELSE completed_steps
                END,
                current_step = $4,
                updated_at = NOW()
            WHERE user_id = $1 AND tour_id = $2
            RETURNING id, user_id, tour_id, completed_steps, current_step, is_completed,
                      is_skipped, started_at, completed_at, created_at, updated_at
            "#,
        )
        .bind(user_id)
        .bind(tour_id)
        .bind(serde_json::json!([step_id]))
        .bind(step_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(progress)
    }

    /// Complete the entire tour.
    pub async fn complete_tour(
        &self,
        user_id: Uuid,
        tour_id: &str,
    ) -> Result<Option<UserOnboardingProgress>, SqlxError> {
        let progress = sqlx::query_as::<_, UserOnboardingProgress>(
            r#"
            UPDATE user_onboarding_progress
            SET
                is_completed = true,
                completed_at = NOW(),
                updated_at = NOW()
            WHERE user_id = $1 AND tour_id = $2
            RETURNING id, user_id, tour_id, completed_steps, current_step, is_completed,
                      is_skipped, started_at, completed_at, created_at, updated_at
            "#,
        )
        .bind(user_id)
        .bind(tour_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(progress)
    }

    /// Skip the tour.
    pub async fn skip_tour(
        &self,
        user_id: Uuid,
        tour_id: &str,
    ) -> Result<Option<UserOnboardingProgress>, SqlxError> {
        let progress = sqlx::query_as::<_, UserOnboardingProgress>(
            r#"
            UPDATE user_onboarding_progress
            SET
                is_skipped = true,
                updated_at = NOW()
            WHERE user_id = $1 AND tour_id = $2
            RETURNING id, user_id, tour_id, completed_steps, current_step, is_completed,
                      is_skipped, started_at, completed_at, created_at, updated_at
            "#,
        )
        .bind(user_id)
        .bind(tour_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(progress)
    }

    /// Reset tour progress (for restarting).
    pub async fn reset_tour(
        &self,
        user_id: Uuid,
        tour_id: &str,
    ) -> Result<Option<UserOnboardingProgress>, SqlxError> {
        let progress = sqlx::query_as::<_, UserOnboardingProgress>(
            r#"
            UPDATE user_onboarding_progress
            SET
                completed_steps = '[]'::jsonb,
                current_step = NULL,
                is_completed = false,
                is_skipped = false,
                completed_at = NULL,
                started_at = NOW(),
                updated_at = NOW()
            WHERE user_id = $1 AND tour_id = $2
            RETURNING id, user_id, tour_id, completed_steps, current_step, is_completed,
                      is_skipped, started_at, completed_at, created_at, updated_at
            "#,
        )
        .bind(user_id)
        .bind(tour_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(progress)
    }

    /// Get tour definition by ID.
    pub async fn get_tour(&self, tour_id: &str) -> Result<Option<OnboardingTour>, SqlxError> {
        let tour = sqlx::query_as::<_, OnboardingTour>(
            r#"
            SELECT id, tour_id, name, description, steps, target_roles, is_active, created_at, updated_at
            FROM onboarding_tours
            WHERE tour_id = $1 AND is_active = true
            "#,
        )
        .bind(tour_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(tour)
    }

    /// Get all tours (admin).
    pub async fn list_all_tours(&self) -> Result<Vec<OnboardingTour>, SqlxError> {
        let tours = sqlx::query_as::<_, OnboardingTour>(
            r#"
            SELECT id, tour_id, name, description, steps, target_roles, is_active, created_at, updated_at
            FROM onboarding_tours
            ORDER BY created_at ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tours)
    }

    /// Create or update a tour (admin).
    pub async fn upsert_tour(
        &self,
        tour_id: &str,
        name: &str,
        description: Option<&str>,
        steps: serde_json::Value,
        target_roles: Option<Vec<String>>,
        is_active: bool,
    ) -> Result<OnboardingTour, SqlxError> {
        let tour = sqlx::query_as::<_, OnboardingTour>(
            r#"
            INSERT INTO onboarding_tours (tour_id, name, description, steps, target_roles, is_active)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (tour_id) DO UPDATE SET
                name = $2,
                description = $3,
                steps = $4,
                target_roles = $5,
                is_active = $6,
                updated_at = NOW()
            RETURNING id, tour_id, name, description, steps, target_roles, is_active, created_at, updated_at
            "#,
        )
        .bind(tour_id)
        .bind(name)
        .bind(description)
        .bind(steps)
        .bind(target_roles.as_deref())
        .bind(is_active)
        .fetch_one(&self.pool)
        .await?;

        Ok(tour)
    }

    /// Delete a tour (admin).
    pub async fn delete_tour(&self, tour_id: &str) -> Result<bool, SqlxError> {
        let result = sqlx::query("DELETE FROM onboarding_tours WHERE tour_id = $1")
            .bind(tour_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Check if user needs to see onboarding.
    pub async fn needs_onboarding(&self, user_id: Uuid) -> Result<bool, SqlxError> {
        let incomplete = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)
            FROM onboarding_tours t
            LEFT JOIN user_onboarding_progress p ON p.tour_id = t.tour_id AND p.user_id = $1
            WHERE t.is_active = true
              AND (p.id IS NULL OR (p.is_completed = false AND p.is_skipped = false))
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(incomplete > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tour_step_serialization() {
        let step = TourStep {
            id: "welcome".to_string(),
            title: "Welcome!".to_string(),
            content: "Welcome to the app.".to_string(),
            target: Some("[data-tour=welcome]".to_string()),
        };

        let json = serde_json::to_string(&step).unwrap();
        assert!(json.contains("welcome"));
        assert!(json.contains("Welcome!"));
    }
}

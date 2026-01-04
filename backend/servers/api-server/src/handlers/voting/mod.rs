//! Voting handlers (UC-04, Epic 5).
//!
//! Implements building voting and decision making including vote creation,
//! question management, voting workflow, delegation, comments, and results.

use crate::state::AppState;
use chrono::{DateTime, NaiveDate, Utc};
use common::{errors::ErrorResponse, TenantContext};
use db::models::{
    CancelVote, CastVote, CreateVote, CreateVoteComment, CreateVoteQuestion, HideVoteComment,
    PublishVote, QuestionOption, UpdateVote, UpdateVoteQuestion, Vote, VoteAuditLog, VoteComment,
    VoteCommentWithUser, VoteEligibility, VoteListQuery, VoteQuestion, VoteReceipt, VoteReportData,
    VoteResponse, VoteResults, VoteSummary, VoteWithDetails,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;
use uuid::Uuid;

// ============================================================================
// Error Types
// ============================================================================

/// Voting handler errors.
#[derive(Debug, Error)]
pub enum VotingHandlerError {
    #[error("Vote not found")]
    VoteNotFound,

    #[error("Question not found")]
    QuestionNotFound,

    #[error("Comment not found")]
    CommentNotFound,

    #[error("Not authorized")]
    NotAuthorized,

    #[error("Vote cannot be edited in current status")]
    CannotEditVote,

    #[error("Vote is not active")]
    VoteNotActive,

    #[error("Vote has already been published")]
    AlreadyPublished,

    #[error("Vote cannot be cancelled in current status")]
    CannotCancel,

    #[error("User has already voted")]
    AlreadyVoted,

    #[error("User is not eligible to vote")]
    NotEligible,

    #[error("Delegation not allowed for this vote")]
    DelegationNotAllowed,

    #[error("Invalid quorum type: {0}")]
    InvalidQuorumType(String),

    #[error("Invalid question type: {0}")]
    InvalidQuestionType(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Missing required answer for question: {0}")]
    MissingRequiredAnswer(String),

    #[error("Invalid answer format")]
    InvalidAnswerFormat,

    #[error("Vote requires at least one question")]
    NoQuestions,

    #[error("Database error: {0}")]
    Database(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<VotingHandlerError> for ErrorResponse {
    fn from(err: VotingHandlerError) -> Self {
        match err {
            VotingHandlerError::VoteNotFound => ErrorResponse::new("NOT_FOUND", "Vote not found"),
            VotingHandlerError::QuestionNotFound => {
                ErrorResponse::new("NOT_FOUND", "Question not found")
            }
            VotingHandlerError::CommentNotFound => {
                ErrorResponse::new("NOT_FOUND", "Comment not found")
            }
            VotingHandlerError::NotAuthorized => ErrorResponse::new(
                "NOT_AUTHORIZED",
                "You are not authorized to perform this action",
            ),
            VotingHandlerError::CannotEditVote => {
                ErrorResponse::new("CANNOT_EDIT", "Vote can only be edited in draft status")
            }
            VotingHandlerError::VoteNotActive => {
                ErrorResponse::new("NOT_ACTIVE", "Vote is not currently active")
            }
            VotingHandlerError::AlreadyPublished => {
                ErrorResponse::new("ALREADY_PUBLISHED", "Vote has already been published")
            }
            VotingHandlerError::CannotCancel => ErrorResponse::new(
                "CANNOT_CANCEL",
                "Vote cannot be cancelled in current status",
            ),
            VotingHandlerError::AlreadyVoted => {
                ErrorResponse::new("ALREADY_VOTED", "You have already voted")
            }
            VotingHandlerError::NotEligible => {
                ErrorResponse::new("NOT_ELIGIBLE", "You are not eligible to vote")
            }
            VotingHandlerError::DelegationNotAllowed => ErrorResponse::new(
                "DELEGATION_NOT_ALLOWED",
                "This vote does not allow delegation",
            ),
            VotingHandlerError::InvalidQuorumType(msg) => {
                ErrorResponse::new("INVALID_QUORUM_TYPE", msg)
            }
            VotingHandlerError::InvalidQuestionType(msg) => {
                ErrorResponse::new("INVALID_QUESTION_TYPE", msg)
            }
            VotingHandlerError::InvalidInput(msg) => ErrorResponse::new("INVALID_INPUT", msg),
            VotingHandlerError::MissingRequiredAnswer(q) => ErrorResponse::new(
                "MISSING_ANSWER",
                format!("Answer required for question: {}", q),
            ),
            VotingHandlerError::InvalidAnswerFormat => {
                ErrorResponse::new("INVALID_ANSWER", "Invalid answer format")
            }
            VotingHandlerError::NoQuestions => {
                ErrorResponse::new("NO_QUESTIONS", "Vote must have at least one question")
            }
            VotingHandlerError::Database(msg) => ErrorResponse::new("DB_ERROR", msg),
            VotingHandlerError::Internal(msg) => ErrorResponse::new("INTERNAL_ERROR", msg),
        }
    }
}

// ============================================================================
// Request/Response Types
// ============================================================================

/// Create vote request data.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateVoteData {
    pub building_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub start_at: Option<DateTime<Utc>>,
    pub end_at: DateTime<Utc>,
    pub quorum_type: String,
    pub quorum_percentage: Option<i32>,
    pub allow_delegation: Option<bool>,
    pub anonymous_voting: Option<bool>,
}

/// Update vote request data.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateVoteData {
    pub title: Option<String>,
    pub description: Option<String>,
    pub start_at: Option<DateTime<Utc>>,
    pub end_at: Option<DateTime<Utc>>,
    pub quorum_type: Option<String>,
    pub quorum_percentage: Option<i32>,
    pub allow_delegation: Option<bool>,
    pub anonymous_voting: Option<bool>,
}

/// Publish vote request data.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct PublishVoteData {
    pub start_at: Option<DateTime<Utc>>,
}

/// Cancel vote request data.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CancelVoteData {
    pub reason: String,
}

/// Add question request data.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct AddQuestionData {
    pub question_text: String,
    pub description: Option<String>,
    pub question_type: String,
    pub options: Vec<QuestionOption>,
    pub display_order: Option<i32>,
    pub is_required: Option<bool>,
}

/// Update question request data.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateQuestionData {
    pub question_text: Option<String>,
    pub description: Option<String>,
    pub options: Option<Vec<QuestionOption>>,
    pub display_order: Option<i32>,
    pub is_required: Option<bool>,
}

/// Cast vote request data.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CastVoteData {
    pub unit_id: Uuid,
    pub delegation_id: Option<Uuid>,
    pub answers: serde_json::Value,
}

/// Add comment request data.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct AddCommentData {
    pub parent_id: Option<Uuid>,
    pub content: String,
    pub ai_consent: bool,
}

/// Hide comment request data.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct HideCommentData {
    pub reason: String,
}

/// List votes query parameters.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ListVotesParams {
    pub building_id: Option<Uuid>,
    pub status: Option<String>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Vote list result.
#[derive(Debug, Serialize, ToSchema)]
pub struct VoteListResult {
    pub votes: Vec<VoteSummary>,
    pub count: usize,
}

// ============================================================================
// Constants
// ============================================================================

const VALID_QUORUM_TYPES: [&str; 4] = [
    "simple_majority",
    "two_thirds",
    "qualified_majority",
    "unanimous",
];
const VALID_QUESTION_TYPES: [&str; 4] = ["yes_no", "single_choice", "multiple_choice", "text"];
const VALID_VOTE_STATUSES: [&str; 5] = ["draft", "published", "active", "closed", "cancelled"];

// ============================================================================
// Handler Implementation
// ============================================================================

/// Voting handler providing business logic for voting operations.
pub struct VotingHandler;

impl VotingHandler {
    // ========================================================================
    // Validation Helpers
    // ========================================================================

    /// Validate quorum type.
    fn validate_quorum_type(quorum_type: &str) -> Result<(), VotingHandlerError> {
        if !VALID_QUORUM_TYPES.contains(&quorum_type) {
            return Err(VotingHandlerError::InvalidQuorumType(format!(
                "Invalid quorum type. Must be one of: {}",
                VALID_QUORUM_TYPES.join(", ")
            )));
        }
        Ok(())
    }

    /// Validate question type.
    fn validate_question_type(question_type: &str) -> Result<(), VotingHandlerError> {
        if !VALID_QUESTION_TYPES.contains(&question_type) {
            return Err(VotingHandlerError::InvalidQuestionType(format!(
                "Invalid question type. Must be one of: {}",
                VALID_QUESTION_TYPES.join(", ")
            )));
        }
        Ok(())
    }

    // ========================================================================
    // Vote CRUD Operations
    // ========================================================================

    /// Create a new vote (Story 5.1).
    pub async fn create_vote(
        state: &AppState,
        context: &TenantContext,
        data: CreateVoteData,
    ) -> Result<Vote, VotingHandlerError> {
        // Validate required fields
        if data.title.trim().is_empty() {
            return Err(VotingHandlerError::InvalidInput("Title is required".into()));
        }

        // Validate quorum type
        Self::validate_quorum_type(&data.quorum_type)?;

        // Validate end_at is in the future
        if data.end_at <= Utc::now() {
            return Err(VotingHandlerError::InvalidInput(
                "End date must be in the future".into(),
            ));
        }

        // Validate start_at if provided
        if let Some(start_at) = &data.start_at {
            if start_at >= &data.end_at {
                return Err(VotingHandlerError::InvalidInput(
                    "Start date must be before end date".into(),
                ));
            }
        }

        // Validate quorum percentage if provided
        if let Some(pct) = data.quorum_percentage {
            if !(1..=100).contains(&pct) {
                return Err(VotingHandlerError::InvalidInput(
                    "Quorum percentage must be between 1 and 100".into(),
                ));
            }
        }

        let create_data = CreateVote {
            organization_id: context.tenant_id,
            building_id: data.building_id,
            title: data.title,
            description: data.description,
            start_at: data.start_at,
            end_at: data.end_at,
            quorum_type: data.quorum_type,
            quorum_percentage: data.quorum_percentage,
            allow_delegation: data.allow_delegation,
            anonymous_voting: data.anonymous_voting,
            created_by: context.user_id,
        };

        // TODO: Migrate to create_poll_rls when route handlers pass RLS connection
        #[allow(deprecated)]
        let vote = state.vote_repo.create(create_data).await.map_err(|e| {
            tracing::error!(error = %e, "Failed to create vote");
            VotingHandlerError::Database("Failed to create vote".into())
        })?;

        tracing::info!(
            vote_id = %vote.id,
            building_id = %vote.building_id,
            created_by = %context.user_id,
            "Vote created"
        );

        Ok(vote)
    }

    /// List votes with filters.
    pub async fn list_votes(
        state: &AppState,
        context: &TenantContext,
        params: ListVotesParams,
    ) -> Result<VoteListResult, VotingHandlerError> {
        let list_query = VoteListQuery {
            building_id: params.building_id,
            status: params.status.map(|s| vec![s]),
            created_by: None,
            from_date: params.from_date,
            to_date: params.to_date,
            limit: params.limit,
            offset: params.offset,
        };

        let votes = state
            .vote_repo
            .list(context.tenant_id, list_query)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to list votes");
                VotingHandlerError::Database("Failed to list votes".into())
            })?;

        let count = votes.len();
        Ok(VoteListResult { votes, count })
    }

    /// Get vote by ID with details.
    pub async fn get_vote(
        state: &AppState,
        vote_id: Uuid,
    ) -> Result<VoteWithDetails, VotingHandlerError> {
        let vote = state
            .vote_repo
            .find_by_id_with_details(vote_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to get vote");
                VotingHandlerError::Database("Failed to get vote".into())
            })?
            .ok_or(VotingHandlerError::VoteNotFound)?;

        Ok(vote)
    }

    /// Update vote.
    pub async fn update_vote(
        state: &AppState,
        vote_id: Uuid,
        data: UpdateVoteData,
    ) -> Result<Vote, VotingHandlerError> {
        // Get existing vote
        // TODO: Migrate to find_poll_by_id_rls when route handlers pass RLS connection
        #[allow(deprecated)]
        let existing = state
            .vote_repo
            .find_by_id(vote_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to find vote");
                VotingHandlerError::Database("Failed to find vote".into())
            })?
            .ok_or(VotingHandlerError::VoteNotFound)?;

        // Check if vote can be edited
        if !existing.can_edit() {
            return Err(VotingHandlerError::CannotEditVote);
        }

        // Validate quorum type if provided
        if let Some(ref quorum_type) = data.quorum_type {
            Self::validate_quorum_type(quorum_type)?;
        }

        // Validate quorum percentage if provided
        if let Some(pct) = data.quorum_percentage {
            if !(1..=100).contains(&pct) {
                return Err(VotingHandlerError::InvalidInput(
                    "Quorum percentage must be between 1 and 100".into(),
                ));
            }
        }

        let update_data = UpdateVote {
            title: data.title,
            description: data.description,
            start_at: data.start_at,
            end_at: data.end_at,
            quorum_type: data.quorum_type,
            quorum_percentage: data.quorum_percentage,
            allow_delegation: data.allow_delegation,
            anonymous_voting: data.anonymous_voting,
        };

        let vote = state
            .vote_repo
            .update(vote_id, update_data)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to update vote");
                VotingHandlerError::Database("Failed to update vote".into())
            })?;

        tracing::info!(vote_id = %vote_id, "Vote updated");

        Ok(vote)
    }

    /// Delete vote.
    pub async fn delete_vote(state: &AppState, vote_id: Uuid) -> Result<(), VotingHandlerError> {
        // Get existing vote
        // TODO: Migrate to find_poll_by_id_rls when route handlers pass RLS connection
        #[allow(deprecated)]
        let existing = state
            .vote_repo
            .find_by_id(vote_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to find vote");
                VotingHandlerError::Database("Failed to find vote".into())
            })?
            .ok_or(VotingHandlerError::VoteNotFound)?;

        // Check if vote can be edited
        if !existing.can_edit() {
            return Err(VotingHandlerError::CannotEditVote);
        }

        state.vote_repo.delete(vote_id).await.map_err(|e| {
            tracing::error!(error = %e, "Failed to delete vote");
            VotingHandlerError::Database("Failed to delete vote".into())
        })?;

        tracing::info!(vote_id = %vote_id, "Vote deleted");

        Ok(())
    }

    // ========================================================================
    // Workflow Operations
    // ========================================================================

    /// Publish a vote (Story 5.2).
    pub async fn publish_vote(
        state: &AppState,
        context: &TenantContext,
        vote_id: Uuid,
        data: PublishVoteData,
    ) -> Result<Vote, VotingHandlerError> {
        // Get existing vote
        // TODO: Migrate to find_poll_by_id_rls when route handlers pass RLS connection
        #[allow(deprecated)]
        let existing = state
            .vote_repo
            .find_by_id(vote_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to find vote");
                VotingHandlerError::Database("Failed to find vote".into())
            })?
            .ok_or(VotingHandlerError::VoteNotFound)?;

        // Check if vote can be published
        if !existing.can_edit() {
            return Err(VotingHandlerError::AlreadyPublished);
        }

        // Check vote has at least one question
        let questions = state.vote_repo.get_questions(vote_id).await.map_err(|e| {
            tracing::error!(error = %e, "Failed to get questions");
            VotingHandlerError::Database("Failed to get questions".into())
        })?;

        if questions.is_empty() {
            return Err(VotingHandlerError::NoQuestions);
        }

        let publish_data = PublishVote {
            start_at: data.start_at,
        };

        let vote = state
            .vote_repo
            .publish(vote_id, context.user_id, publish_data)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to publish vote");
                VotingHandlerError::Database("Failed to publish vote".into())
            })?;

        tracing::info!(
            vote_id = %vote_id,
            published_by = %context.user_id,
            "Vote published"
        );

        Ok(vote)
    }

    /// Cancel a vote.
    pub async fn cancel_vote(
        state: &AppState,
        context: &TenantContext,
        vote_id: Uuid,
        data: CancelVoteData,
    ) -> Result<Vote, VotingHandlerError> {
        // Get existing vote
        // TODO: Migrate to find_poll_by_id_rls when route handlers pass RLS connection
        #[allow(deprecated)]
        let existing = state
            .vote_repo
            .find_by_id(vote_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to find vote");
                VotingHandlerError::Database("Failed to find vote".into())
            })?
            .ok_or(VotingHandlerError::VoteNotFound)?;

        // Check if vote can be cancelled
        if existing.status == "closed" || existing.status == "cancelled" {
            return Err(VotingHandlerError::CannotCancel);
        }

        // Validate reason
        if data.reason.trim().is_empty() {
            return Err(VotingHandlerError::InvalidInput(
                "Cancellation reason is required".into(),
            ));
        }

        let cancel_data = CancelVote {
            reason: data.reason,
        };

        let vote = state
            .vote_repo
            .cancel(vote_id, context.user_id, cancel_data)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to cancel vote");
                VotingHandlerError::Database("Failed to cancel vote".into())
            })?;

        tracing::info!(
            vote_id = %vote_id,
            cancelled_by = %context.user_id,
            "Vote cancelled"
        );

        Ok(vote)
    }

    /// Close a vote and calculate results.
    pub async fn close_vote(state: &AppState, vote_id: Uuid) -> Result<Vote, VotingHandlerError> {
        // Get existing vote
        // TODO: Migrate to find_poll_by_id_rls when route handlers pass RLS connection
        #[allow(deprecated)]
        let existing = state
            .vote_repo
            .find_by_id(vote_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to find vote");
                VotingHandlerError::Database("Failed to find vote".into())
            })?
            .ok_or(VotingHandlerError::VoteNotFound)?;

        // Check if vote is active
        if !existing.is_active() {
            return Err(VotingHandlerError::VoteNotActive);
        }

        let vote = state.vote_repo.close(vote_id).await.map_err(|e| {
            tracing::error!(error = %e, "Failed to close vote");
            VotingHandlerError::Database("Failed to close vote".into())
        })?;

        tracing::info!(vote_id = %vote_id, "Vote closed");

        Ok(vote)
    }

    // ========================================================================
    // Question Operations
    // ========================================================================

    /// Add a question to a vote (Story 5.1).
    pub async fn add_question(
        state: &AppState,
        vote_id: Uuid,
        data: AddQuestionData,
    ) -> Result<VoteQuestion, VotingHandlerError> {
        // Get existing vote
        // TODO: Migrate to find_poll_by_id_rls when route handlers pass RLS connection
        #[allow(deprecated)]
        let existing = state
            .vote_repo
            .find_by_id(vote_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to find vote");
                VotingHandlerError::Database("Failed to find vote".into())
            })?
            .ok_or(VotingHandlerError::VoteNotFound)?;

        // Check if vote can be edited
        if !existing.can_edit() {
            return Err(VotingHandlerError::CannotEditVote);
        }

        // Validate question type
        Self::validate_question_type(&data.question_type)?;

        // Validate question text
        if data.question_text.trim().is_empty() {
            return Err(VotingHandlerError::InvalidInput(
                "Question text is required".into(),
            ));
        }

        // Validate options for choice questions
        if (data.question_type == "single_choice" || data.question_type == "multiple_choice")
            && data.options.is_empty()
        {
            return Err(VotingHandlerError::InvalidInput(
                "Choice questions require at least one option".into(),
            ));
        }

        let question_data = CreateVoteQuestion {
            vote_id,
            question_text: data.question_text,
            description: data.description,
            question_type: data.question_type,
            options: data.options,
            display_order: data.display_order,
            is_required: data.is_required,
        };

        let question = state
            .vote_repo
            .add_question(question_data)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to add question");
                VotingHandlerError::Database("Failed to add question".into())
            })?;

        tracing::info!(
            vote_id = %vote_id,
            question_id = %question.id,
            "Question added"
        );

        Ok(question)
    }

    /// List questions for a vote.
    pub async fn list_questions(
        state: &AppState,
        vote_id: Uuid,
    ) -> Result<Vec<VoteQuestion>, VotingHandlerError> {
        let questions = state.vote_repo.get_questions(vote_id).await.map_err(|e| {
            tracing::error!(error = %e, "Failed to list questions");
            VotingHandlerError::Database("Failed to list questions".into())
        })?;

        Ok(questions)
    }

    /// Update a question.
    pub async fn update_question(
        state: &AppState,
        vote_id: Uuid,
        question_id: Uuid,
        data: UpdateQuestionData,
    ) -> Result<VoteQuestion, VotingHandlerError> {
        // Get existing vote
        // TODO: Migrate to find_poll_by_id_rls when route handlers pass RLS connection
        #[allow(deprecated)]
        let existing = state
            .vote_repo
            .find_by_id(vote_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to find vote");
                VotingHandlerError::Database("Failed to find vote".into())
            })?
            .ok_or(VotingHandlerError::VoteNotFound)?;

        // Check if vote can be edited
        if !existing.can_edit() {
            return Err(VotingHandlerError::CannotEditVote);
        }

        let update_data = UpdateVoteQuestion {
            question_text: data.question_text,
            description: data.description,
            options: data.options,
            display_order: data.display_order,
            is_required: data.is_required,
        };

        let question = state
            .vote_repo
            .update_question(question_id, update_data)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to update question");
                VotingHandlerError::Database("Failed to update question".into())
            })?;

        tracing::info!(question_id = %question_id, "Question updated");

        Ok(question)
    }

    /// Delete a question.
    pub async fn delete_question(
        state: &AppState,
        vote_id: Uuid,
        question_id: Uuid,
    ) -> Result<(), VotingHandlerError> {
        // Get existing vote
        // TODO: Migrate to find_poll_by_id_rls when route handlers pass RLS connection
        #[allow(deprecated)]
        let existing = state
            .vote_repo
            .find_by_id(vote_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to find vote");
                VotingHandlerError::Database("Failed to find vote".into())
            })?
            .ok_or(VotingHandlerError::VoteNotFound)?;

        // Check if vote can be edited
        if !existing.can_edit() {
            return Err(VotingHandlerError::CannotEditVote);
        }

        state
            .vote_repo
            .delete_question(question_id, vote_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to delete question");
                VotingHandlerError::Database("Failed to delete question".into())
            })?;

        tracing::info!(question_id = %question_id, "Question deleted");

        Ok(())
    }

    // ========================================================================
    // Voting Operations
    // ========================================================================

    /// Check vote eligibility for current user.
    pub async fn check_eligibility(
        state: &AppState,
        context: &TenantContext,
        vote_id: Uuid,
    ) -> Result<VoteEligibility, VotingHandlerError> {
        let eligibility = state
            .vote_repo
            .check_eligibility(vote_id, context.user_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to check eligibility");
                VotingHandlerError::Database("Failed to check eligibility".into())
            })?;

        Ok(eligibility)
    }

    /// Cast a vote (Story 5.3).
    pub async fn cast_vote(
        state: &AppState,
        context: &TenantContext,
        vote_id: Uuid,
        data: CastVoteData,
    ) -> Result<VoteReceipt, VotingHandlerError> {
        // Get existing vote
        // TODO: Migrate to find_poll_by_id_rls when route handlers pass RLS connection
        #[allow(deprecated)]
        let existing = state
            .vote_repo
            .find_by_id(vote_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to find vote");
                VotingHandlerError::Database("Failed to find vote".into())
            })?
            .ok_or(VotingHandlerError::VoteNotFound)?;

        // Check if vote is active
        if !existing.is_active() {
            return Err(VotingHandlerError::VoteNotActive);
        }

        // Check delegation if provided
        if data.delegation_id.is_some() && !existing.allow_delegation {
            return Err(VotingHandlerError::DelegationNotAllowed);
        }

        let cast_data = CastVote {
            vote_id,
            user_id: context.user_id,
            unit_id: data.unit_id,
            delegation_id: data.delegation_id,
            answers: data.answers,
        };

        // TODO: Migrate to cast_vote_rls when route handlers pass RLS connection
        #[allow(deprecated)]
        let receipt = state.vote_repo.cast_vote(cast_data).await.map_err(|e| {
            // Check for duplicate vote error
            let err_str = e.to_string();
            if err_str.contains("already") || err_str.contains("duplicate") {
                return VotingHandlerError::AlreadyVoted;
            }
            if err_str.contains("eligible") || err_str.contains("permission") {
                return VotingHandlerError::NotEligible;
            }
            tracing::error!(error = %e, "Failed to cast vote");
            VotingHandlerError::Database("Failed to cast vote".into())
        })?;

        tracing::info!(
            vote_id = %vote_id,
            user_id = %context.user_id,
            unit_id = %data.unit_id,
            "Vote cast"
        );

        Ok(receipt)
    }

    /// Get user's response for a vote.
    pub async fn get_my_response(
        state: &AppState,
        vote_id: Uuid,
        unit_id: Uuid,
    ) -> Result<Option<VoteResponse>, VotingHandlerError> {
        let response = state
            .vote_repo
            .get_user_response(vote_id, unit_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to get response");
                VotingHandlerError::Database("Failed to get response".into())
            })?;

        Ok(response)
    }

    // ========================================================================
    // Comment Operations
    // ========================================================================

    /// Add a comment to a vote (Story 5.4).
    pub async fn add_comment(
        state: &AppState,
        context: &TenantContext,
        vote_id: Uuid,
        data: AddCommentData,
    ) -> Result<VoteComment, VotingHandlerError> {
        // Validate content
        if data.content.trim().is_empty() {
            return Err(VotingHandlerError::InvalidInput(
                "Comment content is required".into(),
            ));
        }

        let comment_data = CreateVoteComment {
            vote_id,
            user_id: context.user_id,
            parent_id: data.parent_id,
            content: data.content,
            ai_consent: data.ai_consent,
        };

        let comment = state
            .vote_repo
            .add_comment(comment_data)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to add comment");
                VotingHandlerError::Database("Failed to add comment".into())
            })?;

        tracing::info!(
            vote_id = %vote_id,
            comment_id = %comment.id,
            user_id = %context.user_id,
            "Comment added"
        );

        Ok(comment)
    }

    /// List comments for a vote.
    pub async fn list_comments(
        state: &AppState,
        vote_id: Uuid,
        include_hidden: bool,
    ) -> Result<Vec<VoteCommentWithUser>, VotingHandlerError> {
        let comments = state
            .vote_repo
            .list_comments(vote_id, include_hidden)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to list comments");
                VotingHandlerError::Database("Failed to list comments".into())
            })?;

        Ok(comments)
    }

    /// List replies to a comment.
    pub async fn list_replies(
        state: &AppState,
        comment_id: Uuid,
        include_hidden: bool,
    ) -> Result<Vec<VoteCommentWithUser>, VotingHandlerError> {
        let replies = state
            .vote_repo
            .list_replies(comment_id, include_hidden)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to list replies");
                VotingHandlerError::Database("Failed to list replies".into())
            })?;

        Ok(replies)
    }

    /// Hide a comment.
    pub async fn hide_comment(
        state: &AppState,
        context: &TenantContext,
        comment_id: Uuid,
        data: HideCommentData,
    ) -> Result<VoteComment, VotingHandlerError> {
        // Validate reason
        if data.reason.trim().is_empty() {
            return Err(VotingHandlerError::InvalidInput(
                "Hide reason is required".into(),
            ));
        }

        let hide_data = HideVoteComment {
            reason: data.reason,
        };

        let comment = state
            .vote_repo
            .hide_comment(comment_id, context.user_id, hide_data)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to hide comment");
                VotingHandlerError::Database("Failed to hide comment".into())
            })?;

        tracing::info!(
            comment_id = %comment_id,
            hidden_by = %context.user_id,
            "Comment hidden"
        );

        Ok(comment)
    }

    // ========================================================================
    // Results and Reports
    // ========================================================================

    /// Get vote results (Story 5.5).
    pub async fn get_results(
        state: &AppState,
        vote_id: Uuid,
    ) -> Result<VoteResults, VotingHandlerError> {
        // TODO: Migrate to get_poll_results_rls when route handlers pass RLS connection
        #[allow(deprecated)]
        let results = state
            .vote_repo
            .get_results(vote_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to get results");
                VotingHandlerError::Database("Failed to get results".into())
            })?
            .ok_or(VotingHandlerError::VoteNotFound)?;

        Ok(results)
    }

    /// Get vote report data.
    pub async fn get_report_data(
        state: &AppState,
        vote_id: Uuid,
    ) -> Result<VoteReportData, VotingHandlerError> {
        let report = state
            .vote_repo
            .generate_report_data(vote_id)
            .await
            .map_err(|e| {
                // Check if it's a not found error
                if e.to_string().contains("RowNotFound") {
                    return VotingHandlerError::VoteNotFound;
                }
                tracing::error!(error = %e, "Failed to generate report");
                VotingHandlerError::Database("Failed to generate report".into())
            })?;

        Ok(report)
    }

    /// Get vote audit log.
    pub async fn get_audit_log(
        state: &AppState,
        vote_id: Uuid,
    ) -> Result<Vec<VoteAuditLog>, VotingHandlerError> {
        let entries = state.vote_repo.get_audit_log(vote_id).await.map_err(|e| {
            tracing::error!(error = %e, "Failed to get audit log");
            VotingHandlerError::Database("Failed to get audit log".into())
        })?;

        Ok(entries)
    }

    // ========================================================================
    // Building-specific Operations
    // ========================================================================

    /// List active votes for a building.
    pub async fn list_active_by_building(
        state: &AppState,
        building_id: Uuid,
    ) -> Result<Vec<VoteSummary>, VotingHandlerError> {
        // TODO: Migrate to list_polls_by_building_rls when route handlers pass RLS connection
        #[allow(deprecated)]
        let votes = state
            .vote_repo
            .list_active_by_building(building_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to list active votes");
                VotingHandlerError::Database("Failed to list active votes".into())
            })?;

        Ok(votes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_conversion() {
        let err = VotingHandlerError::VoteNotFound;
        let response: ErrorResponse = err.into();
        assert_eq!(response.code, "NOT_FOUND");
    }

    #[test]
    fn test_validate_quorum_type() {
        assert!(VotingHandler::validate_quorum_type("simple_majority").is_ok());
        assert!(VotingHandler::validate_quorum_type("two_thirds").is_ok());
        assert!(VotingHandler::validate_quorum_type("invalid").is_err());
    }

    #[test]
    fn test_validate_question_type() {
        assert!(VotingHandler::validate_question_type("yes_no").is_ok());
        assert!(VotingHandler::validate_question_type("single_choice").is_ok());
        assert!(VotingHandler::validate_question_type("multiple_choice").is_ok());
        assert!(VotingHandler::validate_question_type("invalid").is_err());
    }
}

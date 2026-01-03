//! Message key definitions for i18n.
//!
//! Each variant corresponds to a Fluent message ID in the locale files.

use serde::{Deserialize, Serialize};

/// Message keys for localized strings.
///
/// These keys map to Fluent message IDs in the locale files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageKey {
    // Common errors
    ErrorGeneric,
    ErrorNotFound,
    ErrorUnauthorized,
    ErrorForbidden,
    ErrorBadRequest,
    ErrorConflict,
    ErrorValidation,
    ErrorRateLimited,
    ErrorInternal,
    ErrorDatabase,
    ErrorExternalService,

    // Auth errors
    AuthInvalidCredentials,
    AuthEmailRequired,
    AuthPasswordRequired,
    AuthInvalidEmail,
    AuthWeakPassword,
    AuthEmailExists,
    AuthAccountLocked,
    AuthTokenExpired,
    AuthTokenInvalid,
    AuthSessionExpired,

    // Auth success messages
    AuthRegistrationSuccess,
    AuthLoginSuccess,
    AuthLogoutSuccess,
    AuthPasswordResetSent,
    AuthPasswordResetSuccess,
    AuthEmailVerified,
    AuthEmailVerificationSent,
    AuthEmailVerificationSuccess,
    AuthPasswordResetEmailSent,
    AuthSessionRevoked,

    // Validation errors
    ValidationRequired,
    ValidationInvalidFormat,
    ValidationTooShort,
    ValidationTooLong,
    ValidationOutOfRange,
    ValidationInvalidValue,
    ValidationStreetRequired,
    ValidationCityRequired,
    ValidationTitleRequired,
    ValidationQuestionTextRequired,
    ValidationCommentRequired,

    // Resource errors
    ResourceNotFound,
    ResourceAlreadyExists,
    ResourceAccessDenied,

    // Fault/Issue messages
    FaultCreated,
    FaultUpdated,
    FaultAssigned,
    FaultResolved,
    FaultClosed,
    FaultCreatedSuccess,
    FaultUpdatedSuccess,
    FaultTriagedSuccess,
    FaultAssignedSuccess,
    FaultStatusUpdated,
    FaultResolvedSuccess,
    FaultConfirmedSuccess,
    FaultReopenedSuccess,

    // Notification messages
    NotificationSent,
    NotificationFailed,

    // Document messages
    DocumentUploaded,
    DocumentDeleted,
    DocumentNotFound,
    DocumentCreatedSuccess,
    DocumentUpdatedSuccess,
    DocumentMovedSuccess,
    DocumentAccessUpdated,
    DocumentFolderCreatedSuccess,
    DocumentFolderUpdatedSuccess,
    DocumentShareCreatedSuccess,

    // Voting messages
    VoteSubmitted,
    VoteAlreadyCast,
    VotingClosed,
    VotingEndDateMustBeFuture,
    VotingStartBeforeEnd,
    VotingQuorumRangeInvalid,
    VotingChoicesRequired,
    VotingHideReasonRequired,

    // Organization messages
    OrganizationCreated,
    OrganizationUpdated,
    OrganizationMemberAdded,
    OrganizationMemberRemoved,
    OrganizationDeletedSuccess,
    OrganizationMemberAddedSuccess,
    OrganizationRoleUpdatedSuccess,
    OrganizationMemberRemovedSuccess,
    OrganizationRoleDeletedSuccess,

    // Announcement messages
    AnnouncementCreatedSuccess,
    AnnouncementUpdatedSuccess,
    AnnouncementPublishedSuccess,
    AnnouncementScheduledSuccess,
    AnnouncementArchivedSuccess,

    // Form messages
    FormCreatedSuccess,
    FormUpdatedSuccess,
    FormPublishedSuccess,
    FormArchivedSuccess,
    FormFieldAddedSuccess,
    FormFieldUpdatedSuccess,
    FormSubmittedSuccess,

    // Messaging
    MessageSentSuccess,
    UserBlockedSuccess,
    UserUnblockedSuccess,

    // Package & Visitor
    PackageRegisteredSuccess,
    PackageUpdatedSuccess,
    PackageMarkedReceived,
    PackagePickedUpSuccess,
    VisitorUpdatedSuccess,
    VisitorCheckedInSuccess,
    VisitorCheckedOutSuccess,
    VisitorRegistrationCancelled,
}

impl MessageKey {
    /// Get the Fluent message ID for this key.
    pub fn as_fluent_id(&self) -> &'static str {
        match self {
            // Common errors
            Self::ErrorGeneric => "error-generic",
            Self::ErrorNotFound => "error-not-found",
            Self::ErrorUnauthorized => "error-unauthorized",
            Self::ErrorForbidden => "error-forbidden",
            Self::ErrorBadRequest => "error-bad-request",
            Self::ErrorConflict => "error-conflict",
            Self::ErrorValidation => "error-validation",
            Self::ErrorRateLimited => "error-rate-limited",
            Self::ErrorInternal => "error-internal",
            Self::ErrorDatabase => "error-database",
            Self::ErrorExternalService => "error-external-service",

            // Auth errors
            Self::AuthInvalidCredentials => "auth-invalid-credentials",
            Self::AuthEmailRequired => "auth-email-required",
            Self::AuthPasswordRequired => "auth-password-required",
            Self::AuthInvalidEmail => "auth-invalid-email",
            Self::AuthWeakPassword => "auth-weak-password",
            Self::AuthEmailExists => "auth-email-exists",
            Self::AuthAccountLocked => "auth-account-locked",
            Self::AuthTokenExpired => "auth-token-expired",
            Self::AuthTokenInvalid => "auth-token-invalid",
            Self::AuthSessionExpired => "auth-session-expired",

            // Auth success
            Self::AuthRegistrationSuccess => "auth-registration-success",
            Self::AuthLoginSuccess => "auth-login-success",
            Self::AuthLogoutSuccess => "auth-logout-success",
            Self::AuthPasswordResetSent => "auth-password-reset-sent",
            Self::AuthPasswordResetSuccess => "auth-password-reset-success",
            Self::AuthEmailVerified => "auth-email-verified",
            Self::AuthEmailVerificationSent => "auth-email-verification-sent",
            Self::AuthEmailVerificationSuccess => "auth-email-verification-success",
            Self::AuthPasswordResetEmailSent => "auth-password-reset-email-sent",
            Self::AuthSessionRevoked => "auth-session-revoked",

            // Validation
            Self::ValidationRequired => "validation-required",
            Self::ValidationInvalidFormat => "validation-invalid-format",
            Self::ValidationTooShort => "validation-too-short",
            Self::ValidationTooLong => "validation-too-long",
            Self::ValidationOutOfRange => "validation-out-of-range",
            Self::ValidationInvalidValue => "validation-invalid-value",
            Self::ValidationStreetRequired => "validation-street-required",
            Self::ValidationCityRequired => "validation-city-required",
            Self::ValidationTitleRequired => "validation-title-required",
            Self::ValidationQuestionTextRequired => "validation-question-text-required",
            Self::ValidationCommentRequired => "validation-comment-required",

            // Resources
            Self::ResourceNotFound => "resource-not-found",
            Self::ResourceAlreadyExists => "resource-already-exists",
            Self::ResourceAccessDenied => "resource-access-denied",

            // Faults
            Self::FaultCreated => "fault-created",
            Self::FaultUpdated => "fault-updated",
            Self::FaultAssigned => "fault-assigned",
            Self::FaultResolved => "fault-resolved",
            Self::FaultClosed => "fault-closed",
            Self::FaultCreatedSuccess => "fault-created-success",
            Self::FaultUpdatedSuccess => "fault-updated-success",
            Self::FaultTriagedSuccess => "fault-triaged-success",
            Self::FaultAssignedSuccess => "fault-assigned-success",
            Self::FaultStatusUpdated => "fault-status-updated",
            Self::FaultResolvedSuccess => "fault-resolved-success",
            Self::FaultConfirmedSuccess => "fault-confirmed-success",
            Self::FaultReopenedSuccess => "fault-reopened-success",

            // Notifications
            Self::NotificationSent => "notification-sent",
            Self::NotificationFailed => "notification-failed",

            // Documents
            Self::DocumentUploaded => "document-uploaded",
            Self::DocumentDeleted => "document-deleted",
            Self::DocumentNotFound => "document-not-found",
            Self::DocumentCreatedSuccess => "document-created-success",
            Self::DocumentUpdatedSuccess => "document-updated-success",
            Self::DocumentMovedSuccess => "document-moved-success",
            Self::DocumentAccessUpdated => "document-access-updated",
            Self::DocumentFolderCreatedSuccess => "document-folder-created-success",
            Self::DocumentFolderUpdatedSuccess => "document-folder-updated-success",
            Self::DocumentShareCreatedSuccess => "document-share-created-success",

            // Voting
            Self::VoteSubmitted => "vote-submitted",
            Self::VoteAlreadyCast => "vote-already-cast",
            Self::VotingClosed => "voting-closed",
            Self::VotingEndDateMustBeFuture => "voting-end-date-must-be-future",
            Self::VotingStartBeforeEnd => "voting-start-before-end",
            Self::VotingQuorumRangeInvalid => "voting-quorum-range-invalid",
            Self::VotingChoicesRequired => "voting-choices-required",
            Self::VotingHideReasonRequired => "voting-hide-reason-required",

            // Organizations
            Self::OrganizationCreated => "organization-created",
            Self::OrganizationUpdated => "organization-updated",
            Self::OrganizationMemberAdded => "organization-member-added",
            Self::OrganizationMemberRemoved => "organization-member-removed",
            Self::OrganizationDeletedSuccess => "organization-deleted-success",
            Self::OrganizationMemberAddedSuccess => "organization-member-added-success",
            Self::OrganizationRoleUpdatedSuccess => "organization-role-updated-success",
            Self::OrganizationMemberRemovedSuccess => "organization-member-removed-success",
            Self::OrganizationRoleDeletedSuccess => "organization-role-deleted-success",

            // Announcements
            Self::AnnouncementCreatedSuccess => "announcement-created-success",
            Self::AnnouncementUpdatedSuccess => "announcement-updated-success",
            Self::AnnouncementPublishedSuccess => "announcement-published-success",
            Self::AnnouncementScheduledSuccess => "announcement-scheduled-success",
            Self::AnnouncementArchivedSuccess => "announcement-archived-success",

            // Forms
            Self::FormCreatedSuccess => "form-created-success",
            Self::FormUpdatedSuccess => "form-updated-success",
            Self::FormPublishedSuccess => "form-published-success",
            Self::FormArchivedSuccess => "form-archived-success",
            Self::FormFieldAddedSuccess => "form-field-added-success",
            Self::FormFieldUpdatedSuccess => "form-field-updated-success",
            Self::FormSubmittedSuccess => "form-submitted-success",

            // Messaging
            Self::MessageSentSuccess => "message-sent-success",
            Self::UserBlockedSuccess => "user-blocked-success",
            Self::UserUnblockedSuccess => "user-unblocked-success",

            // Package & Visitor
            Self::PackageRegisteredSuccess => "package-registered-success",
            Self::PackageUpdatedSuccess => "package-updated-success",
            Self::PackageMarkedReceived => "package-marked-received",
            Self::PackagePickedUpSuccess => "package-picked-up-success",
            Self::VisitorUpdatedSuccess => "visitor-updated-success",
            Self::VisitorCheckedInSuccess => "visitor-checked-in-success",
            Self::VisitorCheckedOutSuccess => "visitor-checked-out-success",
            Self::VisitorRegistrationCancelled => "visitor-registration-cancelled",
        }
    }
}

impl std::fmt::Display for MessageKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_fluent_id())
    }
}

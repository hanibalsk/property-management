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

    // Validation errors
    ValidationRequired,
    ValidationInvalidFormat,
    ValidationTooShort,
    ValidationTooLong,
    ValidationOutOfRange,
    ValidationInvalidValue,

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

    // Notification messages
    NotificationSent,
    NotificationFailed,

    // Document messages
    DocumentUploaded,
    DocumentDeleted,
    DocumentNotFound,

    // Voting messages
    VoteSubmitted,
    VoteAlreadyCast,
    VotingClosed,

    // Organization messages
    OrganizationCreated,
    OrganizationUpdated,
    OrganizationMemberAdded,
    OrganizationMemberRemoved,
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

            // Validation
            Self::ValidationRequired => "validation-required",
            Self::ValidationInvalidFormat => "validation-invalid-format",
            Self::ValidationTooShort => "validation-too-short",
            Self::ValidationTooLong => "validation-too-long",
            Self::ValidationOutOfRange => "validation-out-of-range",
            Self::ValidationInvalidValue => "validation-invalid-value",

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

            // Notifications
            Self::NotificationSent => "notification-sent",
            Self::NotificationFailed => "notification-failed",

            // Documents
            Self::DocumentUploaded => "document-uploaded",
            Self::DocumentDeleted => "document-deleted",
            Self::DocumentNotFound => "document-not-found",

            // Voting
            Self::VoteSubmitted => "vote-submitted",
            Self::VoteAlreadyCast => "vote-already-cast",
            Self::VotingClosed => "voting-closed",

            // Organizations
            Self::OrganizationCreated => "organization-created",
            Self::OrganizationUpdated => "organization-updated",
            Self::OrganizationMemberAdded => "organization-member-added",
            Self::OrganizationMemberRemoved => "organization-member-removed",
        }
    }
}

impl std::fmt::Display for MessageKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_fluent_id())
    }
}

//! Notification event definitions (Story 84.4).
//!
//! Defines all notification events that can trigger notifications in the system.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::{NotificationCategory, NotificationPriority};

// ============================================================================
// Target Type (for announcements)
// ============================================================================

/// Target type for announcements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum TargetType {
    /// All users in the organization.
    All,
    /// Specific buildings.
    Building,
    /// Specific units.
    Units,
    /// Specific roles.
    Roles,
}

impl TargetType {
    /// Get the string representation.
    pub const fn as_str(&self) -> &'static str {
        match self {
            TargetType::All => "all",
            TargetType::Building => "building",
            TargetType::Units => "units",
            TargetType::Roles => "roles",
        }
    }
}

impl std::fmt::Display for TargetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================================
// Fault Status
// ============================================================================

/// Fault status enum for status change notifications.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum FaultStatus {
    /// Newly reported fault.
    Reported,
    /// Fault has been triaged.
    Triaged,
    /// Work in progress.
    InProgress,
    /// Waiting for parts or external action.
    OnHold,
    /// Fault has been resolved.
    Resolved,
    /// Fault was closed without resolution.
    Closed,
    /// Fault was rejected.
    Rejected,
}

impl FaultStatus {
    /// Get the string representation.
    pub const fn as_str(&self) -> &'static str {
        match self {
            FaultStatus::Reported => "reported",
            FaultStatus::Triaged => "triaged",
            FaultStatus::InProgress => "in_progress",
            FaultStatus::OnHold => "on_hold",
            FaultStatus::Resolved => "resolved",
            FaultStatus::Closed => "closed",
            FaultStatus::Rejected => "rejected",
        }
    }

    /// Get human-readable label.
    pub const fn label(&self) -> &'static str {
        match self {
            FaultStatus::Reported => "Reported",
            FaultStatus::Triaged => "Under Review",
            FaultStatus::InProgress => "In Progress",
            FaultStatus::OnHold => "On Hold",
            FaultStatus::Resolved => "Resolved",
            FaultStatus::Closed => "Closed",
            FaultStatus::Rejected => "Rejected",
        }
    }
}

impl std::fmt::Display for FaultStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

// ============================================================================
// Notification Events
// ============================================================================

/// All notification event types in the system.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NotificationEvent {
    /// An announcement was published.
    AnnouncementPublished {
        /// Announcement ID.
        announcement_id: Uuid,
        /// Organization ID.
        organization_id: Uuid,
        /// Target type.
        target_type: TargetType,
        /// Target IDs (buildings, units, or empty for "all").
        target_ids: Vec<Uuid>,
        /// Announcement title.
        title: String,
    },

    /// A fault status changed.
    FaultStatusChanged {
        /// Fault ID.
        fault_id: Uuid,
        /// Reporter user ID.
        reporter_id: Uuid,
        /// Assigned technician ID (if any).
        technician_id: Option<Uuid>,
        /// Previous status.
        old_status: FaultStatus,
        /// New status.
        new_status: FaultStatus,
        /// Fault title/summary.
        title: String,
    },

    /// A fault was assigned to a technician.
    FaultAssigned {
        /// Fault ID.
        fault_id: Uuid,
        /// Reporter user ID.
        reporter_id: Uuid,
        /// Assigned technician ID.
        technician_id: Uuid,
        /// Fault title/summary.
        title: String,
    },

    /// A new vote was created.
    VoteCreated {
        /// Vote ID.
        vote_id: Uuid,
        /// Organization ID.
        organization_id: Uuid,
        /// Specific building (if targeted).
        building_id: Option<Uuid>,
        /// Vote title.
        title: String,
        /// Voting deadline.
        deadline: DateTime<Utc>,
    },

    /// Reminder for a vote deadline.
    VoteReminder {
        /// Vote ID.
        vote_id: Uuid,
        /// Vote title.
        title: String,
        /// Voting deadline.
        deadline: DateTime<Utc>,
        /// Hours remaining.
        hours_remaining: i32,
    },

    /// A new message was received.
    MessageReceived {
        /// Message ID.
        message_id: Uuid,
        /// Recipient user ID.
        recipient_id: Uuid,
        /// Sender name.
        sender_name: String,
        /// Message preview (truncated).
        preview: String,
    },

    /// A signature request was created.
    SignatureRequested {
        /// Signature request ID.
        request_id: Uuid,
        /// Document name.
        document_name: String,
        /// Sender name.
        sender_name: String,
        /// Expiration date.
        expires_at: DateTime<Utc>,
    },

    /// A signature reminder.
    SignatureReminder {
        /// Signature request ID.
        request_id: Uuid,
        /// Document name.
        document_name: String,
        /// Reminder level (1, 2, 3).
        reminder_level: i32,
        /// Days until expiration.
        days_remaining: i32,
    },

    /// A document was fully signed.
    SignatureCompleted {
        /// Signature request ID.
        request_id: Uuid,
        /// Document name.
        document_name: String,
    },

    /// A payment is due.
    PaymentDue {
        /// Invoice ID.
        invoice_id: Uuid,
        /// Amount due.
        amount: String,
        /// Due date.
        due_date: DateTime<Utc>,
    },

    /// A payment was received.
    PaymentReceived {
        /// Payment ID.
        payment_id: Uuid,
        /// Amount received.
        amount: String,
    },

    /// Emergency alert.
    EmergencyAlert {
        /// Emergency ID.
        emergency_id: Uuid,
        /// Building ID (if specific).
        building_id: Option<Uuid>,
        /// Alert title.
        title: String,
        /// Severity level.
        severity: String,
    },

    /// Community event notification.
    CommunityEvent {
        /// Event ID.
        event_id: Uuid,
        /// Event title.
        title: String,
        /// Event date.
        event_date: DateTime<Utc>,
    },
}

impl NotificationEvent {
    /// Get the category for this event.
    pub fn category(&self) -> NotificationCategory {
        match self {
            NotificationEvent::AnnouncementPublished { .. } => NotificationCategory::Announcements,
            NotificationEvent::FaultStatusChanged { .. }
            | NotificationEvent::FaultAssigned { .. } => NotificationCategory::Faults,
            NotificationEvent::VoteCreated { .. } | NotificationEvent::VoteReminder { .. } => {
                NotificationCategory::Votes
            }
            NotificationEvent::MessageReceived { .. } => NotificationCategory::Messages,
            NotificationEvent::SignatureRequested { .. }
            | NotificationEvent::SignatureReminder { .. }
            | NotificationEvent::SignatureCompleted { .. } => NotificationCategory::Financial,
            NotificationEvent::PaymentDue { .. } | NotificationEvent::PaymentReceived { .. } => {
                NotificationCategory::Financial
            }
            NotificationEvent::EmergencyAlert { .. } => NotificationCategory::System,
            NotificationEvent::CommunityEvent { .. } => NotificationCategory::Community,
        }
    }

    /// Get the priority for this event.
    pub fn priority(&self) -> NotificationPriority {
        match self {
            NotificationEvent::EmergencyAlert { .. } => NotificationPriority::Urgent,
            NotificationEvent::FaultStatusChanged { new_status, .. }
                if *new_status == FaultStatus::Resolved =>
            {
                NotificationPriority::High
            }
            NotificationEvent::PaymentDue { .. } => NotificationPriority::High,
            NotificationEvent::VoteReminder {
                hours_remaining, ..
            } if *hours_remaining <= 24 => NotificationPriority::High,
            NotificationEvent::SignatureReminder { reminder_level, .. } if *reminder_level >= 3 => {
                NotificationPriority::High
            }
            _ => NotificationPriority::Normal,
        }
    }

    /// Get the notification title for this event.
    pub fn title(&self) -> String {
        match self {
            NotificationEvent::AnnouncementPublished { title, .. } => {
                format!("New Announcement: {title}")
            }
            NotificationEvent::FaultStatusChanged {
                new_status, title, ..
            } => {
                format!("Fault {}: {title}", new_status.label())
            }
            NotificationEvent::FaultAssigned { title, .. } => {
                format!("Fault Assigned: {title}")
            }
            NotificationEvent::VoteCreated { title, .. } => {
                format!("New Vote: {title}")
            }
            NotificationEvent::VoteReminder { title, .. } => {
                format!("Vote Reminder: {title}")
            }
            NotificationEvent::MessageReceived { sender_name, .. } => {
                format!("New Message from {sender_name}")
            }
            NotificationEvent::SignatureRequested { document_name, .. } => {
                format!("Signature Required: {document_name}")
            }
            NotificationEvent::SignatureReminder { document_name, .. } => {
                format!("Signature Reminder: {document_name}")
            }
            NotificationEvent::SignatureCompleted { document_name, .. } => {
                format!("Document Signed: {document_name}")
            }
            NotificationEvent::PaymentDue { amount, .. } => {
                format!("Payment Due: {amount}")
            }
            NotificationEvent::PaymentReceived { amount, .. } => {
                format!("Payment Received: {amount}")
            }
            NotificationEvent::EmergencyAlert { title, .. } => {
                format!("EMERGENCY: {title}")
            }
            NotificationEvent::CommunityEvent { title, .. } => {
                format!("Community Event: {title}")
            }
        }
    }

    /// Get the notification body for this event.
    pub fn body(&self) -> String {
        match self {
            NotificationEvent::AnnouncementPublished { .. } => {
                "A new announcement has been published. Tap to view.".to_string()
            }
            NotificationEvent::FaultStatusChanged {
                old_status,
                new_status,
                ..
            } => {
                format!(
                    "Status changed from {} to {}",
                    old_status.label(),
                    new_status.label()
                )
            }
            NotificationEvent::FaultAssigned { .. } => {
                "A technician has been assigned to your reported fault.".to_string()
            }
            NotificationEvent::VoteCreated { deadline, .. } => {
                format!(
                    "A new vote is available. Deadline: {}",
                    deadline.format("%Y-%m-%d %H:%M")
                )
            }
            NotificationEvent::VoteReminder {
                hours_remaining, ..
            } => {
                format!("Only {hours_remaining} hours remaining to cast your vote.")
            }
            NotificationEvent::MessageReceived { preview, .. } => preview.clone(),
            NotificationEvent::SignatureRequested {
                sender_name,
                expires_at,
                ..
            } => {
                format!(
                    "{sender_name} has requested your signature. Expires: {}",
                    expires_at.format("%Y-%m-%d")
                )
            }
            NotificationEvent::SignatureReminder {
                days_remaining,
                reminder_level,
                ..
            } => match reminder_level {
                1 => format!("Please sign the document. {days_remaining} days remaining."),
                2 => format!("Reminder: {days_remaining} days left to sign."),
                _ => format!("URGENT: Only {days_remaining} days left to sign!"),
            },
            NotificationEvent::SignatureCompleted { .. } => {
                "All parties have signed. Download the completed document.".to_string()
            }
            NotificationEvent::PaymentDue {
                amount, due_date, ..
            } => {
                format!(
                    "Payment of {amount} is due on {}",
                    due_date.format("%Y-%m-%d")
                )
            }
            NotificationEvent::PaymentReceived { amount, .. } => {
                format!("Your payment of {amount} has been received. Thank you!")
            }
            NotificationEvent::EmergencyAlert { severity, .. } => {
                format!("Severity: {severity}. Please check the details immediately.")
            }
            NotificationEvent::CommunityEvent { event_date, .. } => {
                format!(
                    "Event scheduled for {}",
                    event_date.format("%Y-%m-%d %H:%M")
                )
            }
        }
    }

    /// Get the action URL for deep linking.
    pub fn action_url(&self) -> Option<String> {
        match self {
            NotificationEvent::AnnouncementPublished {
                announcement_id, ..
            } => Some(format!("/announcements/{announcement_id}")),
            NotificationEvent::FaultStatusChanged { fault_id, .. }
            | NotificationEvent::FaultAssigned { fault_id, .. } => {
                Some(format!("/faults/{fault_id}"))
            }
            NotificationEvent::VoteCreated { vote_id, .. }
            | NotificationEvent::VoteReminder { vote_id, .. } => Some(format!("/votes/{vote_id}")),
            NotificationEvent::MessageReceived { message_id, .. } => {
                Some(format!("/messages/{message_id}"))
            }
            NotificationEvent::SignatureRequested { request_id, .. }
            | NotificationEvent::SignatureReminder { request_id, .. }
            | NotificationEvent::SignatureCompleted { request_id, .. } => {
                Some(format!("/signatures/{request_id}"))
            }
            NotificationEvent::PaymentDue { invoice_id, .. } => {
                Some(format!("/invoices/{invoice_id}"))
            }
            NotificationEvent::PaymentReceived { payment_id, .. } => {
                Some(format!("/payments/{payment_id}"))
            }
            NotificationEvent::EmergencyAlert { emergency_id, .. } => {
                Some(format!("/emergencies/{emergency_id}"))
            }
            NotificationEvent::CommunityEvent { event_id, .. } => {
                Some(format!("/community/events/{event_id}"))
            }
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_announcement_event() {
        let event = NotificationEvent::AnnouncementPublished {
            announcement_id: Uuid::new_v4(),
            organization_id: Uuid::new_v4(),
            target_type: TargetType::All,
            target_ids: vec![],
            title: "Test Announcement".to_string(),
        };

        assert_eq!(event.category(), NotificationCategory::Announcements);
        assert_eq!(event.priority(), NotificationPriority::Normal);
        assert!(event.title().contains("Test Announcement"));
    }

    #[test]
    fn test_fault_status_change_event() {
        let fault_id = Uuid::new_v4();
        let event = NotificationEvent::FaultStatusChanged {
            fault_id,
            reporter_id: Uuid::new_v4(),
            technician_id: None,
            old_status: FaultStatus::Reported,
            new_status: FaultStatus::Resolved,
            title: "Broken pipe".to_string(),
        };

        assert_eq!(event.category(), NotificationCategory::Faults);
        assert_eq!(event.priority(), NotificationPriority::High);
        assert_eq!(event.action_url(), Some(format!("/faults/{fault_id}")));
    }

    #[test]
    fn test_emergency_event_priority() {
        let event = NotificationEvent::EmergencyAlert {
            emergency_id: Uuid::new_v4(),
            building_id: None,
            title: "Fire alarm activated".to_string(),
            severity: "critical".to_string(),
        };

        assert_eq!(event.priority(), NotificationPriority::Urgent);
    }

    #[test]
    fn test_vote_reminder_urgency() {
        let event = NotificationEvent::VoteReminder {
            vote_id: Uuid::new_v4(),
            title: "Budget Vote".to_string(),
            deadline: Utc::now(),
            hours_remaining: 12,
        };

        assert_eq!(event.priority(), NotificationPriority::High);
    }

    #[test]
    fn test_fault_status_labels() {
        assert_eq!(FaultStatus::Reported.label(), "Reported");
        assert_eq!(FaultStatus::InProgress.label(), "In Progress");
        assert_eq!(FaultStatus::Resolved.label(), "Resolved");
    }
}

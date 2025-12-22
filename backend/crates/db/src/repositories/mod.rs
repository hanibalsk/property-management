//! Repository pattern implementations.
//! Each repository handles database operations for a specific domain.

pub mod announcement;
pub mod audit_log;
pub mod building;
pub mod critical_notification;
pub mod data_export;
pub mod delegation;
pub mod document;
pub mod document_template;
pub mod facility;
pub mod fault;
pub mod feature_flag;
pub mod financial;
pub mod granular_notification;
pub mod health_monitoring;
pub mod help;
pub mod messaging;
pub mod meter;
pub mod notification_preference;
pub mod oauth;
pub mod onboarding;
pub mod organization;
pub mod organization_member;
pub mod password_reset;
pub mod person_month;
pub mod platform_admin;
pub mod role;
pub mod session;
pub mod signature_request;
pub mod system_announcement;
pub mod two_factor_auth;
pub mod unit;
pub mod unit_resident;
pub mod user;
pub mod vote;

// Epic 13: AI Assistant & Automation
pub mod ai_chat;
pub mod equipment;
pub mod sentiment;
pub mod workflow;

// Epic 14: IoT & Smart Building
pub mod sensor;

// Epic 15: Property Listings & Multi-Portal Sync
pub mod listing;

// Epic 16: Portal Search & Discovery
pub mod portal;

// Epic 17: Agency & Realtor Management
pub mod agency;

pub use ai_chat::AiChatRepository;
pub use equipment::EquipmentRepository;
pub use sentiment::SentimentRepository;
pub use workflow::WorkflowRepository;

// Epic 14: IoT & Smart Building
pub use sensor::SensorRepository;

pub use announcement::AnnouncementRepository;
pub use audit_log::AuditLogRepository;
pub use building::BuildingRepository;
pub use critical_notification::CriticalNotificationRepository;
pub use data_export::DataExportRepository;
pub use delegation::DelegationRepository;
pub use document::DocumentRepository;
pub use document_template::DocumentTemplateRepository;
pub use facility::FacilityRepository;
pub use fault::FaultRepository;
pub use feature_flag::{
    FeatureFlagRepository, FeatureFlagWithCount, FeatureFlagWithOverrides, ResolvedFeatureFlag,
};
pub use financial::FinancialRepository;
pub use granular_notification::GranularNotificationRepository;
pub use health_monitoring::{
    CurrentMetric, HealthDashboard, HealthMonitoringRepository, MetricDataPoint, MetricHistory,
    MetricStats, MetricStatus,
};
pub use help::{FaqEntry, HelpArticle, HelpCategory, HelpRepository, Tooltip};
pub use messaging::MessagingRepository;
pub use meter::MeterRepository;
pub use notification_preference::NotificationPreferenceRepository;
pub use oauth::OAuthRepository;
pub use onboarding::{
    OnboardingRepository, OnboardingTour, TourStep, TourWithProgress, UserOnboardingProgress,
};
pub use organization::OrganizationRepository;
pub use organization_member::OrganizationMemberRepository;
pub use password_reset::PasswordResetRepository;
pub use person_month::PersonMonthRepository;
pub use platform_admin::{
    PlatformAdminRepository, PlatformStats, SupportActivityLog, SupportUserInfo,
    SupportUserMembership, SupportUserSession,
};
pub use role::RoleRepository;
pub use session::SessionRepository;
pub use signature_request::SignatureRequestRepository;
pub use system_announcement::{ActiveAnnouncement, SystemAnnouncementRepository};
pub use two_factor_auth::TwoFactorAuthRepository;
pub use unit::UnitRepository;
pub use unit_resident::UnitResidentRepository;
pub use user::UserRepository;
pub use vote::VoteRepository;

// Epic 15: Property Listings & Multi-Portal Sync
pub use listing::ListingRepository;

// Epic 16: Portal Search & Discovery
pub use portal::PortalRepository;

// Epic 17: Agency & Realtor Management
pub use agency::AgencyRepository;

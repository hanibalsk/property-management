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

// Epic 18: Short-Term Rental Integration
pub mod rental;

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

// Epic 18: Short-Term Rental Integration
pub use rental::RentalRepository;

// Epic 19: Lease Management & Tenant Screening
pub mod lease;

pub use lease::LeaseRepository;

// Epic 20: Maintenance Scheduling & Work Orders
pub mod work_order;

pub use work_order::WorkOrderRepository;

// Epic 21: Supplier & Vendor Management
pub mod vendor;

pub use vendor::VendorRepository;

// Epic 22: Insurance Management
pub mod insurance;

pub use insurance::InsuranceRepository;

// Epic 23: Emergency Management
pub mod emergency;

pub use emergency::EmergencyRepository;

// Epic 24: Budget & Financial Planning
pub mod budget;

pub use budget::BudgetRepository;

// Epic 25: Legal Document & Compliance
pub mod legal;

pub use legal::LegalRepository;

// Epic 26: Platform Subscription & Billing
pub mod subscription;

pub use subscription::SubscriptionRepository;

// Epic 30: Government Portal Integration
pub mod government_portal;

pub use government_portal::GovernmentPortalRepository;

// Epics 31-34: Reality Portal Professional
pub mod reality_portal;

pub use reality_portal::RealityPortalRepository;

// Epic 37: Community & Social Features
pub mod community;

pub use community::CommunityRepository;

// Epic 38: Workflow Automation
pub mod automation;

// Epic 54: Forms Management
pub mod form;

// Epic 58: Package & Visitor Management
pub mod package_visitor;

pub use automation::AutomationRepository;

// Epic 54: Forms Management
pub use form::FormRepository;

// Epic 58: Package & Visitor Management
pub use package_visitor::PackageVisitorRepository;

// Epic 59: News & Media Management
pub mod news_article;

pub use news_article::NewsArticleRepository;

// Epic 61: External Integrations Suite
pub mod integration;

pub use integration::IntegrationRepository;

// Epic 65: Energy & Sustainability Tracking
pub mod energy;

pub use energy::EnergyRepository;

// Epic 64: Advanced AI & LLM Capabilities
pub mod llm_document;

pub use llm_document::LlmDocumentRepository;

// Epic 70: Competitive Feature Enhancements
pub mod competitive;

pub use competitive::CompetitiveRepository;

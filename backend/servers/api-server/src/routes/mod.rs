//! Route modules for API server.
//!
//! Each module handles a specific domain and provides its own router.

pub mod admin;
pub mod agencies;
pub mod ai;
pub mod announcements;
pub mod auth;
pub mod buildings;
pub mod compliance;
pub mod critical_notifications;
pub mod delegations;
pub mod documents;
pub mod facilities;
pub mod faults;
pub mod financial;
pub mod gdpr;
pub mod granular_notifications;
pub mod health;
pub mod help;
pub mod insurance;
pub mod integrations;
pub mod iot;
pub mod leases;
pub mod listings;
pub mod messaging;
pub mod meters;
pub mod mfa;
pub mod neighbors;
pub mod notification_preferences;
pub mod oauth;
pub mod onboarding;
pub mod organizations;
pub mod person_months;
pub mod platform_admin;
pub mod rentals;
pub mod signatures;
pub mod templates;
pub mod unit_residents;
pub mod vendors;
pub mod voting;
pub mod work_orders;

// Epic 23: Emergency Management
pub mod emergency;

// Epic 24: Budget & Financial Planning
pub mod budgets;

// Epic 25: Legal Document & Compliance
pub mod legal;

// Epic 26: Platform Subscription & Billing
pub mod subscriptions;

// Epic 30: Government Portal Integration
pub mod government_portal;

// Epic 37: Community & Social Features
pub mod community;

// Epic 38: Workflow Automation
pub mod automation;

// Epic 54: Forms Management
pub mod forms;

// Epic 55: Advanced Reporting & Analytics
pub mod reports;

// Epic 58: Package & Visitor Management
pub mod package_visitor;

// Epic 59: News & Media Management
pub mod news_articles;

// Epic 65: Energy & Sustainability Tracking
pub mod energy;

// Epic 70: Competitive Feature Enhancements
pub mod competitive;

// Epic 78: Vendor Operations Portal
pub mod vendor_portal;

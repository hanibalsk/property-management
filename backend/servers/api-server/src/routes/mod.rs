//! Route modules for API server.
//!
//! Each module handles a specific domain and provides its own router.

pub mod admin;
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
pub mod integrations;
pub mod iot;
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
pub mod voting;

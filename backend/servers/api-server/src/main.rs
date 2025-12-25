//! API Server - Property Management System
//!
//! Consolidated backend for all property management operations.
//! Handles authentication, organizations, buildings, faults, voting,
//! rentals, listings, and external integrations.
//!
//! Package: ppt::api_server

// Allow dead code for stub implementations during development
#![allow(dead_code)]

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod handlers;
mod routes;
mod services;
mod state;

use db::repositories::AnnouncementRepository;
use services::{EmailService, JwtService, Scheduler, SchedulerConfig};
use state::AppState;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Property Management API",
        version = "1.0.0",
        description = "API for Property Management System (PPT)",
        contact(name = "PPT Team", email = "api@ppt.example.com"),
        license(name = "MIT")
    ),
    servers(
        (url = "http://localhost:8080", description = "Local development"),
        (url = "https://api.ppt.example.com", description = "Production")
    ),
    paths(
        routes::health::health,
        routes::auth::login,
        routes::auth::register,
        routes::auth::logout,
        routes::auth::verify_email,
        routes::auth::resend_verification,
        routes::auth::refresh_token,
        routes::auth::forgot_password,
        routes::auth::reset_password,
        routes::auth::list_sessions,
        routes::auth::revoke_session,
        routes::auth::revoke_all_sessions,
        routes::admin::list_users,
        routes::admin::get_user,
        routes::admin::suspend_user,
        routes::admin::reactivate_user,
        routes::admin::delete_user,
        routes::organizations::create_organization,
        routes::organizations::list_organizations,
        routes::organizations::list_my_organizations,
        routes::organizations::get_organization,
        routes::organizations::update_organization,
        routes::organizations::delete_organization,
        routes::organizations::list_organization_members,
        routes::organizations::add_organization_member,
        routes::organizations::update_organization_member,
        routes::organizations::remove_organization_member,
        routes::organizations::list_organization_roles,
        routes::organizations::create_organization_role,
        routes::organizations::get_organization_role,
        routes::organizations::update_organization_role,
        routes::organizations::delete_organization_role,
        routes::organizations::get_organization_settings,
        routes::organizations::update_organization_settings,
        routes::organizations::get_organization_branding,
        routes::organizations::update_organization_branding,
        routes::organizations::export_organization_data,
    ),
    components(schemas(
        routes::health::HealthResponse,
        routes::auth::LoginRequest,
        routes::auth::LoginResponse,
        routes::auth::RegisterRequest,
        routes::auth::RegisterResponse,
        routes::auth::VerifyEmailQuery,
        routes::auth::VerifyEmailResponse,
        routes::auth::ResendVerificationRequest,
        routes::auth::ResendVerificationResponse,
        routes::auth::RefreshTokenRequest,
        routes::auth::LogoutRequest,
        routes::auth::LogoutResponse,
        routes::auth::ForgotPasswordRequest,
        routes::auth::ForgotPasswordResponse,
        routes::auth::ResetPasswordRequest,
        routes::auth::ResetPasswordResponse,
        routes::auth::SessionInfo,
        routes::auth::ListSessionsResponse,
        routes::auth::RevokeSessionRequest,
        routes::auth::RevokeSessionResponse,
        routes::auth::RevokeAllSessionsResponse,
        routes::admin::AdminUserInfo,
        routes::admin::ListUsersQuery,
        routes::admin::ListUsersResponse,
        routes::admin::UserActionRequest,
        routes::admin::AdminActionResponse,
        routes::organizations::CreateOrganizationRequest,
        routes::organizations::OrganizationResponse,
        routes::organizations::ListOrganizationsResponse,
        routes::organizations::UpdateOrganizationRequest,
        routes::organizations::DeleteOrganizationResponse,
        routes::organizations::MemberResponse,
        routes::organizations::ListMembersResponse,
        routes::organizations::AddMemberRequest,
        routes::organizations::AddMemberResponse,
        routes::organizations::UpdateMemberRequest,
        routes::organizations::UpdateMemberResponse,
        routes::organizations::RemoveMemberResponse,
        routes::organizations::RoleResponse,
        routes::organizations::ListRolesResponse,
        routes::organizations::CreateRoleRequest,
        routes::organizations::CreateRoleResponse,
        routes::organizations::GetRoleResponse,
        routes::organizations::UpdateRoleRequest,
        routes::organizations::UpdateRoleResponse,
        routes::organizations::DeleteRoleResponse,
        routes::organizations::OrganizationSettingsResponse,
        routes::organizations::UpdateOrganizationSettingsRequest,
        routes::organizations::OrganizationBrandingResponse,
        routes::organizations::UpdateOrganizationBrandingRequest,
        routes::organizations::ExportMember,
        routes::organizations::ExportRole,
        routes::organizations::ExportQuery,
        routes::organizations::OrganizationExportResponse,
        common::errors::ErrorResponse,
        common::errors::ValidationError,
        common::tenant::TenantContext,
        common::tenant::TenantRole,
    )),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Authentication", description = "User authentication and authorization"),
        (name = "Admin", description = "Administrative user management"),
        (name = "Organizations", description = "Multi-tenant organization management"),
        (name = "Buildings", description = "Building and unit management"),
        (name = "Delegations", description = "Ownership delegation management"),
        (name = "Facilities", description = "Common area and facility management"),
        (name = "Facility Bookings", description = "Facility booking and reservation management"),
        (name = "Faults", description = "Fault reporting and tracking"),
        (name = "Voting", description = "Voting and polls"),
        (name = "Announcements", description = "Announcements and communication"),
        (name = "Documents", description = "Document management and sharing"),
        (name = "E-Signatures", description = "Electronic signature workflows for documents"),
        (name = "Notification Preferences", description = "User notification channel preferences"),
        (name = "Critical Notifications", description = "Critical notifications that bypass user preferences"),
        (name = "Multi-Factor Authentication", description = "Two-factor authentication setup and management"),
        (name = "GDPR", description = "GDPR compliance: data export, deletion, and privacy settings"),
        (name = "Compliance", description = "Compliance reports: audit logs, security, and GDPR reports"),
        (name = "Rentals", description = "Short-term rental integrations (Airbnb, Booking)"),
        (name = "Listings", description = "Real estate listing management"),
        (name = "Integrations", description = "External portal integrations"),
        (name = "financial", description = "Financial management: accounts, invoices, payments"),
        (name = "meters", description = "Meter readings and utility management"),
        (name = "AI Chat", description = "AI-powered chat assistant"),
        (name = "Sentiment Analysis", description = "Resident sentiment tracking"),
        (name = "Equipment", description = "Equipment and predictive maintenance"),
        (name = "Workflows", description = "Automated workflow management"),
        (name = "IoT Sensors", description = "IoT sensor registration and management"),
        (name = "Community", description = "Community & social features: groups, posts, events, marketplace"),
        (name = "Automation", description = "Workflow automation rules, templates, and execution")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file if present
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Get database URL from environment
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        tracing::warn!("DATABASE_URL not set, using default");
        "postgres://postgres:postgres@localhost:5432/ppt".to_string()
    });

    // Create database pool
    let db_pool = db::create_pool(&database_url).await?;
    tracing::info!("Connected to database");

    // Create email service (development mode by default)
    let email_enabled = std::env::var("EMAIL_ENABLED")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false);
    let base_url =
        std::env::var("APP_BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let email_service = EmailService::new(base_url, email_enabled);

    // Create JWT service
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| {
        tracing::warn!("JWT_SECRET not set, using development default (NOT FOR PRODUCTION)");
        "development-secret-key-that-is-at-least-32-characters-long".to_string()
    });
    let jwt_service = JwtService::new(&jwt_secret)
        .expect("Failed to create JWT service - secret must be at least 32 characters");

    // Create application state
    let state = AppState::new(db_pool.clone(), email_service, jwt_service);

    // Start background scheduler for scheduled announcements
    let scheduler_enabled = std::env::var("SCHEDULER_ENABLED")
        .map(|v| v != "false" && v != "0")
        .unwrap_or(true);
    let scheduler_config = SchedulerConfig {
        interval_secs: std::env::var("SCHEDULER_INTERVAL_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(60),
        enabled: scheduler_enabled,
    };
    let announcement_repo = AnnouncementRepository::new(db_pool);
    let scheduler = Scheduler::new(announcement_repo, scheduler_config);
    let _scheduler_handle = scheduler.start();

    // Build router
    let app = Router::new()
        // Health check
        .route("/health", get(routes::health::health))
        // Auth routes
        .nest("/api/v1/auth", routes::auth::router())
        // Admin routes
        .nest("/api/v1/admin", routes::admin::router())
        // Organizations routes
        .nest("/api/v1/organizations", routes::organizations::router())
        // Buildings routes
        .nest("/api/v1/buildings", routes::buildings::router())
        // Delegations routes (Epic 3, Story 3.4)
        .nest("/api/v1/delegations", routes::delegations::router())
        // Facilities routes (Epic 3, Story 3.7)
        .nest("/api/v1", routes::facilities::router())
        // Faults routes
        .nest("/api/v1/faults", routes::faults::router())
        // Voting routes
        .nest("/api/v1/voting", routes::voting::router())
        // Announcements routes (Epic 6)
        .nest("/api/v1/announcements", routes::announcements::router())
        // Documents routes (Epic 7A)
        .nest("/api/v1/documents", routes::documents::router())
        // Public shared document routes (no auth required)
        .merge(routes::documents::public_router())
        // Templates routes (Epic 7B)
        .nest("/api/v1/templates", routes::templates::router())
        // E-Signature routes (Epic 7B, Story 7B.3)
        .nest("/api/v1/signature-requests", routes::signatures::router())
        // Messaging routes (Epic 6, Story 6.5)
        .nest("/api/v1/messages", routes::messaging::router())
        // Neighbor routes (Epic 6, Story 6.6)
        .nest("/api/v1", routes::neighbors::router())
        // Notification preferences routes (Epic 8A)
        .nest(
            "/api/v1/users/me/notification-preferences",
            routes::notification_preferences::router(),
        )
        // Granular notification preferences routes (Epic 8B)
        .nest(
            "/api/v1/users/me/notification-preferences/granular",
            routes::granular_notifications::router(),
        )
        // Critical notifications routes (Epic 8A, Story 8A.2)
        .nest(
            "/api/v1/organizations/:org_id/critical-notifications",
            routes::critical_notifications::router(),
        )
        // MFA routes (Epic 9, Story 9.1)
        .nest("/api/v1/auth/mfa", routes::mfa::router())
        // OAuth 2.0 routes (Epic 10A)
        .nest("/api/v1/oauth", routes::oauth::router())
        .nest("/api/v1/admin/oauth", routes::oauth::admin_router())
        // Platform Admin routes (Epic 10B)
        .nest("/api/v1/platform-admin", routes::platform_admin::router())
        // Public feature flags (Epic 10B.2)
        .nest(
            "/api/v1/feature-flags",
            routes::platform_admin::public_feature_flags_router(),
        )
        // Public system announcements (Epic 10B.4)
        .nest(
            "/api/v1/system-announcements",
            routes::platform_admin::public_announcements_router(),
        )
        // Public maintenance (Epic 10B.4)
        .nest(
            "/api/v1/maintenance",
            routes::platform_admin::public_maintenance_router(),
        )
        // Onboarding routes (Epic 10B.6)
        .nest("/api/v1/onboarding", routes::onboarding::router())
        // Help routes (Epic 10B.7)
        .nest("/api/v1/help", routes::help::router())
        // GDPR routes (Epic 9, Stories 9.3-9.5)
        .nest("/api/v1/gdpr", routes::gdpr::router())
        // Compliance routes (Epic 9, Story 9.7)
        .nest("/api/v1/compliance", routes::compliance::router())
        // Rentals routes
        .nest("/api/v1/rentals", routes::rentals::router())
        // Listings routes (management side)
        .nest("/api/v1/listings", routes::listings::router())
        // Integration routes
        .nest("/api/v1/integrations", routes::integrations::router())
        // Financial routes (Epic 11)
        .nest("/api/v1/financial", routes::financial::router())
        // Meters routes (Epic 12)
        .nest("/api/v1/meters", routes::meters::router())
        // AI routes (Epic 13)
        .nest("/api/v1/ai/chat", routes::ai::ai_chat_router())
        .nest("/api/v1/ai/sentiment", routes::ai::sentiment_router())
        .nest("/api/v1/ai/equipment", routes::ai::equipment_router())
        .nest("/api/v1/ai/workflows", routes::ai::workflow_router())
        // IoT routes (Epic 14)
        .nest("/api/v1/iot/sensors", routes::iot::sensor_router())
        // Agency routes (Epic 17)
        .nest("/api/v1/agencies", routes::agencies::router())
        // Lease routes (Epic 19)
        .nest("/api/v1/leases", routes::leases::router())
        // Work Orders routes (Epic 20)
        .nest("/api/v1/work-orders", routes::work_orders::router())
        // Vendor routes (Epic 21)
        .nest("/api/v1/vendors", routes::vendors::router())
        // Insurance routes (Epic 22)
        .nest("/api/v1/insurance", routes::insurance::router())
        // Emergency routes (Epic 23)
        .nest("/api/v1/emergency", routes::emergency::router())
        // Budget routes (Epic 24)
        .nest("/api/v1/budgets", routes::budgets::router())
        // Legal routes (Epic 25)
        .nest("/api/v1/legal", routes::legal::router())
        // Subscription routes (Epic 26)
        .nest("/api/v1/subscriptions", routes::subscriptions::router())
        .nest(
            "/api/v1/admin/subscriptions",
            routes::subscriptions::admin_router(),
        )
        // Government Portal routes (Epic 30)
        .nest(
            "/api/v1/government-portal",
            routes::government_portal::router(),
        )
        // Community routes (Epic 37)
        .nest("/api/v1/community", routes::community::router())
        // Automation routes (Epic 38)
        .nest("/api/v1/automation", routes::automation::router())
        // Forms routes (Epic 54)
        .nest("/api/v1/forms", routes::forms::router())
        // Reports routes (Epic 55)
        .nest("/api/v1/reports", routes::reports::router())
        // Registry routes (Epic 57)
        .nest("/api/v1/registry", routes::registry::router())
        // Swagger UI
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // Middleware
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        // Application state
        .with_state(state);

    // Run server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("API server (Property Management) listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;

    Ok(())
}

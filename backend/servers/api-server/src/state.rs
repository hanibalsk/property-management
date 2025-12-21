//! Application state.

use crate::services::{AuthService, EmailService, JwtService};
use db::{
    repositories::{
        OrganizationMemberRepository, OrganizationRepository, PasswordResetRepository,
        RoleRepository, SessionRepository, UserRepository,
    },
    DbPool,
};

/// Application state shared across all handlers.
#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
    pub user_repo: UserRepository,
    pub session_repo: SessionRepository,
    pub password_reset_repo: PasswordResetRepository,
    pub org_repo: OrganizationRepository,
    pub org_member_repo: OrganizationMemberRepository,
    pub role_repo: RoleRepository,
    pub auth_service: AuthService,
    pub email_service: EmailService,
    pub jwt_service: JwtService,
}

impl AppState {
    /// Create a new AppState.
    pub fn new(db: DbPool, email_service: EmailService, jwt_service: JwtService) -> Self {
        let user_repo = UserRepository::new(db.clone());
        let session_repo = SessionRepository::new(db.clone());
        let password_reset_repo = PasswordResetRepository::new(db.clone());
        let org_repo = OrganizationRepository::new(db.clone());
        let org_member_repo = OrganizationMemberRepository::new(db.clone());
        let role_repo = RoleRepository::new(db.clone());
        let auth_service = AuthService::new();

        Self {
            db,
            user_repo,
            session_repo,
            password_reset_repo,
            org_repo,
            org_member_repo,
            role_repo,
            auth_service,
            email_service,
            jwt_service,
        }
    }
}

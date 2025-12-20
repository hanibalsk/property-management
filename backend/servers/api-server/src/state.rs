//! Application state.

use crate::services::{AuthService, EmailService};
use db::{repositories::UserRepository, DbPool};

/// Application state shared across all handlers.
#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
    pub user_repo: UserRepository,
    pub auth_service: AuthService,
    pub email_service: EmailService,
}

impl AppState {
    /// Create a new AppState.
    pub fn new(db: DbPool, email_service: EmailService) -> Self {
        let user_repo = UserRepository::new(db.clone());
        let auth_service = AuthService::new();

        Self {
            db,
            user_repo,
            auth_service,
            email_service,
        }
    }
}

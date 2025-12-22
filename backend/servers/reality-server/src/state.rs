//! Application state for Reality Server.

use db::{repositories::PortalRepository, DbPool};

/// Application state shared across all handlers.
#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
    pub portal_repo: PortalRepository,
}

impl AppState {
    /// Create a new AppState.
    pub fn new(db: DbPool) -> Self {
        let portal_repo = PortalRepository::new(db.clone());

        Self { db, portal_repo }
    }
}

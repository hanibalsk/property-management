//! Business logic handlers.
//!
//! Each handler module contains the actual implementation logic,
//! while routes handle HTTP concerns (request/response, validation).

pub mod auth;
pub mod buildings;
pub mod faults;
pub mod integrations;
pub mod listings;
pub mod organizations;
pub mod rentals;
pub mod voting;

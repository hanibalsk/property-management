//! Database seeding module.
//!
//! Provides comprehensive sample data seeding for development and testing.
//!
//! # Features
//!
//! - Creates users for all 12 tenant role types
//! - Seeds buildings and units with owner/tenant assignments
//! - Respects RLS using super admin context
//! - Uses Argon2id for password hashing
//!
//! # Usage
//!
//! ```bash
//! # Interactive mode (recommended)
//! cargo run -p api-server --bin ppt-seed
//!
//! # Non-interactive mode
//! cargo run -p api-server --bin ppt-seed -- \
//!   --admin-email admin@example.com \
//!   --admin-password SecurePass123
//! ```

pub mod data;
pub mod factories;
pub mod runner;

pub use data::SeedData;
pub use factories::SeedFactories;
pub use runner::{SeedConfig, SeedError, SeedResult, SeedRunner};

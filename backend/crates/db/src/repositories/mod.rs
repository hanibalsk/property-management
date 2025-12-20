//! Repository pattern implementations.
//! Each repository handles database operations for a specific domain.

pub mod user;

pub use user::UserRepository;

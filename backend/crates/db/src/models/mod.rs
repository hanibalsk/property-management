//! Database models.

pub mod user;

pub use user::{CreateUser, EmailVerificationToken, Locale, UpdateUser, User, UserStatus};

//! Repository pattern implementations.
//! Each repository handles database operations for a specific domain.

pub mod organization;
pub mod organization_member;
pub mod password_reset;
pub mod role;
pub mod session;
pub mod user;

pub use organization::OrganizationRepository;
pub use organization_member::OrganizationMemberRepository;
pub use password_reset::PasswordResetRepository;
pub use role::RoleRepository;
pub use session::SessionRepository;
pub use user::UserRepository;

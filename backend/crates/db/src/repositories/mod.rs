//! Repository pattern implementations.
//! Each repository handles database operations for a specific domain.

pub mod announcement;
pub mod building;
pub mod delegation;
pub mod facility;
pub mod fault;
pub mod messaging;
pub mod organization;
pub mod organization_member;
pub mod password_reset;
pub mod person_month;
pub mod role;
pub mod session;
pub mod unit;
pub mod unit_resident;
pub mod user;
pub mod vote;

pub use announcement::AnnouncementRepository;
pub use building::BuildingRepository;
pub use delegation::DelegationRepository;
pub use facility::FacilityRepository;
pub use fault::FaultRepository;
pub use messaging::MessagingRepository;
pub use organization::OrganizationRepository;
pub use organization_member::OrganizationMemberRepository;
pub use password_reset::PasswordResetRepository;
pub use person_month::PersonMonthRepository;
pub use role::RoleRepository;
pub use session::SessionRepository;
pub use unit::UnitRepository;
pub use unit_resident::UnitResidentRepository;
pub use user::UserRepository;
pub use vote::VoteRepository;

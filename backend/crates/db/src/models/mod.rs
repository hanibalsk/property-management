//! Database models.

pub mod building;
pub mod delegation;
pub mod facility;
pub mod organization;
pub mod organization_member;
pub mod password_reset;
pub mod person_month;
pub mod refresh_token;
pub mod role;
pub mod unit;
pub mod unit_resident;
pub mod user;

pub use building::{
    building_status, Building, BuildingContact, BuildingStatistics, BuildingSummary,
    CreateBuilding, UpdateBuilding,
};
pub use delegation::{
    delegation_scope, delegation_status, AcceptDelegation, CreateDelegation, DeclineDelegation,
    Delegation, DelegationAuditLog, DelegationSummary, DelegationWithUsers, RevokeDelegation,
    UpdateDelegation,
};
pub use facility::{
    booking_status, facility_type, ApproveBooking, AvailableSlot, BookingWithDetails,
    CancelBooking, CreateFacility, CreateFacilityBooking, Facility, FacilityBooking,
    FacilitySummary, RejectBooking, UpdateFacility, UpdateFacilityBooking,
};
pub use organization::{
    CreateOrganization, Organization, OrganizationStatus, OrganizationSummary, UpdateOrganization,
};
pub use organization_member::{
    CreateOrganizationMember, MembershipStatus, OrganizationMember, OrganizationMemberWithUser,
    UpdateOrganizationMember, UserOrganizationMembership,
};
pub use password_reset::{CreatePasswordResetToken, PasswordResetToken};
pub use person_month::{
    person_month_source, BuildingPersonMonthSummary, BulkPersonMonthEntry, BulkUpsertPersonMonths,
    CreatePersonMonth, MonthlyCount, PersonMonth, PersonMonthWithUnit, UpdatePersonMonth,
    YearlyPersonMonthSummary,
};
pub use refresh_token::{CreateRefreshToken, LoginAttempt, RateLimitStatus, RefreshToken};
pub use role::{permissions, system_roles, CreateRole, PermissionDefinition, Role, UpdateRole};
pub use unit::{
    occupancy_status, unit_status, unit_type, AssignUnitOwner, CreateUnit, Unit, UnitOwner,
    UnitOwnerInfo, UnitSummary, UnitWithOwners, UpdateUnit,
};
pub use unit_resident::{
    resident_type, CreateUnitResident, EndResidency, UnitResident, UnitResidentSummary,
    UnitResidentWithUser, UpdateUnitResident,
};
pub use user::{CreateUser, EmailVerificationToken, Locale, UpdateUser, User, UserStatus};

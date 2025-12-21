//! Application state.

use crate::services::{AuthService, EmailService, JwtService, TotpService};
use db::{
    repositories::{
        AnnouncementRepository, AuditLogRepository, BuildingRepository,
        CriticalNotificationRepository, DataExportRepository, DelegationRepository,
        DocumentRepository, FacilityRepository, FaultRepository, NotificationPreferenceRepository,
        OrganizationMemberRepository, OrganizationRepository, PasswordResetRepository,
        PersonMonthRepository, RoleRepository, SessionRepository, TwoFactorAuthRepository,
        UnitRepository, UnitResidentRepository, UserRepository, VoteRepository,
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
    pub building_repo: BuildingRepository,
    pub unit_repo: UnitRepository,
    pub unit_resident_repo: UnitResidentRepository,
    pub delegation_repo: DelegationRepository,
    pub person_month_repo: PersonMonthRepository,
    pub facility_repo: FacilityRepository,
    pub fault_repo: FaultRepository,
    pub vote_repo: VoteRepository,
    pub announcement_repo: AnnouncementRepository,
    pub document_repo: DocumentRepository,
    pub notification_pref_repo: NotificationPreferenceRepository,
    pub critical_notification_repo: CriticalNotificationRepository,
    pub two_factor_repo: TwoFactorAuthRepository,
    pub audit_log_repo: AuditLogRepository,
    pub data_export_repo: DataExportRepository,
    pub auth_service: AuthService,
    pub email_service: EmailService,
    pub jwt_service: JwtService,
    pub totp_service: TotpService,
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
        let building_repo = BuildingRepository::new(db.clone());
        let unit_repo = UnitRepository::new(db.clone());
        let unit_resident_repo = UnitResidentRepository::new(db.clone());
        let delegation_repo = DelegationRepository::new(db.clone());
        let person_month_repo = PersonMonthRepository::new(db.clone());
        let facility_repo = FacilityRepository::new(db.clone());
        let fault_repo = FaultRepository::new(db.clone());
        let vote_repo = VoteRepository::new(db.clone());
        let announcement_repo = AnnouncementRepository::new(db.clone());
        let document_repo = DocumentRepository::new(db.clone());
        let notification_pref_repo = NotificationPreferenceRepository::new(db.clone());
        let critical_notification_repo = CriticalNotificationRepository::new(db.clone());
        let two_factor_repo = TwoFactorAuthRepository::new(db.clone());
        let audit_log_repo = AuditLogRepository::new(db.clone());
        let data_export_repo = DataExportRepository::new(db.clone());
        let auth_service = AuthService::new();
        let totp_service = TotpService::new("Property Management".to_string());

        Self {
            db,
            user_repo,
            session_repo,
            password_reset_repo,
            org_repo,
            org_member_repo,
            role_repo,
            building_repo,
            unit_repo,
            unit_resident_repo,
            delegation_repo,
            person_month_repo,
            facility_repo,
            fault_repo,
            vote_repo,
            announcement_repo,
            document_repo,
            notification_pref_repo,
            critical_notification_repo,
            two_factor_repo,
            audit_log_repo,
            data_export_repo,
            auth_service,
            email_service,
            jwt_service,
            totp_service,
        }
    }
}

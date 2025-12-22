//! Application state.

use crate::services::{AuthService, EmailService, JwtService, OAuthService, TotpService};
use db::{
    repositories::{
        AgencyRepository, AiChatRepository, AnnouncementRepository, AuditLogRepository,
        BuildingRepository, CriticalNotificationRepository, DataExportRepository,
        DelegationRepository, DocumentRepository, DocumentTemplateRepository, EquipmentRepository,
        FacilityRepository, FaultRepository, FeatureFlagRepository, FinancialRepository,
        GranularNotificationRepository, HealthMonitoringRepository, HelpRepository,
        ListingRepository, MeterRepository, NotificationPreferenceRepository, OAuthRepository,
        OnboardingRepository, OrganizationMemberRepository, OrganizationRepository,
        PasswordResetRepository, PersonMonthRepository, PlatformAdminRepository, RentalRepository,
        RoleRepository, SensorRepository, SentimentRepository, SessionRepository,
        SignatureRequestRepository, SystemAnnouncementRepository, TwoFactorAuthRepository,
        UnitRepository, UnitResidentRepository, UserRepository, VoteRepository, WorkflowRepository,
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
    pub document_template_repo: DocumentTemplateRepository,
    pub notification_pref_repo: NotificationPreferenceRepository,
    pub critical_notification_repo: CriticalNotificationRepository,
    pub two_factor_repo: TwoFactorAuthRepository,
    pub audit_log_repo: AuditLogRepository,
    pub data_export_repo: DataExportRepository,
    pub oauth_repo: OAuthRepository,
    pub platform_admin_repo: PlatformAdminRepository,
    pub feature_flag_repo: FeatureFlagRepository,
    pub granular_notification_repo: GranularNotificationRepository,
    pub health_monitoring_repo: HealthMonitoringRepository,
    pub system_announcement_repo: SystemAnnouncementRepository,
    pub onboarding_repo: OnboardingRepository,
    pub help_repo: HelpRepository,
    pub signature_request_repo: SignatureRequestRepository,
    pub financial_repo: FinancialRepository,
    pub meter_repo: MeterRepository,
    // Epic 13: AI Assistant & Automation
    pub ai_chat_repo: AiChatRepository,
    pub sentiment_repo: SentimentRepository,
    pub equipment_repo: EquipmentRepository,
    pub workflow_repo: WorkflowRepository,
    // Epic 14: IoT & Smart Building
    pub sensor_repo: SensorRepository,
    // Epic 15: Property Listings & Multi-Portal Sync
    pub listing_repo: ListingRepository,
    // Epic 17: Agency & Realtor Management
    pub agency_repo: AgencyRepository,
    // Epic 18: Short-Term Rental Integration
    pub rental_repo: RentalRepository,
    pub auth_service: AuthService,
    pub email_service: EmailService,
    pub jwt_service: JwtService,
    pub totp_service: TotpService,
    pub oauth_service: OAuthService,
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
        let document_template_repo = DocumentTemplateRepository::new(db.clone());
        let notification_pref_repo = NotificationPreferenceRepository::new(db.clone());
        let critical_notification_repo = CriticalNotificationRepository::new(db.clone());
        let two_factor_repo = TwoFactorAuthRepository::new(db.clone());
        let audit_log_repo = AuditLogRepository::new(db.clone());
        let data_export_repo = DataExportRepository::new(db.clone());
        let oauth_repo = OAuthRepository::new(db.clone());
        let platform_admin_repo = PlatformAdminRepository::new(db.clone());
        let feature_flag_repo = FeatureFlagRepository::new(db.clone());
        let granular_notification_repo = GranularNotificationRepository::new(db.clone());
        let health_monitoring_repo = HealthMonitoringRepository::new(db.clone());
        let system_announcement_repo = SystemAnnouncementRepository::new(db.clone());
        let onboarding_repo = OnboardingRepository::new(db.clone());
        let help_repo = HelpRepository::new(db.clone());
        let signature_request_repo = SignatureRequestRepository::new(db.clone());
        let financial_repo = FinancialRepository::new(db.clone());
        let meter_repo = MeterRepository::new(db.clone());
        // Epic 13: AI Assistant & Automation
        let ai_chat_repo = AiChatRepository::new(db.clone());
        let sentiment_repo = SentimentRepository::new(db.clone());
        let equipment_repo = EquipmentRepository::new(db.clone());
        let workflow_repo = WorkflowRepository::new(db.clone());
        // Epic 14: IoT & Smart Building
        let sensor_repo = SensorRepository::new(db.clone());
        // Epic 15: Property Listings & Multi-Portal Sync
        let listing_repo = ListingRepository::new(db.clone());
        // Epic 17: Agency & Realtor Management
        let agency_repo = AgencyRepository::new(db.clone());
        // Epic 18: Short-Term Rental Integration
        let rental_repo = RentalRepository::new(db.clone());
        let auth_service = AuthService::new();
        let totp_service = TotpService::new("Property Management".to_string());
        let oauth_service = OAuthService::new(oauth_repo.clone(), auth_service.clone());

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
            document_template_repo,
            notification_pref_repo,
            critical_notification_repo,
            two_factor_repo,
            audit_log_repo,
            data_export_repo,
            oauth_repo,
            platform_admin_repo,
            feature_flag_repo,
            granular_notification_repo,
            health_monitoring_repo,
            system_announcement_repo,
            onboarding_repo,
            help_repo,
            signature_request_repo,
            financial_repo,
            meter_repo,
            ai_chat_repo,
            sentiment_repo,
            equipment_repo,
            workflow_repo,
            sensor_repo,
            listing_repo,
            agency_repo,
            rental_repo,
            auth_service,
            email_service,
            jwt_service,
            totp_service,
            oauth_service,
        }
    }
}

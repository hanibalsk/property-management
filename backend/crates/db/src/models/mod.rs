//! Database models.

pub mod announcement;
pub mod audit_log;
pub mod building;
pub mod critical_notification;
pub mod data_export;
pub mod delegation;
pub mod document;
pub mod document_template;
pub mod facility;
pub mod fault;
pub mod financial;

pub mod granular_notification;
pub mod messaging;
pub mod meter;
pub mod notification_preference;
pub mod oauth;
pub mod organization;
pub mod organization_member;
pub mod password_reset;
pub mod person_month;
pub mod platform_admin;
pub mod refresh_token;
pub mod role;
pub mod signature_request;
pub mod two_factor_auth;
pub mod unit;
pub mod unit_resident;
pub mod user;
pub mod vote;

// Epic 13: AI Assistant & Automation
pub mod ai_chat;
pub mod equipment;
pub mod sentiment;
pub mod workflow;

// Epic 14: IoT & Smart Building
pub mod sensor;

// Epic 15: Property Listings & Multi-Portal Sync
pub mod listing;

// Epic 16: Portal Search & Discovery
pub mod portal;

// Epic 17: Agency & Realtor Management
pub mod agency;

// Epic 18: Short-Term Rental Integration
pub mod rental;

pub use ai_chat::{
    message_role, AiChatMessage, AiChatSession, AiResponse, AiSource, AiTrainingFeedback,
    ChatSessionSummary, CreateChatSession, ProvideFeedback, SendChatMessage,
};
pub use equipment::{
    equipment_status, maintenance_status, maintenance_type, CreateEquipment, CreateMaintenance,
    Equipment, EquipmentMaintenance, EquipmentQuery, EquipmentWithSummary, MaintenancePrediction,
    UpdateEquipment, UpdateMaintenance,
};
pub use sensor::{
    sensor_status, sensor_type, AggregatedReading, AlertQuery, BatchSensorReadings, CreateSensor,
    CreateSensorAlert, CreateSensorFaultCorrelation, CreateSensorReading, CreateSensorThreshold,
    ReadingQuery, Sensor, SensorAlert, SensorDashboard, SensorFaultCorrelation, SensorQuery,
    SensorReading, SensorSummary, SensorThreshold, SensorThresholdTemplate, SensorTypeCount,
    SingleReading, UpdateSensor, UpdateSensorThreshold,
};
pub use sentiment::{
    alert_type, BuildingSentiment, CreateSentimentAlert, SentimentAlert, SentimentDashboard,
    SentimentThresholds, SentimentTrend, SentimentTrendQuery, UpdateSentimentThresholds,
    UpsertSentimentTrend,
};
pub use workflow::{
    action_type, execution_status, on_failure, step_status, trigger_type, CreateWorkflow,
    CreateWorkflowAction, ExecutionQuery, TriggerWorkflow, UpdateWorkflow, Workflow,
    WorkflowAction, WorkflowExecution, WorkflowExecutionStep, WorkflowQuery, WorkflowSchedule,
    WorkflowSummary, WorkflowWithDetails,
};

pub use announcement::{
    announcement_status, target_type, AcknowledgeAnnouncement, AcknowledgmentStats, Announcement,
    AnnouncementAttachment, AnnouncementComment, AnnouncementListQuery, AnnouncementRead,
    AnnouncementStatistics, AnnouncementSummary, AnnouncementWithDetails, CommentWithAuthor,
    CommentWithAuthorRow, CreateAnnouncement, CreateAnnouncementAttachment, CreateComment,
    DeleteComment, MarkAnnouncementRead, PinAnnouncement, PublishAnnouncement, UpdateAnnouncement,
    UserAcknowledgmentStatus,
};
pub use audit_log::{
    ActionCount, AuditAction, AuditLog, AuditLogQuery, AuditLogSummary, CreateAuditLog,
};
pub use building::{
    building_status, Building, BuildingContact, BuildingStatistics, BuildingSummary,
    CreateBuilding, UpdateBuilding,
};
pub use critical_notification::{
    AcknowledgeCriticalNotificationResponse, CreateCriticalNotificationRequest,
    CreateCriticalNotificationResponse, CriticalNotification, CriticalNotificationAcknowledgment,
    CriticalNotificationResponse, CriticalNotificationStats, UnacknowledgedNotificationsResponse,
};
pub use data_export::{
    CreateDataExportRequest, DataExportRequest, DataExportRequestResponse, DataExportStatus,
    DataExportStatusResponse, ExportCategories, ExportCategory, ExportFormat, UserDataExport,
};
pub use delegation::{
    delegation_scope, delegation_status, AcceptDelegation, CreateDelegation, DeclineDelegation,
    Delegation, DelegationAuditLog, DelegationSummary, DelegationWithUsers, RevokeDelegation,
    UpdateDelegation,
};
pub use document::{
    access_scope, document_category, ocr_status, share_type, ClassificationFeedback,
    CreateDocument, CreateDocumentVersion, CreateFolder, CreateShare, CreateVersionResponse,
    Document, DocumentClassificationHistory, DocumentFolder, DocumentIntelligenceStats,
    DocumentListQuery, DocumentOcrQueue, DocumentSearchRequest, DocumentSearchResponse,
    DocumentSearchResult, DocumentShare, DocumentSummarizationQueue, DocumentSummary,
    DocumentVersion, DocumentVersionHistory, DocumentWithDetails, DocumentWithIntelligence,
    FolderTreeNode, FolderWithCount, GenerateSummaryRequest, LogShareAccess, MoveDocument,
    RestoreVersionRequest, RestoreVersionResponse, RevokeShare, ShareAccessLog, ShareWithDocument,
    UpdateDocument, UpdateFolder, ALLOWED_MIME_TYPES, MAX_FILE_SIZE,
};
pub use document_template::{
    placeholder_type, template_type, CreateTemplate, DocumentTemplate, GenerateDocumentRequest,
    GenerateDocumentResponse, TemplateListQuery, TemplatePlaceholder, TemplateSummary,
    TemplateWithDetails, UpdateTemplate,
};
pub use facility::{
    booking_status, facility_type, ApproveBooking, AvailableSlot, BookingWithDetails,
    CancelBooking, CreateFacility, CreateFacilityBooking, Facility, FacilityBooking,
    FacilitySummary, RejectBooking, UpdateFacility, UpdateFacilityBooking,
};
pub use fault::{
    fault_category, fault_priority, fault_status, timeline_action, AddFaultComment, AddWorkNote,
    AiSuggestion, AssignFault, CategoryCount, ConfirmFault, CreateFault, CreateFaultAttachment,
    CreateFaultTimelineEntry, Fault, FaultAttachment, FaultListQuery, FaultStatistics,
    FaultSummary, FaultTimelineEntry, FaultTimelineEntryWithUser, FaultWithDetails, PriorityCount,
    ReopenFault, ResolveFault, StatusCount, TriageFault, UpdateFault, UpdateFaultStatus,
};
pub use financial::{
    ARReportEntry, ARReportTotals, AccountTransaction, AccountsReceivableReport, CreateFeeSchedule,
    CreateFinancialAccount, CreateInvoice, CreateInvoiceItem, CreateTransaction, FeeFrequency,
    FeeSchedule, FinancialAccount, FinancialAccountResponse, FinancialAccountType,
    InitiatePaymentResponse, Invoice, InvoiceItem, InvoiceResponse, InvoiceStatus, LateFeeConfig,
    ListInvoicesResponse, OnlinePaymentSession, Payment, PaymentAllocation, PaymentMethod,
    PaymentResponse, PaymentStatus, RecordPayment, ReminderSchedule, TransactionCategory,
    TransactionType, UnitCreditBalance, UnitFee,
};
pub use granular_notification::{
    AddToGroupRequest, CategorySummary, CreateHeldNotification, DigestNotification,
    EventNotificationPreference, EventPreferenceWithDetails, EventPreferencesResponse,
    GenerateDigestRequest, GroupedNotification, GroupedNotificationsResponse, HeldNotification,
    NotificationDigest, NotificationEventCategory, NotificationEventType, NotificationGroup,
    NotificationGroupWithNotifications, NotificationSchedule, NotificationScheduleResponse,
    RoleDefaultsListResponse, RoleNotificationDefaults, UpdateEventPreferenceRequest,
    UpdateNotificationScheduleRequest, UpdateRoleDefaultsRequest,
};
pub use messaging::{
    BlockWithUserInfo, BlockWithUserInfoRow, CreateBlock, CreateMessage, CreateThread, Message,
    MessagePreview, MessageThread, MessageWithSender, MessageWithSenderRow, ParticipantInfo,
    ThreadWithPreview, ThreadWithPreviewRow, UserBlock,
};
pub use meter::{
    ConsumptionAggregate, ConsumptionComparison, ConsumptionDataPoint, ConsumptionHistoryResponse,
    CreateSubmissionWindow, CreateUtilityBill, DistributeUtilityBill, DistributionMethod,
    IngestSmartMeterReading, ListMetersResponse, ListReadingsResponse, Meter, MeterReading,
    MeterResponse, MeterType, MissingReadingAlert, ReadingApprovalEntry, ReadingSource,
    ReadingStatus, ReadingSubmissionWindow, ReadingValidationRule, RegisterMeter, ReplaceMeter,
    SmartMeterProvider, SmartMeterReadingLog, SubmitReading, UnitDistributionOverride, UtilityBill,
    UtilityBillDistribution, UtilityBillResponse, ValidateReading,
};
pub use notification_preference::{
    DisableAllWarningResponse, NotificationChannel, NotificationPreference,
    NotificationPreferenceResponse, NotificationPreferencesResponse,
    UpdateNotificationPreferenceRequest,
};
pub use oauth::{
    AuthorizeRequest, ConsentPageData, CreateAccessToken, CreateAuthorizationCode,
    CreateOAuthClient, CreateRefreshToken as CreateOAuthRefreshToken, CreateUserOAuthGrant,
    IntrospectionResponse, OAuthAccessToken, OAuthAuthorizationCode, OAuthClient,
    OAuthClientSummary, OAuthError, OAuthRefreshToken, OAuthScope, RegisterClientRequest,
    RegisterClientResponse, RevokeTokenRequest, ScopeDisplay, TokenRequest, TokenResponse,
    UpdateOAuthClient, UserGrantWithClient, UserGrantWithClientRow, UserOAuthGrant,
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
pub use platform_admin::{
    AdminOrganizationDetail, AdminOrganizationSummary, AnnouncementSeverity,
    CreateFeatureFlagOverrideRequest, CreateFeatureFlagRequest, CreateHelpArticleRequest,
    CreateMaintenanceRequest, CreateSystemAnnouncementRequest, FeatureFlag, FeatureFlagOverride,
    FeatureFlagScope, HelpArticle, HelpArticleRevision, MetricAlert, MetricThreshold, MetricType,
    OnboardingStep, OnboardingTour, OrganizationDetailMetrics, OrganizationMetrics, PlatformMetric,
    ReactivateOrganizationRequest, ScheduledMaintenance, StepPlacement, SupportAccessLog,
    SupportAccessRequest, SupportAccessStatus, SuspendOrganizationRequest, SystemAnnouncement,
    SystemAnnouncementAcknowledgment, UserOnboardingProgress,
};
pub use refresh_token::{CreateRefreshToken, LoginAttempt, RateLimitStatus, RefreshToken};
pub use role::{permissions, system_roles, CreateRole, PermissionDefinition, Role, UpdateRole};
pub use signature_request::{
    CancelSignatureRequestRequest, CancelSignatureRequestResponse, CreateSignatureRequest,
    CreateSignatureRequestResponse, CreateSigner, ListSignatureRequestsResponse,
    SendReminderRequest, SendReminderResponse, SignatureRequest, SignatureRequestResponse,
    SignatureRequestStatus, SignatureRequestWithDocument, SignatureWebhookEvent, Signer,
    SignerCounts, SignerStatus, WebhookResponse,
};
pub use two_factor_auth::{
    CreateTwoFactorAuth, TwoFactorAuth, TwoFactorStatus, UpdateTwoFactorStatus,
};
pub use unit::{
    occupancy_status, unit_status, unit_type, AssignUnitOwner, CreateUnit, Unit, UnitOwner,
    UnitOwnerInfo, UnitSummary, UnitWithOwners, UpdateUnit,
};
pub use unit_resident::{
    resident_type, CreateUnitResident, EndResidency, UnitResident, UnitResidentSummary,
    UnitResidentWithUser, UpdateUnitResident,
};
pub use user::{
    CreateUser, EmailVerificationToken, Locale, NeighborRow, NeighborView, PrivacySettings,
    ProfileVisibility, UpdatePrivacySettings, UpdateUser, User, UserStatus,
};
pub use vote::{
    audit_action, question_type, quorum_type, vote_status, CancelVote, CastVote, CreateVote,
    CreateVoteAuditLog, CreateVoteComment, CreateVoteQuestion, EligibleUnit, HideVoteComment,
    OptionResult, ParticipationDetail, PublishVote, QuestionOption, QuestionResult, UpdateVote,
    UpdateVoteQuestion, Vote, VoteAuditLog, VoteComment, VoteCommentWithUser, VoteEligibility,
    VoteListQuery, VoteQuestion, VoteReceipt, VoteReportData, VoteResponse, VoteResults,
    VoteSummary, VoteWithDetails,
};

// Epic 15: Property Listings & Multi-Portal Sync
pub use listing::{
    currency, listing_status, portal as listing_portal, property_type, syndication_status,
    transaction_type, CreateListing, CreateListingFromUnit, CreateListingPhoto, CreateSyndication,
    Listing, ListingListQuery, ListingPhoto, ListingStatistics, ListingSummary, ListingSyndication,
    ListingWithDetails, PropertyTypeCount, PublishListingResponse, ReorderPhotos,
    SyndicationResult, UpdateListing, UpdateListingStatus,
};

// Epic 16: Portal Search & Discovery
pub use portal::{
    alert_frequency, AddFavorite, CreatePortalUser, CreateSavedSearch, Favorite,
    FavoriteWithListing, FavoriteWithListingRow, FavoritesResponse, MatchedListing, PortalSession,
    PortalUser, PublicListingDetail, PublicListingQuery, PublicListingSearchResponse,
    PublicListingSummary, SavedSearch, SavedSearchesResponse, SearchAlert, SearchCriteria,
    SearchSuggestions, UpdatePortalUser, UpdateSavedSearch,
};

// Epic 17: Agency & Realtor Management
pub use agency::{
    agency_status, import_source, import_status, inquiry_assignment, listing_visibility,
    member_role, AcceptInvitation, Agency, AgencyBranding, AgencyInvitation, AgencyListing,
    AgencyListingSummary, AgencyListingsResponse, AgencyMember, AgencyMemberWithUser,
    AgencyMemberWithUserRow, AgencyMembersResponse, AgencyProfile, AgencySummary, CreateAgency,
    CreateAgencyListing, CreateImportJob, FieldMapping, ImportConfig, ImportError, ImportPreview,
    ImportResult, InviteMember, ListingCollaborator, ListingEditHistory, ListingImportJob,
    UpdateAgency, UpdateListingVisibility, UpdateMemberRole,
};

// Epic 18: Short-Term Rental Integration
pub use rental::{
    authority_code, block_reason, booking_status as rental_booking_status, guest_status,
    rental_platform, report_status, report_type, BookingListQuery, BookingSummary,
    BookingWithGuests, BookingsResponse, CalendarBlock, CalendarEvent, CheckInReminder,
    ConnectionStatus, CreateBooking, CreateCalendarBlock, CreateGuest, CreateICalFeed,
    CreatePlatformConnection, GenerateReport, GuestSummary, ICalFeed, ICalFeedSummary,
    NationalityStats, OAuthCallback, PlatformConnectionSummary, PlatformSyncStatus, RegisterGuest,
    RentalBooking, RentalGuest, RentalGuestReport, RentalPlatformConnection, RentalStatistics,
    ReportPreview, ReportSummary, SubmitReport, UnitAvailability, UpdateBooking,
    UpdateBookingStatus, UpdateGuest, UpdateICalFeed, UpdatePlatformConnection,
};

// Epic 19: Lease Management & Tenant Screening
pub mod lease;

pub use lease::{
    application_status, lease_status, screening_status, screening_type, termination_reason,
    ApplicationListQuery, ApplicationSummary, CreateAmendment, CreateApplication, CreateLease,
    CreateLeaseTemplate, CreateReminder, ExpirationOverview, InitiateScreening, Lease,
    LeaseAmendment, LeaseListQuery, LeasePayment, LeaseReminder, LeaseStatistics, LeaseSummary,
    LeaseTemplate, LeaseWithDetails, PaymentSummary, RecordPayment as RecordLeasePayment,
    RenewLease, ReviewApplication, ScreeningConsent, ScreeningSummary, SubmitApplication,
    TenantApplication, TenantScreening, TerminateLease, UpdateApplication, UpdateLease,
    UpdateLeaseTemplate, UpdateScreeningResult,
};

// Epic 20: Maintenance Scheduling & Work Orders
pub mod work_order;

pub use work_order::{
    schedule_execution_status, schedule_frequency, update_type, work_order_priority,
    work_order_source, work_order_status, work_order_type, AddWorkOrderUpdate,
    CreateMaintenanceSchedule, CreateWorkOrder, MaintenanceCostSummary, MaintenanceSchedule,
    ScheduleExecution, ScheduleQuery, ServiceHistoryEntry, UpcomingSchedule,
    UpdateMaintenanceSchedule, UpdateWorkOrder, WorkOrder, WorkOrderQuery, WorkOrderStatistics,
    WorkOrderUpdate, WorkOrderWithDetails,
};

// Epic 21: Supplier & Vendor Management
pub mod vendor;

pub use vendor::{
    contract_status, contract_type, invoice_status, service_type, vendor_status, ContractQuery,
    CreateVendor, CreateVendorContact, CreateVendorContract, CreateVendorInvoice,
    CreateVendorRating, ExpiringContract, InvoiceQuery, InvoiceSummary, ServiceCount, UpdateVendor,
    UpdateVendorContract, UpdateVendorInvoice, Vendor, VendorContact, VendorContract,
    VendorInvoice, VendorQuery, VendorRating, VendorStatistics, VendorWithDetails,
};

// Epic 22: Insurance Management
pub mod insurance;

pub use insurance::{
    claim_status, policy_status, policy_type, premium_frequency, reminder_type, AddClaimDocument,
    AddPolicyDocument, ClaimStatusSummary, CreateInsuranceClaim, CreateInsurancePolicy,
    CreateRenewalReminder, ExpiringPolicy, InsuranceClaim, InsuranceClaimDocument,
    InsuranceClaimHistory, InsuranceClaimQuery, InsuranceClaimWithPolicy, InsurancePolicy,
    InsurancePolicyDocument, InsurancePolicyQuery, InsuranceRenewalReminder, InsuranceStatistics,
    PolicyTypeSummary, UpdateInsuranceClaim, UpdateInsurancePolicy, UpdateRenewalReminder,
};

// Epic 23: Emergency Management
pub mod emergency;

pub use emergency::{
    acknowledgment_status, contact_type, drill_status, drill_type, incident_status, incident_type,
    protocol_type, severity, AcknowledgeBroadcast, AddIncidentAttachment, BroadcastDeliveryStats,
    CompleteDrill, CreateEmergencyBroadcast, CreateEmergencyContact, CreateEmergencyDrill,
    CreateEmergencyIncident, CreateEmergencyProtocol, CreateIncidentUpdate, EmergencyBroadcast,
    EmergencyBroadcastAcknowledgment, EmergencyBroadcastQuery, EmergencyContact,
    EmergencyContactQuery, EmergencyDrill, EmergencyDrillQuery, EmergencyIncident,
    EmergencyIncidentAttachment, EmergencyIncidentQuery, EmergencyIncidentUpdate,
    EmergencyProtocol, EmergencyProtocolQuery, EmergencyStatistics, IncidentSeveritySummary,
    IncidentTypeSummary, UpdateEmergencyContact, UpdateEmergencyDrill, UpdateEmergencyIncident,
    UpdateEmergencyProtocol,
};

// Epic 24: Budget & Financial Planning
pub mod budget;

pub use budget::{
    budget_status, capital_plan_status, forecast_type, funding_source, priority,
    reserve_transaction_type, variance_alert_type, AcknowledgeVarianceAlert, Budget, BudgetActual,
    BudgetCategory, BudgetDashboard, BudgetItem, BudgetQuery, BudgetSummary, BudgetVarianceAlert,
    CapitalPlan, CapitalPlanQuery, CategoryVariance, CreateBudget, CreateBudgetCategory,
    CreateBudgetItem, CreateCapitalPlan, CreateFinancialForecast, CreateReserveFund,
    FinancialForecast, ForecastQuery, RecordBudgetActual, RecordReserveTransaction, ReserveFund,
    ReserveFundProjection, ReserveFundTransaction, UpdateBudget, UpdateBudgetCategory,
    UpdateBudgetItem, UpdateCapitalPlan, UpdateFinancialForecast, UpdateReserveFund,
    YearlyCapitalSummary,
};

// Epic 25: Legal Document & Compliance
pub mod legal;

pub use legal::{
    compliance_category, compliance_frequency, compliance_status, delivery_method, delivery_status,
    document_type, notice_priority, notice_type, recipient_type, AcknowledgeNotice, ApplyTemplate,
    ComplianceAuditTrail, ComplianceCategoryCount, ComplianceQuery, ComplianceRequirement,
    ComplianceRequirementWithDetails, ComplianceStatistics, ComplianceTemplate,
    ComplianceVerification, CreateAuditTrailEntry, CreateComplianceRequirement,
    CreateComplianceTemplate, CreateComplianceVerification, CreateLegalDocument,
    CreateLegalDocumentVersion, CreateLegalNotice, LegalDocument, LegalDocumentQuery,
    LegalDocumentSummary, LegalDocumentVersion, LegalNotice, LegalNoticeQuery,
    LegalNoticeRecipient, NoticeAcknowledgmentStats, NoticeRecipientInput, NoticeStatistics,
    NoticeTypeCount, NoticeWithRecipients, UpcomingVerification, UpdateComplianceRequirement,
    UpdateComplianceTemplate, UpdateLegalDocument, UpdateLegalNotice,
};

// Epic 26: Platform Subscription & Billing
pub mod subscription;

pub use subscription::{
    billing_cycle, coupon_duration, discount_type, line_item_type, metric_type,
    payment_method_type, subscription_invoice_status, subscription_status,
    CancelSubscriptionRequest, ChangePlanRequest, CouponRedemption, CreateOrganizationSubscription,
    CreateSubscriptionCoupon, CreateSubscriptionEvent, CreateSubscriptionPaymentMethod,
    CreateSubscriptionPlan, CreateUsageRecord, InvoiceLineItem, InvoiceQueryParams,
    InvoiceWithDetails, OrganizationSubscription, PlanSubscriptionCount, RedeemCouponRequest,
    SubscriptionCoupon, SubscriptionEvent, SubscriptionInvoice, SubscriptionPaymentMethod,
    SubscriptionPlan, SubscriptionStatistics, SubscriptionWithPlan, UpdateOrganizationSubscription,
    UpdateSubscriptionCoupon, UpdateSubscriptionPlan, UsageRecord, UsageSummary,
};

// Epic 30: Government Portal Integration
pub mod government_portal;

pub use government_portal::{
    AddSubmissionAttachment, CreatePortalConnection, CreateRegulatorySubmission,
    CreateSubmissionAudit, CreateSubmissionSchedule, GovernmentPortalConnection,
    GovernmentPortalStats, GovernmentPortalType, RegulatoryReportTemplate, RegulatorySubmission,
    RegulatorySubmissionAttachment, RegulatorySubmissionAudit, RegulatorySubmissionSchedule,
    SubmissionQuery, SubmissionStatus, SubmissionSummary, TemplateSummaryGov, UpcomingDueDate,
    UpdatePortalConnection, UpdateRegulatorySubmission, UpdateSubmissionSchedule, ValidationError,
    ValidationResult, ValidationWarning,
};

// Epics 31-34: Reality Portal Professional
pub mod reality_portal;

pub use reality_portal::{
    AgencyMemberWithUser as RealityAgencyMemberWithUser, AgencySummary as RealityAgencySummary,
    AssignRealtorListing, CreateAgencyInvitation, CreateCrmConnection, CreateFeedSubscription,
    CreateImportJob as CreatePortalImportJob, CreateListingInquiry, CreatePortalSavedSearch,
    CreateRealityAgency, CreateRealtorProfile, CrmConnection, FeedSubscription, ImportJobProgress,
    InquiryMessage, InquiryWithListing, ListingAnalytics, ListingAnalyticsSummary, ListingInquiry,
    ListingPriceHistory, PortalFavorite, PortalFavoriteWithListing, PortalImportJob,
    PortalImportJobWithStats, PortalSavedSearch, PriceChangeAlert, PublicRealtorProfile,
    RealityAgency, RealityAgencyInvitation, RealityAgencyMember, RealityFeedSubscription,
    RealtorListing, RealtorProfile, ScheduleViewing, SearchAlertQueueEntry, SendInquiryMessage,
    UpdateAgencyBranding, UpdateCrmConnection, UpdateFeedSubscription,
    UpdateImportJob as UpdatePortalImportJob, UpdatePortalFavorite, UpdatePortalSavedSearch,
    UpdateRealityAgency, UpdateRealtorProfile, UpdateViewing, ViewingSchedule,
};

// Epic 37: Community & Social Features
pub mod community;

pub use community::{
    CommunityComment, CommunityEvent, CommunityEventRsvp, CommunityEventWithRsvp, CommunityGroup,
    CommunityGroupMember, CommunityGroupWithMembership, CommunityPost, CommunityPostWithAuthor,
    CreateCommunityComment, CreateCommunityEvent, CreateCommunityGroup, CreateCommunityPost,
    CreateMarketplaceInquiry, CreateMarketplaceItem, EventRsvpRequest, JoinGroupRequest,
    MarketplaceInquiry, MarketplaceItem, MarketplaceItemWithSeller, PollOption,
    UpdateCommunityEvent, UpdateCommunityGroup, UpdateCommunityPost, UpdateMarketplaceItem,
};

// Epic 38: Workflow Automation
pub mod automation;

// Epic 54: Forms Management
pub mod form;

// Epic 55: Advanced Reporting & Analytics
pub mod reports;

pub use automation::{
    AutomationAction, AutomationLogSummary, AutomationRuleWithStats, CallWebhookConfig,
    ConditionTriggerConfig, CreateAutomationRule, CreateRuleFromTemplate, EventTriggerConfig,
    GenerateReportConfig, ScheduleTriggerConfig, SendEmailConfig, SendNotificationConfig,
    UpdateAutomationRule, WorkflowAutomationLog, WorkflowAutomationRule,
    WorkflowAutomationTemplate,
};

// Epic 54: Forms Management
pub use form::{
    field_type, form_status, submission_status, target_type as form_target_type,
    ConditionalDisplay, CreateForm, CreateFormField, CreateFormResponse,
    ExportFormat as FormExportFormat, ExportSubmissionsRequest, FieldOption, Form, FormAttachment,
    FormDownload, FormField, FormListQuery, FormListResponse, FormStatistics, FormSubmission,
    FormSubmissionParams, FormSubmissionSummary, FormSubmissionWithDetails, FormSummary,
    FormWithDetails, ReviewSubmission, SignatureData, SubmissionListQuery, SubmissionListResponse,
    SubmitForm, SubmitFormResponse, UpdateForm, UpdateFormField, ValidationRules,
};

// Epic 55: Advanced Reporting & Analytics
pub use reports::{
    CategoryTrend, ConsumptionAnomaly, ConsumptionReportData, ConsumptionSummary, DateRange,
    ExportReportRequest, ExportReportResponse, FaultTrends, MonthlyAverage, MonthlyConsumption,
    MonthlyPersonCount, OccupancyReportData, OccupancySummary, OccupancyTrends, ReportMonthlyCount,
    UnitConsumption, UnitOccupancy, UtilityTypeConsumption, VoteParticipationDetail,
    VotingParticipationSummary, YearComparison,
};

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

pub use ai_chat::{
    message_role, AiChatMessage, AiChatSession, AiResponse, AiSource, AiTrainingFeedback,
    ChatSessionSummary, CreateChatSession, ProvideFeedback, SendChatMessage,
};
pub use equipment::{
    equipment_status, maintenance_status, maintenance_type, CreateEquipment, CreateMaintenance,
    Equipment, EquipmentMaintenance, EquipmentQuery, EquipmentWithSummary, MaintenancePrediction,
    UpdateEquipment, UpdateMaintenance,
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
    access_scope, document_category, share_type, CreateDocument, CreateDocumentVersion,
    CreateFolder, CreateShare, CreateVersionResponse, Document, DocumentFolder, DocumentListQuery,
    DocumentShare, DocumentSummary, DocumentVersion, DocumentVersionHistory, DocumentWithDetails,
    FolderTreeNode, FolderWithCount, LogShareAccess, MoveDocument, RestoreVersionRequest,
    RestoreVersionResponse, RevokeShare, ShareAccessLog, ShareWithDocument, UpdateDocument,
    UpdateFolder, ALLOWED_MIME_TYPES, MAX_FILE_SIZE,
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
    CategorySummary, CreateHeldNotification, EventNotificationPreference,
    EventPreferenceWithDetails, EventPreferencesResponse, HeldNotification,
    NotificationEventCategory, NotificationEventType, NotificationSchedule,
    NotificationScheduleResponse, RoleDefaultsListResponse, RoleNotificationDefaults,
    UpdateEventPreferenceRequest, UpdateNotificationScheduleRequest, UpdateRoleDefaultsRequest,
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

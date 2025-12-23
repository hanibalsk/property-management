/**
 * Government Portal Types
 *
 * Type definitions for the Government Portal API (Epic 30, UC-22.3).
 */

export type GovernmentPortalType =
  | 'tax_authority'
  | 'statistical_office'
  | 'building_authority'
  | 'housing_registry'
  | 'police_registry'
  | 'energy_authority'
  | 'environmental_agency'
  | 'labor_office'
  | 'social_insurance'
  | 'business_registry'
  | 'data_protection'
  | 'other';

export type SubmissionStatus =
  | 'draft'
  | 'pending_validation'
  | 'validated'
  | 'submitted'
  | 'acknowledged'
  | 'processing'
  | 'accepted'
  | 'rejected'
  | 'requires_correction'
  | 'cancelled';

export interface GovernmentPortalConnection {
  id: string;
  organizationId: string;
  portalType: GovernmentPortalType;
  portalName: string;
  portalCode?: string;
  countryCode: string;
  apiEndpoint?: string;
  portalUsername?: string;
  oauthClientId?: string;
  apiKeyId?: string;
  certificateId?: string;
  isActive: boolean;
  autoSubmit: boolean;
  testMode: boolean;
  lastConnectionTest?: string;
  lastSuccessfulSubmission?: string;
  createdBy?: string;
  createdAt: string;
  updatedAt: string;
}

export interface CreatePortalConnectionRequest {
  portalType: GovernmentPortalType;
  portalName: string;
  portalCode?: string;
  countryCode?: string;
  apiEndpoint?: string;
  portalUsername?: string;
  oauthClientId?: string;
  autoSubmit?: boolean;
  testMode?: boolean;
}

export interface UpdatePortalConnectionRequest {
  portalName?: string;
  apiEndpoint?: string;
  portalUsername?: string;
  oauthClientId?: string;
  isActive?: boolean;
  autoSubmit?: boolean;
  testMode?: boolean;
}

export interface RegulatoryReportTemplate {
  id: string;
  templateCode: string;
  templateName: string;
  description?: string;
  portalType: GovernmentPortalType;
  countryCode: string;
  schemaVersion: string;
  fieldMappings: Record<string, unknown>;
  validationRules: Record<string, unknown>;
  xmlTemplate?: string;
  frequency?: string;
  dueDayOfMonth?: number;
  dueMonthOfQuarter?: number;
  isActive: boolean;
  effectiveFrom: string;
  effectiveTo?: string;
  createdAt: string;
  updatedAt: string;
}

export interface TemplateSummary {
  id: string;
  templateCode: string;
  templateName: string;
  portalType: GovernmentPortalType;
  countryCode: string;
  frequency?: string;
}

export interface RegulatorySubmission {
  id: string;
  organizationId: string;
  portalConnectionId?: string;
  templateId?: string;
  submissionReference: string;
  externalReference?: string;
  reportType: string;
  reportPeriodStart: string;
  reportPeriodEnd: string;
  reportData: Record<string, unknown>;
  reportXml?: string;
  reportPdfUrl?: string;
  status: SubmissionStatus;
  validationResult?: ValidationResult;
  submissionResponse?: Record<string, unknown>;
  validatedAt?: string;
  submittedAt?: string;
  acknowledgedAt?: string;
  processedAt?: string;
  preparedBy?: string;
  submittedBy?: string;
  submissionAttempts: number;
  lastError?: string;
  nextRetryAt?: string;
  createdAt: string;
  updatedAt: string;
}

export interface SubmissionSummary {
  id: string;
  submissionReference: string;
  reportType: string;
  reportPeriodStart: string;
  reportPeriodEnd: string;
  status: SubmissionStatus;
  submittedAt?: string;
  createdAt: string;
}

export interface CreateRegulatorySubmissionRequest {
  portalConnectionId?: string;
  templateId?: string;
  reportType: string;
  reportPeriodStart: string;
  reportPeriodEnd: string;
  reportData: Record<string, unknown>;
}

export interface UpdateRegulatorySubmissionRequest {
  reportData?: Record<string, unknown>;
  reportXml?: string;
}

export interface ListSubmissionsParams {
  status?: SubmissionStatus;
  reportType?: string;
  fromDate?: string;
  toDate?: string;
  limit?: number;
  offset?: number;
}

export interface RegulatorySubmissionAudit {
  id: string;
  submissionId: string;
  action: string;
  previousStatus?: SubmissionStatus;
  newStatus?: SubmissionStatus;
  actorId?: string;
  actorType: string;
  details?: Record<string, unknown>;
  errorMessage?: string;
  createdAt: string;
}

export interface RegulatorySubmissionAttachment {
  id: string;
  submissionId: string;
  fileName: string;
  fileType: string;
  fileSize: number;
  fileUrl: string;
  checksum?: string;
  attachmentType: string;
  description?: string;
  portalDocumentId?: string;
  createdAt: string;
}

export interface AddSubmissionAttachmentRequest {
  fileName: string;
  fileType: string;
  fileSize: number;
  fileUrl: string;
  checksum?: string;
  attachmentType: string;
  description?: string;
}

export interface RegulatorySubmissionSchedule {
  id: string;
  organizationId: string;
  portalConnectionId: string;
  templateId: string;
  isActive: boolean;
  nextDueDate?: string;
  lastGeneratedAt?: string;
  lastSubmissionId?: string;
  autoGenerate: boolean;
  autoSubmit: boolean;
  notifyBeforeDays: number;
  notifyUsers: string[];
  createdBy?: string;
  createdAt: string;
  updatedAt: string;
}

export interface CreateSubmissionScheduleRequest {
  portalConnectionId: string;
  templateId: string;
  autoGenerate?: boolean;
  autoSubmit?: boolean;
  notifyBeforeDays?: number;
  notifyUsers?: string[];
}

export interface UpdateSubmissionScheduleRequest {
  isActive?: boolean;
  autoGenerate?: boolean;
  autoSubmit?: boolean;
  notifyBeforeDays?: number;
  notifyUsers?: string[];
}

export interface GovernmentPortalStats {
  totalConnections: number;
  activeConnections: number;
  totalSubmissions: number;
  submissionsThisMonth: number;
  pendingSubmissions: number;
  rejectedSubmissions: number;
  upcomingDueDates: UpcomingDueDate[];
}

export interface UpcomingDueDate {
  scheduleId: string;
  templateName: string;
  portalType: GovernmentPortalType;
  dueDate: string;
  daysUntilDue: number;
}

export interface ValidationResult {
  isValid: boolean;
  errors: ValidationError[];
  warnings: ValidationWarning[];
}

export interface ValidationError {
  field: string;
  code: string;
  message: string;
}

export interface ValidationWarning {
  field: string;
  code: string;
  message: string;
}

export interface TestConnectionResponse {
  success: boolean;
  message: string;
}

export interface ListTemplatesParams {
  portalType?: GovernmentPortalType;
  countryCode?: string;
}

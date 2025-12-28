/**
 * Developer Portal Types (Epic 69)
 *
 * Type definitions for API keys, webhooks, rate limits, and SDK management.
 */

// ==================== API Key Types (Story 69.1) ====================

export type ApiKeyStatus = 'active' | 'revoked' | 'expired' | 'suspended';

export type ApiKeyScope =
  | 'read'
  | 'write'
  | 'admin'
  | 'buildings:read'
  | 'buildings:write'
  | 'faults:read'
  | 'faults:write'
  | 'financial:read'
  | 'financial:write'
  | 'residents:read'
  | 'residents:write'
  | 'webhooks:manage';

export interface DeveloperAccount {
  id: string;
  userId: string;
  organizationId?: string;
  companyName?: string;
  website?: string;
  description?: string;
  contactEmail: string;
  contactName?: string;
  tier: RateLimitTier;
  isVerified: boolean;
  isActive: boolean;
  metadata?: Record<string, unknown>;
  createdAt: string;
  updatedAt: string;
  verifiedAt?: string;
}

export interface CreateDeveloperAccount {
  companyName?: string;
  website?: string;
  description?: string;
  contactEmail: string;
  contactName?: string;
  metadata?: Record<string, unknown>;
}

export interface UpdateDeveloperAccount {
  companyName?: string;
  website?: string;
  description?: string;
  contactEmail?: string;
  contactName?: string;
  tier?: RateLimitTier;
  isActive?: boolean;
  metadata?: Record<string, unknown>;
}

export interface ApiKey {
  id: string;
  developerAccountId: string;
  name: string;
  keyPrefix: string;
  scopes: ApiKeyScope[];
  rateLimitPerMinute?: number;
  rateLimitPerHour?: number;
  rateLimitPerDay?: number;
  lastUsedAt?: string;
  totalRequests: number;
  status: ApiKeyStatus;
  expiresAt?: string;
  allowedIps?: string[];
  allowedOrigins?: string[];
  createdAt: string;
}

export interface CreateApiKey {
  name: string;
  scopes: ApiKeyScope[];
  rateLimitPerMinute?: number;
  rateLimitPerHour?: number;
  rateLimitPerDay?: number;
  expiresAt?: string;
  allowedIps?: string[];
  allowedOrigins?: string[];
  metadata?: Record<string, unknown>;
}

export interface CreateApiKeyResponse {
  id: string;
  name: string;
  keyPrefix: string;
  secret: string;
  scopes: ApiKeyScope[];
  expiresAt?: string;
  createdAt: string;
}

export interface UpdateApiKey {
  name?: string;
  scopes?: ApiKeyScope[];
  rateLimitPerMinute?: number;
  rateLimitPerHour?: number;
  rateLimitPerDay?: number;
  expiresAt?: string;
  allowedIps?: string[];
  allowedOrigins?: string[];
  metadata?: Record<string, unknown>;
}

export interface ApiKeyUsageStats {
  apiKeyId: string;
  date: string;
  totalRequests: number;
  successfulRequests: number;
  failedRequests: number;
  rateLimitedRequests: number;
  avgResponseTimeMs?: number;
}

export interface RotateApiKeyResponse {
  oldKeyId: string;
  newKey: CreateApiKeyResponse;
  oldKeyExpiresAt: string;
}

// ==================== Webhook Types (Story 69.3) ====================

export type WebhookEventType =
  | 'fault.created'
  | 'fault.updated'
  | 'fault.resolved'
  | 'payment.received'
  | 'payment.overdue'
  | 'resident.moved_in'
  | 'resident.moved_out'
  | 'vote.started'
  | 'vote.ended'
  | 'announcement.published'
  | 'document.uploaded'
  | 'work_order.created'
  | 'work_order.completed';

export type WebhookDeliveryStatus = 'pending' | 'delivered' | 'failed' | 'retrying' | 'exhausted';

export interface WebhookSubscription {
  id: string;
  developerAccountId: string;
  organizationId?: string;
  name: string;
  endpointUrl: string;
  eventTypes: WebhookEventType[];
  isActive: boolean;
  retryCount: number;
  timeoutSeconds: number;
  customHeaders?: Record<string, string>;
  lastTriggeredAt?: string;
  lastSuccessAt?: string;
  lastFailureAt?: string;
  totalDeliveries: number;
  successfulDeliveries: number;
  failedDeliveries: number;
  createdAt: string;
  updatedAt: string;
}

export interface CreateWebhookSubscription {
  name: string;
  endpointUrl: string;
  eventTypes: WebhookEventType[];
  retryCount?: number;
  timeoutSeconds?: number;
  customHeaders?: Record<string, string>;
  metadata?: Record<string, unknown>;
}

export interface CreateWebhookResponse {
  id: string;
  name: string;
  endpointUrl: string;
  secret: string;
  eventTypes: WebhookEventType[];
  createdAt: string;
}

export interface UpdateWebhookSubscription {
  name?: string;
  endpointUrl?: string;
  eventTypes?: WebhookEventType[];
  isActive?: boolean;
  retryCount?: number;
  timeoutSeconds?: number;
  customHeaders?: Record<string, string>;
  metadata?: Record<string, unknown>;
}

export interface WebhookDelivery {
  id: string;
  subscriptionId: string;
  eventType: WebhookEventType;
  eventId: string;
  payload: Record<string, unknown>;
  attemptNumber: number;
  status: WebhookDeliveryStatus;
  responseStatusCode?: number;
  responseBody?: string;
  responseTimeMs?: number;
  errorMessage?: string;
  createdAt: string;
  deliveredAt?: string;
  nextRetryAt?: string;
}

export interface TestWebhookRequest {
  eventType: WebhookEventType;
  payload?: Record<string, unknown>;
}

export interface TestWebhookResponse {
  success: boolean;
  responseStatusCode?: number;
  responseBody?: string;
  responseTimeMs?: number;
  errorMessage?: string;
}

export interface RotateWebhookSecretResponse {
  webhookId: string;
  newSecret: string;
  rotatedAt: string;
}

// ==================== Rate Limiting Types (Story 69.4) ====================

export type RateLimitTier = 'free' | 'basic' | 'professional' | 'enterprise';

export interface RateLimitConfig {
  id: string;
  tier: RateLimitTier;
  requestsPerMinute: number;
  requestsPerHour: number;
  requestsPerDay: number;
  burstLimit?: number;
  endpointLimits?: Record<string, number>;
  description?: string;
  isActive: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface RateLimitWindow {
  limit: number;
  remaining: number;
  resetAt: string;
}

export interface RateLimitStatus {
  tier: RateLimitTier;
  requestsPerMinute: RateLimitWindow;
  requestsPerHour: RateLimitWindow;
  requestsPerDay: RateLimitWindow;
}

// ==================== API Documentation Types (Story 69.2) ====================

export interface ApiEndpointDoc {
  id: string;
  path: string;
  method: 'GET' | 'POST' | 'PUT' | 'PATCH' | 'DELETE';
  summary: string;
  description?: string;
  tag: string;
  category?: string;
  requestBody?: Record<string, unknown>;
  responseBody?: Record<string, unknown>;
  parameters?: Record<string, unknown>;
  requiresAuth: boolean;
  requiredScopes?: ApiKeyScope[];
  examples?: Record<string, unknown>;
  rateLimitTier?: RateLimitTier;
  isDeprecated: boolean;
  deprecatedMessage?: string;
  version?: string;
  createdAt: string;
  updatedAt: string;
}

export interface ApiChangelog {
  id: string;
  version: string;
  releaseDate: string;
  title: string;
  description?: string;
  changes: ChangelogEntry[];
  breakingChanges?: ChangelogEntry[];
  migrationGuide?: string;
  createdAt: string;
}

export interface ChangelogEntry {
  type: 'added' | 'changed' | 'deprecated' | 'removed' | 'fixed' | 'security';
  description: string;
  endpoint?: string;
}

// ==================== SDK Types (Story 69.5) ====================

export type SdkLanguage = 'typescript' | 'python' | 'go' | 'java' | 'csharp';

export interface SdkLanguageInfo {
  language: SdkLanguage;
  displayName: string;
  packageManager: string;
  latestVersion?: string;
  installationCommand: string;
  documentationUrl?: string;
}

export interface SdkVersion {
  id: string;
  language: SdkLanguage;
  version: string;
  apiVersion: string;
  downloadUrl?: string;
  packageName?: string;
  packageManagerUrl?: string;
  buildStatus: 'pending' | 'building' | 'success' | 'failed';
  buildLog?: string;
  checksumSha256?: string;
  downloadCount: number;
  releaseNotes?: string;
  isLatest: boolean;
  isStable: boolean;
  createdAt: string;
  publishedAt?: string;
}

export interface SdkDownloadInfo {
  language: SdkLanguage;
  version: string;
  apiVersion: string;
  downloadUrl: string;
  packageName?: string;
  packageManagerUrl?: string;
  checksumSha256?: string;
  releaseNotes?: string;
}

// ==================== Sandbox Types (Story 69.2) ====================

export interface SandboxEnvironment {
  id: string;
  developerAccountId: string;
  name: string;
  description?: string;
  mockDataEnabled: boolean;
  rateLimitsEnabled: boolean;
  isActive: boolean;
  expiresAt?: string;
  createdAt: string;
  updatedAt: string;
}

export interface SandboxTestRequest {
  endpoint: string;
  method: 'GET' | 'POST' | 'PUT' | 'PATCH' | 'DELETE';
  headers?: Record<string, string>;
  body?: Record<string, unknown>;
}

export interface SandboxTestResponse {
  statusCode: number;
  headers: Record<string, string>;
  body: Record<string, unknown>;
  responseTimeMs: number;
}

// ==================== Analytics Types ====================

export interface DeveloperPortalStats {
  totalDevelopers: number;
  activeApiKeys: number;
  totalApiRequestsToday: number;
  totalApiRequestsMonth: number;
  webhookDeliveriesToday: number;
  successfulWebhookRate: number;
  topEndpoints: EndpointUsage[];
  requestsByTier: TierUsage[];
}

export interface EndpointUsage {
  endpoint: string;
  method: string;
  requestCount: number;
  avgResponseTimeMs?: number;
  errorRate?: number;
}

export interface TierUsage {
  tier: RateLimitTier;
  developerCount: number;
  requestCount: number;
  percentage: number;
}

export interface DeveloperUsageSummary {
  developerAccountId: string;
  companyName?: string;
  tier: RateLimitTier;
  apiKeysCount: number;
  webhooksCount: number;
  totalRequestsToday: number;
  totalRequestsMonth: number;
  rateLimitHits: number;
  lastApiCall?: string;
}

// ==================== Pagination Types ====================

export interface PaginatedResponse<T> {
  data: T[];
  total: number;
  limit: number;
  offset: number;
  hasMore: boolean;
}

export interface PaginationParams {
  limit?: number;
  offset?: number;
}

// ==================== Query Parameter Types ====================

export interface ApiKeyQuery extends PaginationParams {
  status?: ApiKeyStatus;
  scope?: ApiKeyScope;
}

export interface WebhookSubscriptionQuery extends PaginationParams {
  eventType?: WebhookEventType;
  isActive?: boolean;
}

export interface WebhookDeliveryQuery extends PaginationParams {
  subscriptionId?: string;
  eventType?: WebhookEventType;
  status?: WebhookDeliveryStatus;
}

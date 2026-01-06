/**
 * Developer Portal API Service (Epic 69)
 *
 * Provides API functions for developer portal operations including
 * API key management, webhooks, rate limits, and SDK downloads.
 */

import type {
  ApiKey,
  ApiKeyQuery,
  ApiKeyUsageStats,
  CreateApiKey,
  CreateApiKeyResponse,
  CreateWebhookResponse,
  CreateWebhookSubscription,
  DeveloperAccount,
  DeveloperUsageSummary,
  RateLimitConfig,
  RateLimitStatus,
  RotateApiKeyResponse,
  RotateWebhookSecretResponse,
  SdkDownloadInfo,
  SdkLanguage,
  SdkLanguageInfo,
  SdkVersion,
  TestWebhookRequest,
  TestWebhookResponse,
  UpdateWebhookSubscription,
  WebhookDelivery,
  WebhookDeliveryQuery,
  WebhookSubscription,
  WebhookSubscriptionQuery,
} from '../types';

const API_BASE = '/api/v1/developer';

/**
 * Helper function to make API requests with error handling.
 */
async function apiRequest<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
  const response = await fetch(`${API_BASE}${endpoint}`, {
    ...options,
    headers: {
      'Content-Type': 'application/json',
      ...options.headers,
    },
  });

  if (!response.ok) {
    const error = await response.json().catch(() => ({
      code: 'UNKNOWN_ERROR',
      message: 'An unknown error occurred',
    }));
    throw new Error(error.message || `HTTP ${response.status}`);
  }

  // Handle 204 No Content
  if (response.status === 204) {
    return undefined as T;
  }

  return response.json();
}

// ==================== Developer Account ====================

export async function getMyDeveloperAccount(): Promise<DeveloperAccount> {
  return apiRequest<DeveloperAccount>('/accounts/me');
}

export async function getMyUsageSummary(): Promise<DeveloperUsageSummary> {
  return apiRequest<DeveloperUsageSummary>('/accounts/me/usage');
}

// ==================== API Keys ====================

export async function listApiKeys(query?: ApiKeyQuery): Promise<ApiKey[]> {
  const params = new URLSearchParams();
  if (query?.limit) params.set('limit', String(query.limit));
  if (query?.offset) params.set('offset', String(query.offset));
  if (query?.status) params.set('status', query.status);
  if (query?.scope) params.set('scope', query.scope);

  const queryString = params.toString();
  return apiRequest<ApiKey[]>(`/keys${queryString ? `?${queryString}` : ''}`);
}

export async function createApiKey(data: CreateApiKey): Promise<CreateApiKeyResponse> {
  return apiRequest<CreateApiKeyResponse>('/keys', {
    method: 'POST',
    body: JSON.stringify(data),
  });
}

export async function rotateApiKey(id: string): Promise<RotateApiKeyResponse> {
  return apiRequest<RotateApiKeyResponse>(`/keys/${id}/rotate`, {
    method: 'POST',
  });
}

export async function revokeApiKey(id: string): Promise<void> {
  return apiRequest<void>(`/keys/${id}`, {
    method: 'DELETE',
  });
}

export async function getApiKeyUsage(
  id: string,
  startDate?: string,
  endDate?: string
): Promise<ApiKeyUsageStats[]> {
  const params = new URLSearchParams();
  if (startDate) params.set('start_date', startDate);
  if (endDate) params.set('end_date', endDate);

  const queryString = params.toString();
  return apiRequest<ApiKeyUsageStats[]>(`/keys/${id}/usage${queryString ? `?${queryString}` : ''}`);
}

// ==================== Webhooks ====================

export async function listWebhooks(
  query?: WebhookSubscriptionQuery
): Promise<WebhookSubscription[]> {
  const params = new URLSearchParams();
  if (query?.limit) params.set('limit', String(query.limit));
  if (query?.offset) params.set('offset', String(query.offset));
  if (query?.eventType) params.set('event_type', query.eventType);
  if (query?.isActive !== undefined) params.set('is_active', String(query.isActive));

  const queryString = params.toString();
  return apiRequest<WebhookSubscription[]>(`/webhooks${queryString ? `?${queryString}` : ''}`);
}

export async function createWebhook(
  data: CreateWebhookSubscription
): Promise<CreateWebhookResponse> {
  return apiRequest<CreateWebhookResponse>('/webhooks', {
    method: 'POST',
    body: JSON.stringify(data),
  });
}

export async function updateWebhook(
  id: string,
  data: UpdateWebhookSubscription
): Promise<WebhookSubscription> {
  return apiRequest<WebhookSubscription>(`/webhooks/${id}`, {
    method: 'PATCH',
    body: JSON.stringify(data),
  });
}

export async function deleteWebhook(id: string): Promise<void> {
  return apiRequest<void>(`/webhooks/${id}`, {
    method: 'DELETE',
  });
}

export async function testWebhook(
  id: string,
  request: TestWebhookRequest
): Promise<TestWebhookResponse> {
  return apiRequest<TestWebhookResponse>(`/webhooks/${id}/test`, {
    method: 'POST',
    body: JSON.stringify(request),
  });
}

export async function rotateWebhookSecret(id: string): Promise<RotateWebhookSecretResponse> {
  return apiRequest<RotateWebhookSecretResponse>(`/webhooks/${id}/rotate-secret`, {
    method: 'POST',
  });
}

export async function listWebhookDeliveries(
  webhookId: string,
  query?: WebhookDeliveryQuery
): Promise<WebhookDelivery[]> {
  const params = new URLSearchParams();
  if (query?.limit) params.set('limit', String(query.limit));
  if (query?.offset) params.set('offset', String(query.offset));
  if (query?.eventType) params.set('event_type', query.eventType);
  if (query?.status) params.set('status', query.status);

  const queryString = params.toString();
  return apiRequest<WebhookDelivery[]>(
    `/webhooks/${webhookId}/deliveries${queryString ? `?${queryString}` : ''}`
  );
}

// ==================== Rate Limits ====================

export async function getRateLimitStatus(): Promise<RateLimitStatus> {
  return apiRequest<RateLimitStatus>('/rate-limits/status');
}

export async function listRateLimitTiers(): Promise<RateLimitConfig[]> {
  return apiRequest<RateLimitConfig[]>('/rate-limits/tiers');
}

// ==================== SDKs ====================

export async function listSdkLanguages(): Promise<SdkLanguageInfo[]> {
  return apiRequest<SdkLanguageInfo[]>('/sdks');
}

export async function getSdkInfo(language: SdkLanguage): Promise<SdkDownloadInfo> {
  return apiRequest<SdkDownloadInfo>(`/sdks/${language}`);
}

export async function downloadSdk(language: SdkLanguage): Promise<void> {
  // Open download in new tab
  window.open(`${API_BASE}/sdks/${language}/download`, '_blank');
}

export async function listSdkVersions(language: SdkLanguage): Promise<SdkVersion[]> {
  return apiRequest<SdkVersion[]>(`/sdks/${language}/versions`);
}

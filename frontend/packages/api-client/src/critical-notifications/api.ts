/**
 * Critical Notifications API (Epic 8A, Story 8A.2)
 */

import type {
  AcknowledgeCriticalNotificationResponse,
  CreateCriticalNotificationRequest,
  CreateCriticalNotificationResponse,
  CriticalNotificationResponse,
  CriticalNotificationStats,
  UnacknowledgedNotificationsResponse,
} from './types';

const API_BASE = '/api/v1';

interface FetchOptions {
  accessToken: string;
  tenantContext?: string;
}

function getHeaders(options: FetchOptions): HeadersInit {
  const headers: HeadersInit = {
    'Content-Type': 'application/json',
    Authorization: `Bearer ${options.accessToken}`,
  };
  if (options.tenantContext) {
    headers['X-Tenant-Context'] = options.tenantContext;
  }
  return headers;
}

/**
 * Safely parse JSON error response, returning default message if parsing fails
 */
async function getErrorMessage(response: Response, defaultMessage: string): Promise<string> {
  try {
    const error = await response.json();
    return error.error?.message || defaultMessage;
  } catch {
    return defaultMessage;
  }
}

/**
 * Get all critical notifications for an organization
 */
export async function getCriticalNotifications(
  organizationId: string,
  options: FetchOptions
): Promise<CriticalNotificationResponse[]> {
  const response = await fetch(
    `${API_BASE}/organizations/${organizationId}/critical-notifications`,
    {
      method: 'GET',
      headers: getHeaders(options),
    }
  );

  if (!response.ok) {
    const message = await getErrorMessage(response, 'Failed to fetch critical notifications');
    throw new Error(message);
  }

  return response.json();
}

/**
 * Get unacknowledged critical notifications for the current user
 */
export async function getUnacknowledgedNotifications(
  organizationId: string,
  options: FetchOptions
): Promise<UnacknowledgedNotificationsResponse> {
  const response = await fetch(
    `${API_BASE}/organizations/${organizationId}/critical-notifications/unacknowledged`,
    {
      method: 'GET',
      headers: getHeaders(options),
    }
  );

  if (!response.ok) {
    const message = await getErrorMessage(response, 'Failed to fetch unacknowledged notifications');
    throw new Error(message);
  }

  return response.json();
}

/**
 * Create a new critical notification (admin only)
 */
export async function createCriticalNotification(
  organizationId: string,
  request: CreateCriticalNotificationRequest,
  options: FetchOptions
): Promise<CreateCriticalNotificationResponse> {
  const response = await fetch(
    `${API_BASE}/organizations/${organizationId}/critical-notifications`,
    {
      method: 'POST',
      headers: getHeaders(options),
      body: JSON.stringify(request),
    }
  );

  if (!response.ok) {
    if (response.status === 403) {
      throw new Error('Only administrators can create critical notifications');
    }
    const message = await getErrorMessage(response, 'Failed to create critical notification');
    throw new Error(message);
  }

  return response.json();
}

/**
 * Acknowledge a critical notification
 */
export async function acknowledgeCriticalNotification(
  organizationId: string,
  notificationId: string,
  options: FetchOptions
): Promise<AcknowledgeCriticalNotificationResponse> {
  const response = await fetch(
    `${API_BASE}/organizations/${organizationId}/critical-notifications/${notificationId}/acknowledge`,
    {
      method: 'POST',
      headers: getHeaders(options),
    }
  );

  if (!response.ok) {
    if (response.status === 404) {
      throw new Error('Notification not found');
    }
    const message = await getErrorMessage(response, 'Failed to acknowledge notification');
    throw new Error(message);
  }

  return response.json();
}

/**
 * Get acknowledgment statistics for a notification (admin only)
 */
export async function getCriticalNotificationStats(
  organizationId: string,
  notificationId: string,
  options: FetchOptions
): Promise<CriticalNotificationStats> {
  const response = await fetch(
    `${API_BASE}/organizations/${organizationId}/critical-notifications/${notificationId}/stats`,
    {
      method: 'GET',
      headers: getHeaders(options),
    }
  );

  if (!response.ok) {
    if (response.status === 403) {
      throw new Error('Only administrators can view notification statistics');
    }
    if (response.status === 404) {
      throw new Error('Notification not found');
    }
    const message = await getErrorMessage(response, 'Failed to fetch notification statistics');
    throw new Error(message);
  }

  return response.json();
}

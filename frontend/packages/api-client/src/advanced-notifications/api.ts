/**
 * Advanced Notifications API (Epic 40)
 */

import type {
  AdvancedPreferencesResponse,
  CategoryPreferencesResponse,
  DigestResponse,
  GroupingResponse,
  NotificationCategory,
  QuietHoursResponse,
  UpdateCategoryPreferenceRequest,
  UpdateDigestRequest,
  UpdateGroupingRequest,
  UpdateQuietHoursRequest,
} from './types';

const API_BASE = '/api/v1/users/me/notification-preferences';

// ============================================================================
// Story 40.1: Category Preferences API
// ============================================================================

/**
 * Fetch category-based notification preferences.
 */
export async function getCategoryPreferences(
  baseUrl: string,
  accessToken: string
): Promise<CategoryPreferencesResponse> {
  const response = await fetch(`${baseUrl}${API_BASE}/categories`, {
    method: 'GET',
    headers: {
      Authorization: `Bearer ${accessToken}`,
      'Content-Type': 'application/json',
    },
  });

  if (!response.ok) {
    const error = await parseError(response);
    throw new Error(error);
  }

  return response.json();
}

/**
 * Update preferences for a specific category.
 */
export async function updateCategoryPreference(
  baseUrl: string,
  accessToken: string,
  category: NotificationCategory,
  request: UpdateCategoryPreferenceRequest
): Promise<CategoryPreferencesResponse> {
  const response = await fetch(`${baseUrl}${API_BASE}/categories/${category}`, {
    method: 'PATCH',
    headers: {
      Authorization: `Bearer ${accessToken}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(request),
  });

  if (!response.ok) {
    const error = await parseError(response);
    throw new Error(error);
  }

  return response.json();
}

// ============================================================================
// Story 40.2: Quiet Hours API
// ============================================================================

/**
 * Fetch quiet hours configuration.
 */
export async function getQuietHours(
  baseUrl: string,
  accessToken: string
): Promise<QuietHoursResponse> {
  const response = await fetch(`${baseUrl}${API_BASE}/quiet-hours`, {
    method: 'GET',
    headers: {
      Authorization: `Bearer ${accessToken}`,
      'Content-Type': 'application/json',
    },
  });

  if (!response.ok) {
    const error = await parseError(response);
    throw new Error(error);
  }

  return response.json();
}

/**
 * Update quiet hours configuration.
 */
export async function updateQuietHours(
  baseUrl: string,
  accessToken: string,
  request: UpdateQuietHoursRequest
): Promise<QuietHoursResponse> {
  const response = await fetch(`${baseUrl}${API_BASE}/quiet-hours`, {
    method: 'PATCH',
    headers: {
      Authorization: `Bearer ${accessToken}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(request),
  });

  if (!response.ok) {
    const error = await parseError(response);
    throw new Error(error);
  }

  return response.json();
}

// ============================================================================
// Story 40.3: Digest Preferences API
// ============================================================================

/**
 * Fetch digest configuration.
 */
export async function getDigestPreferences(
  baseUrl: string,
  accessToken: string
): Promise<DigestResponse> {
  const response = await fetch(`${baseUrl}${API_BASE}/digest`, {
    method: 'GET',
    headers: {
      Authorization: `Bearer ${accessToken}`,
      'Content-Type': 'application/json',
    },
  });

  if (!response.ok) {
    const error = await parseError(response);
    throw new Error(error);
  }

  return response.json();
}

/**
 * Update digest preferences.
 */
export async function updateDigestPreferences(
  baseUrl: string,
  accessToken: string,
  request: UpdateDigestRequest
): Promise<DigestResponse> {
  const response = await fetch(`${baseUrl}${API_BASE}/digest`, {
    method: 'PATCH',
    headers: {
      Authorization: `Bearer ${accessToken}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(request),
  });

  if (!response.ok) {
    const error = await parseError(response);
    throw new Error(error);
  }

  return response.json();
}

// ============================================================================
// Story 40.4: Grouping Preferences API
// ============================================================================

/**
 * Fetch notification grouping configuration.
 */
export async function getGroupingPreferences(
  baseUrl: string,
  accessToken: string
): Promise<GroupingResponse> {
  const response = await fetch(`${baseUrl}${API_BASE}/grouping`, {
    method: 'GET',
    headers: {
      Authorization: `Bearer ${accessToken}`,
      'Content-Type': 'application/json',
    },
  });

  if (!response.ok) {
    const error = await parseError(response);
    throw new Error(error);
  }

  return response.json();
}

/**
 * Update notification grouping preferences.
 */
export async function updateGroupingPreferences(
  baseUrl: string,
  accessToken: string,
  request: UpdateGroupingRequest
): Promise<GroupingResponse> {
  const response = await fetch(`${baseUrl}${API_BASE}/grouping`, {
    method: 'PATCH',
    headers: {
      Authorization: `Bearer ${accessToken}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(request),
  });

  if (!response.ok) {
    const error = await parseError(response);
    throw new Error(error);
  }

  return response.json();
}

// ============================================================================
// Combined API
// ============================================================================

/**
 * Fetch all advanced notification preferences at once.
 */
export async function getAdvancedPreferences(
  baseUrl: string,
  accessToken: string
): Promise<AdvancedPreferencesResponse> {
  const response = await fetch(`${baseUrl}${API_BASE}/advanced`, {
    method: 'GET',
    headers: {
      Authorization: `Bearer ${accessToken}`,
      'Content-Type': 'application/json',
    },
  });

  if (!response.ok) {
    const error = await parseError(response);
    throw new Error(error);
  }

  return response.json();
}

// ============================================================================
// Helpers
// ============================================================================

async function parseError(response: Response): Promise<string> {
  try {
    const data = await response.json();
    return data.error?.message || 'An error occurred';
  } catch {
    return 'An error occurred';
  }
}

/**
 * Notification Preferences API (Epic 8A, Story 8A.1)
 */

import type {
  NotificationChannel,
  NotificationPreferencesResponse,
  UpdateNotificationPreferenceRequest,
  UpdatePreferenceResponse,
} from './types';

const API_BASE = '/api/v1/users/me/notification-preferences';

/**
 * Fetch all notification preferences for the current user.
 */
export async function getNotificationPreferences(
  baseUrl: string,
  accessToken: string
): Promise<NotificationPreferencesResponse> {
  const response = await fetch(`${baseUrl}${API_BASE}`, {
    method: 'GET',
    headers: {
      Authorization: `Bearer ${accessToken}`,
      'Content-Type': 'application/json',
    },
  });

  if (!response.ok) {
    let errorMessage = 'Failed to fetch notification preferences';
    try {
      const error = await response.json();
      errorMessage = error.error?.message || errorMessage;
    } catch {
      // Response is not JSON, use default message
    }
    throw new Error(errorMessage);
  }

  return response.json();
}

/**
 * Update a specific notification channel preference.
 */
export async function updateNotificationPreference(
  baseUrl: string,
  accessToken: string,
  channel: NotificationChannel,
  request: UpdateNotificationPreferenceRequest
): Promise<UpdatePreferenceResponse> {
  const response = await fetch(`${baseUrl}${API_BASE}/${channel}`, {
    method: 'PATCH',
    headers: {
      Authorization: `Bearer ${accessToken}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(request),
  });

  if (!response.ok) {
    let errorMessage = 'Failed to update notification preference';
    let errorData: { error?: { message?: string } } = {};
    try {
      errorData = await response.json();
      errorMessage = errorData.error?.message || errorMessage;
    } catch {
      // Response is not JSON, use default message
    }

    // Check if this is a confirmation required error (409)
    if (response.status === 409) {
      throw new ConfirmationRequiredError(
        errorData.error?.message || 'Confirmation required to disable all channels',
        channel
      );
    }

    throw new Error(errorMessage);
  }

  return response.json();
}

/**
 * Custom error for when confirmation is required to disable all channels.
 */
export class ConfirmationRequiredError extends Error {
  channel: NotificationChannel;

  constructor(message: string, channel: NotificationChannel) {
    super(message);
    this.name = 'ConfirmationRequiredError';
    this.channel = channel;
  }
}

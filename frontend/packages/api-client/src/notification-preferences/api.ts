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
    const error = await response.json();
    throw new Error(error.error?.message || 'Failed to fetch notification preferences');
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
    const error = await response.json();

    // Check if this is a confirmation required error (409)
    if (response.status === 409) {
      throw new ConfirmationRequiredError(
        error.error?.message || 'Confirmation required to disable all channels',
        channel
      );
    }

    throw new Error(error.error?.message || 'Failed to update notification preference');
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

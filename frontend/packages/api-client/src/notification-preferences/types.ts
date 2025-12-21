/**
 * Notification Preferences Types (Epic 8A, Story 8A.1)
 */

/** Notification channel types */
export type NotificationChannel = 'push' | 'email' | 'in_app';

/** Single notification preference */
export interface NotificationPreference {
  channel: NotificationChannel;
  enabled: boolean;
  updatedAt: string;
}

/** Response from GET /users/me/notification-preferences */
export interface NotificationPreferencesResponse {
  preferences: NotificationPreference[];
  allDisabledWarning?: string | null;
}

/** Request to update a preference */
export interface UpdateNotificationPreferenceRequest {
  enabled: boolean;
  confirmDisableAll?: boolean;
}

/** Response from PATCH /users/me/notification-preferences/{channel} */
export interface UpdatePreferenceResponse {
  preference: NotificationPreference;
  allDisabledWarning?: string | null;
}

/** Error response when confirmation is required */
export interface DisableAllWarningResponse {
  message: string;
  requiresConfirmation: boolean;
  channel: NotificationChannel;
}

/** Human-readable labels for channels */
export const CHANNEL_LABELS: Record<NotificationChannel, string> = {
  push: 'Push Notifications',
  email: 'Email Notifications',
  in_app: 'In-App Notifications',
};

/** Descriptions for each channel */
export const CHANNEL_DESCRIPTIONS: Record<NotificationChannel, string> = {
  push: 'Receive notifications on your mobile device',
  email: 'Receive notifications via email',
  in_app: 'See notifications within the application',
};

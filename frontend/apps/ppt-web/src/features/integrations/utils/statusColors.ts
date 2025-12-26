/**
 * Shared status color utilities for integration components.
 *
 * Provides consistent styling for status badges across
 * Calendar, Video, and Webhook integrations.
 */

import type { CalendarProvider } from '@ppt/api-client';

/**
 * Common status colors used across integration components.
 * Maps status strings to Tailwind CSS classes.
 */
export const connectionStatusColors: Record<string, string> = {
  active: 'bg-green-100 text-green-800',
  paused: 'bg-yellow-100 text-yellow-800',
  error: 'bg-red-100 text-red-800',
  disconnected: 'bg-gray-100 text-gray-800',
};

/**
 * Video meeting status colors.
 */
export const meetingStatusColors: Record<string, string> = {
  scheduled: 'bg-blue-100 text-blue-800',
  started: 'bg-green-100 text-green-800',
  ended: 'bg-gray-100 text-gray-800',
  cancelled: 'bg-red-100 text-red-800',
};

/**
 * Webhook subscription status colors.
 */
export const webhookStatusColors: Record<string, string> = {
  active: 'bg-green-100 text-green-800',
  paused: 'bg-yellow-100 text-yellow-800',
  disabled: 'bg-gray-100 text-gray-800',
};

/**
 * Calendar provider display icons (single letter abbreviations).
 */
export const calendarProviderIcons: Record<CalendarProvider, string> = {
  google: 'G',
  outlook: 'O',
  apple: 'A',
  caldav: 'C',
};

/**
 * Calendar provider background and text colors.
 */
export const calendarProviderColors: Record<CalendarProvider, string> = {
  google: 'bg-red-100 text-red-600',
  outlook: 'bg-blue-100 text-blue-600',
  apple: 'bg-gray-100 text-gray-600',
  caldav: 'bg-purple-100 text-purple-600',
};

/**
 * Get status color classes for a given status.
 * Falls back to a neutral gray if status is not found.
 */
export function getStatusColor(status: string, colorMap: Record<string, string>): string {
  return colorMap[status] ?? 'bg-gray-100 text-gray-800';
}

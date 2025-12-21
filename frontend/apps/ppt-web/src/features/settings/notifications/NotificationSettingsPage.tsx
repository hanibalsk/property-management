/**
 * NotificationSettingsPage Component (Epic 8A, Story 8A.1)
 *
 * Settings page for managing notification channel preferences.
 */

import type { NotificationChannel } from '@ppt/api-client';
import {
  ConfirmationRequiredError,
  useNotificationPreferences,
  useUpdateNotificationPreference,
} from '@ppt/api-client';
import { useCallback, useState } from 'react';
import { ChannelToggle, DisableAllWarningDialog } from './components';

interface NotificationSettingsPageProps {
  baseUrl: string;
  accessToken: string;
}

export function NotificationSettingsPage({ baseUrl, accessToken }: NotificationSettingsPageProps) {
  const [showWarningDialog, setShowWarningDialog] = useState(false);
  const [pendingChannel, setPendingChannel] = useState<NotificationChannel | null>(null);

  const { data, isLoading, error } = useNotificationPreferences({ baseUrl, accessToken });
  const updatePreference = useUpdateNotificationPreference({ baseUrl, accessToken });

  const handleToggle = useCallback(
    async (channel: NotificationChannel, enabled: boolean) => {
      try {
        await updatePreference.mutateAsync({
          channel,
          request: { enabled },
        });
      } catch (err) {
        if (err instanceof ConfirmationRequiredError) {
          // Show confirmation dialog
          setPendingChannel(channel);
          setShowWarningDialog(true);
        } else {
          // Handle other errors
          console.error('Failed to update preference:', err);
        }
      }
    },
    [updatePreference]
  );

  const handleConfirmDisableAll = useCallback(async () => {
    if (!pendingChannel) return;

    try {
      await updatePreference.mutateAsync({
        channel: pendingChannel,
        request: { enabled: false, confirmDisableAll: true },
      });
    } catch (err) {
      console.error('Failed to disable all:', err);
    } finally {
      setShowWarningDialog(false);
      setPendingChannel(null);
    }
  }, [pendingChannel, updatePreference]);

  const handleCancelDisableAll = useCallback(() => {
    setShowWarningDialog(false);
    setPendingChannel(null);
  }, []);

  if (isLoading) {
    return (
      <div className="max-w-2xl mx-auto p-6">
        <div className="animate-pulse">
          <div className="h-8 bg-gray-200 rounded w-1/3 mb-6" />
          <div className="space-y-4">
            <div className="h-16 bg-gray-100 rounded" />
            <div className="h-16 bg-gray-100 rounded" />
            <div className="h-16 bg-gray-100 rounded" />
          </div>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="max-w-2xl mx-auto p-6">
        <div className="bg-red-50 border border-red-200 rounded-lg p-4">
          <p className="text-red-700">Failed to load notification preferences. Please try again.</p>
        </div>
      </div>
    );
  }

  return (
    <div className="max-w-2xl mx-auto p-6">
      <h1 className="text-2xl font-bold text-gray-900 mb-2">Notification Settings</h1>
      <p className="text-gray-600 mb-6">Choose how you want to receive notifications.</p>

      {/* All disabled warning */}
      {data?.allDisabledWarning && (
        <div className="mb-6 bg-amber-50 border border-amber-200 rounded-lg p-4">
          <div className="flex">
            <div className="flex-shrink-0">
              <svg
                className="h-5 w-5 text-amber-400"
                viewBox="0 0 20 20"
                fill="currentColor"
                aria-hidden="true"
              >
                <path
                  fillRule="evenodd"
                  d="M8.485 2.495c.673-1.167 2.357-1.167 3.03 0l6.28 10.875c.673 1.167-.17 2.625-1.516 2.625H3.72c-1.347 0-2.189-1.458-1.515-2.625L8.485 2.495zM10 5a.75.75 0 01.75.75v3.5a.75.75 0 01-1.5 0v-3.5A.75.75 0 0110 5zm0 9a1 1 0 100-2 1 1 0 000 2z"
                  clipRule="evenodd"
                />
              </svg>
            </div>
            <div className="ml-3">
              <p className="text-sm text-amber-700">{data.allDisabledWarning}</p>
            </div>
          </div>
        </div>
      )}

      {/* Channel toggles */}
      <div className="bg-white rounded-lg border border-gray-200 shadow-sm">
        <div className="px-4">
          {data?.preferences.map((pref) => (
            <ChannelToggle
              key={pref.channel}
              channel={pref.channel}
              enabled={pref.enabled}
              loading={updatePreference.isPending}
              onToggle={(enabled) => handleToggle(pref.channel, enabled)}
            />
          ))}
        </div>
      </div>

      {/* Last updated info */}
      {data?.preferences && data.preferences.length > 0 && (
        <p className="mt-4 text-xs text-gray-500">
          Last updated:{' '}
          {new Date(
            Math.max(...data.preferences.map((p) => new Date(p.updatedAt).getTime()))
          ).toLocaleString()}
        </p>
      )}

      {/* Confirmation dialog */}
      <DisableAllWarningDialog
        isOpen={showWarningDialog}
        onConfirm={handleConfirmDisableAll}
        onCancel={handleCancelDisableAll}
      />
    </div>
  );
}

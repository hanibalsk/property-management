/**
 * Calendar Connect Dialog Component
 *
 * OAuth connection flow for Google Calendar and Outlook (Story 61.1).
 */

import type { CalendarProvider, CreateCalendarConnection } from '@ppt/api-client';
import { useCreateCalendarConnection } from '@ppt/api-client';
import { useState } from 'react';

interface CalendarConnectDialogProps {
  organizationId: string;
  isOpen: boolean;
  onClose: () => void;
}

const providers: { id: CalendarProvider; name: string; description: string; color: string }[] = [
  {
    id: 'google',
    name: 'Google Calendar',
    description: 'Connect your Google Calendar for bidirectional sync',
    color: 'bg-red-100 text-red-600',
  },
  {
    id: 'outlook',
    name: 'Microsoft Outlook',
    description: 'Connect your Outlook calendar via Microsoft 365',
    color: 'bg-blue-100 text-blue-600',
  },
  {
    id: 'apple',
    name: 'Apple Calendar',
    description: 'Connect your iCloud Calendar (coming soon)',
    color: 'bg-gray-100 text-gray-600',
  },
  {
    id: 'caldav',
    name: 'CalDAV',
    description: 'Connect any CalDAV-compatible calendar server',
    color: 'bg-purple-100 text-purple-600',
  },
];

export function CalendarConnectDialog({
  organizationId,
  isOpen,
  onClose,
}: CalendarConnectDialogProps) {
  const [selectedProvider, setSelectedProvider] = useState<CalendarProvider | null>(null);
  const [step, setStep] = useState<'select' | 'authorize' | 'configure'>('select');
  const [authCode, setAuthCode] = useState('');
  const [calendarId, setCalendarId] = useState('');
  const [syncDirection, setSyncDirection] = useState<'push' | 'pull' | 'bidirectional'>(
    'bidirectional'
  );

  const createConnection = useCreateCalendarConnection(organizationId);

  if (!isOpen) return null;

  const handleProviderSelect = (provider: CalendarProvider) => {
    setSelectedProvider(provider);

    if (provider === 'google' || provider === 'outlook') {
      // In a real implementation, this would redirect to the OAuth provider
      // For now, we'll show a message about the OAuth flow
      setStep('authorize');
    } else if (provider === 'caldav') {
      setStep('configure');
    } else {
      // Apple - coming soon
      alert('Apple Calendar integration is coming soon!');
    }
  };

  const handleConnect = async () => {
    if (!selectedProvider) return;

    const data: CreateCalendarConnection = {
      provider: selectedProvider,
      authCode: authCode || undefined,
      calendarId: calendarId || undefined,
      syncDirection,
    };

    try {
      await createConnection.mutateAsync(data);
      handleClose();
    } catch (error) {
      console.error('Failed to create calendar connection:', error);
    }
  };

  const handleClose = () => {
    setSelectedProvider(null);
    setStep('select');
    setAuthCode('');
    setCalendarId('');
    setSyncDirection('bidirectional');
    onClose();
  };

  const initiateOAuth = (provider: CalendarProvider) => {
    // In a real implementation, this would:
    // 1. Call the backend to get an OAuth URL with state
    // 2. Redirect the user to the OAuth provider
    // 3. Handle the callback with the auth code

    const redirectUri = `${window.location.origin}/integrations/calendar/callback`;

    if (provider === 'google') {
      // Google Calendar OAuth URL would be generated server-side
      const googleAuthUrl = `/api/v1/integrations/calendars/oauth/google?redirect_uri=${encodeURIComponent(redirectUri)}`;
      window.open(googleAuthUrl, '_blank', 'width=600,height=700');
    } else if (provider === 'outlook') {
      // Microsoft OAuth URL would be generated server-side
      const microsoftAuthUrl = `/api/v1/integrations/calendars/oauth/microsoft?redirect_uri=${encodeURIComponent(redirectUri)}`;
      window.open(microsoftAuthUrl, '_blank', 'width=600,height=700');
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div className="w-full max-w-lg bg-background rounded-lg shadow-lg p-6">
        <div className="flex items-center justify-between mb-6">
          <h2 className="text-xl font-semibold">
            {step === 'select' && 'Connect Calendar'}
            {step === 'authorize' &&
              `Connect ${selectedProvider === 'google' ? 'Google' : 'Microsoft'} Calendar`}
            {step === 'configure' && 'Configure Calendar'}
          </h2>
          <button
            type="button"
            onClick={handleClose}
            className="text-muted-foreground hover:text-foreground"
          >
            X
          </button>
        </div>

        {step === 'select' && (
          <div className="space-y-3">
            <p className="text-sm text-muted-foreground mb-4">
              Select a calendar provider to connect. Your events will sync automatically.
            </p>
            {providers.map((provider) => (
              <button
                type="button"
                key={provider.id}
                onClick={() => handleProviderSelect(provider.id)}
                disabled={provider.id === 'apple'}
                className={`w-full flex items-center gap-4 p-4 rounded-lg border hover:border-primary transition-colors text-left ${
                  provider.id === 'apple' ? 'opacity-50 cursor-not-allowed' : ''
                }`}
              >
                <div
                  className={`flex h-10 w-10 items-center justify-center rounded-lg ${provider.color}`}
                >
                  {provider.id === 'google' && 'G'}
                  {provider.id === 'outlook' && 'O'}
                  {provider.id === 'apple' && 'A'}
                  {provider.id === 'caldav' && 'C'}
                </div>
                <div>
                  <div className="font-medium">{provider.name}</div>
                  <div className="text-sm text-muted-foreground">{provider.description}</div>
                </div>
                {provider.id === 'apple' && (
                  <span className="ml-auto text-xs text-muted-foreground">Coming Soon</span>
                )}
              </button>
            ))}
          </div>
        )}

        {step === 'authorize' && selectedProvider && (
          <div className="space-y-4">
            <div className="rounded-lg bg-muted/50 p-4">
              <p className="text-sm">
                To connect your {selectedProvider === 'google' ? 'Google' : 'Microsoft'} Calendar,
                you will be redirected to sign in with your account. We will request permission to:
              </p>
              <ul className="mt-2 text-sm text-muted-foreground list-disc list-inside">
                <li>Read your calendar events</li>
                <li>Create and modify events</li>
                <li>Access your calendar list</li>
              </ul>
            </div>

            <div>
              <label htmlFor="sync-direction" className="block text-sm font-medium mb-1">
                Sync Direction
              </label>
              <select
                id="sync-direction"
                value={syncDirection}
                onChange={(e) =>
                  setSyncDirection(e.target.value as 'push' | 'pull' | 'bidirectional')
                }
                className="w-full px-3 py-2 border rounded-md bg-background"
              >
                <option value="bidirectional">Bidirectional (recommended)</option>
                <option value="push">Push only (from PPT to calendar)</option>
                <option value="pull">Pull only (from calendar to PPT)</option>
              </select>
              <p className="mt-1 text-xs text-muted-foreground">
                {syncDirection === 'bidirectional' &&
                  'Events will sync both ways between PPT and your calendar'}
                {syncDirection === 'push' && 'Only PPT events will be pushed to your calendar'}
                {syncDirection === 'pull' && 'Only calendar events will be pulled into PPT'}
              </p>
            </div>

            {/* For demo purposes, show a manual auth code input */}
            <div>
              <label htmlFor="auth-code" className="block text-sm font-medium mb-1">
                Authorization Code (for testing)
              </label>
              <input
                id="auth-code"
                type="text"
                value={authCode}
                onChange={(e) => setAuthCode(e.target.value)}
                placeholder="Paste authorization code here"
                className="w-full px-3 py-2 border rounded-md bg-background"
              />
              <p className="mt-1 text-xs text-muted-foreground">
                In production, this would be handled automatically via OAuth redirect.
              </p>
            </div>

            <div className="flex justify-between pt-4">
              <button
                type="button"
                onClick={() => setStep('select')}
                className="px-4 py-2 text-sm border rounded-md hover:bg-muted"
              >
                Back
              </button>
              <div className="flex gap-2">
                <button
                  type="button"
                  onClick={() => initiateOAuth(selectedProvider)}
                  className="px-4 py-2 text-sm border rounded-md hover:bg-muted"
                >
                  Open OAuth Window
                </button>
                <button
                  type="button"
                  onClick={handleConnect}
                  disabled={createConnection.isPending}
                  className="px-4 py-2 text-sm bg-primary text-primary-foreground rounded-md hover:bg-primary/90 disabled:opacity-50"
                >
                  {createConnection.isPending ? 'Connecting...' : 'Connect'}
                </button>
              </div>
            </div>
          </div>
        )}

        {step === 'configure' && (
          <div className="space-y-4">
            <p className="text-sm text-muted-foreground">
              Configure your CalDAV calendar server connection.
            </p>

            <div>
              <label htmlFor="caldav-url" className="block text-sm font-medium mb-1">
                Calendar URL
              </label>
              <input
                id="caldav-url"
                type="url"
                value={calendarId}
                onChange={(e) => setCalendarId(e.target.value)}
                placeholder="https://calendar.example.com/dav/user/calendar"
                className="w-full px-3 py-2 border rounded-md bg-background"
              />
            </div>

            <div>
              <label htmlFor="caldav-sync-direction" className="block text-sm font-medium mb-1">
                Sync Direction
              </label>
              <select
                id="caldav-sync-direction"
                value={syncDirection}
                onChange={(e) =>
                  setSyncDirection(e.target.value as 'push' | 'pull' | 'bidirectional')
                }
                className="w-full px-3 py-2 border rounded-md bg-background"
              >
                <option value="bidirectional">Bidirectional</option>
                <option value="push">Push only</option>
                <option value="pull">Pull only</option>
              </select>
            </div>

            <div className="flex justify-between pt-4">
              <button
                type="button"
                onClick={() => setStep('select')}
                className="px-4 py-2 text-sm border rounded-md hover:bg-muted"
              >
                Back
              </button>
              <button
                type="button"
                onClick={handleConnect}
                disabled={!calendarId || createConnection.isPending}
                className="px-4 py-2 text-sm bg-primary text-primary-foreground rounded-md hover:bg-primary/90 disabled:opacity-50"
              >
                {createConnection.isPending ? 'Connecting...' : 'Connect'}
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

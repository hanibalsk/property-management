/**
 * Calendar Connections List Component
 *
 * Displays and manages calendar connections (Story 61.1).
 */

import type { CalendarConnection, CalendarProvider } from '@ppt/api-client';
import {
  useCalendarConnections,
  useDeleteCalendarConnection,
  useSyncCalendar,
} from '@ppt/api-client';
import { useState } from 'react';
import { CalendarConnectDialog } from './CalendarConnectDialog';

interface CalendarConnectionsListProps {
  organizationId: string;
}

const statusColors: Record<string, string> = {
  active: 'bg-green-100 text-green-800',
  paused: 'bg-yellow-100 text-yellow-800',
  error: 'bg-red-100 text-red-800',
  disconnected: 'bg-gray-100 text-gray-800',
};

const providerIcons: Record<CalendarProvider, string> = {
  google: 'G',
  outlook: 'O',
  apple: 'A',
  caldav: 'C',
};

const providerColors: Record<CalendarProvider, string> = {
  google: 'bg-red-100 text-red-600',
  outlook: 'bg-blue-100 text-blue-600',
  apple: 'bg-gray-100 text-gray-600',
  caldav: 'bg-purple-100 text-purple-600',
};

export function CalendarConnectionsList({ organizationId }: CalendarConnectionsListProps) {
  const [showConnectDialog, setShowConnectDialog] = useState(false);

  const { data: connections, isLoading } = useCalendarConnections(organizationId);
  const deleteConnection = useDeleteCalendarConnection(organizationId);
  const syncCalendar = useSyncCalendar(organizationId);

  const handleSync = async (id: string) => {
    await syncCalendar.mutateAsync({ id });
  };

  const handleDelete = async (id: string) => {
    if (confirm('Are you sure you want to disconnect this calendar?')) {
      await deleteConnection.mutateAsync(id);
    }
  };

  if (isLoading) {
    return (
      <div className="rounded-lg border bg-card p-6">
        <h3 className="text-lg font-semibold">Calendar Connections</h3>
        <p className="text-muted-foreground">Loading...</p>
      </div>
    );
  }

  return (
    <>
      <div className="rounded-lg border bg-card p-6">
        <div className="flex items-center justify-between mb-4">
          <div>
            <h3 className="text-lg font-semibold">Calendar Connections</h3>
            <p className="text-sm text-muted-foreground">
              Sync events with Google Calendar, Outlook, and more
            </p>
          </div>
          <button
            type="button"
            onClick={() => setShowConnectDialog(true)}
            className="inline-flex items-center px-4 py-2 text-sm font-medium bg-primary text-primary-foreground rounded-md hover:bg-primary/90"
          >
            + Connect Calendar
          </button>
        </div>

        {connections?.length === 0 ? (
          <div className="flex flex-col items-center justify-center py-8 text-center">
            <div className="flex h-16 w-16 items-center justify-center rounded-full bg-muted mb-4">
              <span className="text-2xl text-muted-foreground">CAL</span>
            </div>
            <p className="text-muted-foreground">No calendars connected</p>
            <p className="text-sm text-muted-foreground">
              Connect a calendar to sync your property events
            </p>
            <button
              type="button"
              onClick={() => setShowConnectDialog(true)}
              className="mt-4 px-4 py-2 text-sm font-medium bg-primary text-primary-foreground rounded-md hover:bg-primary/90"
            >
              Connect Your First Calendar
            </button>
          </div>
        ) : (
          <div className="space-y-4">
            {connections?.map((connection: CalendarConnection) => {
              const provider = connection.provider as CalendarProvider;
              return (
                <div
                  key={connection.id}
                  className="flex items-center justify-between rounded-lg border p-4"
                >
                  <div className="flex items-center gap-4">
                    <div
                      className={`flex h-10 w-10 items-center justify-center rounded-full ${
                        providerColors[provider] || 'bg-muted'
                      }`}
                    >
                      <span className="font-semibold">{providerIcons[provider] || 'C'}</span>
                    </div>
                    <div>
                      <div className="flex items-center gap-2">
                        <span className="font-medium capitalize">{connection.provider}</span>
                        <span
                          className={`inline-flex items-center px-2 py-0.5 rounded text-xs font-medium ${
                            statusColors[connection.syncStatus]
                          }`}
                        >
                          {connection.syncStatus}
                        </span>
                        <span className="text-xs text-muted-foreground">
                          ({connection.syncDirection})
                        </span>
                      </div>
                      <div className="text-sm text-muted-foreground">
                        {connection.calendarName ?? 'Default Calendar'}
                      </div>
                      {connection.lastSyncAt && (
                        <div className="text-xs text-muted-foreground">
                          Last synced: {new Date(connection.lastSyncAt).toLocaleString()}
                        </div>
                      )}
                      {connection.lastError && (
                        <div className="text-xs text-red-600">Error: {connection.lastError}</div>
                      )}
                    </div>
                  </div>
                  <div className="flex items-center gap-2">
                    <button
                      type="button"
                      onClick={() => handleSync(connection.id)}
                      disabled={syncCalendar.isPending}
                      className="px-3 py-1 text-sm border rounded-md hover:bg-muted disabled:opacity-50"
                    >
                      {syncCalendar.isPending ? 'Syncing...' : 'Sync Now'}
                    </button>
                    <button
                      type="button"
                      onClick={() => handleDelete(connection.id)}
                      disabled={deleteConnection.isPending}
                      className="px-3 py-1 text-sm text-red-600 border border-red-200 rounded-md hover:bg-red-50 disabled:opacity-50"
                    >
                      Disconnect
                    </button>
                  </div>
                </div>
              );
            })}
          </div>
        )}
      </div>

      <CalendarConnectDialog
        organizationId={organizationId}
        isOpen={showConnectDialog}
        onClose={() => setShowConnectDialog(false)}
      />
    </>
  );
}

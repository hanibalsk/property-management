/**
 * Video Connections List Component
 *
 * Displays and manages video conference provider connections (Story 61.4).
 */

import type { VideoConferenceConnection } from '@ppt/api-client';
import { useDeleteVideoConnection, useVideoConnections } from '@ppt/api-client';

interface VideoConnectionsListProps {
  organizationId: string;
  onConnect?: () => void;
}

const providerLabels: Record<string, string> = {
  zoom: 'Zoom',
  teams: 'Microsoft Teams',
  google_meet: 'Google Meet',
  webex: 'Cisco Webex',
};

const providerColors: Record<string, string> = {
  zoom: 'bg-blue-100 text-blue-800',
  teams: 'bg-purple-100 text-purple-800',
  google_meet: 'bg-green-100 text-green-800',
  webex: 'bg-cyan-100 text-cyan-800',
};

export function VideoConnectionsList({ organizationId, onConnect }: VideoConnectionsListProps) {
  const { data: connections, isLoading } = useVideoConnections(organizationId);
  const deleteConnection = useDeleteVideoConnection(organizationId);

  const handleDelete = async (id: string) => {
    if (confirm('Are you sure you want to disconnect this video provider?')) {
      await deleteConnection.mutateAsync(id);
    }
  };

  if (isLoading) {
    return (
      <div className="rounded-lg border bg-card p-6">
        <h3 className="text-lg font-semibold">Video Connections</h3>
        <p className="text-muted-foreground">Loading...</p>
      </div>
    );
  }

  return (
    <div className="rounded-lg border bg-card p-6">
      <div className="flex items-center justify-between mb-4">
        <div>
          <h3 className="text-lg font-semibold">Connected Providers</h3>
          <p className="text-sm text-muted-foreground">Your video conferencing integrations</p>
        </div>
        <button
          type="button"
          onClick={onConnect}
          className="inline-flex items-center px-4 py-2 text-sm font-medium bg-primary text-primary-foreground rounded-md hover:bg-primary/90"
        >
          + Connect Provider
        </button>
      </div>

      {connections?.length === 0 ? (
        <div className="flex flex-col items-center justify-center py-8 text-center">
          <div className="text-4xl mb-4">vid</div>
          <p className="text-muted-foreground">No video providers connected</p>
          <p className="text-sm text-muted-foreground">
            Connect Zoom, Teams, or Meet to schedule video meetings
          </p>
        </div>
      ) : (
        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
          {connections?.map((connection: VideoConferenceConnection) => (
            <div key={connection.id} className="rounded-lg border p-4">
              <div className="flex items-center justify-between mb-3">
                <span
                  className={`inline-flex items-center px-2 py-0.5 rounded text-xs font-medium ${providerColors[connection.provider] || 'bg-gray-100'}`}
                >
                  {providerLabels[connection.provider] || connection.provider}
                </span>
                {connection.isActive && (
                  <span className="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-green-100 text-green-800">
                    Active
                  </span>
                )}
              </div>
              <div className="text-sm text-muted-foreground mb-3">
                Connected {new Date(connection.createdAt).toLocaleDateString()}
              </div>
              <button
                type="button"
                onClick={() => handleDelete(connection.id)}
                className="w-full px-3 py-1 text-sm text-red-600 border border-red-200 rounded-md hover:bg-red-50"
              >
                Disconnect
              </button>
            </div>
          ))}
        </div>
      )}

      <div className="mt-6 pt-6 border-t">
        <h4 className="text-sm font-medium mb-3">Available Providers</h4>
        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
          <ProviderCard
            name="Zoom"
            description="Video conferencing platform"
            connected={connections?.some((c) => c.provider === 'zoom') ?? false}
          />
          <ProviderCard
            name="Microsoft Teams"
            description="Collaboration and video calls"
            connected={connections?.some((c) => c.provider === 'teams') ?? false}
          />
          <ProviderCard
            name="Google Meet"
            description="Video meetings by Google"
            connected={connections?.some((c) => c.provider === 'google_meet') ?? false}
          />
          <ProviderCard
            name="Cisco Webex"
            description="Enterprise video conferencing"
            connected={connections?.some((c) => c.provider === 'webex') ?? false}
          />
        </div>
      </div>
    </div>
  );
}

function ProviderCard({
  name,
  description,
  connected,
}: {
  name: string;
  description: string;
  connected: boolean;
}) {
  return (
    <div
      className={`rounded-lg border p-4 ${connected ? 'opacity-50' : 'hover:border-primary cursor-pointer'} transition-colors`}
    >
      <div className="flex items-center gap-3 mb-2">
        <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-muted">
          <span className="font-bold text-sm">vid</span>
        </div>
        <div>
          <div className="font-medium">{name}</div>
          {connected && <div className="text-xs text-green-600">Connected</div>}
        </div>
      </div>
      <p className="text-sm text-muted-foreground">{description}</p>
    </div>
  );
}

/**
 * Video Meetings List Component
 *
 * Displays and manages video meetings (Story 61.4).
 */

import type { VideoMeeting } from '@ppt/api-client';
import { useDeleteVideoMeeting, useStartVideoMeeting, useVideoMeetings } from '@ppt/api-client';

interface VideoMeetingsListProps {
  organizationId: string;
  onCreateMeeting?: () => void;
}

const statusColors: Record<string, string> = {
  scheduled: 'bg-blue-100 text-blue-800',
  started: 'bg-green-100 text-green-800',
  ended: 'bg-gray-100 text-gray-800',
  cancelled: 'bg-red-100 text-red-800',
};

export function VideoMeetingsList({ organizationId, onCreateMeeting }: VideoMeetingsListProps) {
  const { data: meetings, isLoading } = useVideoMeetings(organizationId);
  const deleteMeeting = useDeleteVideoMeeting(organizationId);
  const startMeeting = useStartVideoMeeting(organizationId);

  const handleDelete = async (id: string) => {
    if (confirm('Are you sure you want to cancel this meeting?')) {
      await deleteMeeting.mutateAsync(id);
    }
  };

  const handleStart = async (id: string) => {
    await startMeeting.mutateAsync(id);
  };

  const formatDateTime = (dateStr: string) => {
    const date = new Date(dateStr);
    return {
      date: date.toLocaleDateString(),
      time: date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
    };
  };

  if (isLoading) {
    return (
      <div className="rounded-lg border bg-card p-6">
        <h3 className="text-lg font-semibold">Video Meetings</h3>
        <p className="text-muted-foreground">Loading...</p>
      </div>
    );
  }

  return (
    <div className="rounded-lg border bg-card p-6">
      <div className="flex items-center justify-between mb-4">
        <div>
          <h3 className="text-lg font-semibold">Video Meetings</h3>
          <p className="text-sm text-muted-foreground">
            Schedule and manage video conferences with Zoom, Teams, and more
          </p>
        </div>
        <button
          type="button"
          onClick={onCreateMeeting}
          className="inline-flex items-center px-4 py-2 text-sm font-medium bg-primary text-primary-foreground rounded-md hover:bg-primary/90"
        >
          + Schedule Meeting
        </button>
      </div>

      {meetings?.length === 0 ? (
        <div className="flex flex-col items-center justify-center py-8 text-center">
          <div className="text-4xl mb-4">video</div>
          <p className="text-muted-foreground">No meetings scheduled</p>
          <p className="text-sm text-muted-foreground">
            Schedule a video meeting to collaborate with your team
          </p>
        </div>
      ) : (
        <div className="space-y-4">
          {meetings?.map((meeting: VideoMeeting) => {
            const { date, time } = formatDateTime(meeting.startTime);
            return (
              <div
                key={meeting.id}
                className="flex items-center justify-between rounded-lg border p-4"
              >
                <div className="flex items-center gap-4">
                  <div
                    className={`flex h-10 w-10 items-center justify-center rounded-full ${
                      meeting.status === 'started' ? 'bg-green-100' : 'bg-muted'
                    }`}
                  >
                    vid
                  </div>
                  <div>
                    <div className="flex items-center gap-2">
                      <span className="font-medium">{meeting.title}</span>
                      <span
                        className={`inline-flex items-center px-2 py-0.5 rounded text-xs font-medium ${statusColors[meeting.status]}`}
                      >
                        {meeting.status}
                      </span>
                    </div>
                    <div className="flex items-center gap-4 text-sm text-muted-foreground">
                      <span>{date}</span>
                      <span>
                        {time} ({meeting.durationMinutes} min)
                      </span>
                    </div>
                  </div>
                </div>
                <div className="flex items-center gap-2">
                  {meeting.status === 'scheduled' && meeting.joinUrl && (
                    <a
                      href={meeting.joinUrl}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="px-3 py-1 text-sm bg-primary text-primary-foreground rounded-md hover:bg-primary/90"
                    >
                      Join
                    </a>
                  )}
                  {meeting.status === 'started' && meeting.joinUrl && (
                    <a
                      href={meeting.joinUrl}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="px-3 py-1 text-sm bg-green-600 text-white rounded-md hover:bg-green-700"
                    >
                      Join Now
                    </a>
                  )}
                  {meeting.status === 'scheduled' && (
                    <button
                      type="button"
                      onClick={() => handleStart(meeting.id)}
                      className="px-3 py-1 text-sm border rounded-md hover:bg-muted"
                    >
                      Start
                    </button>
                  )}
                  <button
                    type="button"
                    onClick={() => handleDelete(meeting.id)}
                    className="px-3 py-1 text-sm text-red-600 border border-red-200 rounded-md hover:bg-red-50"
                  >
                    Cancel
                  </button>
                </div>
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
}

/**
 * MediationSessionCard - displays a mediation session card.
 * Epic 77: Dispute Resolution (Story 77.2)
 */

export type SessionType = 'in_person' | 'video_call' | 'phone' | 'written';
export type SessionStatus = 'scheduled' | 'in_progress' | 'completed' | 'cancelled' | 'rescheduled';

export interface MediationSession {
  id: string;
  disputeId: string;
  mediatorId: string;
  mediatorName: string;
  sessionType: SessionType;
  scheduledAt: string;
  durationMinutes?: number;
  location?: string;
  meetingUrl?: string;
  status: SessionStatus;
  notes?: string;
  outcome?: string;
}

export interface SessionAttendee {
  partyId: string;
  partyName: string;
  role: string;
  confirmed: boolean;
  attended?: boolean;
}

const sessionTypeLabels: Record<SessionType, string> = {
  in_person: 'In Person',
  video_call: 'Video Call',
  phone: 'Phone',
  written: 'Written Exchange',
};

const sessionTypeIcons: Record<SessionType, string> = {
  in_person: 'building-office',
  video_call: 'video-camera',
  phone: 'phone',
  written: 'document-text',
};

const statusColors: Record<SessionStatus, string> = {
  scheduled: 'bg-blue-100 text-blue-800',
  in_progress: 'bg-yellow-100 text-yellow-800',
  completed: 'bg-green-100 text-green-800',
  cancelled: 'bg-red-100 text-red-800',
  rescheduled: 'bg-orange-100 text-orange-800',
};

const statusLabels: Record<SessionStatus, string> = {
  scheduled: 'Scheduled',
  in_progress: 'In Progress',
  completed: 'Completed',
  cancelled: 'Cancelled',
  rescheduled: 'Rescheduled',
};

interface MediationSessionCardProps {
  session: MediationSession;
  attendees?: SessionAttendee[];
  isMediator?: boolean;
  onView?: (id: string) => void;
  onJoin?: (id: string) => void;
  onCancel?: (id: string) => void;
  onComplete?: (id: string) => void;
  onConfirmAttendance?: (sessionId: string, partyId: string) => void;
}

export function MediationSessionCard({
  session,
  attendees,
  isMediator,
  onView,
  onJoin,
  onCancel,
  onComplete,
  onConfirmAttendance,
}: MediationSessionCardProps) {
  const scheduledDate = new Date(session.scheduledAt);
  const isUpcoming = scheduledDate > new Date() && session.status === 'scheduled';
  const isPast = scheduledDate < new Date();
  const canJoin =
    session.status === 'scheduled' &&
    session.sessionType === 'video_call' &&
    session.meetingUrl;

  const formatDateTime = (dateString: string) => {
    const date = new Date(dateString);
    return date.toLocaleString(undefined, {
      weekday: 'short',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  return (
    <div className="bg-white rounded-lg shadow border border-gray-200 p-4">
      {/* Header */}
      <div className="flex items-start justify-between mb-3">
        <div>
          <div className="flex items-center gap-2">
            <span className="text-lg font-medium">
              {sessionTypeLabels[session.sessionType]}
            </span>
            <span className={`px-2 py-0.5 text-xs font-medium rounded ${statusColors[session.status]}`}>
              {statusLabels[session.status]}
            </span>
          </div>
          <p className="text-sm text-gray-500 mt-1">
            Mediator: {session.mediatorName}
          </p>
        </div>
        <div className="text-right">
          <p className="text-sm font-medium">{formatDateTime(session.scheduledAt)}</p>
          {session.durationMinutes && (
            <p className="text-sm text-gray-500">{session.durationMinutes} minutes</p>
          )}
        </div>
      </div>

      {/* Location/Meeting Info */}
      {(session.location || session.meetingUrl) && (
        <div className="mb-3 p-2 bg-gray-50 rounded text-sm">
          {session.location && (
            <p className="text-gray-700">
              <span className="font-medium">Location:</span> {session.location}
            </p>
          )}
          {session.meetingUrl && (
            <p className="text-gray-700">
              <span className="font-medium">Meeting Link:</span>{' '}
              <a
                href={session.meetingUrl}
                target="_blank"
                rel="noopener noreferrer"
                className="text-blue-600 hover:text-blue-800"
              >
                Join Meeting
              </a>
            </p>
          )}
        </div>
      )}

      {/* Attendees */}
      {attendees && attendees.length > 0 && (
        <div className="mb-3">
          <p className="text-sm font-medium text-gray-700 mb-2">Attendees:</p>
          <div className="flex flex-wrap gap-2">
            {attendees.map((attendee) => (
              <div
                key={attendee.partyId}
                className={`px-2 py-1 text-sm rounded-full ${
                  attendee.confirmed
                    ? 'bg-green-100 text-green-800'
                    : 'bg-gray-100 text-gray-600'
                }`}
              >
                {attendee.partyName}
                {attendee.attended !== undefined && (
                  <span className="ml-1">
                    {attendee.attended ? ' (attended)' : ' (absent)'}
                  </span>
                )}
                {!attendee.confirmed && isUpcoming && onConfirmAttendance && (
                  <button
                    type="button"
                    onClick={() => onConfirmAttendance(session.id, attendee.partyId)}
                    className="ml-1 text-blue-600 hover:text-blue-800"
                  >
                    Confirm
                  </button>
                )}
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Outcome (if completed) */}
      {session.status === 'completed' && session.outcome && (
        <div className="mb-3 p-2 bg-green-50 border border-green-100 rounded">
          <p className="text-sm font-medium text-green-800">Outcome:</p>
          <p className="text-sm text-green-700">{session.outcome}</p>
        </div>
      )}

      {/* Notes */}
      {session.notes && (
        <div className="mb-3 p-2 bg-gray-50 rounded">
          <p className="text-sm font-medium text-gray-700">Notes:</p>
          <p className="text-sm text-gray-600 whitespace-pre-wrap">{session.notes}</p>
        </div>
      )}

      {/* Actions */}
      <div className="flex justify-end gap-2 pt-3 border-t">
        {onView && (
          <button
            type="button"
            onClick={() => onView(session.id)}
            className="px-3 py-1.5 text-sm border border-gray-300 rounded-lg hover:bg-gray-50"
          >
            View Details
          </button>
        )}
        {canJoin && onJoin && (
          <button
            type="button"
            onClick={() => onJoin(session.id)}
            className="px-3 py-1.5 text-sm bg-blue-600 text-white rounded-lg hover:bg-blue-700"
          >
            Join Session
          </button>
        )}
        {isMediator && session.status === 'scheduled' && onCancel && (
          <button
            type="button"
            onClick={() => onCancel(session.id)}
            className="px-3 py-1.5 text-sm border border-red-300 text-red-600 rounded-lg hover:bg-red-50"
          >
            Cancel
          </button>
        )}
        {isMediator && session.status === 'in_progress' && onComplete && (
          <button
            type="button"
            onClick={() => onComplete(session.id)}
            className="px-3 py-1.5 text-sm bg-green-600 text-white rounded-lg hover:bg-green-700"
          >
            Complete
          </button>
        )}
      </div>
    </div>
  );
}

export { sessionTypeLabels, statusLabels };

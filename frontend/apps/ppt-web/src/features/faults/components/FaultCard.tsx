/**
 * FaultCard component - displays a fault summary card in lists.
 * Epic 4: Fault Reporting & Resolution (UC-03)
 */

export type FaultStatus =
  | 'new'
  | 'triaged'
  | 'in_progress'
  | 'waiting_parts'
  | 'scheduled'
  | 'resolved'
  | 'closed'
  | 'reopened';

export type FaultPriority = 'low' | 'medium' | 'high' | 'urgent';

export type FaultCategory =
  | 'plumbing'
  | 'electrical'
  | 'heating'
  | 'structural'
  | 'exterior'
  | 'elevator'
  | 'common_area'
  | 'security'
  | 'cleaning'
  | 'other';

export interface FaultSummary {
  id: string;
  buildingId: string;
  unitId?: string;
  title: string;
  category: FaultCategory;
  priority: FaultPriority;
  status: FaultStatus;
  createdAt: string;
}

interface FaultCardProps {
  fault: FaultSummary;
  onView?: (id: string) => void;
  onEdit?: (id: string) => void;
  onTriage?: (id: string) => void;
}

const statusColors: Record<FaultStatus, string> = {
  new: 'bg-red-100 text-red-800',
  triaged: 'bg-blue-100 text-blue-800',
  in_progress: 'bg-yellow-100 text-yellow-800',
  waiting_parts: 'bg-orange-100 text-orange-800',
  scheduled: 'bg-purple-100 text-purple-800',
  resolved: 'bg-green-100 text-green-800',
  closed: 'bg-gray-100 text-gray-800',
  reopened: 'bg-red-100 text-red-800',
};

const priorityColors: Record<FaultPriority, string> = {
  low: 'text-gray-500',
  medium: 'text-blue-500',
  high: 'text-orange-500',
  urgent: 'text-red-600 font-bold',
};

const categoryLabels: Record<FaultCategory, string> = {
  plumbing: 'Plumbing',
  electrical: 'Electrical',
  heating: 'Heating',
  structural: 'Structural',
  exterior: 'Exterior',
  elevator: 'Elevator',
  common_area: 'Common Area',
  security: 'Security',
  cleaning: 'Cleaning',
  other: 'Other',
};

const statusLabels: Record<FaultStatus, string> = {
  new: 'New',
  triaged: 'Triaged',
  in_progress: 'In Progress',
  waiting_parts: 'Waiting for Parts',
  scheduled: 'Scheduled',
  resolved: 'Resolved',
  closed: 'Closed',
  reopened: 'Reopened',
};

const priorityLabels: Record<FaultPriority, string> = {
  low: 'Low',
  medium: 'Medium',
  high: 'High',
  urgent: 'Urgent',
};

export function FaultCard({ fault, onView, onEdit, onTriage }: FaultCardProps) {
  const canEdit = fault.status === 'new';
  const canTriage = fault.status === 'new';

  return (
    <div className="bg-white rounded-lg shadow p-4 hover:shadow-md transition-shadow">
      <div className="flex items-start justify-between">
        <div className="flex-1">
          <div className="flex items-center gap-2">
            {fault.priority === 'urgent' && (
              <span className="text-red-500" title="Urgent">
                <svg
                  className="w-4 h-4"
                  fill="currentColor"
                  viewBox="0 0 20 20"
                  aria-label="Urgent"
                >
                  <title>Urgent</title>
                  <path
                    fillRule="evenodd"
                    d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
                    clipRule="evenodd"
                  />
                </svg>
              </span>
            )}
            <h3 className="text-lg font-semibold text-gray-900">{fault.title}</h3>
          </div>
          <div className="mt-2 flex items-center gap-2 flex-wrap">
            <span className={`px-2 py-1 text-xs font-medium rounded ${statusColors[fault.status]}`}>
              {statusLabels[fault.status]}
            </span>
            <span className={`text-xs font-medium ${priorityColors[fault.priority]}`}>
              {priorityLabels[fault.priority]}
            </span>
            <span className="text-xs text-gray-500">{categoryLabels[fault.category]}</span>
          </div>
          <p className="mt-1 text-xs text-gray-400">
            Reported: {new Date(fault.createdAt).toLocaleDateString()}
          </p>
        </div>
      </div>

      <div className="mt-4 flex items-center gap-2 border-t pt-3">
        <button
          type="button"
          onClick={() => onView?.(fault.id)}
          className="text-sm text-blue-600 hover:text-blue-800"
        >
          View
        </button>
        {canEdit && (
          <button
            type="button"
            onClick={() => onEdit?.(fault.id)}
            className="text-sm text-gray-600 hover:text-gray-800"
          >
            Edit
          </button>
        )}
        {canTriage && (
          <button
            type="button"
            onClick={() => onTriage?.(fault.id)}
            className="text-sm text-green-600 hover:text-green-800"
          >
            Triage
          </button>
        )}
      </div>
    </div>
  );
}

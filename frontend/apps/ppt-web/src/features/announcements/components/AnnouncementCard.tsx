import type { AnnouncementSummary, AnnouncementStatus } from '@ppt/api-client';

interface AnnouncementCardProps {
  announcement: AnnouncementSummary;
  onView?: (id: string) => void;
  onEdit?: (id: string) => void;
  onDelete?: (id: string) => void;
  onPublish?: (id: string) => void;
  onArchive?: (id: string) => void;
  onPin?: (id: string, pinned: boolean) => void;
}

const statusColors: Record<AnnouncementStatus, string> = {
  draft: 'bg-gray-100 text-gray-800',
  scheduled: 'bg-blue-100 text-blue-800',
  published: 'bg-green-100 text-green-800',
  archived: 'bg-yellow-100 text-yellow-800',
};

const targetTypeLabels = {
  all: 'All Users',
  building: 'Building',
  units: 'Specific Units',
  roles: 'By Role',
};

export function AnnouncementCard({
  announcement,
  onView,
  onEdit,
  onDelete,
  onPublish,
  onArchive,
  onPin,
}: AnnouncementCardProps) {
  const canEdit = announcement.status === 'draft' || announcement.status === 'scheduled';
  const canDelete = announcement.status === 'draft';
  const canPublish = announcement.status === 'draft' || announcement.status === 'scheduled';
  const canArchive = announcement.status === 'published';

  return (
    <div className="bg-white rounded-lg shadow p-4 hover:shadow-md transition-shadow">
      <div className="flex items-start justify-between">
        <div className="flex-1">
          <div className="flex items-center gap-2">
            {announcement.pinned && (
              <span className="text-amber-500" title="Pinned">
                <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20" aria-label="Pinned">
                  <title>Pinned</title>
                  <path d="M9.828.722a.5.5 0 01.354 0l7 3A.5.5 0 0117.5 4v1.5a.5.5 0 01-.5.5h-1v4.5a.5.5 0 01-.5.5H13v5.5a.5.5 0 01-.5.5h-5a.5.5 0 01-.5-.5V11H4.5a.5.5 0 01-.5-.5V6h-1a.5.5 0 01-.5-.5V4a.5.5 0 01.328-.472l7-3z" />
                </svg>
              </span>
            )}
            <h3 className="text-lg font-semibold text-gray-900">{announcement.title}</h3>
          </div>
          <div className="mt-2 flex items-center gap-2 flex-wrap">
            <span className={`px-2 py-1 text-xs font-medium rounded ${statusColors[announcement.status]}`}>
              {announcement.status.charAt(0).toUpperCase() + announcement.status.slice(1)}
            </span>
            <span className="text-xs text-gray-500">
              {targetTypeLabels[announcement.targetType]}
            </span>
            {announcement.acknowledgmentRequired && (
              <span className="text-xs text-orange-600 font-medium">Acknowledgment Required</span>
            )}
            {announcement.commentsEnabled && (
              <span className="text-xs text-blue-600">Comments Enabled</span>
            )}
          </div>
          {announcement.publishedAt && (
            <p className="mt-1 text-xs text-gray-400">
              Published: {new Date(announcement.publishedAt).toLocaleDateString()}
            </p>
          )}
        </div>
      </div>

      <div className="mt-4 flex items-center gap-2 border-t pt-3">
        <button
          type="button"
          onClick={() => onView?.(announcement.id)}
          className="text-sm text-blue-600 hover:text-blue-800"
        >
          View
        </button>
        {canEdit && (
          <button
            type="button"
            onClick={() => onEdit?.(announcement.id)}
            className="text-sm text-gray-600 hover:text-gray-800"
          >
            Edit
          </button>
        )}
        {canPublish && (
          <button
            type="button"
            onClick={() => onPublish?.(announcement.id)}
            className="text-sm text-green-600 hover:text-green-800"
          >
            Publish
          </button>
        )}
        {canArchive && (
          <button
            type="button"
            onClick={() => onArchive?.(announcement.id)}
            className="text-sm text-yellow-600 hover:text-yellow-800"
          >
            Archive
          </button>
        )}
        <button
          type="button"
          onClick={() => onPin?.(announcement.id, !announcement.pinned)}
          className="text-sm text-amber-600 hover:text-amber-800"
        >
          {announcement.pinned ? 'Unpin' : 'Pin'}
        </button>
        {canDelete && (
          <button
            type="button"
            onClick={() => onDelete?.(announcement.id)}
            className="text-sm text-red-600 hover:text-red-800 ml-auto"
          >
            Delete
          </button>
        )}
      </div>
    </div>
  );
}

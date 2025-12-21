import type { AnnouncementWithDetails, AnnouncementAttachment, AnnouncementStatus } from '@ppt/api-client';

interface ViewAnnouncementPageProps {
  announcement: AnnouncementWithDetails;
  attachments: AnnouncementAttachment[];
  isLoading?: boolean;
  onEdit: () => void;
  onPublish: () => void;
  onArchive: () => void;
  onPin: (pinned: boolean) => void;
  onDelete: () => void;
  onBack: () => void;
  onMarkRead?: () => void;
  onAcknowledge?: () => void;
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

export function ViewAnnouncementPage({
  announcement,
  attachments,
  isLoading,
  onEdit,
  onPublish,
  onArchive,
  onPin,
  onDelete,
  onBack,
  onMarkRead,
  onAcknowledge,
}: ViewAnnouncementPageProps) {
  const canEdit = announcement.status === 'draft' || announcement.status === 'scheduled';
  const canDelete = announcement.status === 'draft';
  const canPublish = announcement.status === 'draft' || announcement.status === 'scheduled';
  const canArchive = announcement.status === 'published';

  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-12">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600" />
      </div>
    );
  }

  return (
    <div className="max-w-3xl mx-auto px-4 py-8">
      {/* Back button */}
      <button
        type="button"
        onClick={onBack}
        className="mb-4 text-blue-600 hover:text-blue-800 flex items-center gap-1"
      >
        <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
        </svg>
        Back to Announcements
      </button>

      <div className="bg-white rounded-lg shadow">
        {/* Header */}
        <div className="p-6 border-b">
          <div className="flex items-start justify-between">
            <div className="flex-1">
              <div className="flex items-center gap-2 mb-2">
                {announcement.pinned && (
                  <span className="text-amber-500" title="Pinned">
                    <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                      <path d="M9.828.722a.5.5 0 01.354 0l7 3A.5.5 0 0117.5 4v1.5a.5.5 0 01-.5.5h-1v4.5a.5.5 0 01-.5.5H13v5.5a.5.5 0 01-.5.5h-5a.5.5 0 01-.5-.5V11H4.5a.5.5 0 01-.5-.5V6h-1a.5.5 0 01-.5-.5V4a.5.5 0 01.328-.472l7-3z" />
                    </svg>
                  </span>
                )}
                <span className={`px-2 py-1 text-xs font-medium rounded ${statusColors[announcement.status]}`}>
                  {announcement.status.charAt(0).toUpperCase() + announcement.status.slice(1)}
                </span>
              </div>
              <h1 className="text-2xl font-bold text-gray-900">{announcement.title}</h1>
              <div className="mt-2 flex items-center gap-4 text-sm text-gray-500">
                <span>By {announcement.authorName}</span>
                <span>{targetTypeLabels[announcement.targetType]}</span>
                {announcement.publishedAt && (
                  <span>Published: {new Date(announcement.publishedAt).toLocaleDateString()}</span>
                )}
              </div>
            </div>
          </div>

          {/* Stats */}
          <div className="mt-4 flex items-center gap-6 text-sm">
            <span className="text-gray-600">
              <strong>{announcement.readCount}</strong> reads
            </span>
            {announcement.acknowledgmentRequired && (
              <span className="text-gray-600">
                <strong>{announcement.acknowledgedCount}</strong> acknowledged
              </span>
            )}
            {announcement.commentsEnabled && (
              <span className="text-gray-600">
                <strong>{announcement.commentCount}</strong> comments
              </span>
            )}
            <span className="text-gray-600">
              <strong>{announcement.attachmentCount}</strong> attachments
            </span>
          </div>
        </div>

        {/* Content */}
        <div className="p-6">
          <div className="prose prose-sm max-w-none">
            {/* In a real app, this would render markdown */}
            <div className="whitespace-pre-wrap">{announcement.content}</div>
          </div>
        </div>

        {/* Attachments */}
        {attachments.length > 0 && (
          <div className="p-6 border-t">
            <h3 className="text-lg font-semibold mb-3">Attachments</h3>
            <ul className="space-y-2">
              {attachments.map((attachment) => (
                <li key={attachment.id} className="flex items-center gap-2">
                  <svg className="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13" />
                  </svg>
                  <span className="text-blue-600 hover:underline cursor-pointer">
                    {attachment.fileName}
                  </span>
                  <span className="text-xs text-gray-400">
                    ({Math.round(attachment.fileSize / 1024)} KB)
                  </span>
                </li>
              ))}
            </ul>
          </div>
        )}

        {/* Actions */}
        <div className="p-6 border-t bg-gray-50 flex items-center gap-3 flex-wrap">
          {canEdit && (
            <button
              type="button"
              onClick={onEdit}
              className="px-4 py-2 border border-gray-300 rounded-md hover:bg-gray-100"
            >
              Edit
            </button>
          )}
          {canPublish && (
            <button
              type="button"
              onClick={onPublish}
              className="px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700"
            >
              Publish Now
            </button>
          )}
          {canArchive && (
            <button
              type="button"
              onClick={onArchive}
              className="px-4 py-2 bg-yellow-600 text-white rounded-md hover:bg-yellow-700"
            >
              Archive
            </button>
          )}
          <button
            type="button"
            onClick={() => onPin(!announcement.pinned)}
            className="px-4 py-2 border border-amber-300 text-amber-700 rounded-md hover:bg-amber-50"
          >
            {announcement.pinned ? 'Unpin' : 'Pin'}
          </button>
          {announcement.acknowledgmentRequired && onAcknowledge && (
            <button
              type="button"
              onClick={onAcknowledge}
              className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
            >
              Acknowledge
            </button>
          )}
          {onMarkRead && (
            <button
              type="button"
              onClick={onMarkRead}
              className="px-4 py-2 text-gray-600 hover:text-gray-800"
            >
              Mark as Read
            </button>
          )}
          {canDelete && (
            <button
              type="button"
              onClick={onDelete}
              className="px-4 py-2 text-red-600 hover:text-red-800 ml-auto"
            >
              Delete
            </button>
          )}
        </div>
      </div>
    </div>
  );
}

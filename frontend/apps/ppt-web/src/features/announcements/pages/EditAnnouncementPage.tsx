import type { Announcement, UpdateAnnouncementRequest } from '@ppt/api-client';
import { AnnouncementForm } from '../components/AnnouncementForm';

interface EditAnnouncementPageProps {
  announcement: Announcement;
  buildings?: { id: string; name: string }[];
  units?: { id: string; name: string; buildingName: string }[];
  roles?: { id: string; name: string }[];
  onSubmit: (data: UpdateAnnouncementRequest) => void;
  onCancel: () => void;
  isLoading?: boolean;
}

export function EditAnnouncementPage({
  announcement,
  buildings = [],
  units = [],
  roles = [],
  onSubmit,
  onCancel,
  isLoading,
}: EditAnnouncementPageProps) {
  const canEdit = announcement.status === 'draft' || announcement.status === 'scheduled';

  if (!canEdit) {
    return (
      <div className="max-w-2xl mx-auto px-4 py-8">
        <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
          <h2 className="text-lg font-semibold text-yellow-800">Cannot Edit</h2>
          <p className="text-yellow-700">
            Only draft and scheduled announcements can be edited. This announcement has been{' '}
            {announcement.status}.
          </p>
          <button
            type="button"
            onClick={onCancel}
            className="mt-4 px-4 py-2 bg-yellow-600 text-white rounded-md hover:bg-yellow-700"
          >
            Go Back
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="max-w-2xl mx-auto px-4 py-8">
      <div className="mb-6">
        <h1 className="text-2xl font-bold text-gray-900">Edit Announcement</h1>
        <p className="mt-1 text-gray-600">
          Update your announcement. Changes will be saved as a draft unless you publish.
        </p>
      </div>

      <div className="bg-white rounded-lg shadow p-6">
        <AnnouncementForm
          announcement={announcement}
          buildings={buildings}
          units={units}
          roles={roles}
          onSubmit={(data) => onSubmit(data as UpdateAnnouncementRequest)}
          onCancel={onCancel}
          isLoading={isLoading}
        />
      </div>
    </div>
  );
}

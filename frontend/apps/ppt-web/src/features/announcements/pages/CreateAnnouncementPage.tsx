import type { CreateAnnouncementRequest } from '@ppt/api-client';
import { AnnouncementForm } from '../components/AnnouncementForm';

interface CreateAnnouncementPageProps {
  buildings?: { id: string; name: string }[];
  units?: { id: string; name: string; buildingName: string }[];
  roles?: { id: string; name: string }[];
  onSubmit: (data: CreateAnnouncementRequest) => void;
  onCancel: () => void;
  isLoading?: boolean;
}

export function CreateAnnouncementPage({
  buildings = [],
  units = [],
  roles = [],
  onSubmit,
  onCancel,
  isLoading,
}: CreateAnnouncementPageProps) {
  return (
    <div className="max-w-2xl mx-auto px-4 py-8">
      <div className="mb-6">
        <h1 className="text-2xl font-bold text-gray-900">Create Announcement</h1>
        <p className="mt-1 text-gray-600">Create a new announcement for your organization.</p>
      </div>

      <div className="bg-white rounded-lg shadow p-6">
        <AnnouncementForm
          buildings={buildings}
          units={units}
          roles={roles}
          onSubmit={(data) => onSubmit(data as CreateAnnouncementRequest)}
          onCancel={onCancel}
          isLoading={isLoading}
        />
      </div>
    </div>
  );
}

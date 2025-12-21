import type {
  Announcement,
  AnnouncementTargetType,
  CreateAnnouncementRequest,
  UpdateAnnouncementRequest,
} from '@ppt/api-client';
import { useEffect, useState } from 'react';
import { SchedulePicker } from './SchedulePicker';
import { TargetSelector } from './TargetSelector';

interface AnnouncementFormProps {
  announcement?: Announcement;
  buildings?: { id: string; name: string }[];
  units?: { id: string; name: string; buildingName: string }[];
  roles?: { id: string; name: string }[];
  onSubmit: (data: CreateAnnouncementRequest | UpdateAnnouncementRequest) => void;
  onCancel: () => void;
  isLoading?: boolean;
}

export function AnnouncementForm({
  announcement,
  buildings = [],
  units = [],
  roles = [],
  onSubmit,
  onCancel,
  isLoading,
}: AnnouncementFormProps) {
  const [title, setTitle] = useState(announcement?.title ?? '');
  const [content, setContent] = useState(announcement?.content ?? '');
  const [targetType, setTargetType] = useState<AnnouncementTargetType>(
    announcement?.targetType ?? 'all'
  );
  const [targetIds, setTargetIds] = useState<string[]>((announcement?.targetIds as string[]) ?? []);
  const [scheduledAt, setScheduledAt] = useState<string | undefined>(
    announcement?.scheduledAt ?? undefined
  );
  const [commentsEnabled, setCommentsEnabled] = useState(announcement?.commentsEnabled ?? false);
  const [acknowledgmentRequired, setAcknowledgmentRequired] = useState(
    announcement?.acknowledgmentRequired ?? false
  );
  const [errors, setErrors] = useState<Record<string, string>>({});

  useEffect(() => {
    if (announcement) {
      setTitle(announcement.title);
      setContent(announcement.content);
      setTargetType(announcement.targetType);
      setTargetIds((announcement.targetIds as string[]) ?? []);
      setScheduledAt(announcement.scheduledAt);
      setCommentsEnabled(announcement.commentsEnabled);
      setAcknowledgmentRequired(announcement.acknowledgmentRequired);
    }
  }, [announcement]);

  const validate = (): boolean => {
    const newErrors: Record<string, string> = {};

    if (!title.trim()) {
      newErrors.title = 'Title is required';
    } else if (title.length > 255) {
      newErrors.title = 'Title must be 255 characters or less';
    }

    if (!content.trim()) {
      newErrors.content = 'Content is required';
    }

    if (targetType !== 'all' && targetIds.length === 0) {
      newErrors.targetIds = 'Please select at least one target';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!validate()) return;

    const data: CreateAnnouncementRequest | UpdateAnnouncementRequest = {
      title: title.trim(),
      content: content.trim(),
      targetType,
      targetIds: targetType === 'all' ? undefined : targetIds,
      scheduledAt: scheduledAt || undefined,
      commentsEnabled,
      acknowledgmentRequired,
    };

    onSubmit(data);
  };

  const isEditing = !!announcement;

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      {/* Title */}
      <div>
        <label htmlFor="title" className="block text-sm font-medium text-gray-700 mb-2">
          Title *
        </label>
        <input
          id="title"
          type="text"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          placeholder="Enter announcement title"
          className={`w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 ${
            errors.title ? 'border-red-500' : 'border-gray-300'
          }`}
          maxLength={255}
        />
        {errors.title && <p className="mt-1 text-sm text-red-500">{errors.title}</p>}
        <p className="mt-1 text-xs text-gray-500">{title.length}/255 characters</p>
      </div>

      {/* Content */}
      <div>
        <label htmlFor="content" className="block text-sm font-medium text-gray-700 mb-2">
          Content * (Markdown supported)
        </label>
        <textarea
          id="content"
          value={content}
          onChange={(e) => setContent(e.target.value)}
          placeholder="Write your announcement content here. Markdown formatting is supported."
          rows={8}
          className={`w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 font-mono text-sm ${
            errors.content ? 'border-red-500' : 'border-gray-300'
          }`}
        />
        {errors.content && <p className="mt-1 text-sm text-red-500">{errors.content}</p>}
      </div>

      {/* Target Selector */}
      <div>
        <TargetSelector
          targetType={targetType}
          targetIds={targetIds}
          onTargetTypeChange={setTargetType}
          onTargetIdsChange={setTargetIds}
          buildings={buildings}
          units={units}
          roles={roles}
        />
        {errors.targetIds && <p className="mt-1 text-sm text-red-500">{errors.targetIds}</p>}
      </div>

      {/* Schedule */}
      <SchedulePicker scheduledAt={scheduledAt} onScheduleChange={setScheduledAt} />

      {/* Options */}
      <div className="space-y-3">
        <label className="flex items-center">
          <input
            type="checkbox"
            checked={commentsEnabled}
            onChange={(e) => setCommentsEnabled(e.target.checked)}
            className="mr-2"
          />
          <span className="text-sm text-gray-700">Enable comments</span>
        </label>
        <label className="flex items-center">
          <input
            type="checkbox"
            checked={acknowledgmentRequired}
            onChange={(e) => setAcknowledgmentRequired(e.target.checked)}
            className="mr-2"
          />
          <span className="text-sm text-gray-700">Require acknowledgment</span>
        </label>
      </div>

      {/* Actions */}
      <div className="flex items-center justify-end gap-3 pt-4 border-t">
        <button
          type="button"
          onClick={onCancel}
          disabled={isLoading}
          className="px-4 py-2 text-gray-700 hover:text-gray-900 disabled:opacity-50"
        >
          Cancel
        </button>
        <button
          type="submit"
          disabled={isLoading}
          className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 transition-colors"
        >
          {isLoading ? 'Saving...' : isEditing ? 'Update Announcement' : 'Create Announcement'}
        </button>
      </div>
    </form>
  );
}

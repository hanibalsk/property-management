/**
 * EventForm Component
 *
 * Form for creating and editing community events.
 * Part of Story 42.3: Community Events.
 */

import type { CreateEventRequest, UpdateEventRequest } from '@ppt/api-client';
import { useState } from 'react';

interface EventFormProps {
  groupId: string;
  initialData?: Partial<CreateEventRequest>;
  isEditing?: boolean;
  isSubmitting?: boolean;
  onSubmit: (data: CreateEventRequest | UpdateEventRequest) => void;
  onCancel: () => void;
}

export function EventForm({
  groupId,
  initialData,
  isEditing = false,
  isSubmitting = false,
  onSubmit,
  onCancel,
}: EventFormProps) {
  const [title, setTitle] = useState(initialData?.title || '');
  const [description, setDescription] = useState(initialData?.description || '');
  const [location, setLocation] = useState(initialData?.location || '');
  const [locationDetails, setLocationDetails] = useState(initialData?.locationDetails || '');
  const [startDate, setStartDate] = useState(initialData?.startDate?.slice(0, 16) || '');
  const [endDate, setEndDate] = useState(initialData?.endDate?.slice(0, 16) || '');
  const [allDay, setAllDay] = useState(initialData?.allDay || false);
  const [coverImageUrl, setCoverImageUrl] = useState(initialData?.coverImageUrl || '');
  const [maxAttendees, setMaxAttendees] = useState(initialData?.maxAttendees?.toString() || '');
  const [errors, setErrors] = useState<Record<string, string>>({});

  const validate = (): boolean => {
    const newErrors: Record<string, string> = {};

    if (!title.trim()) {
      newErrors.title = 'Title is required';
    } else if (title.length < 3) {
      newErrors.title = 'Title must be at least 3 characters';
    }

    if (!description.trim()) {
      newErrors.description = 'Description is required';
    } else if (description.length < 10) {
      newErrors.description = 'Description must be at least 10 characters';
    }

    if (!location.trim()) {
      newErrors.location = 'Location is required';
    }

    if (!startDate) {
      newErrors.startDate = 'Start date is required';
    }

    if (!endDate) {
      newErrors.endDate = 'End date is required';
    } else if (new Date(endDate) <= new Date(startDate)) {
      newErrors.endDate = 'End date must be after start date';
    }

    if (coverImageUrl && !isValidUrl(coverImageUrl)) {
      newErrors.coverImageUrl = 'Please enter a valid URL';
    }

    if (maxAttendees && (Number.isNaN(Number(maxAttendees)) || Number(maxAttendees) < 1)) {
      newErrors.maxAttendees = 'Please enter a valid number';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const isValidUrl = (url: string): boolean => {
    try {
      new URL(url);
      return true;
    } catch {
      return false;
    }
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (validate()) {
      const data: CreateEventRequest = {
        groupId,
        title: title.trim(),
        description: description.trim(),
        location: location.trim(),
        locationDetails: locationDetails.trim() || undefined,
        startDate: new Date(startDate).toISOString(),
        endDate: new Date(endDate).toISOString(),
        allDay,
        coverImageUrl: coverImageUrl.trim() || undefined,
        maxAttendees: maxAttendees ? Number(maxAttendees) : undefined,
      };
      onSubmit(data);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      {/* Cover Image Preview */}
      <div className="relative h-40 bg-gradient-to-r from-purple-500 to-pink-500 rounded-lg overflow-hidden">
        {coverImageUrl && (
          <img
            src={coverImageUrl}
            alt="Cover preview"
            className="w-full h-full object-cover"
            onError={(e) => {
              (e.target as HTMLImageElement).style.display = 'none';
            }}
          />
        )}
        <div className="absolute inset-0 flex items-center justify-center bg-black bg-opacity-30">
          <span className="text-white text-lg font-medium">{title || 'Event Title'}</span>
        </div>
      </div>

      {/* Title */}
      <div>
        <label htmlFor="title" className="block text-sm font-medium text-gray-700">
          Event Title <span className="text-red-500">*</span>
        </label>
        <input
          type="text"
          id="title"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          placeholder="Enter event title"
          className={`mt-1 block w-full px-3 py-2 border rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 ${
            errors.title ? 'border-red-300' : 'border-gray-300'
          }`}
        />
        {errors.title && <p className="mt-1 text-sm text-red-600">{errors.title}</p>}
      </div>

      {/* Description */}
      <div>
        <label htmlFor="description" className="block text-sm font-medium text-gray-700">
          Description <span className="text-red-500">*</span>
        </label>
        <textarea
          id="description"
          value={description}
          onChange={(e) => setDescription(e.target.value)}
          placeholder="What's this event about?"
          rows={4}
          className={`mt-1 block w-full px-3 py-2 border rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 ${
            errors.description ? 'border-red-300' : 'border-gray-300'
          }`}
        />
        {errors.description && <p className="mt-1 text-sm text-red-600">{errors.description}</p>}
      </div>

      {/* Location */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div>
          <label htmlFor="location" className="block text-sm font-medium text-gray-700">
            Location <span className="text-red-500">*</span>
          </label>
          <input
            type="text"
            id="location"
            value={location}
            onChange={(e) => setLocation(e.target.value)}
            placeholder="e.g., Community Room"
            className={`mt-1 block w-full px-3 py-2 border rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 ${
              errors.location ? 'border-red-300' : 'border-gray-300'
            }`}
          />
          {errors.location && <p className="mt-1 text-sm text-red-600">{errors.location}</p>}
        </div>
        <div>
          <label htmlFor="locationDetails" className="block text-sm font-medium text-gray-700">
            Location Details (optional)
          </label>
          <input
            type="text"
            id="locationDetails"
            value={locationDetails}
            onChange={(e) => setLocationDetails(e.target.value)}
            placeholder="e.g., Building A, Floor 2"
            className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
          />
        </div>
      </div>

      {/* Date & Time */}
      <div>
        <div className="flex items-center justify-between mb-2">
          <span className="text-sm font-medium text-gray-700">Date & Time</span>
          <label className="flex items-center cursor-pointer">
            <input
              type="checkbox"
              checked={allDay}
              onChange={(e) => setAllDay(e.target.checked)}
              className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
            />
            <span className="ml-2 text-sm text-gray-600">All day event</span>
          </label>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label htmlFor="startDate" className="block text-sm text-gray-600">
              Start <span className="text-red-500">*</span>
            </label>
            <input
              type={allDay ? 'date' : 'datetime-local'}
              id="startDate"
              value={allDay ? startDate.slice(0, 10) : startDate}
              onChange={(e) => setStartDate(allDay ? e.target.value : e.target.value)}
              className={`mt-1 block w-full px-3 py-2 border rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 ${
                errors.startDate ? 'border-red-300' : 'border-gray-300'
              }`}
            />
            {errors.startDate && <p className="mt-1 text-sm text-red-600">{errors.startDate}</p>}
          </div>
          <div>
            <label htmlFor="endDate" className="block text-sm text-gray-600">
              End <span className="text-red-500">*</span>
            </label>
            <input
              type={allDay ? 'date' : 'datetime-local'}
              id="endDate"
              value={allDay ? endDate.slice(0, 10) : endDate}
              onChange={(e) => setEndDate(allDay ? e.target.value : e.target.value)}
              className={`mt-1 block w-full px-3 py-2 border rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 ${
                errors.endDate ? 'border-red-300' : 'border-gray-300'
              }`}
            />
            {errors.endDate && <p className="mt-1 text-sm text-red-600">{errors.endDate}</p>}
          </div>
        </div>
      </div>

      {/* Cover Image URL */}
      <div>
        <label htmlFor="coverImageUrl" className="block text-sm font-medium text-gray-700">
          Cover Image URL (optional)
        </label>
        <input
          type="url"
          id="coverImageUrl"
          value={coverImageUrl}
          onChange={(e) => setCoverImageUrl(e.target.value)}
          placeholder="https://example.com/image.jpg"
          className={`mt-1 block w-full px-3 py-2 border rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 ${
            errors.coverImageUrl ? 'border-red-300' : 'border-gray-300'
          }`}
        />
        {errors.coverImageUrl && (
          <p className="mt-1 text-sm text-red-600">{errors.coverImageUrl}</p>
        )}
      </div>

      {/* Max Attendees */}
      <div>
        <label htmlFor="maxAttendees" className="block text-sm font-medium text-gray-700">
          Maximum Attendees (optional)
        </label>
        <input
          type="number"
          id="maxAttendees"
          value={maxAttendees}
          onChange={(e) => setMaxAttendees(e.target.value)}
          placeholder="Leave empty for unlimited"
          min="1"
          className={`mt-1 block w-full px-3 py-2 border rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 ${
            errors.maxAttendees ? 'border-red-300' : 'border-gray-300'
          }`}
        />
        {errors.maxAttendees && <p className="mt-1 text-sm text-red-600">{errors.maxAttendees}</p>}
      </div>

      {/* Actions */}
      <div className="flex items-center justify-end gap-3 pt-4 border-t">
        <button
          type="button"
          onClick={onCancel}
          disabled={isSubmitting}
          className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 disabled:opacity-50"
        >
          Cancel
        </button>
        <button
          type="submit"
          disabled={isSubmitting}
          className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 disabled:opacity-50"
        >
          {isSubmitting
            ? isEditing
              ? 'Updating...'
              : 'Creating...'
            : isEditing
              ? 'Update Event'
              : 'Create Event'}
        </button>
      </div>
    </form>
  );
}

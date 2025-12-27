/**
 * Virtual Tour Form Component
 * Story 70.1: Virtual Tour Integration
 *
 * Form for creating and editing virtual tours with support for different tour types.
 */

import { useState } from 'react';
import type { TourType, VirtualTour } from './VirtualTourViewer';
import { TOUR_TYPES } from './VirtualTourViewer';

export interface VirtualTourFormData {
  tourType: TourType;
  title?: string;
  description?: string;
  photoUrl?: string;
  embedUrl?: string;
  externalId?: string;
  videoUrl?: string;
  thumbnailUrl?: string;
  isFeatured: boolean;
}

export interface VirtualTourFormProps {
  tour?: VirtualTour;
  onSubmit: (data: VirtualTourFormData) => Promise<void>;
  onCancel: () => void;
  isLoading?: boolean;
}

/**
 * Form for creating or editing virtual tours.
 */
export function VirtualTourForm({
  tour,
  onSubmit,
  onCancel,
  isLoading = false,
}: VirtualTourFormProps) {
  const [formData, setFormData] = useState<VirtualTourFormData>({
    tourType: tour?.tourType ?? TOUR_TYPES.MATTERPORT,
    title: tour?.title ?? '',
    description: tour?.description ?? '',
    photoUrl: tour?.photoUrl ?? '',
    embedUrl: tour?.embedUrl ?? '',
    externalId: tour?.externalId ?? '',
    videoUrl: tour?.videoUrl ?? '',
    thumbnailUrl: tour?.thumbnailUrl ?? '',
    isFeatured: tour?.isFeatured ?? false,
  });
  const [errors, setErrors] = useState<Record<string, string>>({});

  const validateForm = (): boolean => {
    const newErrors: Record<string, string> = {};

    // Validate required fields based on tour type
    switch (formData.tourType) {
      case TOUR_TYPES.PHOTO_360:
        if (!formData.photoUrl) {
          newErrors.photoUrl = '360 photo URL is required';
        }
        break;
      case TOUR_TYPES.MATTERPORT:
      case TOUR_TYPES.EXTERNAL_EMBED:
        if (!formData.embedUrl) {
          newErrors.embedUrl = 'Embed URL is required';
        }
        break;
      case TOUR_TYPES.VIDEO:
        if (!formData.videoUrl) {
          newErrors.videoUrl = 'Video URL is required';
        }
        break;
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (validateForm()) {
      await onSubmit(formData);
    }
  };

  const handleChange = (
    field: keyof VirtualTourFormData,
    value: string | boolean
  ) => {
    setFormData((prev) => ({ ...prev, [field]: value }));
    if (errors[field]) {
      setErrors((prev) => ({ ...prev, [field]: '' }));
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      {/* Tour Type Selection */}
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-2">
          Tour Type
        </label>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
          {Object.entries(TOUR_TYPES).map(([key, value]) => (
            <button
              key={key}
              type="button"
              className={`p-3 border-2 rounded-lg text-center transition-colors ${
                formData.tourType === value
                  ? 'border-blue-600 bg-blue-50 text-blue-700'
                  : 'border-gray-200 hover:border-gray-300'
              }`}
              onClick={() => handleChange('tourType', value)}
            >
              <div className="text-lg mb-1">
                {value === TOUR_TYPES.PHOTO_360 && '360'}
                {value === TOUR_TYPES.MATTERPORT && 'M'}
                {value === TOUR_TYPES.VIDEO && 'V'}
                {value === TOUR_TYPES.EXTERNAL_EMBED && 'E'}
              </div>
              <div className="text-xs capitalize">
                {value.replace('_', ' ')}
              </div>
            </button>
          ))}
        </div>
      </div>

      {/* Title */}
      <div>
        <label
          htmlFor="title"
          className="block text-sm font-medium text-gray-700 mb-1"
        >
          Title (optional)
        </label>
        <input
          type="text"
          id="title"
          value={formData.title}
          onChange={(e) => handleChange('title', e.target.value)}
          className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
          placeholder="e.g., Full Property Tour"
        />
      </div>

      {/* Description */}
      <div>
        <label
          htmlFor="description"
          className="block text-sm font-medium text-gray-700 mb-1"
        >
          Description (optional)
        </label>
        <textarea
          id="description"
          value={formData.description}
          onChange={(e) => handleChange('description', e.target.value)}
          rows={3}
          className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
          placeholder="Describe what viewers will see..."
        />
      </div>

      {/* Type-specific fields */}
      {formData.tourType === TOUR_TYPES.PHOTO_360 && (
        <div>
          <label
            htmlFor="photoUrl"
            className="block text-sm font-medium text-gray-700 mb-1"
          >
            360 Photo URL *
          </label>
          <input
            type="url"
            id="photoUrl"
            value={formData.photoUrl}
            onChange={(e) => handleChange('photoUrl', e.target.value)}
            className={`w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 ${
              errors.photoUrl ? 'border-red-500' : 'border-gray-300'
            }`}
            placeholder="https://example.com/360-photo.jpg"
          />
          {errors.photoUrl && (
            <p className="mt-1 text-sm text-red-600">{errors.photoUrl}</p>
          )}
        </div>
      )}

      {(formData.tourType === TOUR_TYPES.MATTERPORT ||
        formData.tourType === TOUR_TYPES.EXTERNAL_EMBED) && (
        <>
          <div>
            <label
              htmlFor="embedUrl"
              className="block text-sm font-medium text-gray-700 mb-1"
            >
              Embed URL *
            </label>
            <input
              type="url"
              id="embedUrl"
              value={formData.embedUrl}
              onChange={(e) => handleChange('embedUrl', e.target.value)}
              className={`w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 ${
                errors.embedUrl ? 'border-red-500' : 'border-gray-300'
              }`}
              placeholder={
                formData.tourType === TOUR_TYPES.MATTERPORT
                  ? 'https://my.matterport.com/show/?m=...'
                  : 'https://example.com/embed/...'
              }
            />
            {errors.embedUrl && (
              <p className="mt-1 text-sm text-red-600">{errors.embedUrl}</p>
            )}
          </div>

          {formData.tourType === TOUR_TYPES.MATTERPORT && (
            <div>
              <label
                htmlFor="externalId"
                className="block text-sm font-medium text-gray-700 mb-1"
              >
                Matterport Model ID (optional)
              </label>
              <input
                type="text"
                id="externalId"
                value={formData.externalId}
                onChange={(e) => handleChange('externalId', e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                placeholder="e.g., SxQL3iGaB..."
              />
            </div>
          )}
        </>
      )}

      {formData.tourType === TOUR_TYPES.VIDEO && (
        <div>
          <label
            htmlFor="videoUrl"
            className="block text-sm font-medium text-gray-700 mb-1"
          >
            Video URL *
          </label>
          <input
            type="url"
            id="videoUrl"
            value={formData.videoUrl}
            onChange={(e) => handleChange('videoUrl', e.target.value)}
            className={`w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 ${
              errors.videoUrl ? 'border-red-500' : 'border-gray-300'
            }`}
            placeholder="https://example.com/tour-video.mp4"
          />
          {errors.videoUrl && (
            <p className="mt-1 text-sm text-red-600">{errors.videoUrl}</p>
          )}
        </div>
      )}

      {/* Thumbnail */}
      <div>
        <label
          htmlFor="thumbnailUrl"
          className="block text-sm font-medium text-gray-700 mb-1"
        >
          Thumbnail URL (optional)
        </label>
        <input
          type="url"
          id="thumbnailUrl"
          value={formData.thumbnailUrl}
          onChange={(e) => handleChange('thumbnailUrl', e.target.value)}
          className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
          placeholder="https://example.com/thumbnail.jpg"
        />
      </div>

      {/* Featured toggle */}
      <div className="flex items-center gap-3">
        <input
          type="checkbox"
          id="isFeatured"
          checked={formData.isFeatured}
          onChange={(e) => handleChange('isFeatured', e.target.checked)}
          className="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
        />
        <label htmlFor="isFeatured" className="text-sm font-medium text-gray-700">
          Mark as featured tour
        </label>
      </div>

      {/* Actions */}
      <div className="flex items-center justify-end gap-3 pt-4 border-t">
        <button
          type="button"
          onClick={onCancel}
          className="px-4 py-2 text-gray-700 bg-gray-100 hover:bg-gray-200 rounded-lg transition-colors"
          disabled={isLoading}
        >
          Cancel
        </button>
        <button
          type="submit"
          className="px-4 py-2 text-white bg-blue-600 hover:bg-blue-700 rounded-lg transition-colors disabled:opacity-50"
          disabled={isLoading}
        >
          {isLoading ? 'Saving...' : tour ? 'Update Tour' : 'Add Tour'}
        </button>
      </div>
    </form>
  );
}

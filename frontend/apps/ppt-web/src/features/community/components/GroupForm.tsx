/**
 * GroupForm Component
 *
 * Form for creating and editing community groups.
 * Part of Story 42.1: Community Groups.
 */

import type { CreateGroupRequest, GroupVisibility, UpdateGroupRequest } from '@ppt/api-client';
import { useState } from 'react';

interface GroupFormProps {
  initialData?: Partial<CreateGroupRequest>;
  isEditing?: boolean;
  isSubmitting?: boolean;
  onSubmit: (data: CreateGroupRequest | UpdateGroupRequest) => void;
  onCancel: () => void;
}

const categories = ['Social', 'Sports', 'Hobbies', 'Parenting', 'Pets', 'Business', 'Other'];

const visibilityOptions: { value: GroupVisibility; label: string; description: string }[] = [
  { value: 'public', label: 'Public', description: 'Anyone can find and join this group' },
  { value: 'private', label: 'Private', description: 'Anyone can find but must request to join' },
  { value: 'hidden', label: 'Hidden', description: 'Only invited users can find this group' },
];

export function GroupForm({
  initialData,
  isEditing = false,
  isSubmitting = false,
  onSubmit,
  onCancel,
}: GroupFormProps) {
  const [name, setName] = useState(initialData?.name || '');
  const [description, setDescription] = useState(initialData?.description || '');
  const [category, setCategory] = useState(initialData?.category || categories[0]);
  const [visibility, setVisibility] = useState<GroupVisibility>(
    initialData?.visibility || 'public'
  );
  const [coverImageUrl, setCoverImageUrl] = useState(initialData?.coverImageUrl || '');
  const [errors, setErrors] = useState<Record<string, string>>({});

  const validate = (): boolean => {
    const newErrors: Record<string, string> = {};

    if (!name.trim()) {
      newErrors.name = 'Group name is required';
    } else if (name.length < 3) {
      newErrors.name = 'Group name must be at least 3 characters';
    } else if (name.length > 100) {
      newErrors.name = 'Group name must be less than 100 characters';
    }

    if (!description.trim()) {
      newErrors.description = 'Description is required';
    } else if (description.length < 10) {
      newErrors.description = 'Description must be at least 10 characters';
    } else if (description.length > 1000) {
      newErrors.description = 'Description must be less than 1000 characters';
    }

    if (!category) {
      newErrors.category = 'Category is required';
    }

    if (coverImageUrl && !isValidUrl(coverImageUrl)) {
      newErrors.coverImageUrl = 'Please enter a valid URL';
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
      onSubmit({
        name: name.trim(),
        description: description.trim(),
        category,
        visibility,
        coverImageUrl: coverImageUrl.trim() || undefined,
      });
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      {/* Cover Image Preview */}
      <div className="relative h-40 bg-gradient-to-r from-blue-500 to-purple-600 rounded-lg overflow-hidden">
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
          <span className="text-white text-lg font-medium">{name || 'Group Name'}</span>
        </div>
      </div>

      {/* Name */}
      <div>
        <label htmlFor="name" className="block text-sm font-medium text-gray-700">
          Group Name <span className="text-red-500">*</span>
        </label>
        <input
          type="text"
          id="name"
          value={name}
          onChange={(e) => setName(e.target.value)}
          placeholder="Enter group name"
          className={`mt-1 block w-full px-3 py-2 border rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 ${
            errors.name ? 'border-red-300' : 'border-gray-300'
          }`}
        />
        {errors.name && <p className="mt-1 text-sm text-red-600">{errors.name}</p>}
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
          placeholder="Describe what your group is about..."
          rows={4}
          className={`mt-1 block w-full px-3 py-2 border rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 ${
            errors.description ? 'border-red-300' : 'border-gray-300'
          }`}
        />
        <div className="mt-1 flex justify-between">
          {errors.description ? (
            <p className="text-sm text-red-600">{errors.description}</p>
          ) : (
            <span />
          )}
          <span className="text-xs text-gray-400">{description.length}/1000</span>
        </div>
      </div>

      {/* Category */}
      <div>
        <label htmlFor="category" className="block text-sm font-medium text-gray-700">
          Category <span className="text-red-500">*</span>
        </label>
        <select
          id="category"
          value={category}
          onChange={(e) => setCategory(e.target.value)}
          className={`mt-1 block w-full px-3 py-2 border rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 ${
            errors.category ? 'border-red-300' : 'border-gray-300'
          }`}
        >
          {categories.map((cat) => (
            <option key={cat} value={cat}>
              {cat}
            </option>
          ))}
        </select>
        {errors.category && <p className="mt-1 text-sm text-red-600">{errors.category}</p>}
      </div>

      {/* Visibility */}
      <fieldset>
        <legend className="block text-sm font-medium text-gray-700 mb-2">Visibility</legend>
        <div className="space-y-2">
          {visibilityOptions.map((option) => (
            <label
              key={option.value}
              className={`flex items-start p-3 border rounded-lg cursor-pointer transition-colors ${
                visibility === option.value
                  ? 'border-blue-500 bg-blue-50'
                  : 'border-gray-200 hover:border-gray-300'
              }`}
            >
              <input
                type="radio"
                name="visibility"
                value={option.value}
                checked={visibility === option.value}
                onChange={(e) => setVisibility(e.target.value as GroupVisibility)}
                className="mt-1 h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300"
              />
              <div className="ml-3">
                <span className="block text-sm font-medium text-gray-900">{option.label}</span>
                <span className="block text-xs text-gray-500">{option.description}</span>
              </div>
            </label>
          ))}
        </div>
      </fieldset>

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
              ? 'Update Group'
              : 'Create Group'}
        </button>
      </div>
    </form>
  );
}

/**
 * FaultForm component - form for creating/editing faults.
 * Epic 4: Fault Reporting & Resolution (UC-03.1)
 */

import { useState } from 'react';
import type { FaultCategory, FaultPriority } from './FaultCard';

export interface FaultFormData {
  buildingId: string;
  unitId?: string;
  title: string;
  description: string;
  locationDescription?: string;
  category: FaultCategory;
  priority?: FaultPriority;
}

interface FaultFormProps {
  initialData?: Partial<FaultFormData>;
  buildings?: Array<{ id: string; name: string }>;
  units?: Array<{ id: string; designation: string }>;
  isSubmitting?: boolean;
  onSubmit: (data: FaultFormData) => void;
  onCancel: () => void;
}

const categoryOptions: { value: FaultCategory; label: string }[] = [
  { value: 'plumbing', label: 'Plumbing' },
  { value: 'electrical', label: 'Electrical' },
  { value: 'heating', label: 'Heating' },
  { value: 'structural', label: 'Structural' },
  { value: 'exterior', label: 'Exterior' },
  { value: 'elevator', label: 'Elevator' },
  { value: 'common_area', label: 'Common Area' },
  { value: 'security', label: 'Security' },
  { value: 'cleaning', label: 'Cleaning' },
  { value: 'other', label: 'Other' },
];

const priorityOptions: { value: FaultPriority; label: string }[] = [
  { value: 'low', label: 'Low' },
  { value: 'medium', label: 'Medium' },
  { value: 'high', label: 'High' },
  { value: 'urgent', label: 'Urgent' },
];

export function FaultForm({
  initialData,
  buildings = [],
  units = [],
  isSubmitting,
  onSubmit,
  onCancel,
}: FaultFormProps) {
  const [formData, setFormData] = useState<FaultFormData>({
    buildingId: initialData?.buildingId || '',
    unitId: initialData?.unitId || undefined,
    title: initialData?.title || '',
    description: initialData?.description || '',
    locationDescription: initialData?.locationDescription || '',
    category: initialData?.category || 'other',
    priority: initialData?.priority || undefined,
  });
  const [errors, setErrors] = useState<Partial<Record<keyof FaultFormData, string>>>({});

  const validate = (): boolean => {
    const newErrors: Partial<Record<keyof FaultFormData, string>> = {};

    if (!formData.buildingId) {
      newErrors.buildingId = 'Building is required';
    }
    if (!formData.title.trim()) {
      newErrors.title = 'Title is required';
    } else if (formData.title.length > 255) {
      newErrors.title = 'Title must be 255 characters or less';
    }
    if (!formData.description.trim()) {
      newErrors.description = 'Description is required';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (validate()) {
      onSubmit(formData);
    }
  };

  const handleChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>
  ) => {
    const { name, value } = e.target;
    setFormData((prev) => ({
      ...prev,
      [name]: value || undefined,
    }));
    // Clear error on change
    if (errors[name as keyof FaultFormData]) {
      setErrors((prev) => ({ ...prev, [name]: undefined }));
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      {/* Building */}
      <div>
        <label htmlFor="buildingId" className="block text-sm font-medium text-gray-700">
          Building *
        </label>
        <select
          id="buildingId"
          name="buildingId"
          value={formData.buildingId}
          onChange={handleChange}
          className={`mt-1 block w-full rounded-md border ${
            errors.buildingId ? 'border-red-500' : 'border-gray-300'
          } px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500`}
        >
          <option value="">Select a building</option>
          {buildings.map((b) => (
            <option key={b.id} value={b.id}>
              {b.name}
            </option>
          ))}
        </select>
        {errors.buildingId && <p className="mt-1 text-sm text-red-500">{errors.buildingId}</p>}
      </div>

      {/* Unit (optional) */}
      <div>
        <label htmlFor="unitId" className="block text-sm font-medium text-gray-700">
          Unit (optional)
        </label>
        <select
          id="unitId"
          name="unitId"
          value={formData.unitId || ''}
          onChange={handleChange}
          className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          <option value="">Common area / Not applicable</option>
          {units.map((u) => (
            <option key={u.id} value={u.id}>
              {u.designation}
            </option>
          ))}
        </select>
      </div>

      {/* Title */}
      <div>
        <label htmlFor="title" className="block text-sm font-medium text-gray-700">
          Title *
        </label>
        <input
          type="text"
          id="title"
          name="title"
          value={formData.title}
          onChange={handleChange}
          placeholder="Brief description of the issue"
          className={`mt-1 block w-full rounded-md border ${
            errors.title ? 'border-red-500' : 'border-gray-300'
          } px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500`}
        />
        {errors.title && <p className="mt-1 text-sm text-red-500">{errors.title}</p>}
      </div>

      {/* Description */}
      <div>
        <label htmlFor="description" className="block text-sm font-medium text-gray-700">
          Description *
        </label>
        <textarea
          id="description"
          name="description"
          value={formData.description}
          onChange={handleChange}
          rows={4}
          placeholder="Provide detailed information about the issue..."
          className={`mt-1 block w-full rounded-md border ${
            errors.description ? 'border-red-500' : 'border-gray-300'
          } px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500`}
        />
        {errors.description && <p className="mt-1 text-sm text-red-500">{errors.description}</p>}
      </div>

      {/* Location Description */}
      <div>
        <label htmlFor="locationDescription" className="block text-sm font-medium text-gray-700">
          Location (optional)
        </label>
        <input
          type="text"
          id="locationDescription"
          name="locationDescription"
          value={formData.locationDescription || ''}
          onChange={handleChange}
          placeholder="e.g., Kitchen sink, Hallway 3rd floor"
          className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>

      {/* Category */}
      <div>
        <label htmlFor="category" className="block text-sm font-medium text-gray-700">
          Category *
        </label>
        <select
          id="category"
          name="category"
          value={formData.category}
          onChange={handleChange}
          className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          {categoryOptions.map((opt) => (
            <option key={opt.value} value={opt.value}>
              {opt.label}
            </option>
          ))}
        </select>
      </div>

      {/* Priority (optional, shown for managers) */}
      <div>
        <label htmlFor="priority" className="block text-sm font-medium text-gray-700">
          Priority (optional)
        </label>
        <select
          id="priority"
          name="priority"
          value={formData.priority || ''}
          onChange={handleChange}
          className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          <option value="">Not specified</option>
          {priorityOptions.map((opt) => (
            <option key={opt.value} value={opt.value}>
              {opt.label}
            </option>
          ))}
        </select>
      </div>

      {/* Actions */}
      <div className="flex items-center justify-end gap-3 pt-4 border-t">
        <button
          type="button"
          onClick={onCancel}
          disabled={isSubmitting}
          className="px-4 py-2 text-gray-700 border border-gray-300 rounded-lg hover:bg-gray-50 disabled:opacity-50"
        >
          Cancel
        </button>
        <button
          type="submit"
          disabled={isSubmitting}
          className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 flex items-center gap-2"
        >
          {isSubmitting && (
            <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white" />
          )}
          {isSubmitting ? 'Submitting...' : 'Submit Fault'}
        </button>
      </div>
    </form>
  );
}

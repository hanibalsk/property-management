/**
 * PetRegistrationForm Component
 *
 * Form for creating/updating pet registrations (Epic 57).
 */

import type {
  CreatePetRegistrationRequest,
  PetSize,
  PetType,
  UpdatePetRegistrationRequest,
} from '@ppt/api-client';
import { useState } from 'react';

interface PetRegistrationFormProps {
  initialData?: UpdatePetRegistrationRequest;
  onSubmit: (data: CreatePetRegistrationRequest | UpdatePetRegistrationRequest) => void;
  onCancel: () => void;
  isSubmitting?: boolean;
}

const petTypes: { value: PetType; label: string }[] = [
  { value: 'dog', label: 'Dog' },
  { value: 'cat', label: 'Cat' },
  { value: 'bird', label: 'Bird' },
  { value: 'fish', label: 'Fish' },
  { value: 'reptile', label: 'Reptile' },
  { value: 'rodent', label: 'Rodent' },
  { value: 'other', label: 'Other' },
];

const petSizes: { value: PetSize; label: string }[] = [
  { value: 'small', label: 'Small (< 10 kg)' },
  { value: 'medium', label: 'Medium (10-25 kg)' },
  { value: 'large', label: 'Large (25-45 kg)' },
  { value: 'extra_large', label: 'Extra Large (> 45 kg)' },
];

export function PetRegistrationForm({
  initialData,
  onSubmit,
  onCancel,
  isSubmitting,
}: PetRegistrationFormProps) {
  const [formData, setFormData] = useState<CreatePetRegistrationRequest>({
    petType: initialData?.petType || 'dog',
    petSize: initialData?.petSize || 'medium',
    name: initialData?.name || '',
    breed: initialData?.breed || '',
    age: initialData?.age,
    weight: initialData?.weight,
    description: initialData?.description || '',
    veterinarianName: initialData?.veterinarianName || '',
    veterinarianContact: initialData?.veterinarianContact || '',
    vaccinationRecordUrl: initialData?.vaccinationRecordUrl || '',
    registrationDocumentUrl: initialData?.registrationDocumentUrl || '',
    photoUrl: initialData?.photoUrl || '',
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSubmit(formData);
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* Pet Type */}
        <div>
          <label htmlFor="petType" className="block text-sm font-medium text-gray-700 mb-1">
            Pet Type *
          </label>
          <select
            id="petType"
            required
            value={formData.petType}
            onChange={(e) => setFormData({ ...formData, petType: e.target.value as PetType })}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            {petTypes.map((type) => (
              <option key={type.value} value={type.value}>
                {type.label}
              </option>
            ))}
          </select>
        </div>

        {/* Pet Size */}
        <div>
          <label htmlFor="petSize" className="block text-sm font-medium text-gray-700 mb-1">
            Pet Size *
          </label>
          <select
            id="petSize"
            required
            value={formData.petSize}
            onChange={(e) => setFormData({ ...formData, petSize: e.target.value as PetSize })}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            {petSizes.map((size) => (
              <option key={size.value} value={size.value}>
                {size.label}
              </option>
            ))}
          </select>
        </div>

        {/* Name */}
        <div>
          <label htmlFor="name" className="block text-sm font-medium text-gray-700 mb-1">
            Pet Name *
          </label>
          <input
            id="name"
            type="text"
            required
            value={formData.name}
            onChange={(e) => setFormData({ ...formData, name: e.target.value })}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="Enter pet name"
          />
        </div>

        {/* Breed */}
        <div>
          <label htmlFor="breed" className="block text-sm font-medium text-gray-700 mb-1">
            Breed
          </label>
          <input
            id="breed"
            type="text"
            value={formData.breed || ''}
            onChange={(e) => setFormData({ ...formData, breed: e.target.value })}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="Enter breed (optional)"
          />
        </div>

        {/* Age */}
        <div>
          <label htmlFor="age" className="block text-sm font-medium text-gray-700 mb-1">
            Age (years)
          </label>
          <input
            id="age"
            type="number"
            min="0"
            max="50"
            value={formData.age || ''}
            onChange={(e) =>
              setFormData({ ...formData, age: e.target.value ? Number(e.target.value) : undefined })
            }
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="Enter age"
          />
        </div>

        {/* Weight */}
        <div>
          <label htmlFor="weight" className="block text-sm font-medium text-gray-700 mb-1">
            Weight (kg)
          </label>
          <input
            id="weight"
            type="number"
            min="0"
            max="200"
            step="0.1"
            value={formData.weight || ''}
            onChange={(e) =>
              setFormData({
                ...formData,
                weight: e.target.value ? Number(e.target.value) : undefined,
              })
            }
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="Enter weight"
          />
        </div>
      </div>

      {/* Description */}
      <div>
        <label htmlFor="description" className="block text-sm font-medium text-gray-700 mb-1">
          Description
        </label>
        <textarea
          id="description"
          rows={3}
          value={formData.description || ''}
          onChange={(e) => setFormData({ ...formData, description: e.target.value })}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="Any additional information about your pet"
        />
      </div>

      {/* Veterinarian Information */}
      <div className="border-t pt-6">
        <h3 className="text-lg font-medium text-gray-900 mb-4">Veterinarian Information</h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div>
            <label
              htmlFor="veterinarianName"
              className="block text-sm font-medium text-gray-700 mb-1"
            >
              Veterinarian Name
            </label>
            <input
              id="veterinarianName"
              type="text"
              value={formData.veterinarianName || ''}
              onChange={(e) => setFormData({ ...formData, veterinarianName: e.target.value })}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="Dr. Jane Smith"
            />
          </div>

          <div>
            <label
              htmlFor="veterinarianContact"
              className="block text-sm font-medium text-gray-700 mb-1"
            >
              Veterinarian Contact
            </label>
            <input
              id="veterinarianContact"
              type="text"
              value={formData.veterinarianContact || ''}
              onChange={(e) => setFormData({ ...formData, veterinarianContact: e.target.value })}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="Phone or email"
            />
          </div>
        </div>
      </div>

      {/* Documents */}
      <div className="border-t pt-6">
        <h3 className="text-lg font-medium text-gray-900 mb-4">Documents</h3>
        <div className="space-y-4">
          <div>
            <label
              htmlFor="vaccinationRecordUrl"
              className="block text-sm font-medium text-gray-700 mb-1"
            >
              Vaccination Record URL
            </label>
            <input
              id="vaccinationRecordUrl"
              type="url"
              value={formData.vaccinationRecordUrl || ''}
              onChange={(e) => setFormData({ ...formData, vaccinationRecordUrl: e.target.value })}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="https://..."
            />
          </div>

          <div>
            <label
              htmlFor="registrationDocumentUrl"
              className="block text-sm font-medium text-gray-700 mb-1"
            >
              Registration Document URL
            </label>
            <input
              id="registrationDocumentUrl"
              type="url"
              value={formData.registrationDocumentUrl || ''}
              onChange={(e) =>
                setFormData({ ...formData, registrationDocumentUrl: e.target.value })
              }
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="https://..."
            />
          </div>

          <div>
            <label htmlFor="photoUrl" className="block text-sm font-medium text-gray-700 mb-1">
              Pet Photo URL
            </label>
            <input
              id="photoUrl"
              type="url"
              value={formData.photoUrl || ''}
              onChange={(e) => setFormData({ ...formData, photoUrl: e.target.value })}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="https://..."
            />
          </div>
        </div>
      </div>

      {/* Actions */}
      <div className="flex justify-end gap-3 pt-6 border-t">
        <button
          type="button"
          onClick={onCancel}
          disabled={isSubmitting}
          className="px-4 py-2 border border-gray-300 rounded-md text-gray-700 hover:bg-gray-50 disabled:opacity-50"
        >
          Cancel
        </button>
        <button
          type="submit"
          disabled={isSubmitting}
          className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 flex items-center gap-2"
        >
          {isSubmitting && (
            <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white" />
          )}
          {isSubmitting ? 'Submitting...' : 'Submit Registration'}
        </button>
      </div>
    </form>
  );
}

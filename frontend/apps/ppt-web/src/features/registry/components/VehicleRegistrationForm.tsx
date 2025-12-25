/**
 * VehicleRegistrationForm Component
 *
 * Form for creating/updating vehicle registrations (Epic 57).
 */

import type {
  CreateVehicleRegistrationRequest,
  ParkingSpot,
  UpdateVehicleRegistrationRequest,
  VehicleType,
} from '@ppt/api-client';
import { useState } from 'react';

interface VehicleRegistrationFormProps {
  initialData?: UpdateVehicleRegistrationRequest;
  parkingSpots?: ParkingSpot[];
  onSubmit: (data: CreateVehicleRegistrationRequest | UpdateVehicleRegistrationRequest) => void;
  onCancel: () => void;
  isSubmitting?: boolean;
}

const vehicleTypes: { value: VehicleType; label: string }[] = [
  { value: 'car', label: 'Car' },
  { value: 'motorcycle', label: 'Motorcycle' },
  { value: 'bicycle', label: 'Bicycle' },
  { value: 'scooter', label: 'Scooter' },
  { value: 'other', label: 'Other' },
];

export function VehicleRegistrationForm({
  initialData,
  parkingSpots = [],
  onSubmit,
  onCancel,
  isSubmitting,
}: VehicleRegistrationFormProps) {
  const [formData, setFormData] = useState<CreateVehicleRegistrationRequest>({
    vehicleType: initialData?.vehicleType || 'car',
    make: initialData?.make || '',
    model: initialData?.model || '',
    year: initialData?.year,
    color: initialData?.color || '',
    licensePlate: initialData?.licensePlate || '',
    parkingSpotId: initialData?.parkingSpotId || '',
    registrationDocumentUrl: initialData?.registrationDocumentUrl || '',
    photoUrl: initialData?.photoUrl || '',
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSubmit(formData);
  };

  const availableSpots = parkingSpots.filter((spot) => !spot.isAssigned);

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* Vehicle Type */}
        <div>
          <label htmlFor="vehicleType" className="block text-sm font-medium text-gray-700 mb-1">
            Vehicle Type *
          </label>
          <select
            id="vehicleType"
            required
            value={formData.vehicleType}
            onChange={(e) =>
              setFormData({ ...formData, vehicleType: e.target.value as VehicleType })
            }
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            {vehicleTypes.map((type) => (
              <option key={type.value} value={type.value}>
                {type.label}
              </option>
            ))}
          </select>
        </div>

        {/* License Plate */}
        <div>
          <label htmlFor="licensePlate" className="block text-sm font-medium text-gray-700 mb-1">
            License Plate *
          </label>
          <input
            id="licensePlate"
            type="text"
            required
            value={formData.licensePlate}
            onChange={(e) => setFormData({ ...formData, licensePlate: e.target.value })}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 font-mono"
            placeholder="ABC-123"
          />
        </div>

        {/* Make */}
        <div>
          <label htmlFor="make" className="block text-sm font-medium text-gray-700 mb-1">
            Make
          </label>
          <input
            id="make"
            type="text"
            value={formData.make || ''}
            onChange={(e) => setFormData({ ...formData, make: e.target.value })}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="Toyota, BMW, etc."
          />
        </div>

        {/* Model */}
        <div>
          <label htmlFor="model" className="block text-sm font-medium text-gray-700 mb-1">
            Model
          </label>
          <input
            id="model"
            type="text"
            value={formData.model || ''}
            onChange={(e) => setFormData({ ...formData, model: e.target.value })}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="Corolla, X5, etc."
          />
        </div>

        {/* Year */}
        <div>
          <label htmlFor="year" className="block text-sm font-medium text-gray-700 mb-1">
            Year
          </label>
          <input
            id="year"
            type="number"
            min="1900"
            max={new Date().getFullYear() + 1}
            value={formData.year || ''}
            onChange={(e) =>
              setFormData({
                ...formData,
                year: e.target.value ? Number(e.target.value) : undefined,
              })
            }
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder={new Date().getFullYear().toString()}
          />
        </div>

        {/* Color */}
        <div>
          <label htmlFor="color" className="block text-sm font-medium text-gray-700 mb-1">
            Color
          </label>
          <input
            id="color"
            type="text"
            value={formData.color || ''}
            onChange={(e) => setFormData({ ...formData, color: e.target.value })}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="Black, White, Red, etc."
          />
        </div>

        {/* Parking Spot */}
        <div className="md:col-span-2">
          <label htmlFor="parkingSpotId" className="block text-sm font-medium text-gray-700 mb-1">
            Parking Spot
          </label>
          <select
            id="parkingSpotId"
            value={formData.parkingSpotId || ''}
            onChange={(e) => setFormData({ ...formData, parkingSpotId: e.target.value })}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <option value="">No parking spot assigned</option>
            {availableSpots.map((spot) => (
              <option key={spot.id} value={spot.id}>
                {spot.spotNumber}
                {spot.level && ` - Level ${spot.level}`}
                {spot.zone && ` - Zone ${spot.zone}`}
              </option>
            ))}
          </select>
          {availableSpots.length === 0 && (
            <p className="mt-1 text-sm text-gray-500">No available parking spots</p>
          )}
        </div>
      </div>

      {/* Documents */}
      <div className="border-t pt-6">
        <h3 className="text-lg font-medium text-gray-900 mb-4">Documents</h3>
        <div className="space-y-4">
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
              Vehicle Photo URL
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

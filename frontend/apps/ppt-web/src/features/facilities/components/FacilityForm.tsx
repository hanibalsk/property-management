/**
 * Facility Form Component (Epic 56: Facility Booking).
 *
 * Form for creating or editing a facility with booking rules.
 */

import type {
  CreateFacilityRequest,
  Facility,
  FacilityType,
  UpdateFacilityRequest,
} from '@ppt/api-client';
import { useState } from 'react';

interface FacilityFormProps {
  facility?: Facility;
  onSubmit: (data: CreateFacilityRequest | UpdateFacilityRequest) => void;
  onCancel: () => void;
  isSubmitting?: boolean;
}

const facilityTypes: { value: FacilityType; label: string }[] = [
  { value: 'gym', label: 'Gym' },
  { value: 'laundry', label: 'Laundry' },
  { value: 'meeting_room', label: 'Meeting Room' },
  { value: 'party_room', label: 'Party Room' },
  { value: 'sauna', label: 'Sauna' },
  { value: 'pool', label: 'Pool' },
  { value: 'playground', label: 'Playground' },
  { value: 'parking', label: 'Parking' },
  { value: 'storage', label: 'Storage' },
  { value: 'garden', label: 'Garden' },
  { value: 'bbq', label: 'BBQ Area' },
  { value: 'bike_storage', label: 'Bike Storage' },
  { value: 'other', label: 'Other' },
];

const daysOfWeek = [
  { bit: 1, label: 'Mon' },
  { bit: 2, label: 'Tue' },
  { bit: 4, label: 'Wed' },
  { bit: 8, label: 'Thu' },
  { bit: 16, label: 'Fri' },
  { bit: 32, label: 'Sat' },
  { bit: 64, label: 'Sun' },
];

export function FacilityForm({ facility, onSubmit, onCancel, isSubmitting }: FacilityFormProps) {
  const [name, setName] = useState(facility?.name || '');
  const [facilityType, setFacilityType] = useState<FacilityType>(
    facility?.facility_type || 'other'
  );
  const [description, setDescription] = useState(facility?.description || '');
  const [location, setLocation] = useState(facility?.location || '');
  const [capacity, setCapacity] = useState<number | ''>(facility?.capacity || '');
  const [isBookable, setIsBookable] = useState(facility?.is_bookable ?? true);
  const [isActive, setIsActive] = useState(facility?.is_active ?? true);
  const [requiresApproval, setRequiresApproval] = useState(facility?.requires_approval ?? false);
  const [maxBookingHours, setMaxBookingHours] = useState<number | ''>(
    facility?.max_booking_hours || ''
  );
  const [maxAdvanceDays, setMaxAdvanceDays] = useState<number | ''>(
    facility?.max_advance_days || ''
  );
  const [minAdvanceHours, setMinAdvanceHours] = useState<number | ''>(
    facility?.min_advance_hours || ''
  );
  const [availableFrom, setAvailableFrom] = useState(facility?.available_from || '');
  const [availableTo, setAvailableTo] = useState(facility?.available_to || '');
  const [availableDays, setAvailableDays] = useState<number>(facility?.available_days ?? 127);
  const [hourlyFee, setHourlyFee] = useState(facility?.hourly_fee || '');
  const [depositAmount, setDepositAmount] = useState(facility?.deposit_amount || '');
  const [amenities, setAmenities] = useState<string[]>(facility?.amenities || []);
  const [newAmenity, setNewAmenity] = useState('');
  const [errors, setErrors] = useState<Record<string, string>>({});

  const validate = (): boolean => {
    const newErrors: Record<string, string> = {};

    if (!name.trim()) {
      newErrors.name = 'Name is required';
    }
    if (!facilityType) {
      newErrors.facilityType = 'Facility type is required';
    }
    if (availableFrom && availableTo && availableFrom >= availableTo) {
      newErrors.availableTo = 'End time must be after start time';
    }
    if (maxBookingHours && Number(maxBookingHours) <= 0) {
      newErrors.maxBookingHours = 'Must be greater than 0';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!validate()) return;

    const data: CreateFacilityRequest | UpdateFacilityRequest = {
      name,
      facility_type: facilityType,
      description: description || undefined,
      location: location || undefined,
      capacity: capacity ? Number(capacity) : undefined,
      is_bookable: isBookable,
      requires_approval: requiresApproval,
      max_booking_hours: maxBookingHours ? Number(maxBookingHours) : undefined,
      max_advance_days: maxAdvanceDays ? Number(maxAdvanceDays) : undefined,
      min_advance_hours: minAdvanceHours ? Number(minAdvanceHours) : undefined,
      available_from: availableFrom || undefined,
      available_to: availableTo || undefined,
      available_days: availableDays,
      hourly_fee: hourlyFee || undefined,
      deposit_amount: depositAmount || undefined,
      amenities: amenities.length > 0 ? amenities : undefined,
    };

    if (facility) {
      (data as UpdateFacilityRequest).is_active = isActive;
    }

    onSubmit(data);
  };

  const toggleDay = (bit: number) => {
    setAvailableDays((prev) => prev ^ bit);
  };

  const addAmenity = () => {
    if (newAmenity.trim() && !amenities.includes(newAmenity.trim())) {
      setAmenities([...amenities, newAmenity.trim()]);
      setNewAmenity('');
    }
  };

  const removeAmenity = (amenity: string) => {
    setAmenities(amenities.filter((a) => a !== amenity));
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      {/* Basic Information */}
      <div className="bg-white rounded-lg shadow p-6">
        <h2 className="text-lg font-semibold text-gray-900 mb-4">Basic Information</h2>

        <div className="grid gap-4 md:grid-cols-2">
          <div>
            <label htmlFor="name" className="block text-sm font-medium text-gray-700">
              Name *
            </label>
            <input
              type="text"
              id="name"
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
            {errors.name && <p className="mt-1 text-sm text-red-600">{errors.name}</p>}
          </div>

          <div>
            <label htmlFor="facilityType" className="block text-sm font-medium text-gray-700">
              Type *
            </label>
            <select
              id="facilityType"
              value={facilityType}
              onChange={(e) => setFacilityType(e.target.value as FacilityType)}
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              {facilityTypes.map((type) => (
                <option key={type.value} value={type.value}>
                  {type.label}
                </option>
              ))}
            </select>
            {errors.facilityType && (
              <p className="mt-1 text-sm text-red-600">{errors.facilityType}</p>
            )}
          </div>

          <div className="md:col-span-2">
            <label htmlFor="description" className="block text-sm font-medium text-gray-700">
              Description
            </label>
            <textarea
              id="description"
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              rows={3}
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>

          <div>
            <label htmlFor="location" className="block text-sm font-medium text-gray-700">
              Location
            </label>
            <input
              type="text"
              id="location"
              value={location}
              onChange={(e) => setLocation(e.target.value)}
              placeholder="e.g., Ground Floor, Building A"
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>

          <div>
            <label htmlFor="capacity" className="block text-sm font-medium text-gray-700">
              Capacity
            </label>
            <input
              type="number"
              id="capacity"
              value={capacity}
              onChange={(e) => setCapacity(e.target.value ? Number(e.target.value) : '')}
              min={1}
              placeholder="Maximum number of people"
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>
        </div>
      </div>

      {/* Booking Rules */}
      <div className="bg-white rounded-lg shadow p-6">
        <h2 className="text-lg font-semibold text-gray-900 mb-4">Booking Rules</h2>

        <div className="space-y-4">
          {/* Toggles */}
          <div className="flex flex-wrap gap-6">
            <label className="flex items-center gap-2 cursor-pointer">
              <input
                type="checkbox"
                checked={isBookable}
                onChange={(e) => setIsBookable(e.target.checked)}
                className="w-4 h-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
              />
              <span className="text-sm text-gray-700">Allow Booking</span>
            </label>

            {facility && (
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="checkbox"
                  checked={isActive}
                  onChange={(e) => setIsActive(e.target.checked)}
                  className="w-4 h-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                />
                <span className="text-sm text-gray-700">Active</span>
              </label>
            )}

            <label className="flex items-center gap-2 cursor-pointer">
              <input
                type="checkbox"
                checked={requiresApproval}
                onChange={(e) => setRequiresApproval(e.target.checked)}
                className="w-4 h-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
              />
              <span className="text-sm text-gray-700">Requires Approval</span>
            </label>
          </div>

          {/* Time Constraints */}
          <div className="grid gap-4 md:grid-cols-3">
            <div>
              <label htmlFor="maxBookingHours" className="block text-sm font-medium text-gray-700">
                Max Booking Duration (hours)
              </label>
              <input
                type="number"
                id="maxBookingHours"
                value={maxBookingHours}
                onChange={(e) => setMaxBookingHours(e.target.value ? Number(e.target.value) : '')}
                min={1}
                className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
              {errors.maxBookingHours && (
                <p className="mt-1 text-sm text-red-600">{errors.maxBookingHours}</p>
              )}
            </div>

            <div>
              <label htmlFor="maxAdvanceDays" className="block text-sm font-medium text-gray-700">
                Max Advance Booking (days)
              </label>
              <input
                type="number"
                id="maxAdvanceDays"
                value={maxAdvanceDays}
                onChange={(e) => setMaxAdvanceDays(e.target.value ? Number(e.target.value) : '')}
                min={1}
                className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>

            <div>
              <label htmlFor="minAdvanceHours" className="block text-sm font-medium text-gray-700">
                Min Advance Booking (hours)
              </label>
              <input
                type="number"
                id="minAdvanceHours"
                value={minAdvanceHours}
                onChange={(e) => setMinAdvanceHours(e.target.value ? Number(e.target.value) : '')}
                min={0}
                className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>
          </div>

          {/* Availability Hours */}
          <div className="grid gap-4 md:grid-cols-2">
            <div>
              <label htmlFor="availableFrom" className="block text-sm font-medium text-gray-700">
                Available From
              </label>
              <input
                type="time"
                id="availableFrom"
                value={availableFrom}
                onChange={(e) => setAvailableFrom(e.target.value)}
                className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>

            <div>
              <label htmlFor="availableTo" className="block text-sm font-medium text-gray-700">
                Available Until
              </label>
              <input
                type="time"
                id="availableTo"
                value={availableTo}
                onChange={(e) => setAvailableTo(e.target.value)}
                className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
              {errors.availableTo && (
                <p className="mt-1 text-sm text-red-600">{errors.availableTo}</p>
              )}
            </div>
          </div>

          {/* Available Days */}
          <fieldset>
            <legend className="block text-sm font-medium text-gray-700 mb-2">Available Days</legend>
            <div className="flex flex-wrap gap-2">
              {daysOfWeek.map((day) => (
                <button
                  key={day.bit}
                  type="button"
                  onClick={() => toggleDay(day.bit)}
                  className={`px-3 py-1 rounded-md text-sm font-medium transition-colors ${
                    availableDays & day.bit
                      ? 'bg-blue-600 text-white'
                      : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                  }`}
                >
                  {day.label}
                </button>
              ))}
            </div>
          </fieldset>
        </div>
      </div>

      {/* Pricing */}
      <div className="bg-white rounded-lg shadow p-6">
        <h2 className="text-lg font-semibold text-gray-900 mb-4">Pricing</h2>

        <div className="grid gap-4 md:grid-cols-2">
          <div>
            <label htmlFor="hourlyFee" className="block text-sm font-medium text-gray-700">
              Hourly Fee
            </label>
            <input
              type="text"
              id="hourlyFee"
              value={hourlyFee}
              onChange={(e) => setHourlyFee(e.target.value)}
              placeholder="e.g., 25.00"
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>

          <div>
            <label htmlFor="depositAmount" className="block text-sm font-medium text-gray-700">
              Deposit Amount
            </label>
            <input
              type="text"
              id="depositAmount"
              value={depositAmount}
              onChange={(e) => setDepositAmount(e.target.value)}
              placeholder="e.g., 50.00"
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>
        </div>
      </div>

      {/* Amenities */}
      <div className="bg-white rounded-lg shadow p-6">
        <h2 className="text-lg font-semibold text-gray-900 mb-4">Amenities</h2>

        <div className="flex gap-2 mb-4">
          <input
            type="text"
            value={newAmenity}
            onChange={(e) => setNewAmenity(e.target.value)}
            placeholder="Add amenity (e.g., WiFi, Projector)"
            className="flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            onKeyDown={(e) => {
              if (e.key === 'Enter') {
                e.preventDefault();
                addAmenity();
              }
            }}
          />
          <button
            type="button"
            onClick={addAmenity}
            className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
          >
            Add
          </button>
        </div>

        {amenities.length > 0 && (
          <div className="flex flex-wrap gap-2">
            {amenities.map((amenity) => (
              <span
                key={amenity}
                className="inline-flex items-center gap-1 px-3 py-1 bg-gray-100 rounded-full text-sm"
              >
                {amenity}
                <button
                  type="button"
                  onClick={() => removeAmenity(amenity)}
                  className="text-gray-500 hover:text-red-600"
                  aria-label={`Remove ${amenity}`}
                >
                  <svg
                    className="w-4 h-4"
                    fill="currentColor"
                    viewBox="0 0 20 20"
                    aria-hidden="true"
                  >
                    <path
                      fillRule="evenodd"
                      d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
                      clipRule="evenodd"
                    />
                  </svg>
                </button>
              </span>
            ))}
          </div>
        )}
      </div>

      {/* Actions */}
      <div className="flex gap-4 pt-4">
        <button
          type="button"
          onClick={onCancel}
          className="flex-1 px-4 py-2 border border-gray-300 rounded-md hover:bg-gray-50 transition-colors"
        >
          Cancel
        </button>
        <button
          type="submit"
          disabled={isSubmitting}
          className="flex-1 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
        >
          {isSubmitting ? 'Saving...' : facility ? 'Update Facility' : 'Create Facility'}
        </button>
      </div>
    </form>
  );
}

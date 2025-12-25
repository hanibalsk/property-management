/**
 * RegistryRulesPage Component
 *
 * Page for configuring building registry rules (Epic 57, Story 57.7).
 */

import type { BuildingRegistryRules, PetType, UpdateRegistryRulesRequest } from '@ppt/api-client';
import { RegistryRulesForm } from '../components/RegistryRulesForm';

export function RegistryRulesPage() {
  // Mock data - replace with actual hooks in implementation
  const mockRules: BuildingRegistryRules = {
    id: '1',
    buildingId: '1',
    petsAllowed: true,
    petsRequireApproval: true,
    maxPetsPerUnit: 2,
    allowedPetTypes: ['dog', 'cat'] as PetType[],
    bannedPetBreeds: ['Pit Bull', 'Rottweiler'],
    maxPetWeight: 25,
    vehiclesRequireApproval: false,
    maxVehiclesPerUnit: 2,
    notes: 'All pet registrations must include vaccination records.',
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
  };

  const handleSubmit = (data: UpdateRegistryRulesRequest) => {
    console.log('Update rules:', data);
    // TODO: Implement actual API call
  };

  const handleCancel = () => {
    console.log('Cancel');
    // TODO: Navigate back or reset form
  };

  return (
    <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900">Registry Rules</h1>
        <p className="mt-2 text-gray-600">
          Configure pet and vehicle registration rules for your building
        </p>
      </div>

      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        <RegistryRulesForm
          initialData={mockRules}
          onSubmit={handleSubmit}
          onCancel={handleCancel}
        />
      </div>

      {/* Info Section */}
      <div className="mt-6 bg-blue-50 border border-blue-200 rounded-lg p-4">
        <div className="flex">
          <svg
            className="w-5 h-5 text-blue-400 mt-0.5"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <title>Info</title>
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
            />
          </svg>
          <div className="ml-3">
            <h3 className="text-sm font-medium text-blue-800">About Registry Rules</h3>
            <div className="mt-2 text-sm text-blue-700">
              <ul className="list-disc pl-5 space-y-1">
                <li>Rules apply to all new registrations after they are saved</li>
                <li>Existing approved registrations are not affected by rule changes</li>
                <li>If approval is required, managers will be notified of new registrations</li>
                <li>Residents will be notified if their registration is rejected</li>
              </ul>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

/**
 * RegistryPage Component
 *
 * Main page for building registries with tabs for pets and vehicles (Epic 57).
 */

import { useState } from 'react';
import { PetRegistrationList } from '../components/PetRegistrationList';
import { VehicleRegistrationList } from '../components/VehicleRegistrationList';

export function RegistryPage() {
  const [activeTab, setActiveTab] = useState<'pets' | 'vehicles'>('pets');

  // Mock data and handlers - replace with actual hooks in implementation
  const mockPetData = {
    registrations: [],
    total: 0,
    page: 1,
    pageSize: 10,
  };

  const mockVehicleData = {
    registrations: [],
    total: 0,
    page: 1,
    pageSize: 10,
  };

  // TODO: Wire up to API when registry endpoints are implemented
  const handlePetPageChange = (_page: number) => {
    // API: GET /api/v1/registrations/pets?page={page}
  };

  const handlePetStatusFilter = (_status?: string) => {
    // API: GET /api/v1/registrations/pets?status={status}
  };

  const handlePetSearchChange = (_search: string) => {
    // API: GET /api/v1/registrations/pets?search={search}
  };

  const handlePetView = (_id: string) => {
    // Navigate to pet detail page
  };

  const handlePetEdit = (_id: string) => {
    // Navigate to pet edit page
  };

  const handlePetDelete = (_id: string) => {
    // API: DELETE /api/v1/registrations/pets/{id}
  };

  const handlePetReview = (_id: string) => {
    // API: PATCH /api/v1/registrations/pets/{id}/review
  };

  const handlePetCreate = () => {
    // Navigate to pet create page
  };

  const handleVehiclePageChange = (_page: number) => {
    // API: GET /api/v1/registrations/vehicles?page={page}
  };

  const handleVehicleStatusFilter = (_status?: string) => {
    // API: GET /api/v1/registrations/vehicles?status={status}
  };

  const handleVehicleSearchChange = (_search: string) => {
    // API: GET /api/v1/registrations/vehicles?search={search}
  };

  const handleVehicleView = (_id: string) => {
    // Navigate to vehicle detail page
  };

  const handleVehicleEdit = (_id: string) => {
    // Navigate to vehicle edit page
  };

  const handleVehicleDelete = (_id: string) => {
    // API: DELETE /api/v1/registrations/vehicles/{id}
  };

  const handleVehicleReview = (_id: string) => {
    // API: PATCH /api/v1/registrations/vehicles/{id}/review
  };

  const handleVehicleCreate = () => {
    // Navigate to vehicle create page
  };

  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900">Building Registries</h1>
        <p className="mt-2 text-gray-600">Manage pet and vehicle registrations for your building</p>
      </div>

      {/* Tabs */}
      <div className="border-b border-gray-200 mb-6">
        <nav className="-mb-px flex space-x-8">
          <button
            type="button"
            onClick={() => setActiveTab('pets')}
            className={`${
              activeTab === 'pets'
                ? 'border-blue-500 text-blue-600'
                : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
            } whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm transition-colors`}
          >
            <div className="flex items-center gap-2">
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <title>Pets</title>
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M14.828 14.828a4 4 0 01-5.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                />
              </svg>
              Pets
            </div>
          </button>
          <button
            type="button"
            onClick={() => setActiveTab('vehicles')}
            className={`${
              activeTab === 'vehicles'
                ? 'border-blue-500 text-blue-600'
                : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
            } whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm transition-colors`}
          >
            <div className="flex items-center gap-2">
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <title>Vehicles</title>
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M8 7v8a2 2 0 002 2h6M8 7V5a2 2 0 012-2h4.586a1 1 0 01.707.293l4.414 4.414a1 1 0 01.293.707V15a2 2 0 01-2 2h-2M8 7H6a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2v-2"
                />
              </svg>
              Vehicles
            </div>
          </button>
        </nav>
      </div>

      {/* Tab Content */}
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        {activeTab === 'pets' && (
          <PetRegistrationList
            registrations={mockPetData.registrations}
            total={mockPetData.total}
            page={mockPetData.page}
            pageSize={mockPetData.pageSize}
            onPageChange={handlePetPageChange}
            onStatusFilter={handlePetStatusFilter}
            onSearchChange={handlePetSearchChange}
            onView={handlePetView}
            onEdit={handlePetEdit}
            onDelete={handlePetDelete}
            onReview={handlePetReview}
            onCreate={handlePetCreate}
          />
        )}

        {activeTab === 'vehicles' && (
          <VehicleRegistrationList
            registrations={mockVehicleData.registrations}
            total={mockVehicleData.total}
            page={mockVehicleData.page}
            pageSize={mockVehicleData.pageSize}
            onPageChange={handleVehiclePageChange}
            onStatusFilter={handleVehicleStatusFilter}
            onSearchChange={handleVehicleSearchChange}
            onView={handleVehicleView}
            onEdit={handleVehicleEdit}
            onDelete={handleVehicleDelete}
            onReview={handleVehicleReview}
            onCreate={handleVehicleCreate}
          />
        )}
      </div>
    </div>
  );
}

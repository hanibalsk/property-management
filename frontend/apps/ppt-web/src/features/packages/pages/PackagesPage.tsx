/**
 * PackagesPage
 *
 * Main page for package management (Epic 58, Story 58.1-58.3).
 */

import type { PackageStatus } from '@ppt/api-client';
import { useState } from 'react';
import { PackageCard } from '../components/PackageCard';

// Mock data for development - will be replaced with API calls
const mockPackages = [
  {
    id: '1',
    unitId: 'unit-1',
    unitNumber: '101',
    residentId: 'user-1',
    residentName: 'John Doe',
    trackingNumber: '1Z999AA10123456784',
    carrier: 'ups' as const,
    status: 'expected' as PackageStatus,
    expectedDate: '2024-01-15',
    createdAt: '2024-01-10T10:00:00Z',
  },
  {
    id: '2',
    unitId: 'unit-2',
    unitNumber: '205',
    residentId: 'user-2',
    residentName: 'Jane Smith',
    trackingNumber: '9400111899223534567890',
    carrier: 'usps' as const,
    status: 'received' as PackageStatus,
    receivedAt: '2024-01-12T14:30:00Z',
    createdAt: '2024-01-10T10:00:00Z',
  },
  {
    id: '3',
    unitId: 'unit-3',
    unitNumber: '310',
    residentId: 'user-3',
    residentName: 'Bob Johnson',
    carrier: 'amazon' as const,
    status: 'picked_up' as PackageStatus,
    receivedAt: '2024-01-11T09:00:00Z',
    createdAt: '2024-01-09T10:00:00Z',
  },
];

type FilterStatus = 'all' | PackageStatus;

export function PackagesPage() {
  const [filter, setFilter] = useState<FilterStatus>('all');

  const filteredPackages = mockPackages.filter((pkg) => filter === 'all' || pkg.status === filter);

  const handleView = (id: string) => {
    console.log('View package:', id);
  };

  const handleReceive = (id: string) => {
    console.log('Receive package:', id);
  };

  const handlePickup = (id: string) => {
    console.log('Pickup package:', id);
  };

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Package Management</h1>
          <p className="text-gray-600 mt-1">Track and manage building packages</p>
        </div>
        <button
          type="button"
          className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 flex items-center gap-2"
        >
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <title>Add</title>
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
          </svg>
          Register Package
        </button>
      </div>

      {/* Filters */}
      <div className="flex gap-2 mb-6">
        {(['all', 'expected', 'received', 'notified', 'picked_up', 'unclaimed'] as const).map(
          (status) => (
            <button
              key={status}
              type="button"
              onClick={() => setFilter(status)}
              className={`px-3 py-1.5 rounded-md text-sm font-medium transition-colors ${
                filter === status
                  ? 'bg-blue-600 text-white'
                  : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
              }`}
            >
              {status === 'all'
                ? 'All'
                : status === 'picked_up'
                  ? 'Picked Up'
                  : status.charAt(0).toUpperCase() + status.slice(1)}
            </button>
          )
        )}
      </div>

      {/* Package List */}
      <div className="space-y-4">
        {filteredPackages.length === 0 ? (
          <div className="text-center py-12 bg-gray-50 rounded-lg">
            <svg
              className="w-12 h-12 text-gray-400 mx-auto mb-4"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <title>No packages</title>
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4"
              />
            </svg>
            <p className="text-gray-500">No packages found</p>
          </div>
        ) : (
          filteredPackages.map((pkg) => (
            <PackageCard
              key={pkg.id}
              pkg={pkg}
              onView={handleView}
              onReceive={handleReceive}
              onPickup={handlePickup}
            />
          ))
        )}
      </div>
    </div>
  );
}

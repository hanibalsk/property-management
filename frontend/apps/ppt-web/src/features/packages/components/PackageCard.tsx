/**
 * PackageCard Component
 *
 * Displays a package summary with status and actions (Epic 58).
 */

import type { PackageStatus, PackageSummary } from '@ppt/api-client';

interface PackageCardProps {
  pkg: PackageSummary;
  onView: (id: string) => void;
  onReceive?: (id: string) => void;
  onPickup?: (id: string) => void;
}

const statusColors: Record<PackageStatus, string> = {
  expected: 'bg-blue-100 text-blue-800',
  received: 'bg-yellow-100 text-yellow-800',
  notified: 'bg-purple-100 text-purple-800',
  picked_up: 'bg-green-100 text-green-800',
  returned: 'bg-gray-100 text-gray-800',
  unclaimed: 'bg-red-100 text-red-800',
};

const statusLabels: Record<PackageStatus, string> = {
  expected: 'Expected',
  received: 'Received',
  notified: 'Notified',
  picked_up: 'Picked Up',
  returned: 'Returned',
  unclaimed: 'Unclaimed',
};

const carrierLabels: Record<string, string> = {
  usps: 'USPS',
  ups: 'UPS',
  fedex: 'FedEx',
  dhl: 'DHL',
  amazon: 'Amazon',
  other: 'Other',
};

export function PackageCard({ pkg, onView, onReceive, onPickup }: PackageCardProps) {
  const status = pkg.status as PackageStatus;
  const showReceive = status === 'expected' && onReceive;
  const showPickup = (status === 'received' || status === 'notified') && onPickup;

  return (
    <div className="bg-white border border-gray-200 rounded-lg p-4 hover:shadow-md transition-shadow">
      <div className="flex items-start justify-between">
        <div className="flex-1">
          <div className="flex items-center gap-2 mb-2">
            <span
              className={`px-2 py-0.5 text-xs font-medium rounded-full ${statusColors[status]}`}
            >
              {statusLabels[status]}
            </span>
            <span className="text-sm font-medium text-gray-700">
              {carrierLabels[pkg.carrier] || pkg.carrier}
            </span>
          </div>

          {pkg.trackingNumber && (
            <p className="text-sm text-gray-900 font-mono mb-1">#{pkg.trackingNumber}</p>
          )}

          <div className="flex flex-wrap gap-3 text-sm text-gray-500 mt-2">
            <span className="flex items-center gap-1">
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <title>Unit</title>
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"
                />
              </svg>
              {pkg.unitNumber || 'N/A'}
            </span>
            <span className="flex items-center gap-1">
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <title>Resident</title>
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
                />
              </svg>
              {pkg.residentName || 'Unknown'}
            </span>
            {pkg.expectedDate && (
              <span className="flex items-center gap-1">
                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <title>Expected</title>
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
                  />
                </svg>
                {new Date(pkg.expectedDate).toLocaleDateString()}
              </span>
            )}
            {pkg.receivedAt && (
              <span className="flex items-center gap-1">
                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <title>Received</title>
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M5 13l4 4L19 7"
                  />
                </svg>
                {new Date(pkg.receivedAt).toLocaleString()}
              </span>
            )}
          </div>
        </div>

        {/* Actions */}
        <div className="flex items-center gap-2 ml-4">
          <button
            type="button"
            onClick={() => onView(pkg.id)}
            className="p-2 text-gray-400 hover:text-gray-600 rounded-md hover:bg-gray-100"
            title="View Details"
          >
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <title>View</title>
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
              />
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
              />
            </svg>
          </button>

          {showReceive && (
            <button
              type="button"
              onClick={() => onReceive(pkg.id)}
              className="p-2 text-yellow-500 hover:text-yellow-700 rounded-md hover:bg-yellow-50"
              title="Log Receipt"
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <title>Receive</title>
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M20 7l-8 8-4-4M4 7v10a2 2 0 002 2h12a2 2 0 002-2V9"
                />
              </svg>
            </button>
          )}

          {showPickup && (
            <button
              type="button"
              onClick={() => onPickup(pkg.id)}
              className="p-2 text-green-500 hover:text-green-700 rounded-md hover:bg-green-50"
              title="Log Pickup"
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <title>Pickup</title>
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M5 13l4 4L19 7"
                />
              </svg>
            </button>
          )}
        </div>
      </div>
    </div>
  );
}

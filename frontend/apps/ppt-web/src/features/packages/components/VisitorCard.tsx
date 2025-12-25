/**
 * VisitorCard Component
 *
 * Displays a visitor summary with access code and actions (Epic 58).
 */

import type { VisitorPurpose, VisitorStatus, VisitorSummary } from '@ppt/api-client';

interface VisitorCardProps {
  visitor: VisitorSummary;
  onView: (id: string) => void;
  onCheckIn?: (id: string) => void;
  onCheckOut?: (id: string) => void;
  onCancel?: (id: string) => void;
}

const statusColors: Record<VisitorStatus, string> = {
  pending: 'bg-blue-100 text-blue-800',
  checked_in: 'bg-green-100 text-green-800',
  checked_out: 'bg-gray-100 text-gray-800',
  expired: 'bg-red-100 text-red-800',
  cancelled: 'bg-gray-100 text-gray-500',
};

const statusLabels: Record<VisitorStatus, string> = {
  pending: 'Expected',
  checked_in: 'Checked In',
  checked_out: 'Checked Out',
  expired: 'Expired',
  cancelled: 'Cancelled',
};

const purposeLabels: Record<VisitorPurpose, string> = {
  guest: 'Guest',
  delivery: 'Delivery',
  service: 'Service',
  contractor: 'Contractor',
  real_estate: 'Real Estate',
  other: 'Other',
};

export function VisitorCard({
  visitor,
  onView,
  onCheckIn,
  onCheckOut,
  onCancel,
}: VisitorCardProps) {
  const status = visitor.status as VisitorStatus;
  const purpose = visitor.purpose as VisitorPurpose;
  const showCheckIn = status === 'pending' && onCheckIn;
  const showCheckOut = status === 'checked_in' && onCheckOut;
  const showCancel = status === 'pending' && onCancel;

  return (
    <div className="bg-white border border-gray-200 rounded-lg p-4 hover:shadow-md transition-shadow">
      <div className="flex items-start justify-between">
        <div className="flex-1">
          <div className="flex items-center gap-2 mb-2">
            <h3 className="text-lg font-medium text-gray-900">{visitor.visitorName}</h3>
            <span
              className={`px-2 py-0.5 text-xs font-medium rounded-full ${statusColors[status]}`}
            >
              {statusLabels[status]}
            </span>
          </div>

          <div className="flex flex-wrap gap-3 text-sm text-gray-500 mb-3">
            <span className="flex items-center gap-1">
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <title>Purpose</title>
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"
                />
              </svg>
              {purposeLabels[purpose]}
            </span>
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
              {visitor.unitNumber || 'N/A'}
            </span>
            <span className="flex items-center gap-1">
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <title>Host</title>
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
                />
              </svg>
              {visitor.hostName || 'Unknown Host'}
            </span>
            <span className="flex items-center gap-1">
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <title>Expected Arrival</title>
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
                />
              </svg>
              {new Date(visitor.expectedArrival).toLocaleString()}
            </span>
          </div>

          {/* Access Code Display */}
          <div className="bg-gray-50 rounded-md px-3 py-2 inline-block">
            <span className="text-xs text-gray-500 block">Access Code</span>
            <span className="text-xl font-mono font-bold tracking-wider text-gray-900">
              {visitor.accessCode}
            </span>
          </div>
        </div>

        {/* Actions */}
        <div className="flex flex-col gap-2 ml-4">
          <button
            type="button"
            onClick={() => onView(visitor.id)}
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

          {showCheckIn && (
            <button
              type="button"
              onClick={() => onCheckIn(visitor.id)}
              className="p-2 text-green-500 hover:text-green-700 rounded-md hover:bg-green-50"
              title="Check In"
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <title>Check In</title>
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M11 16l-4-4m0 0l4-4m-4 4h14m-5 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h7a3 3 0 013 3v1"
                />
              </svg>
            </button>
          )}

          {showCheckOut && (
            <button
              type="button"
              onClick={() => onCheckOut(visitor.id)}
              className="p-2 text-blue-500 hover:text-blue-700 rounded-md hover:bg-blue-50"
              title="Check Out"
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <title>Check Out</title>
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"
                />
              </svg>
            </button>
          )}

          {showCancel && (
            <button
              type="button"
              onClick={() => onCancel(visitor.id)}
              className="p-2 text-red-400 hover:text-red-600 rounded-md hover:bg-red-50"
              title="Cancel"
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <title>Cancel</title>
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M6 18L18 6M6 6l12 12"
                />
              </svg>
            </button>
          )}
        </div>
      </div>
    </div>
  );
}

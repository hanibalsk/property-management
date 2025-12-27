/**
 * Developer Account Card Component (Epic 69)
 *
 * Displays developer account information and settings.
 */

import type { DeveloperAccount, UpdateDeveloperAccount } from '../types';

interface DeveloperAccountCardProps {
  account: DeveloperAccount;
  onEdit?: () => void;
}

export function DeveloperAccountCard({ account, onEdit }: DeveloperAccountCardProps) {
  return (
    <div className="p-6 bg-white rounded-lg border shadow-sm">
      <div className="flex items-start justify-between mb-4">
        <div>
          <h3 className="text-lg font-semibold">Developer Account</h3>
          <p className="text-sm text-muted-foreground">
            Manage your API access settings
          </p>
        </div>
        {onEdit && (
          <button
            type="button"
            onClick={onEdit}
            className="p-2 text-gray-600 hover:bg-gray-100 rounded-md transition-colors"
          >
            <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
              />
            </svg>
          </button>
        )}
      </div>

      {/* Status Badges */}
      <div className="flex gap-2 mb-4">
        {account.isVerified ? (
          <span className="inline-flex items-center gap-1 px-2 py-1 text-xs font-medium bg-green-100 text-green-800 rounded-full">
            <svg className="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
              <path
                fillRule="evenodd"
                d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                clipRule="evenodd"
              />
            </svg>
            Verified
          </span>
        ) : (
          <span className="inline-flex items-center px-2 py-1 text-xs font-medium bg-yellow-100 text-yellow-800 rounded-full">
            Pending Verification
          </span>
        )}
        <span className="inline-flex items-center px-2 py-1 text-xs font-medium bg-blue-100 text-blue-800 rounded-full capitalize">
          {account.tier} Tier
        </span>
        {account.isActive ? (
          <span className="inline-flex items-center px-2 py-1 text-xs font-medium bg-green-100 text-green-800 rounded-full">
            Active
          </span>
        ) : (
          <span className="inline-flex items-center px-2 py-1 text-xs font-medium bg-red-100 text-red-800 rounded-full">
            Inactive
          </span>
        )}
      </div>

      {/* Account Details */}
      <dl className="grid gap-3 text-sm">
        {account.companyName && (
          <div className="flex justify-between">
            <dt className="text-muted-foreground">Company</dt>
            <dd className="font-medium">{account.companyName}</dd>
          </div>
        )}
        <div className="flex justify-between">
          <dt className="text-muted-foreground">Contact Email</dt>
          <dd className="font-medium">{account.contactEmail}</dd>
        </div>
        {account.contactName && (
          <div className="flex justify-between">
            <dt className="text-muted-foreground">Contact Name</dt>
            <dd className="font-medium">{account.contactName}</dd>
          </div>
        )}
        {account.website && (
          <div className="flex justify-between">
            <dt className="text-muted-foreground">Website</dt>
            <dd>
              <a
                href={account.website}
                target="_blank"
                rel="noopener noreferrer"
                className="font-medium text-blue-600 hover:underline"
              >
                {account.website}
              </a>
            </dd>
          </div>
        )}
        <div className="flex justify-between">
          <dt className="text-muted-foreground">Member Since</dt>
          <dd className="font-medium">{formatDate(account.createdAt)}</dd>
        </div>
        {account.verifiedAt && (
          <div className="flex justify-between">
            <dt className="text-muted-foreground">Verified On</dt>
            <dd className="font-medium">{formatDate(account.verifiedAt)}</dd>
          </div>
        )}
      </dl>

      {/* Description */}
      {account.description && (
        <div className="mt-4 pt-4 border-t">
          <h4 className="text-sm font-medium mb-1">Description</h4>
          <p className="text-sm text-muted-foreground">{account.description}</p>
        </div>
      )}
    </div>
  );
}

function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  });
}

// Usage Summary Card
interface UsageSummaryCardProps {
  apiKeysCount: number;
  webhooksCount: number;
  totalRequestsToday: number;
  totalRequestsMonth: number;
}

export function UsageSummaryCard({
  apiKeysCount,
  webhooksCount,
  totalRequestsToday,
  totalRequestsMonth,
}: UsageSummaryCardProps) {
  return (
    <div className="p-6 bg-white rounded-lg border shadow-sm">
      <h3 className="text-lg font-semibold mb-4">Usage Summary</h3>

      <div className="grid grid-cols-2 gap-4">
        <StatItem label="API Keys" value={apiKeysCount.toString()} />
        <StatItem label="Webhooks" value={webhooksCount.toString()} />
        <StatItem label="Requests Today" value={formatNumber(totalRequestsToday)} />
        <StatItem label="Requests This Month" value={formatNumber(totalRequestsMonth)} />
      </div>
    </div>
  );
}

function StatItem({ label, value }: { label: string; value: string }) {
  return (
    <div className="p-3 bg-gray-50 rounded-md">
      <p className="text-xs text-muted-foreground">{label}</p>
      <p className="text-xl font-bold">{value}</p>
    </div>
  );
}

function formatNumber(num: number): string {
  if (num >= 1000000) {
    return `${(num / 1000000).toFixed(1)}M`;
  }
  if (num >= 1000) {
    return `${(num / 1000).toFixed(1)}K`;
  }
  return num.toLocaleString();
}

// Stub for SDK install instructions
export function SdkInstallInstructions() {
  return <div>SDK Install Instructions</div>;
}

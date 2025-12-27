/**
 * Migration Overview Page (Epic 66).
 *
 * Main dashboard for data migration features including import and export.
 */

import { useCallback, useState } from 'react';

type MigrationTab = 'overview' | 'import' | 'export' | 'templates';

interface QuickStat {
  label: string;
  value: string;
  trend?: 'up' | 'down' | 'neutral';
  trendValue?: string;
}

const QUICK_STATS: QuickStat[] = [
  { label: 'Total Imports', value: '24', trend: 'up', trendValue: '+3 this month' },
  { label: 'Records Imported', value: '15,234' },
  { label: 'Total Exports', value: '8' },
  { label: 'Last Activity', value: '2 hours ago' },
];

interface RecentActivity {
  id: string;
  type: 'import' | 'export';
  description: string;
  status: 'completed' | 'failed' | 'in_progress';
  timestamp: string;
}

const RECENT_ACTIVITIES: RecentActivity[] = [
  {
    id: '1',
    type: 'import',
    description: 'Buildings data import - 45 records',
    status: 'completed',
    timestamp: '2 hours ago',
  },
  {
    id: '2',
    type: 'export',
    description: 'Full organization export',
    status: 'completed',
    timestamp: '1 day ago',
  },
  {
    id: '3',
    type: 'import',
    description: 'Residents import - 120 records',
    status: 'completed',
    timestamp: '3 days ago',
  },
  {
    id: '4',
    type: 'import',
    description: 'Financial transactions',
    status: 'failed',
    timestamp: '5 days ago',
  },
];

interface MigrationPageProps {
  onNavigate?: (tab: MigrationTab) => void;
}

export function MigrationPage({ onNavigate }: MigrationPageProps) {
  const [activeTab, setActiveTab] = useState<MigrationTab>('overview');

  const handleNavigate = useCallback(
    (tab: MigrationTab) => {
      setActiveTab(tab);
      onNavigate?.(tab);
    },
    [onNavigate]
  );

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div className="border-b border-gray-200 pb-4">
        <h1 className="text-2xl font-bold text-gray-900">Data Migration</h1>
        <p className="mt-1 text-sm text-gray-500">
          Import data from legacy systems or export for migration to other platforms.
        </p>
      </div>

      {/* Tab Navigation */}
      <div className="border-b border-gray-200">
        <nav className="-mb-px flex gap-6">
          {[
            { id: 'overview', label: 'Overview' },
            { id: 'import', label: 'Import Data' },
            { id: 'export', label: 'Export Data' },
            { id: 'templates', label: 'Templates' },
          ].map((tab) => (
            <button
              key={tab.id}
              type="button"
              onClick={() => handleNavigate(tab.id as MigrationTab)}
              className={`border-b-2 px-1 py-3 text-sm font-medium ${
                activeTab === tab.id
                  ? 'border-blue-500 text-blue-600'
                  : 'border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700'
              }`}
            >
              {tab.label}
            </button>
          ))}
        </nav>
      </div>

      {/* Overview Content */}
      {activeTab === 'overview' && (
        <div className="space-y-6">
          {/* Quick Stats */}
          <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
            {QUICK_STATS.map((stat) => (
              <div key={stat.label} className="rounded-lg bg-white p-4 shadow-sm ring-1 ring-gray-200">
                <p className="text-sm text-gray-500">{stat.label}</p>
                <p className="mt-1 text-2xl font-semibold text-gray-900">{stat.value}</p>
                {stat.trendValue && (
                  <p
                    className={`mt-1 text-xs ${
                      stat.trend === 'up'
                        ? 'text-green-600'
                        : stat.trend === 'down'
                        ? 'text-red-600'
                        : 'text-gray-500'
                    }`}
                  >
                    {stat.trendValue}
                  </p>
                )}
              </div>
            ))}
          </div>

          {/* Quick Actions */}
          <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
            <button
              type="button"
              onClick={() => handleNavigate('import')}
              className="flex items-center gap-4 rounded-lg border border-gray-200 bg-white p-4 text-left shadow-sm transition-shadow hover:shadow-md"
            >
              <div className="flex h-12 w-12 items-center justify-center rounded-lg bg-blue-100">
                <svg
                  className="h-6 w-6 text-blue-600"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"
                  />
                </svg>
              </div>
              <div>
                <p className="font-medium text-gray-900">Import Data</p>
                <p className="text-sm text-gray-500">Upload spreadsheets to import</p>
              </div>
            </button>

            <button
              type="button"
              onClick={() => handleNavigate('export')}
              className="flex items-center gap-4 rounded-lg border border-gray-200 bg-white p-4 text-left shadow-sm transition-shadow hover:shadow-md"
            >
              <div className="flex h-12 w-12 items-center justify-center rounded-lg bg-green-100">
                <svg
                  className="h-6 w-6 text-green-600"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
                  />
                </svg>
              </div>
              <div>
                <p className="font-medium text-gray-900">Export Data</p>
                <p className="text-sm text-gray-500">Download your organization data</p>
              </div>
            </button>

            <button
              type="button"
              onClick={() => handleNavigate('templates')}
              className="flex items-center gap-4 rounded-lg border border-gray-200 bg-white p-4 text-left shadow-sm transition-shadow hover:shadow-md"
            >
              <div className="flex h-12 w-12 items-center justify-center rounded-lg bg-purple-100">
                <svg
                  className="h-6 w-6 text-purple-600"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                  />
                </svg>
              </div>
              <div>
                <p className="font-medium text-gray-900">Manage Templates</p>
                <p className="text-sm text-gray-500">Create and download templates</p>
              </div>
            </button>
          </div>

          {/* Recent Activity */}
          <div className="rounded-lg bg-white shadow-sm ring-1 ring-gray-200">
            <div className="border-b border-gray-200 px-4 py-3">
              <h2 className="text-sm font-medium text-gray-900">Recent Activity</h2>
            </div>
            <div className="divide-y divide-gray-200">
              {RECENT_ACTIVITIES.map((activity) => (
                <div key={activity.id} className="flex items-center justify-between px-4 py-3">
                  <div className="flex items-center gap-3">
                    <div
                      className={`flex h-8 w-8 items-center justify-center rounded-full ${
                        activity.type === 'import' ? 'bg-blue-100' : 'bg-green-100'
                      }`}
                    >
                      {activity.type === 'import' ? (
                        <svg
                          className="h-4 w-4 text-blue-600"
                          fill="none"
                          stroke="currentColor"
                          viewBox="0 0 24 24"
                        >
                          <path
                            strokeLinecap="round"
                            strokeLinejoin="round"
                            strokeWidth={2}
                            d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"
                          />
                        </svg>
                      ) : (
                        <svg
                          className="h-4 w-4 text-green-600"
                          fill="none"
                          stroke="currentColor"
                          viewBox="0 0 24 24"
                        >
                          <path
                            strokeLinecap="round"
                            strokeLinejoin="round"
                            strokeWidth={2}
                            d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
                          />
                        </svg>
                      )}
                    </div>
                    <div>
                      <p className="text-sm text-gray-900">{activity.description}</p>
                      <p className="text-xs text-gray-500">{activity.timestamp}</p>
                    </div>
                  </div>
                  <span
                    className={`inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium ${
                      activity.status === 'completed'
                        ? 'bg-green-100 text-green-800'
                        : activity.status === 'failed'
                        ? 'bg-red-100 text-red-800'
                        : 'bg-blue-100 text-blue-800'
                    }`}
                  >
                    {activity.status === 'completed'
                      ? 'Completed'
                      : activity.status === 'failed'
                      ? 'Failed'
                      : 'In Progress'}
                  </span>
                </div>
              ))}
            </div>
          </div>

          {/* Help Section */}
          <div className="rounded-lg bg-blue-50 p-4">
            <div className="flex">
              <svg
                className="h-5 w-5 text-blue-400"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                />
              </svg>
              <div className="ml-3">
                <h3 className="text-sm font-medium text-blue-800">Migrating from another system?</h3>
                <p className="mt-1 text-sm text-blue-700">
                  Download our import templates, fill them with your data, and upload to import.
                  Our validation will help identify any issues before importing.
                </p>
                <button
                  type="button"
                  onClick={() => handleNavigate('templates')}
                  className="mt-2 text-sm font-medium text-blue-800 hover:text-blue-900"
                >
                  View import templates
                </button>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Import Tab Content */}
      {activeTab === 'import' && (
        <div className="rounded-lg border border-gray-200 bg-white p-6">
          <p className="text-gray-500">Import page content - use ImportPage component</p>
        </div>
      )}

      {/* Export Tab Content */}
      {activeTab === 'export' && (
        <div className="rounded-lg border border-gray-200 bg-white p-6">
          <p className="text-gray-500">Export page content - use ExportPage component</p>
        </div>
      )}

      {/* Templates Tab Content */}
      {activeTab === 'templates' && (
        <div className="rounded-lg border border-gray-200 bg-white p-6">
          <p className="text-gray-500">Templates page content - use TemplatesPage component</p>
        </div>
      )}
    </div>
  );
}

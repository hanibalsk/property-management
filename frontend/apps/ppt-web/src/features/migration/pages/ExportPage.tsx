/**
 * Export Page (Epic 66, Story 66.3).
 *
 * Main page for data export and migration out.
 */

import { useCallback, useState } from 'react';
import {
  ExportCategorySelector,
  type ExportCategoryInfo,
  type ExportDataCategory,
  type ExportPrivacyOptions,
} from '../components/ExportCategorySelector';
import { ExportProgress, type ExportStatusData } from '../components/ExportProgress';

type ExportStep = 'select' | 'exporting' | 'complete';

// Mock data for demonstration
const MOCK_CATEGORIES: ExportCategoryInfo[] = [
  {
    id: 'buildings',
    name: 'Buildings',
    description: 'Building master data including addresses and details',
    recordCount: 45,
    containsPersonalData: false,
  },
  {
    id: 'units',
    name: 'Units',
    description: 'Individual units within buildings',
    recordCount: 320,
    containsPersonalData: false,
  },
  {
    id: 'residents',
    name: 'Residents',
    description: 'Resident and owner information',
    recordCount: 580,
    containsPersonalData: true,
  },
  {
    id: 'financials',
    name: 'Financials',
    description: 'Financial transactions and balances',
    recordCount: 12500,
    containsPersonalData: true,
  },
  {
    id: 'faults',
    name: 'Faults',
    description: 'Fault reports and maintenance issues',
    recordCount: 890,
    containsPersonalData: false,
  },
  {
    id: 'documents',
    name: 'Documents',
    description: 'Document metadata (not file contents)',
    recordCount: 2340,
    containsPersonalData: true,
  },
  {
    id: 'votes',
    name: 'Votes',
    description: 'Voting history and results',
    recordCount: 156,
    containsPersonalData: true,
  },
  {
    id: 'meters',
    name: 'Meters',
    description: 'Utility meters and readings',
    recordCount: 640,
    containsPersonalData: false,
  },
];

interface ExportHistoryItem {
  id: string;
  status: 'ready' | 'expired' | 'downloaded';
  categories: string[];
  fileSizeBytes: number;
  createdAt: string;
  expiresAt: string;
  downloadCount: number;
}

const MOCK_HISTORY: ExportHistoryItem[] = [
  {
    id: '1',
    status: 'ready',
    categories: ['buildings', 'units'],
    fileSizeBytes: 5234567,
    createdAt: new Date(Date.now() - 3 * 86400000).toISOString(),
    expiresAt: new Date(Date.now() + 4 * 86400000).toISOString(),
    downloadCount: 2,
  },
  {
    id: '2',
    status: 'expired',
    categories: ['buildings', 'units', 'residents', 'financials'],
    fileSizeBytes: 25678901,
    createdAt: new Date(Date.now() - 14 * 86400000).toISOString(),
    expiresAt: new Date(Date.now() - 7 * 86400000).toISOString(),
    downloadCount: 1,
  },
];

export function ExportPage() {
  const [step, setStep] = useState<ExportStep>('select');
  const [currentExportId, setCurrentExportId] = useState<string | null>(null);
  const [showHistory, setShowHistory] = useState(false);

  // Handle export start
  const handleExport = useCallback((categories: ExportDataCategory[], privacyOptions: ExportPrivacyOptions) => {
    // In real implementation, call API
    const exportId = crypto.randomUUID();
    setCurrentExportId(exportId);
    setStep('exporting');
  }, []);

  // Handle export completion
  const handleExportComplete = useCallback(() => {
    setStep('complete');
  }, []);

  // Handle starting a new export
  const handleStartNew = useCallback(() => {
    setStep('select');
    setCurrentExportId(null);
  }, []);

  const formatFileSize = (bytes: number): string => {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
  };

  const formatDate = (dateString: string): string => {
    return new Intl.DateTimeFormat('en-US', {
      month: 'short',
      day: 'numeric',
      year: 'numeric',
    }).format(new Date(dateString));
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-xl font-semibold text-gray-900">Export Data</h1>
          <p className="mt-1 text-sm text-gray-500">
            Export your organization data for migration or backup purposes.
          </p>
        </div>
        <button
          type="button"
          onClick={() => setShowHistory(!showHistory)}
          className="text-sm font-medium text-blue-600 hover:text-blue-700"
        >
          {showHistory ? 'Hide History' : 'View History'}
        </button>
      </div>

      {/* Main Content */}
      <div className="rounded-lg border border-gray-200 bg-white p-6">
        {/* Step 1: Select Categories */}
        {step === 'select' && (
          <ExportCategorySelector
            categories={MOCK_CATEGORIES}
            onExport={handleExport}
            onCancel={() => {}}
          />
        )}

        {/* Step 2: Exporting */}
        {step === 'exporting' && currentExportId && (
          <ExportProgress
            exportId={currentExportId}
            onComplete={handleExportComplete}
          />
        )}

        {/* Step 3: Complete */}
        {step === 'complete' && currentExportId && (
          <div className="space-y-6">
            <ExportProgress exportId={currentExportId} />
            <div className="flex justify-center">
              <button
                type="button"
                onClick={handleStartNew}
                className="inline-flex items-center rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50"
              >
                Create New Export
              </button>
            </div>
          </div>
        )}
      </div>

      {/* Export History */}
      {showHistory && (
        <div className="rounded-lg border border-gray-200 bg-white">
          <div className="border-b border-gray-200 px-4 py-3">
            <h2 className="text-sm font-medium text-gray-900">Export History</h2>
          </div>
          {MOCK_HISTORY.length === 0 ? (
            <div className="py-8 text-center text-sm text-gray-500">
              No previous exports found.
            </div>
          ) : (
            <table className="min-w-full divide-y divide-gray-200">
              <thead className="bg-gray-50">
                <tr>
                  <th className="px-4 py-3 text-left text-xs font-medium uppercase text-gray-500">
                    Categories
                  </th>
                  <th className="px-4 py-3 text-left text-xs font-medium uppercase text-gray-500">
                    Status
                  </th>
                  <th className="px-4 py-3 text-left text-xs font-medium uppercase text-gray-500">
                    Size
                  </th>
                  <th className="px-4 py-3 text-left text-xs font-medium uppercase text-gray-500">
                    Created
                  </th>
                  <th className="px-4 py-3 text-left text-xs font-medium uppercase text-gray-500">
                    Expires
                  </th>
                  <th className="px-4 py-3 text-right text-xs font-medium uppercase text-gray-500">
                    Actions
                  </th>
                </tr>
              </thead>
              <tbody className="divide-y divide-gray-200 bg-white">
                {MOCK_HISTORY.map((item) => (
                  <tr key={item.id}>
                    <td className="px-4 py-3">
                      <div className="flex flex-wrap gap-1">
                        {item.categories.slice(0, 3).map((cat) => (
                          <span
                            key={cat}
                            className="rounded bg-gray-100 px-1.5 py-0.5 text-xs capitalize text-gray-600"
                          >
                            {cat}
                          </span>
                        ))}
                        {item.categories.length > 3 && (
                          <span className="text-xs text-gray-500">
                            +{item.categories.length - 3} more
                          </span>
                        )}
                      </div>
                    </td>
                    <td className="px-4 py-3">
                      <span
                        className={`inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium ${
                          item.status === 'ready'
                            ? 'bg-green-100 text-green-800'
                            : item.status === 'downloaded'
                            ? 'bg-blue-100 text-blue-800'
                            : 'bg-gray-100 text-gray-800'
                        }`}
                      >
                        {item.status === 'ready'
                          ? 'Ready'
                          : item.status === 'downloaded'
                          ? 'Downloaded'
                          : 'Expired'}
                      </span>
                    </td>
                    <td className="px-4 py-3 text-sm text-gray-600">
                      {formatFileSize(item.fileSizeBytes)}
                    </td>
                    <td className="px-4 py-3 text-sm text-gray-600">
                      {formatDate(item.createdAt)}
                    </td>
                    <td className="px-4 py-3 text-sm text-gray-600">
                      {formatDate(item.expiresAt)}
                    </td>
                    <td className="px-4 py-3 text-right">
                      {item.status === 'ready' ? (
                        <button
                          type="button"
                          className="text-sm font-medium text-blue-600 hover:text-blue-700"
                        >
                          Download
                        </button>
                      ) : (
                        <span className="text-sm text-gray-400">-</span>
                      )}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </div>
      )}

      {/* Info Section */}
      <div className="rounded-lg bg-gray-50 p-4">
        <h3 className="text-sm font-medium text-gray-900">About Data Export</h3>
        <ul className="mt-2 space-y-1 text-sm text-gray-600">
          <li>- Exports are packaged as ZIP files containing CSV files for each category</li>
          <li>- Download links expire after 7 days</li>
          <li>- Large exports may take several minutes to generate</li>
          <li>- Consider using privacy options when exporting personal data (GDPR compliance)</li>
        </ul>
      </div>
    </div>
  );
}

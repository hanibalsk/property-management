/**
 * Export Category Selector Component (Story 66.3).
 *
 * Allows users to select data categories for migration export.
 */

import { useCallback, useState } from 'react';

export type ExportDataCategory =
  | 'buildings'
  | 'units'
  | 'residents'
  | 'financials'
  | 'faults'
  | 'documents'
  | 'votes'
  | 'announcements'
  | 'meters'
  | 'leases'
  | 'vendors'
  | 'work_orders';

export interface ExportCategoryInfo {
  id: ExportDataCategory;
  name: string;
  description: string;
  recordCount: number;
  containsPersonalData: boolean;
}

export interface ExportPrivacyOptions {
  anonymizePersonalData: boolean;
  maskFinancialData: boolean;
  excludeDocumentContents: boolean;
  hashIdentifiers: boolean;
}

interface ExportCategorySelectorProps {
  categories: ExportCategoryInfo[];
  onExport: (categories: ExportDataCategory[], privacyOptions: ExportPrivacyOptions) => void;
  onCancel: () => void;
  isLoading?: boolean;
}

export function ExportCategorySelector({
  categories,
  onExport,
  onCancel,
  isLoading = false,
}: ExportCategorySelectorProps) {
  const [selectedCategories, setSelectedCategories] = useState<Set<ExportDataCategory>>(new Set());
  const [privacyOptions, setPrivacyOptions] = useState<ExportPrivacyOptions>({
    anonymizePersonalData: false,
    maskFinancialData: false,
    excludeDocumentContents: false,
    hashIdentifiers: false,
  });
  const [showPrivacyOptions, setShowPrivacyOptions] = useState(false);

  const handleToggleCategory = useCallback((categoryId: ExportDataCategory) => {
    setSelectedCategories((prev) => {
      const next = new Set(prev);
      if (next.has(categoryId)) {
        next.delete(categoryId);
      } else {
        next.add(categoryId);
      }
      return next;
    });
  }, []);

  const handleSelectAll = useCallback(() => {
    setSelectedCategories(new Set(categories.map((c) => c.id)));
  }, [categories]);

  const handleSelectNone = useCallback(() => {
    setSelectedCategories(new Set());
  }, []);

  const handleExport = useCallback(() => {
    if (selectedCategories.size === 0) return;
    onExport(Array.from(selectedCategories), privacyOptions);
  }, [selectedCategories, privacyOptions, onExport]);

  const selectedHasPersonalData = categories.some(
    (c) => selectedCategories.has(c.id) && c.containsPersonalData
  );

  const totalRecords = categories
    .filter((c) => selectedCategories.has(c.id))
    .reduce((sum, c) => sum + c.recordCount, 0);

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="border-b border-gray-200 pb-4">
        <h2 className="text-lg font-medium text-gray-900">Export Data for Migration</h2>
        <p className="mt-1 text-sm text-gray-500">
          Select the data categories you want to export. The export will be packaged as a ZIP file
          with CSV files for each category.
        </p>
      </div>

      {/* Quick Actions */}
      <div className="flex items-center gap-4">
        <button
          type="button"
          onClick={handleSelectAll}
          className="text-sm font-medium text-blue-600 hover:text-blue-700"
        >
          Select All
        </button>
        <button
          type="button"
          onClick={handleSelectNone}
          className="text-sm font-medium text-gray-600 hover:text-gray-700"
        >
          Clear Selection
        </button>
        <span className="text-sm text-gray-500">
          {selectedCategories.size} of {categories.length} selected
        </span>
      </div>

      {/* Category Grid */}
      <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
        {categories.map((category) => (
          <label
            key={category.id}
            className={`relative flex cursor-pointer rounded-lg border p-4 transition-colors ${
              selectedCategories.has(category.id)
                ? 'border-blue-500 bg-blue-50 ring-1 ring-blue-500'
                : 'border-gray-200 hover:border-gray-300'
            }`}
          >
            <input
              type="checkbox"
              checked={selectedCategories.has(category.id)}
              onChange={() => handleToggleCategory(category.id)}
              className="sr-only"
            />
            <div className="flex-1">
              <div className="flex items-center justify-between">
                <span className="font-medium text-gray-900">{category.name}</span>
                {category.containsPersonalData && (
                  <span className="rounded bg-yellow-100 px-1.5 py-0.5 text-xs text-yellow-700">
                    Personal Data
                  </span>
                )}
              </div>
              <p className="mt-1 text-sm text-gray-500">{category.description}</p>
              <p className="mt-2 text-xs text-gray-400">
                {category.recordCount.toLocaleString()} records
              </p>
            </div>
            {selectedCategories.has(category.id) && (
              <svg
                className="absolute right-3 top-3 h-5 w-5 text-blue-600"
                fill="currentColor"
                viewBox="0 0 20 20"
              >
                <path
                  fillRule="evenodd"
                  d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                  clipRule="evenodd"
                />
              </svg>
            )}
          </label>
        ))}
      </div>

      {/* Privacy Options */}
      {selectedHasPersonalData && (
        <div className="rounded-lg border border-yellow-200 bg-yellow-50 p-4">
          <div className="flex items-start">
            <svg
              className="h-5 w-5 text-yellow-400"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
              />
            </svg>
            <div className="ml-3 flex-1">
              <h3 className="text-sm font-medium text-yellow-800">
                Selected categories contain personal data
              </h3>
              <p className="mt-1 text-sm text-yellow-700">
                Consider GDPR compliance when exporting personal data.
              </p>
              <button
                type="button"
                onClick={() => setShowPrivacyOptions(!showPrivacyOptions)}
                className="mt-2 text-sm font-medium text-yellow-800 hover:text-yellow-900"
              >
                {showPrivacyOptions ? 'Hide privacy options' : 'Configure privacy options'}
              </button>
            </div>
          </div>

          {showPrivacyOptions && (
            <div className="mt-4 space-y-3 border-t border-yellow-200 pt-4">
              <label className="flex items-start gap-3">
                <input
                  type="checkbox"
                  checked={privacyOptions.anonymizePersonalData}
                  onChange={(e) =>
                    setPrivacyOptions((prev) => ({
                      ...prev,
                      anonymizePersonalData: e.target.checked,
                    }))
                  }
                  className="mt-0.5 h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                />
                <div>
                  <span className="text-sm font-medium text-gray-900">Anonymize personal data</span>
                  <p className="text-xs text-gray-500">
                    Replace names, emails, and phone numbers with anonymized values
                  </p>
                </div>
              </label>

              <label className="flex items-start gap-3">
                <input
                  type="checkbox"
                  checked={privacyOptions.maskFinancialData}
                  onChange={(e) =>
                    setPrivacyOptions((prev) => ({
                      ...prev,
                      maskFinancialData: e.target.checked,
                    }))
                  }
                  className="mt-0.5 h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                />
                <div>
                  <span className="text-sm font-medium text-gray-900">Mask financial data</span>
                  <p className="text-xs text-gray-500">
                    Hide bank account numbers and sensitive financial identifiers
                  </p>
                </div>
              </label>

              <label className="flex items-start gap-3">
                <input
                  type="checkbox"
                  checked={privacyOptions.excludeDocumentContents}
                  onChange={(e) =>
                    setPrivacyOptions((prev) => ({
                      ...prev,
                      excludeDocumentContents: e.target.checked,
                    }))
                  }
                  className="mt-0.5 h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                />
                <div>
                  <span className="text-sm font-medium text-gray-900">
                    Exclude document contents
                  </span>
                  <p className="text-xs text-gray-500">
                    Export document metadata only, not file contents
                  </p>
                </div>
              </label>

              <label className="flex items-start gap-3">
                <input
                  type="checkbox"
                  checked={privacyOptions.hashIdentifiers}
                  onChange={(e) =>
                    setPrivacyOptions((prev) => ({
                      ...prev,
                      hashIdentifiers: e.target.checked,
                    }))
                  }
                  className="mt-0.5 h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                />
                <div>
                  <span className="text-sm font-medium text-gray-900">Hash identifiers</span>
                  <p className="text-xs text-gray-500">
                    Replace IDs with hashed values (maintains relationships)
                  </p>
                </div>
              </label>
            </div>
          )}
        </div>
      )}

      {/* Summary */}
      {selectedCategories.size > 0 && (
        <div className="rounded-lg bg-gray-50 p-4">
          <h3 className="text-sm font-medium text-gray-900">Export Summary</h3>
          <div className="mt-2 flex items-center gap-6 text-sm text-gray-600">
            <span>{selectedCategories.size} categories</span>
            <span>{totalRecords.toLocaleString()} total records</span>
            <span>Format: ZIP with CSV files</span>
          </div>
        </div>
      )}

      {/* Actions */}
      <div className="flex justify-end gap-3 border-t border-gray-200 pt-4">
        <button
          type="button"
          onClick={onCancel}
          disabled={isLoading}
          className="rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50"
        >
          Cancel
        </button>
        <button
          type="button"
          onClick={handleExport}
          disabled={selectedCategories.size === 0 || isLoading}
          className="inline-flex items-center rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50"
        >
          {isLoading ? 'Preparing Export...' : 'Start Export'}
        </button>
      </div>
    </div>
  );
}

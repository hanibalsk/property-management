/**
 * Import Page (Epic 66, Stories 66.2, 66.4).
 *
 * Main page for bulk data import workflow.
 */

import { useCallback, useState } from 'react';
import { FileUploader } from '../components/FileUploader';
import { type ImportJobHistoryItem, ImportJobList } from '../components/ImportJobList';
import { ImportJobProgress, type ImportJobStatusData } from '../components/ImportJobProgress';
import { ImportPreview, type ImportPreviewData } from '../components/ImportPreview';
import { ImportTemplateList, type ImportTemplateSummary } from '../components/ImportTemplateList';

type ImportStep = 'select_template' | 'upload' | 'preview' | 'importing' | 'complete';

// Mock data for demonstration
const MOCK_TEMPLATES: ImportTemplateSummary[] = [
  {
    id: '1',
    name: 'Buildings Import',
    dataType: 'buildings',
    description: 'Import building master data including address and details',
    isSystemTemplate: true,
    fieldCount: 12,
    updatedAt: new Date().toISOString(),
  },
  {
    id: '2',
    name: 'Units Import',
    dataType: 'units',
    description: 'Import unit data with building references',
    isSystemTemplate: true,
    fieldCount: 15,
    updatedAt: new Date().toISOString(),
  },
  {
    id: '3',
    name: 'Residents Import',
    dataType: 'residents',
    description: 'Import resident and owner information',
    isSystemTemplate: true,
    fieldCount: 18,
    updatedAt: new Date().toISOString(),
  },
];

const MOCK_JOB_HISTORY: ImportJobHistoryItem[] = [
  {
    id: '1',
    status: 'completed',
    filename: 'buildings_2024.csv',
    dataType: 'buildings',
    recordsImported: 45,
    recordsFailed: 0,
    createdByName: 'John Manager',
    createdAt: new Date(Date.now() - 86400000).toISOString(),
    completedAt: new Date(Date.now() - 86400000).toISOString(),
  },
  {
    id: '2',
    status: 'partially_completed',
    filename: 'residents_import.xlsx',
    dataType: 'residents',
    recordsImported: 120,
    recordsFailed: 5,
    createdByName: 'John Manager',
    createdAt: new Date(Date.now() - 21600000).toISOString(),
    completedAt: new Date(Date.now() - 21600000).toISOString(),
  },
];

export function ImportPage() {
  const [step, setStep] = useState<ImportStep>('select_template');
  const [selectedTemplate, setSelectedTemplate] = useState<ImportTemplateSummary | null>(null);
  const [currentJobId, setCurrentJobId] = useState<string | null>(null);
  const [previewData, setPreviewData] = useState<ImportPreviewData | null>(null);
  const [showHistory, setShowHistory] = useState(false);

  // Handle template selection
  const handleSelectTemplate = useCallback((template: ImportTemplateSummary) => {
    setSelectedTemplate(template);
    setStep('upload');
  }, []);

  // Handle file upload completion
  const handleUploadComplete = useCallback((jobId: string) => {
    setCurrentJobId(jobId);
    // Simulate fetching preview data
    setPreviewData({
      jobId,
      isValid: true,
      totalRows: 150,
      importableRows: 145,
      errorRows: 3,
      warningRows: 7,
      recordCounts: {
        newRecords: 120,
        updates: 25,
        skipped: 5,
      },
      issues: [
        {
          rowNumber: 23,
          column: 'email',
          severity: 'error',
          code: 'INVALID_EMAIL',
          message: 'Invalid email format',
          originalValue: 'not.an',
        },
        {
          rowNumber: 45,
          column: 'phone',
          severity: 'warning',
          code: 'PHONE_FORMAT',
          message: 'Phone number missing country code',
          originalValue: '0901234567',
          suggestedValue: '+421901234567',
        },
      ],
      totalIssueCount: 10,
      sampleRecords: [
        { name: 'Building A', address: '123 Main St', units: 24 },
        { name: 'Building B', address: '456 Oak Ave', units: 36 },
      ],
      columnMapping: [
        {
          sourceColumn: 'Building Name',
          targetField: 'name',
          isMapped: true,
          isRequired: true,
          sampleValues: ['Building A', 'Building B'],
        },
        {
          sourceColumn: 'Street Address',
          targetField: 'address',
          isMapped: true,
          isRequired: true,
          sampleValues: ['123 Main St', '456 Oak Ave'],
        },
      ],
    });
    setStep('preview');
  }, []);

  // Handle import approval
  const handleApproveImport = useCallback((_acknowledgeWarnings: boolean) => {
    // In real implementation, use acknowledgeWarnings to confirm import with warnings
    setStep('importing');
  }, []);

  // Handle import completion
  const handleImportComplete = useCallback((_status: ImportJobStatusData) => {
    // In real implementation, use status to show import results
    setStep('complete');
  }, []);

  // Handle starting a new import
  const handleStartNew = useCallback(() => {
    setStep('select_template');
    setSelectedTemplate(null);
    setCurrentJobId(null);
    setPreviewData(null);
  }, []);

  // Handle cancel
  const handleCancel = useCallback(() => {
    if (step === 'upload') {
      setStep('select_template');
      setSelectedTemplate(null);
    } else if (step === 'preview') {
      setStep('upload');
      setPreviewData(null);
    }
  }, [step]);

  // Handle template download
  const handleDownloadTemplate = useCallback(
    (_template: ImportTemplateSummary, _format: 'csv' | 'xlsx') => {
      // TODO: API call to download template
    },
    []
  );

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-xl font-semibold text-gray-900">Import Data</h1>
          <p className="mt-1 text-sm text-gray-500">
            Upload spreadsheets to import data into your organization.
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

      {/* Progress Steps */}
      <div className="flex items-center gap-2">
        {[
          { id: 'select_template', label: '1. Select Template' },
          { id: 'upload', label: '2. Upload File' },
          { id: 'preview', label: '3. Review' },
          { id: 'importing', label: '4. Import' },
        ].map((s, index) => (
          <div key={s.id} className="flex items-center">
            <div
              className={`flex items-center gap-2 rounded-full px-3 py-1 text-sm ${
                step === s.id
                  ? 'bg-blue-100 text-blue-800'
                  : ['select_template', 'upload', 'preview', 'importing'].indexOf(step) >
                      ['select_template', 'upload', 'preview', 'importing'].indexOf(
                        s.id as ImportStep
                      )
                    ? 'bg-green-100 text-green-800'
                    : 'bg-gray-100 text-gray-600'
              }`}
            >
              <span>{s.label}</span>
            </div>
            {index < 3 && (
              <svg
                className="mx-2 h-4 w-4 text-gray-400"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M9 5l7 7-7 7"
                />
              </svg>
            )}
          </div>
        ))}
      </div>

      {/* Main Content */}
      <div className="rounded-lg border border-gray-200 bg-white p-6">
        {/* Step 1: Select Template */}
        {step === 'select_template' && (
          <ImportTemplateList
            templates={MOCK_TEMPLATES}
            onSelect={handleSelectTemplate}
            onEdit={(_template) => {
              // Not available in import flow - use Templates page to edit
            }}
            onDownload={handleDownloadTemplate}
            onDuplicate={(_template) => {
              // Not available in import flow - use Templates page to duplicate
            }}
            onDelete={(_template) => {
              // Not available in import flow - use Templates page to delete
            }}
            onCreate={() => {
              // Not available in import flow - use Templates page to create
            }}
          />
        )}

        {/* Step 2: Upload File */}
        {step === 'upload' && selectedTemplate && (
          <FileUploader
            templateId={selectedTemplate.id}
            templateName={selectedTemplate.name}
            onUploadComplete={handleUploadComplete}
            onCancel={handleCancel}
          />
        )}

        {/* Step 3: Preview/Validation */}
        {step === 'preview' && previewData && (
          <ImportPreview
            preview={previewData}
            onApprove={handleApproveImport}
            onCancel={handleCancel}
          />
        )}

        {/* Step 4: Importing */}
        {step === 'importing' && currentJobId && (
          <ImportJobProgress
            jobId={currentJobId}
            onComplete={handleImportComplete}
            onCancel={() => setStep('preview')}
          />
        )}

        {/* Step 5: Complete */}
        {step === 'complete' && (
          <div className="text-center py-12">
            <div className="mx-auto h-16 w-16 rounded-full bg-green-100 p-4">
              <svg
                className="h-8 w-8 text-green-600"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M5 13l4 4L19 7"
                />
              </svg>
            </div>
            <h2 className="mt-4 text-lg font-medium text-gray-900">Import Complete</h2>
            <p className="mt-2 text-sm text-gray-500">Your data has been successfully imported.</p>
            <button
              type="button"
              onClick={handleStartNew}
              className="mt-6 inline-flex items-center rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700"
            >
              Start New Import
            </button>
          </div>
        )}
      </div>

      {/* Import History */}
      {showHistory && (
        <div className="rounded-lg border border-gray-200 bg-white p-6">
          <ImportJobList
            jobs={MOCK_JOB_HISTORY}
            onViewJob={(_jobId) => {
              // TODO: Navigate to job details view
            }}
            onRetryJob={(_jobId) => {
              // TODO: API call to retry failed job
            }}
            onViewErrors={(_jobId) => {
              // TODO: Navigate to job errors view
            }}
          />
        </div>
      )}
    </div>
  );
}

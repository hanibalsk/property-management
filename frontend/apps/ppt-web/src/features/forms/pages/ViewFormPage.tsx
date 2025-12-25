/**
 * ViewFormPage Component
 *
 * View form details and submissions (Epic 54, Story 54.1, 54.4).
 */

import type {
  FormSubmissionStatus,
  FormSubmissionSummary,
  FormWithDetails,
  ListFormSubmissionsParams,
} from '@ppt/api-client';
import { useState } from 'react';
import { FormBuilder } from '../components/FormBuilder';
import { SubmissionList } from '../components/SubmissionList';

interface ViewFormPageProps {
  form: FormWithDetails;
  submissions: FormSubmissionSummary[];
  submissionsTotal: number;
  isLoading?: boolean;
  onNavigateToEdit: (id: string) => void;
  onPublish: (id: string) => void;
  onArchive: (id: string) => void;
  onDuplicate: (id: string) => void;
  onDownloadPdf: (id: string) => void;
  onExportSubmissions: (id: string, format: 'csv' | 'xlsx' | 'pdf') => void;
  onViewSubmission: (formId: string, submissionId: string) => void;
  onApproveSubmission: (formId: string, submissionId: string) => void;
  onRejectSubmission: (formId: string, submissionId: string) => void;
  onSubmissionsFilterChange: (params: ListFormSubmissionsParams) => void;
  onBack: () => void;
}

export function ViewFormPage({
  form,
  submissions,
  submissionsTotal,
  isLoading,
  onNavigateToEdit,
  onPublish,
  onArchive,
  onDuplicate,
  onDownloadPdf,
  onExportSubmissions,
  onViewSubmission,
  onApproveSubmission,
  onRejectSubmission,
  onSubmissionsFilterChange,
  onBack,
}: ViewFormPageProps) {
  const [activeTab, setActiveTab] = useState<'preview' | 'submissions'>('preview');
  const [subPage, setSubPage] = useState(1);
  const [subPageSize] = useState(10);

  const isDraft = form.status === 'draft';
  const isPublished = form.status === 'published';

  const statusColors = {
    draft: 'bg-yellow-100 text-yellow-800',
    published: 'bg-green-100 text-green-800',
    archived: 'bg-gray-100 text-gray-800',
  };

  const handleSubmissionsPageChange = (newPage: number) => {
    setSubPage(newPage);
    onSubmissionsFilterChange({ page: newPage, pageSize: subPageSize });
  };

  const handleSubmissionsStatusFilter = (status?: FormSubmissionStatus) => {
    setSubPage(1);
    onSubmissionsFilterChange({ status, page: 1, pageSize: subPageSize });
  };

  return (
    <div className="max-w-6xl mx-auto px-4 py-8">
      {/* Header */}
      <div className="mb-6">
        <button
          type="button"
          onClick={onBack}
          className="text-gray-500 hover:text-gray-700 flex items-center gap-1 mb-4"
        >
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <title>Back</title>
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M10 19l-7-7m0 0l7-7m-7 7h18"
            />
          </svg>
          Back to Forms
        </button>

        <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
          <div>
            <div className="flex items-center gap-3">
              <h1 className="text-2xl font-bold text-gray-900">{form.title}</h1>
              <span
                className={`px-2 py-0.5 text-xs font-medium rounded-full ${statusColors[form.status as keyof typeof statusColors]}`}
              >
                {form.status.charAt(0).toUpperCase() + form.status.slice(1)}
              </span>
            </div>
            {form.description && <p className="text-gray-500 mt-1">{form.description}</p>}
          </div>

          <div className="flex flex-wrap gap-2">
            {isDraft && (
              <>
                <button
                  type="button"
                  onClick={() => onNavigateToEdit(form.id)}
                  className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
                >
                  Edit Form
                </button>
                <button
                  type="button"
                  onClick={() => onPublish(form.id)}
                  className="px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700"
                >
                  Publish
                </button>
              </>
            )}
            {isPublished && (
              <button
                type="button"
                onClick={() => onArchive(form.id)}
                className="px-4 py-2 border border-gray-300 text-gray-700 rounded-md hover:bg-gray-50"
              >
                Archive
              </button>
            )}
            <button
              type="button"
              onClick={() => onDuplicate(form.id)}
              className="px-4 py-2 border border-gray-300 text-gray-700 rounded-md hover:bg-gray-50"
            >
              Duplicate
            </button>
            <button
              type="button"
              onClick={() => onDownloadPdf(form.id)}
              className="px-4 py-2 border border-gray-300 text-gray-700 rounded-md hover:bg-gray-50"
            >
              Download PDF
            </button>
          </div>
        </div>
      </div>

      {/* Stats */}
      <div className="grid grid-cols-2 sm:grid-cols-4 gap-4 mb-6">
        <div className="bg-white border border-gray-200 rounded-lg p-4">
          <p className="text-sm text-gray-500">Fields</p>
          <p className="text-2xl font-semibold text-gray-900">{form.fields.length}</p>
        </div>
        <div className="bg-white border border-gray-200 rounded-lg p-4">
          <p className="text-sm text-gray-500">Submissions</p>
          <p className="text-2xl font-semibold text-gray-900">{form.submissionCount}</p>
        </div>
        <div className="bg-white border border-gray-200 rounded-lg p-4">
          <p className="text-sm text-gray-500">Pending Review</p>
          <p className="text-2xl font-semibold text-yellow-600">{form.pendingSubmissions}</p>
        </div>
        <div className="bg-white border border-gray-200 rounded-lg p-4">
          <p className="text-sm text-gray-500">Downloads</p>
          <p className="text-2xl font-semibold text-gray-900">{form.downloadCount}</p>
        </div>
      </div>

      {/* Tabs */}
      <div className="border-b border-gray-200 mb-6">
        <nav className="flex gap-4">
          <button
            type="button"
            onClick={() => setActiveTab('preview')}
            className={`py-4 px-1 border-b-2 font-medium text-sm ${
              activeTab === 'preview'
                ? 'border-blue-500 text-blue-600'
                : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
            }`}
          >
            Form Preview
          </button>
          <button
            type="button"
            onClick={() => setActiveTab('submissions')}
            className={`py-4 px-1 border-b-2 font-medium text-sm ${
              activeTab === 'submissions'
                ? 'border-blue-500 text-blue-600'
                : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
            }`}
          >
            Submissions ({form.submissionCount})
          </button>
        </nav>
      </div>

      {/* Tab Content */}
      {activeTab === 'preview' ? (
        <FormBuilder
          fields={form.fields}
          isEditing={false}
          onAddField={() => {}}
          onUpdateField={() => {}}
          onDeleteField={() => {}}
          onReorderFields={() => {}}
        />
      ) : (
        <>
          {submissions.length > 0 && (
            <div className="mb-4 flex gap-2">
              <button
                type="button"
                onClick={() => onExportSubmissions(form.id, 'csv')}
                className="px-3 py-1.5 text-sm border border-gray-300 text-gray-700 rounded-md hover:bg-gray-50"
              >
                Export CSV
              </button>
              <button
                type="button"
                onClick={() => onExportSubmissions(form.id, 'xlsx')}
                className="px-3 py-1.5 text-sm border border-gray-300 text-gray-700 rounded-md hover:bg-gray-50"
              >
                Export Excel
              </button>
            </div>
          )}
          <SubmissionList
            submissions={submissions}
            total={submissionsTotal}
            page={subPage}
            pageSize={subPageSize}
            isLoading={isLoading}
            onPageChange={handleSubmissionsPageChange}
            onStatusFilter={handleSubmissionsStatusFilter}
            onView={(submissionId) => onViewSubmission(form.id, submissionId)}
            onApprove={(submissionId) => onApproveSubmission(form.id, submissionId)}
            onReject={(submissionId) => onRejectSubmission(form.id, submissionId)}
          />
        </>
      )}
    </div>
  );
}

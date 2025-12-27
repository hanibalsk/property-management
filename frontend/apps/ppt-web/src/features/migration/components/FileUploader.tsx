/**
 * File Uploader Component (Story 66.2).
 *
 * Drag-and-drop file upload for import files with validation.
 */

import { type ChangeEvent, type DragEvent, useCallback, useState } from 'react';

interface FileUploaderProps {
  templateId: string;
  templateName: string;
  onUploadComplete: (jobId: string) => void;
  onCancel: () => void;
  acceptedTypes?: string[];
  maxSizeBytes?: number;
}

interface UploadState {
  status: 'idle' | 'uploading' | 'error';
  progress: number;
  error?: string;
}

const DEFAULT_ACCEPTED_TYPES = [
  '.csv',
  '.xlsx',
  '.xls',
  'text/csv',
  'application/vnd.ms-excel',
  'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
];

const DEFAULT_MAX_SIZE = 100 * 1024 * 1024; // 100MB

export function FileUploader({
  templateId,
  templateName,
  onUploadComplete,
  onCancel,
  acceptedTypes = DEFAULT_ACCEPTED_TYPES,
  maxSizeBytes = DEFAULT_MAX_SIZE,
}: FileUploaderProps) {
  const [file, setFile] = useState<File | null>(null);
  const [uploadState, setUploadState] = useState<UploadState>({
    status: 'idle',
    progress: 0,
  });
  const [isDragOver, setIsDragOver] = useState(false);
  const [skipErrors, setSkipErrors] = useState(false);
  const [updateExisting, setUpdateExisting] = useState(false);

  const validateFile = useCallback(
    (f: File): string | null => {
      // Check file type
      const extension = `.${f.name.split('.').pop()?.toLowerCase()}`;
      const isValidType = acceptedTypes.includes(f.type) || acceptedTypes.includes(extension);
      if (!isValidType) {
        return `Invalid file type. Accepted: CSV, Excel (.xlsx, .xls)`;
      }

      // Check file size
      if (f.size > maxSizeBytes) {
        return `File too large. Maximum size: ${Math.round(maxSizeBytes / 1024 / 1024)}MB`;
      }

      return null;
    },
    [acceptedTypes, maxSizeBytes]
  );

  const handleFileSelect = useCallback(
    (files: FileList | null) => {
      if (!files || files.length === 0) return;

      const selectedFile = files[0];
      const error = validateFile(selectedFile);

      if (error) {
        setUploadState({ status: 'error', progress: 0, error });
        return;
      }

      setFile(selectedFile);
      setUploadState({ status: 'idle', progress: 0 });
    },
    [validateFile]
  );

  const handleDrop = useCallback(
    (e: DragEvent<HTMLDivElement>) => {
      e.preventDefault();
      setIsDragOver(false);
      handleFileSelect(e.dataTransfer.files);
    },
    [handleFileSelect]
  );

  const handleDragOver = useCallback((e: DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    setIsDragOver(true);
  }, []);

  const handleDragLeave = useCallback((e: DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    setIsDragOver(false);
  }, []);

  const handleInputChange = useCallback(
    (e: ChangeEvent<HTMLInputElement>) => {
      handleFileSelect(e.target.files);
    },
    [handleFileSelect]
  );

  const handleUpload = useCallback(async () => {
    if (!file) return;

    setUploadState({ status: 'uploading', progress: 0 });

    try {
      // Create form data
      const formData = new FormData();
      formData.append('file', file);
      formData.append('template_id', templateId);
      formData.append(
        'options',
        JSON.stringify({
          skip_errors: skipErrors,
          update_existing: updateExisting,
          dry_run: false,
        })
      );

      // Simulate upload progress (in real implementation, use XMLHttpRequest or fetch with progress)
      const simulateProgress = () => {
        let progress = 0;
        const interval = setInterval(() => {
          progress += Math.random() * 20;
          if (progress >= 100) {
            progress = 100;
            clearInterval(interval);
          }
          setUploadState((prev) => ({ ...prev, progress: Math.min(progress, 100) }));
        }, 200);
        return interval;
      };

      const progressInterval = simulateProgress();

      // In a real implementation, this would be an API call:
      // const response = await fetch('/api/v1/migration/import/upload', {
      //   method: 'POST',
      //   body: formData,
      // });

      // Simulate API call delay
      await new Promise((resolve) => setTimeout(resolve, 2000));
      clearInterval(progressInterval);

      // Simulate success
      const jobId = crypto.randomUUID();
      onUploadComplete(jobId);
    } catch (error) {
      setUploadState({
        status: 'error',
        progress: 0,
        error: error instanceof Error ? error.message : 'Upload failed',
      });
    }
  }, [file, templateId, skipErrors, updateExisting, onUploadComplete]);

  const handleRemoveFile = useCallback(() => {
    setFile(null);
    setUploadState({ status: 'idle', progress: 0 });
  }, []);

  const formatFileSize = (bytes: number): string => {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="border-b border-gray-200 pb-4">
        <h2 className="text-lg font-medium text-gray-900">Upload Import File</h2>
        <p className="mt-1 text-sm text-gray-500">
          Using template: <span className="font-medium">{templateName}</span>
        </p>
      </div>

      {/* Drop Zone */}
      <div
        onDrop={handleDrop}
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
        className={`relative rounded-lg border-2 border-dashed p-8 text-center transition-colors ${
          isDragOver
            ? 'border-blue-500 bg-blue-50'
            : uploadState.status === 'error'
              ? 'border-red-300 bg-red-50'
              : 'border-gray-300 hover:border-gray-400'
        }`}
      >
        {!file ? (
          <>
            <svg
              className="mx-auto h-12 w-12 text-gray-400"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={1.5}
                d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
              />
            </svg>
            <p className="mt-4 text-sm text-gray-600">
              <label className="cursor-pointer font-medium text-blue-600 hover:text-blue-500">
                Click to upload
                <input
                  type="file"
                  className="sr-only"
                  accept={acceptedTypes.join(',')}
                  onChange={handleInputChange}
                />
              </label>
              {' or drag and drop'}
            </p>
            <p className="mt-1 text-xs text-gray-500">
              CSV or Excel files up to {Math.round(maxSizeBytes / 1024 / 1024)}MB
            </p>
          </>
        ) : (
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-blue-100">
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
                    d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                  />
                </svg>
              </div>
              <div className="text-left">
                <p className="font-medium text-gray-900">{file.name}</p>
                <p className="text-sm text-gray-500">{formatFileSize(file.size)}</p>
              </div>
            </div>
            {uploadState.status !== 'uploading' && (
              <button
                type="button"
                onClick={handleRemoveFile}
                className="rounded p-1 text-gray-400 hover:bg-gray-100 hover:text-gray-600"
              >
                <svg className="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
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
        )}

        {/* Upload Progress */}
        {uploadState.status === 'uploading' && (
          <div className="mt-4">
            <div className="h-2 overflow-hidden rounded-full bg-gray-200">
              <div
                className="h-full bg-blue-600 transition-all duration-300"
                style={{ width: `${uploadState.progress}%` }}
              />
            </div>
            <p className="mt-2 text-sm text-gray-600">
              Uploading... {Math.round(uploadState.progress)}%
            </p>
          </div>
        )}
      </div>

      {/* Error Message */}
      {uploadState.status === 'error' && uploadState.error && (
        <div className="rounded-md bg-red-50 p-4">
          <div className="flex">
            <svg
              className="h-5 w-5 text-red-400"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
            <p className="ml-3 text-sm text-red-700">{uploadState.error}</p>
          </div>
        </div>
      )}

      {/* Import Options */}
      {file && uploadState.status !== 'uploading' && (
        <div className="rounded-lg bg-gray-50 p-4">
          <h3 className="text-sm font-medium text-gray-900">Import Options</h3>
          <div className="mt-3 space-y-3">
            <label className="flex items-center gap-2">
              <input
                type="checkbox"
                checked={skipErrors}
                onChange={(e) => setSkipErrors(e.target.checked)}
                className="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
              />
              <span className="text-sm text-gray-700">
                Skip rows with errors (import valid rows only)
              </span>
            </label>
            <label className="flex items-center gap-2">
              <input
                type="checkbox"
                checked={updateExisting}
                onChange={(e) => setUpdateExisting(e.target.checked)}
                className="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
              />
              <span className="text-sm text-gray-700">
                Update existing records if found (by unique key)
              </span>
            </label>
          </div>
        </div>
      )}

      {/* Actions */}
      <div className="flex justify-end gap-3 border-t border-gray-200 pt-4">
        <button
          type="button"
          onClick={onCancel}
          disabled={uploadState.status === 'uploading'}
          className="rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50"
        >
          Cancel
        </button>
        <button
          type="button"
          onClick={handleUpload}
          disabled={!file || uploadState.status === 'uploading'}
          className="inline-flex items-center rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50"
        >
          {uploadState.status === 'uploading' ? 'Uploading...' : 'Upload & Validate'}
        </button>
      </div>
    </div>
  );
}

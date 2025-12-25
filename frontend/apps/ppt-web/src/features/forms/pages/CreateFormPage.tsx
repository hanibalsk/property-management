/**
 * CreateFormPage Component
 *
 * Form creation page with builder (Epic 54, Story 54.1).
 */

import type { CreateFormField, CreateFormRequest, FormField } from '@ppt/api-client';
import { useState } from 'react';
import { FormBuilder } from '../components/FormBuilder';

interface CreateFormPageProps {
  isCreating?: boolean;
  onCreate: (data: CreateFormRequest) => Promise<string>;
  onAddField: (formId: string, field: CreateFormField) => Promise<FormField>;
  onUpdateField: (
    formId: string,
    fieldId: string,
    field: Partial<CreateFormField>
  ) => Promise<void>;
  onDeleteField: (formId: string, fieldId: string) => Promise<void>;
  onReorderFields: (formId: string, fieldIds: string[]) => Promise<void>;
  onNavigateToEdit: (id: string) => void;
  onCancel: () => void;
}

export function CreateFormPage({
  isCreating,
  onCreate,
  onAddField,
  onUpdateField,
  onDeleteField,
  onReorderFields,
  onNavigateToEdit,
  onCancel,
}: CreateFormPageProps) {
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [category, setCategory] = useState('');
  const [requireSignatures, setRequireSignatures] = useState(false);
  const [allowMultipleSubmissions, setAllowMultipleSubmissions] = useState(false);
  const [submissionDeadline, setSubmissionDeadline] = useState('');
  const [formId, setFormId] = useState<string | null>(null);
  const [fields, setFields] = useState<FormField[]>([]);
  const [error, setError] = useState<string | null>(null);

  const handleCreateForm = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);

    if (!title.trim()) {
      setError('Form title is required');
      return;
    }

    try {
      const id = await onCreate({
        title: title.trim(),
        description: description.trim() || undefined,
        category: category.trim() || undefined,
        requireSignatures,
        allowMultipleSubmissions,
        submissionDeadline: submissionDeadline || undefined,
      });
      setFormId(id);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create form');
    }
  };

  const handleAddField = async (field: CreateFormField) => {
    if (!formId) return;
    try {
      const newField = await onAddField(formId, field);
      setFields((prev) => [...prev, newField]);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to add field');
    }
  };

  const handleUpdateField = async (fieldId: string, updates: Partial<CreateFormField>) => {
    if (!formId) return;
    try {
      await onUpdateField(formId, fieldId, updates);
      setFields((prev) => prev.map((f) => (f.id === fieldId ? { ...f, ...updates } : f)));
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to update field');
    }
  };

  const handleDeleteField = async (fieldId: string) => {
    if (!formId) return;
    try {
      await onDeleteField(formId, fieldId);
      setFields((prev) => prev.filter((f) => f.id !== fieldId));
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to delete field');
    }
  };

  const handleReorderFields = async (fieldIds: string[]) => {
    if (!formId) return;
    try {
      await onReorderFields(formId, fieldIds);
      const reorderedFields = fieldIds.map((id) => fields.find((f) => f.id === id)!);
      setFields(reorderedFields);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to reorder fields');
    }
  };

  const handleContinueToEdit = () => {
    if (formId) {
      onNavigateToEdit(formId);
    }
  };

  return (
    <div className="max-w-6xl mx-auto px-4 py-8">
      <div className="mb-6">
        <h1 className="text-2xl font-bold text-gray-900">Create New Form</h1>
        <p className="text-gray-500 mt-1">
          {formId
            ? 'Add fields to your form using the drag-and-drop builder.'
            : 'Start by filling in the basic form details.'}
        </p>
      </div>

      {error && (
        <div className="mb-6 p-4 bg-red-50 border border-red-200 rounded-lg">
          <p className="text-red-700">{error}</p>
        </div>
      )}

      {!formId ? (
        <form
          onSubmit={handleCreateForm}
          className="bg-white border border-gray-200 rounded-lg p-6"
        >
          <div className="space-y-6">
            <label className="block">
              <span className="block text-sm font-medium text-gray-700 mb-1">
                Form Title <span className="text-red-500">*</span>
              </span>
              <input
                type="text"
                value={title}
                onChange={(e) => setTitle(e.target.value)}
                placeholder="e.g., Maintenance Request Form"
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </label>

            <label className="block">
              <span className="block text-sm font-medium text-gray-700 mb-1">Description</span>
              <textarea
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                placeholder="Describe the purpose of this form..."
                rows={3}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </label>

            <label className="block">
              <span className="block text-sm font-medium text-gray-700 mb-1">Category</span>
              <input
                type="text"
                value={category}
                onChange={(e) => setCategory(e.target.value)}
                placeholder="e.g., Maintenance, Requests, Complaints"
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </label>

            <label className="block">
              <span className="block text-sm font-medium text-gray-700 mb-1">
                Submission Deadline (optional)
              </span>
              <input
                type="datetime-local"
                value={submissionDeadline}
                onChange={(e) => setSubmissionDeadline(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </label>

            <div className="flex flex-col gap-3">
              <label className="flex items-center gap-2">
                <input
                  type="checkbox"
                  checked={requireSignatures}
                  onChange={(e) => setRequireSignatures(e.target.checked)}
                  className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                />
                <span className="text-sm text-gray-700">Require digital signature</span>
              </label>

              <label className="flex items-center gap-2">
                <input
                  type="checkbox"
                  checked={allowMultipleSubmissions}
                  onChange={(e) => setAllowMultipleSubmissions(e.target.checked)}
                  className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                />
                <span className="text-sm text-gray-700">Allow multiple submissions per user</span>
              </label>
            </div>
          </div>

          <div className="mt-6 flex gap-4">
            <button
              type="submit"
              disabled={isCreating}
              className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {isCreating ? 'Creating...' : 'Create Form'}
            </button>
            <button
              type="button"
              onClick={onCancel}
              className="px-4 py-2 border border-gray-300 text-gray-700 rounded-md hover:bg-gray-50"
            >
              Cancel
            </button>
          </div>
        </form>
      ) : (
        <>
          <FormBuilder
            fields={fields}
            isEditing={true}
            onAddField={handleAddField}
            onUpdateField={handleUpdateField}
            onDeleteField={handleDeleteField}
            onReorderFields={handleReorderFields}
          />

          <div className="mt-6 flex gap-4">
            <button
              type="button"
              onClick={handleContinueToEdit}
              className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
            >
              Continue Editing
            </button>
            <button
              type="button"
              onClick={onCancel}
              className="px-4 py-2 border border-gray-300 text-gray-700 rounded-md hover:bg-gray-50"
            >
              Save and Exit
            </button>
          </div>
        </>
      )}
    </div>
  );
}

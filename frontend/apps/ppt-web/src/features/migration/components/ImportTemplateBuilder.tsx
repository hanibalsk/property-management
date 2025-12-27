/**
 * Import Template Builder Component (Story 66.1).
 *
 * Allows administrators to create and customize import templates
 * with field mappings and validation rules.
 */

import { type ChangeEvent, useCallback, useState } from 'react';

export type FieldDataType = 'string' | 'integer' | 'decimal' | 'boolean' | 'date' | 'datetime' | 'email' | 'phone' | 'uuid' | 'enum';

export interface FieldValidation {
  required: boolean;
  minLength?: number;
  maxLength?: number;
  minValue?: number;
  maxValue?: number;
  pattern?: string;
  allowedValues?: string[];
  message?: string;
}

export interface ImportFieldMapping {
  id: string;
  fieldName: string;
  displayLabel: string;
  columnHeader: string;
  dataType: FieldDataType;
  validation: FieldValidation;
  exampleValue?: string;
  description?: string;
}

export type ImportDataType = 'buildings' | 'units' | 'residents' | 'financials' | 'faults' | 'documents' | 'meters' | 'votes' | 'custom';

interface ImportTemplateBuilderProps {
  initialTemplate?: {
    name: string;
    description?: string;
    dataType: ImportDataType;
    fieldMappings: ImportFieldMapping[];
  };
  onSave: (template: {
    name: string;
    description?: string;
    dataType: ImportDataType;
    fieldMappings: ImportFieldMapping[];
  }) => void;
  onCancel: () => void;
  isLoading?: boolean;
}

const DATA_TYPE_OPTIONS: { value: ImportDataType; label: string }[] = [
  { value: 'buildings', label: 'Buildings' },
  { value: 'units', label: 'Units' },
  { value: 'residents', label: 'Residents' },
  { value: 'financials', label: 'Financials' },
  { value: 'faults', label: 'Faults' },
  { value: 'documents', label: 'Documents' },
  { value: 'meters', label: 'Meters' },
  { value: 'votes', label: 'Votes' },
  { value: 'custom', label: 'Custom' },
];

const FIELD_DATA_TYPE_OPTIONS: { value: FieldDataType; label: string }[] = [
  { value: 'string', label: 'Text' },
  { value: 'integer', label: 'Integer' },
  { value: 'decimal', label: 'Decimal' },
  { value: 'boolean', label: 'Yes/No' },
  { value: 'date', label: 'Date' },
  { value: 'datetime', label: 'Date & Time' },
  { value: 'email', label: 'Email' },
  { value: 'phone', label: 'Phone' },
  { value: 'uuid', label: 'UUID' },
  { value: 'enum', label: 'Enum (Choice)' },
];

function generateId(): string {
  return Math.random().toString(36).substring(2, 11);
}

export function ImportTemplateBuilder({
  initialTemplate,
  onSave,
  onCancel,
  isLoading = false,
}: ImportTemplateBuilderProps) {
  const [name, setName] = useState(initialTemplate?.name ?? '');
  const [description, setDescription] = useState(initialTemplate?.description ?? '');
  const [dataType, setDataType] = useState<ImportDataType>(initialTemplate?.dataType ?? 'buildings');
  const [fieldMappings, setFieldMappings] = useState<ImportFieldMapping[]>(
    initialTemplate?.fieldMappings ?? []
  );
  const [editingFieldId, setEditingFieldId] = useState<string | null>(null);

  const handleAddField = useCallback(() => {
    const newField: ImportFieldMapping = {
      id: generateId(),
      fieldName: '',
      displayLabel: '',
      columnHeader: '',
      dataType: 'string',
      validation: {
        required: false,
      },
    };
    setFieldMappings((prev) => [...prev, newField]);
    setEditingFieldId(newField.id);
  }, []);

  const handleUpdateField = useCallback((fieldId: string, updates: Partial<ImportFieldMapping>) => {
    setFieldMappings((prev) =>
      prev.map((field) => (field.id === fieldId ? { ...field, ...updates } : field))
    );
  }, []);

  const handleRemoveField = useCallback((fieldId: string) => {
    setFieldMappings((prev) => prev.filter((field) => field.id !== fieldId));
    if (editingFieldId === fieldId) {
      setEditingFieldId(null);
    }
  }, [editingFieldId]);

  const handleMoveField = useCallback((fieldId: string, direction: 'up' | 'down') => {
    setFieldMappings((prev) => {
      const index = prev.findIndex((f) => f.id === fieldId);
      if (index === -1) return prev;
      if (direction === 'up' && index === 0) return prev;
      if (direction === 'down' && index === prev.length - 1) return prev;

      const newFields = [...prev];
      const swapIndex = direction === 'up' ? index - 1 : index + 1;
      [newFields[index], newFields[swapIndex]] = [newFields[swapIndex], newFields[index]];
      return newFields;
    });
  }, []);

  const handleSave = useCallback(() => {
    if (!name.trim()) {
      return;
    }
    if (fieldMappings.length === 0) {
      return;
    }

    onSave({
      name: name.trim(),
      description: description.trim() || undefined,
      dataType,
      fieldMappings,
    });
  }, [name, description, dataType, fieldMappings, onSave]);

  const isValid = name.trim().length > 0 && fieldMappings.length > 0;

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="border-b border-gray-200 pb-4">
        <h2 className="text-lg font-medium text-gray-900">
          {initialTemplate ? 'Edit Import Template' : 'Create Import Template'}
        </h2>
        <p className="mt-1 text-sm text-gray-500">
          Define the structure and validation rules for importing data.
        </p>
      </div>

      {/* Basic Info */}
      <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
        <div>
          <label htmlFor="template-name" className="block text-sm font-medium text-gray-700">
            Template Name *
          </label>
          <input
            type="text"
            id="template-name"
            value={name}
            onChange={(e: ChangeEvent<HTMLInputElement>) => setName(e.target.value)}
            className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
            placeholder="e.g., Buildings Import v2"
          />
        </div>

        <div>
          <label htmlFor="data-type" className="block text-sm font-medium text-gray-700">
            Data Type *
          </label>
          <select
            id="data-type"
            value={dataType}
            onChange={(e: ChangeEvent<HTMLSelectElement>) => setDataType(e.target.value as ImportDataType)}
            className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
          >
            {DATA_TYPE_OPTIONS.map((option) => (
              <option key={option.value} value={option.value}>
                {option.label}
              </option>
            ))}
          </select>
        </div>

        <div className="sm:col-span-2">
          <label htmlFor="description" className="block text-sm font-medium text-gray-700">
            Description
          </label>
          <textarea
            id="description"
            value={description}
            onChange={(e: ChangeEvent<HTMLTextAreaElement>) => setDescription(e.target.value)}
            rows={2}
            className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
            placeholder="Optional description of the template purpose"
          />
        </div>
      </div>

      {/* Field Mappings */}
      <div>
        <div className="flex items-center justify-between">
          <h3 className="text-sm font-medium text-gray-900">Field Mappings</h3>
          <button
            type="button"
            onClick={handleAddField}
            className="inline-flex items-center rounded-md bg-blue-600 px-3 py-1.5 text-sm font-medium text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
          >
            + Add Field
          </button>
        </div>

        {fieldMappings.length === 0 ? (
          <div className="mt-4 rounded-lg border-2 border-dashed border-gray-300 p-8 text-center">
            <p className="text-sm text-gray-500">
              No fields defined yet. Click "Add Field" to start building your template.
            </p>
          </div>
        ) : (
          <div className="mt-4 space-y-3">
            {fieldMappings.map((field, index) => (
              <FieldMappingRow
                key={field.id}
                field={field}
                index={index}
                isFirst={index === 0}
                isLast={index === fieldMappings.length - 1}
                isExpanded={editingFieldId === field.id}
                onToggleExpand={() => setEditingFieldId(editingFieldId === field.id ? null : field.id)}
                onUpdate={(updates) => handleUpdateField(field.id, updates)}
                onRemove={() => handleRemoveField(field.id)}
                onMove={(direction) => handleMoveField(field.id, direction)}
              />
            ))}
          </div>
        )}
      </div>

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
          onClick={handleSave}
          disabled={!isValid || isLoading}
          className="inline-flex items-center rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50"
        >
          {isLoading ? 'Saving...' : 'Save Template'}
        </button>
      </div>
    </div>
  );
}

interface FieldMappingRowProps {
  field: ImportFieldMapping;
  index: number;
  isFirst: boolean;
  isLast: boolean;
  isExpanded: boolean;
  onToggleExpand: () => void;
  onUpdate: (updates: Partial<ImportFieldMapping>) => void;
  onRemove: () => void;
  onMove: (direction: 'up' | 'down') => void;
}

function FieldMappingRow({
  field,
  index,
  isFirst,
  isLast,
  isExpanded,
  onToggleExpand,
  onUpdate,
  onRemove,
  onMove,
}: FieldMappingRowProps) {
  return (
    <div className="rounded-lg border border-gray-200 bg-white shadow-sm">
      {/* Collapsed View */}
      <div className="flex items-center justify-between p-3">
        <div className="flex items-center gap-3">
          <span className="flex h-6 w-6 items-center justify-center rounded-full bg-gray-100 text-xs font-medium text-gray-600">
            {index + 1}
          </span>
          <div>
            <span className="font-medium text-gray-900">
              {field.displayLabel || field.fieldName || '(Unnamed field)'}
            </span>
            <span className="ml-2 text-sm text-gray-500">
              {field.columnHeader && `"${field.columnHeader}"`}
            </span>
          </div>
          <span className="rounded bg-gray-100 px-2 py-0.5 text-xs text-gray-600">
            {field.dataType}
          </span>
          {field.validation.required && (
            <span className="rounded bg-red-100 px-2 py-0.5 text-xs text-red-600">Required</span>
          )}
        </div>

        <div className="flex items-center gap-1">
          <button
            type="button"
            onClick={() => onMove('up')}
            disabled={isFirst}
            className="rounded p-1 text-gray-400 hover:bg-gray-100 hover:text-gray-600 disabled:opacity-30"
            title="Move up"
          >
            <svg className="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 15l7-7 7 7" />
            </svg>
          </button>
          <button
            type="button"
            onClick={() => onMove('down')}
            disabled={isLast}
            className="rounded p-1 text-gray-400 hover:bg-gray-100 hover:text-gray-600 disabled:opacity-30"
            title="Move down"
          >
            <svg className="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
            </svg>
          </button>
          <button
            type="button"
            onClick={onToggleExpand}
            className="rounded p-1 text-gray-400 hover:bg-gray-100 hover:text-gray-600"
            title={isExpanded ? 'Collapse' : 'Expand'}
          >
            <svg className="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
              />
            </svg>
          </button>
          <button
            type="button"
            onClick={onRemove}
            className="rounded p-1 text-gray-400 hover:bg-red-100 hover:text-red-600"
            title="Remove field"
          >
            <svg className="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
              />
            </svg>
          </button>
        </div>
      </div>

      {/* Expanded View */}
      {isExpanded && (
        <div className="border-t border-gray-200 bg-gray-50 p-4">
          <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
            <div>
              <label className="block text-xs font-medium text-gray-700">Field Name</label>
              <input
                type="text"
                value={field.fieldName}
                onChange={(e) => onUpdate({ fieldName: e.target.value })}
                className="mt-1 block w-full rounded-md border-gray-300 text-sm shadow-sm focus:border-blue-500 focus:ring-blue-500"
                placeholder="e.g., building_name"
              />
            </div>

            <div>
              <label className="block text-xs font-medium text-gray-700">Display Label</label>
              <input
                type="text"
                value={field.displayLabel}
                onChange={(e) => onUpdate({ displayLabel: e.target.value })}
                className="mt-1 block w-full rounded-md border-gray-300 text-sm shadow-sm focus:border-blue-500 focus:ring-blue-500"
                placeholder="e.g., Building Name"
              />
            </div>

            <div>
              <label className="block text-xs font-medium text-gray-700">Column Header</label>
              <input
                type="text"
                value={field.columnHeader}
                onChange={(e) => onUpdate({ columnHeader: e.target.value })}
                className="mt-1 block w-full rounded-md border-gray-300 text-sm shadow-sm focus:border-blue-500 focus:ring-blue-500"
                placeholder="Column name in CSV/Excel"
              />
            </div>

            <div>
              <label className="block text-xs font-medium text-gray-700">Data Type</label>
              <select
                value={field.dataType}
                onChange={(e) => onUpdate({ dataType: e.target.value as FieldDataType })}
                className="mt-1 block w-full rounded-md border-gray-300 text-sm shadow-sm focus:border-blue-500 focus:ring-blue-500"
              >
                {FIELD_DATA_TYPE_OPTIONS.map((option) => (
                  <option key={option.value} value={option.value}>
                    {option.label}
                  </option>
                ))}
              </select>
            </div>

            <div>
              <label className="block text-xs font-medium text-gray-700">Example Value</label>
              <input
                type="text"
                value={field.exampleValue ?? ''}
                onChange={(e) => onUpdate({ exampleValue: e.target.value || undefined })}
                className="mt-1 block w-full rounded-md border-gray-300 text-sm shadow-sm focus:border-blue-500 focus:ring-blue-500"
                placeholder="Sample data for template"
              />
            </div>

            <div className="flex items-center">
              <label className="flex items-center gap-2">
                <input
                  type="checkbox"
                  checked={field.validation.required}
                  onChange={(e) =>
                    onUpdate({
                      validation: { ...field.validation, required: e.target.checked },
                    })
                  }
                  className="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                />
                <span className="text-sm text-gray-700">Required field</span>
              </label>
            </div>

            <div className="sm:col-span-2 lg:col-span-3">
              <label className="block text-xs font-medium text-gray-700">Description</label>
              <input
                type="text"
                value={field.description ?? ''}
                onChange={(e) => onUpdate({ description: e.target.value || undefined })}
                className="mt-1 block w-full rounded-md border-gray-300 text-sm shadow-sm focus:border-blue-500 focus:ring-blue-500"
                placeholder="Help text for users filling the template"
              />
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

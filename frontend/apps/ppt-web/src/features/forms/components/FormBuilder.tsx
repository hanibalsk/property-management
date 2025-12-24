/**
 * FormBuilder Component
 *
 * Drag-and-drop form builder for creating form fields (Epic 54, Story 54.1).
 */

import type { CreateFormField, FormField, FormFieldType } from '@ppt/api-client';
import { useState } from 'react';

interface FormBuilderProps {
  fields: FormField[];
  isEditing?: boolean;
  onAddField: (field: CreateFormField) => void;
  onUpdateField: (fieldId: string, field: Partial<CreateFormField>) => void;
  onDeleteField: (fieldId: string) => void;
  onReorderFields: (fieldIds: string[]) => void;
}

const fieldTypes: { type: FormFieldType; label: string; icon: string }[] = [
  { type: 'text', label: 'Text', icon: 'Aa' },
  { type: 'textarea', label: 'Long Text', icon: 'Tt' },
  { type: 'number', label: 'Number', icon: '#' },
  { type: 'email', label: 'Email', icon: '@' },
  { type: 'phone', label: 'Phone', icon: 'Tel' },
  { type: 'date', label: 'Date', icon: 'Cal' },
  { type: 'datetime', label: 'Date & Time', icon: 'DT' },
  { type: 'checkbox', label: 'Checkbox', icon: 'Chk' },
  { type: 'radio', label: 'Radio', icon: 'Rad' },
  { type: 'select', label: 'Dropdown', icon: 'Sel' },
  { type: 'multiselect', label: 'Multi-Select', icon: 'MSel' },
  { type: 'file', label: 'File Upload', icon: 'File' },
  { type: 'signature', label: 'Signature', icon: 'Sig' },
];

export function FormBuilder({
  fields,
  isEditing = true,
  onAddField,
  onUpdateField,
  onDeleteField,
  onReorderFields,
}: FormBuilderProps) {
  const [draggedFieldId, setDraggedFieldId] = useState<string | null>(null);
  const [editingFieldId, setEditingFieldId] = useState<string | null>(null);

  const handleDragStart = (fieldId: string) => {
    setDraggedFieldId(fieldId);
  };

  const handleDragOver = (e: React.DragEvent, targetFieldId: string) => {
    e.preventDefault();
    if (!draggedFieldId || draggedFieldId === targetFieldId) return;

    const fieldIds = fields.map((f) => f.id);
    const draggedIndex = fieldIds.indexOf(draggedFieldId);
    const targetIndex = fieldIds.indexOf(targetFieldId);

    if (draggedIndex !== -1 && targetIndex !== -1) {
      const newOrder = [...fieldIds];
      newOrder.splice(draggedIndex, 1);
      newOrder.splice(targetIndex, 0, draggedFieldId);
      onReorderFields(newOrder);
    }
  };

  const handleDragEnd = () => {
    setDraggedFieldId(null);
  };

  const handleAddField = (type: FormFieldType) => {
    onAddField({
      label: `New ${type} field`,
      fieldType: type,
      required: false,
      sortOrder: fields.length,
    });
  };

  return (
    <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
      {/* Field Palette */}
      {isEditing && (
        <div className="lg:col-span-1">
          <div className="bg-gray-50 rounded-lg p-4 sticky top-4">
            <h3 className="text-sm font-medium text-gray-700 mb-3">Add Fields</h3>
            <div className="grid grid-cols-2 gap-2">
              {fieldTypes.map(({ type, label, icon }) => (
                <button
                  key={type}
                  type="button"
                  onClick={() => handleAddField(type)}
                  className="flex flex-col items-center justify-center p-3 bg-white border border-gray-200 rounded-lg hover:border-blue-500 hover:bg-blue-50 transition-colors"
                >
                  <span className="text-xs font-mono text-gray-500 mb-1">{icon}</span>
                  <span className="text-xs text-gray-700">{label}</span>
                </button>
              ))}
            </div>
          </div>
        </div>
      )}

      {/* Form Preview / Builder */}
      <div className={isEditing ? 'lg:col-span-3' : 'lg:col-span-4'}>
        <div className="bg-white border border-gray-200 rounded-lg p-6">
          <h3 className="text-lg font-medium text-gray-900 mb-4">Form Fields</h3>

          {fields.length === 0 ? (
            <div className="text-center py-12 text-gray-500 border-2 border-dashed border-gray-200 rounded-lg">
              <svg
                className="mx-auto h-12 w-12 text-gray-400 mb-4"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <title>No fields</title>
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M12 6v6m0 0v6m0-6h6m-6 0H6"
                />
              </svg>
              <p className="text-lg font-medium mb-2">No fields yet</p>
              <p>Click on a field type from the palette to add it to your form.</p>
            </div>
          ) : (
            <div className="space-y-4">
              {fields.map((field) => (
                <div
                  key={field.id}
                  draggable={isEditing}
                  onDragStart={() => handleDragStart(field.id)}
                  onDragOver={(e) => handleDragOver(e, field.id)}
                  onDragEnd={handleDragEnd}
                  className={`border rounded-lg p-4 ${
                    draggedFieldId === field.id
                      ? 'opacity-50 border-blue-500 bg-blue-50'
                      : 'border-gray-200 bg-white'
                  } ${isEditing ? 'cursor-move' : ''}`}
                >
                  {editingFieldId === field.id ? (
                    <FieldEditor
                      field={field}
                      onSave={(updates) => {
                        onUpdateField(field.id, updates);
                        setEditingFieldId(null);
                      }}
                      onCancel={() => setEditingFieldId(null)}
                    />
                  ) : (
                    <FieldPreview
                      field={field}
                      isEditing={isEditing}
                      onEdit={() => setEditingFieldId(field.id)}
                      onDelete={() => onDeleteField(field.id)}
                    />
                  )}
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

// Field Preview Component
interface FieldPreviewProps {
  field: FormField;
  isEditing: boolean;
  onEdit: () => void;
  onDelete: () => void;
}

function FieldPreview({ field, isEditing, onEdit, onDelete }: FieldPreviewProps) {
  return (
    <div className="flex items-start justify-between">
      <div className="flex-1">
        <div className="flex items-center gap-2 mb-2">
          <span className="text-sm font-medium text-gray-900">
            {field.label}
            {field.required && <span className="text-red-500 ml-1">*</span>}
          </span>
          <span className="px-2 py-0.5 text-xs bg-gray-100 text-gray-600 rounded">
            {field.fieldType}
          </span>
        </div>
        {field.helpText && <p className="text-sm text-gray-500">{field.helpText}</p>}
        {field.options && field.options.length > 0 && (
          <p className="text-xs text-gray-400 mt-1">
            Options: {field.options.map((o) => o.label).join(', ')}
          </p>
        )}
      </div>
      {isEditing && (
        <div className="flex gap-2">
          <button
            type="button"
            onClick={onEdit}
            className="p-1 text-gray-400 hover:text-blue-600"
            title="Edit field"
          >
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <title>Edit</title>
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
            onClick={onDelete}
            className="p-1 text-gray-400 hover:text-red-600"
            title="Delete field"
          >
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <title>Delete</title>
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
              />
            </svg>
          </button>
        </div>
      )}
    </div>
  );
}

// Field Editor Component
interface FieldEditorProps {
  field: FormField;
  onSave: (updates: Partial<CreateFormField>) => void;
  onCancel: () => void;
}

function FieldEditor({ field, onSave, onCancel }: FieldEditorProps) {
  const [label, setLabel] = useState(field.label);
  const [required, setRequired] = useState(field.required);
  const [placeholder, setPlaceholder] = useState(field.placeholder || '');
  const [helpText, setHelpText] = useState(field.helpText || '');
  // Convert FieldOption[] to newline-separated string for editing
  const [options, setOptions] = useState(field.options?.map((o) => o.label).join('\n') || '');

  const needsOptions = ['radio', 'select', 'multiselect'].includes(field.fieldType);

  const handleSave = () => {
    // Convert newline-separated string back to FieldOption[]
    const optionLines = options.split('\n').filter(Boolean);
    const fieldOptions = optionLines.map((line) => ({
      value: line.toLowerCase().replace(/\s+/g, '_'),
      label: line,
    }));

    onSave({
      label,
      required,
      placeholder: placeholder || undefined,
      helpText: helpText || undefined,
      options: needsOptions ? fieldOptions : undefined,
    });
  };

  return (
    <div className="space-y-4">
      <label className="block">
        <span className="block text-sm font-medium text-gray-700 mb-1">Label</span>
        <input
          type="text"
          value={label}
          onChange={(e) => setLabel(e.target.value)}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </label>

      <label className="flex items-center gap-2">
        <input
          type="checkbox"
          checked={required}
          onChange={(e) => setRequired(e.target.checked)}
          className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
        />
        <span className="text-sm text-gray-700">Required field</span>
      </label>

      <label className="block">
        <span className="block text-sm font-medium text-gray-700 mb-1">Placeholder</span>
        <input
          type="text"
          value={placeholder}
          onChange={(e) => setPlaceholder(e.target.value)}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </label>

      <label className="block">
        <span className="block text-sm font-medium text-gray-700 mb-1">Help Text</span>
        <input
          type="text"
          value={helpText}
          onChange={(e) => setHelpText(e.target.value)}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </label>

      {needsOptions && (
        <label className="block">
          <span className="block text-sm font-medium text-gray-700 mb-1">
            Options (one per line)
          </span>
          <textarea
            value={options}
            onChange={(e) => setOptions(e.target.value)}
            rows={4}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </label>
      )}

      <div className="flex gap-2">
        <button
          type="button"
          onClick={handleSave}
          className="px-3 py-1.5 bg-blue-600 text-white text-sm rounded-md hover:bg-blue-700"
        >
          Save
        </button>
        <button
          type="button"
          onClick={onCancel}
          className="px-3 py-1.5 border border-gray-300 text-gray-700 text-sm rounded-md hover:bg-gray-50"
        >
          Cancel
        </button>
      </div>
    </div>
  );
}

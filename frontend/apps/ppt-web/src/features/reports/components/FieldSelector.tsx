/**
 * FieldSelector component for drag-and-drop field selection in report builder.
 */

import type { ReportField } from '@ppt/api-client';
import { useState } from 'react';

interface FieldSelectorProps {
  availableFields: ReportField[];
  selectedFields: ReportField[];
  onFieldsChange: (fields: ReportField[]) => void;
}

export function FieldSelector({
  availableFields,
  selectedFields,
  onFieldsChange,
}: FieldSelectorProps) {
  const [draggedField, setDraggedField] = useState<ReportField | null>(null);

  const handleDragStart = (field: ReportField) => {
    setDraggedField(field);
  };

  const handleDragEnd = () => {
    setDraggedField(null);
  };

  const handleDrop = () => {
    if (draggedField && !selectedFields.find((f) => f.id === draggedField.id)) {
      onFieldsChange([...selectedFields, draggedField]);
    }
    setDraggedField(null);
  };

  const handleRemoveField = (fieldId: string) => {
    onFieldsChange(selectedFields.filter((f) => f.id !== fieldId));
  };

  const handleMoveUp = (index: number) => {
    if (index === 0) return;
    const newFields = [...selectedFields];
    [newFields[index - 1], newFields[index]] = [newFields[index], newFields[index - 1]];
    onFieldsChange(newFields);
  };

  const handleMoveDown = (index: number) => {
    if (index === selectedFields.length - 1) return;
    const newFields = [...selectedFields];
    [newFields[index], newFields[index + 1]] = [newFields[index + 1], newFields[index]];
    onFieldsChange(newFields);
  };

  const unselectedFields = availableFields.filter(
    (f) => !selectedFields.find((sf) => sf.id === f.id)
  );

  const getTypeIcon = (type: ReportField['type']) => {
    switch (type) {
      case 'number':
      case 'currency':
        return '#';
      case 'date':
        return '📅';
      case 'percentage':
        return '%';
      default:
        return 'Aa';
    }
  };

  return (
    <div className="grid grid-cols-2 gap-4">
      {/* Available Fields */}
      <div>
        <h4 className="text-sm font-medium text-gray-700 mb-2">Available Fields</h4>
        <div className="border rounded-lg bg-gray-50 p-2 min-h-[200px] max-h-[400px] overflow-y-auto">
          {unselectedFields.length === 0 ? (
            <p className="text-sm text-gray-500 text-center py-4">All fields selected</p>
          ) : (
            <div className="space-y-1">
              {unselectedFields.map((field) => (
                <div
                  key={field.id}
                  draggable
                  onDragStart={() => handleDragStart(field)}
                  onDragEnd={handleDragEnd}
                  className="flex items-center gap-2 px-3 py-2 bg-white rounded border border-gray-200 cursor-move hover:border-blue-400 hover:bg-blue-50 transition-colors"
                >
                  <span className="text-xs text-gray-400 w-6">{getTypeIcon(field.type)}</span>
                  <span className="text-sm text-gray-700 flex-1">{field.name}</span>
                  <button
                    type="button"
                    onClick={() => onFieldsChange([...selectedFields, field])}
                    className="text-blue-600 hover:text-blue-800"
                    title="Add field"
                  >
                    <svg
                      className="w-4 h-4"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                      aria-hidden="true"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M12 4v16m8-8H4"
                      />
                    </svg>
                  </button>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>

      {/* Selected Fields */}
      <div>
        <h4 className="text-sm font-medium text-gray-700 mb-2">
          Selected Fields ({selectedFields.length})
        </h4>
        <div
          onDragOver={(e) => e.preventDefault()}
          onDrop={handleDrop}
          className={`border-2 border-dashed rounded-lg p-2 min-h-[200px] max-h-[400px] overflow-y-auto transition-colors ${
            draggedField ? 'border-blue-400 bg-blue-50' : 'border-gray-300'
          }`}
        >
          {selectedFields.length === 0 ? (
            <p className="text-sm text-gray-500 text-center py-8">
              Drag fields here or click + to add
            </p>
          ) : (
            <div className="space-y-1">
              {selectedFields.map((field, index) => (
                <div
                  key={field.id}
                  className="flex items-center gap-2 px-3 py-2 bg-white rounded border border-blue-200 shadow-sm"
                >
                  <span className="text-xs text-gray-400 w-4">{index + 1}</span>
                  <span className="text-xs text-gray-400 w-6">{getTypeIcon(field.type)}</span>
                  <span className="text-sm text-gray-700 flex-1">{field.name}</span>
                  <div className="flex items-center gap-1">
                    <button
                      type="button"
                      onClick={() => handleMoveUp(index)}
                      disabled={index === 0}
                      className="text-gray-400 hover:text-gray-600 disabled:opacity-30"
                      title="Move up"
                    >
                      <svg
                        className="w-4 h-4"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                        aria-hidden="true"
                      >
                        <path
                          strokeLinecap="round"
                          strokeLinejoin="round"
                          strokeWidth={2}
                          d="M5 15l7-7 7 7"
                        />
                      </svg>
                    </button>
                    <button
                      type="button"
                      onClick={() => handleMoveDown(index)}
                      disabled={index === selectedFields.length - 1}
                      className="text-gray-400 hover:text-gray-600 disabled:opacity-30"
                      title="Move down"
                    >
                      <svg
                        className="w-4 h-4"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                        aria-hidden="true"
                      >
                        <path
                          strokeLinecap="round"
                          strokeLinejoin="round"
                          strokeWidth={2}
                          d="M19 9l-7 7-7-7"
                        />
                      </svg>
                    </button>
                    <button
                      type="button"
                      onClick={() => handleRemoveField(field.id)}
                      className="text-red-500 hover:text-red-700"
                      title="Remove field"
                    >
                      <svg
                        className="w-4 h-4"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                        aria-hidden="true"
                      >
                        <path
                          strokeLinecap="round"
                          strokeLinejoin="round"
                          strokeWidth={2}
                          d="M6 18L18 6M6 6l12 12"
                        />
                      </svg>
                    </button>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

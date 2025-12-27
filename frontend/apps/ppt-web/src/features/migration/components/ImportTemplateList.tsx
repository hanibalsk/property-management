/**
 * Import Template List Component (Story 66.1).
 *
 * Displays available import templates with actions to download,
 * edit, duplicate, or delete.
 */

import { useCallback, useState } from 'react';
import type { ImportDataType } from './ImportTemplateBuilder';

export interface ImportTemplateSummary {
  id: string;
  name: string;
  dataType: ImportDataType;
  description?: string;
  isSystemTemplate: boolean;
  fieldCount: number;
  updatedAt: string;
}

interface ImportTemplateListProps {
  templates: ImportTemplateSummary[];
  isLoading?: boolean;
  onSelect: (template: ImportTemplateSummary) => void;
  onEdit: (template: ImportTemplateSummary) => void;
  onDownload: (template: ImportTemplateSummary, format: 'csv' | 'xlsx') => void;
  onDuplicate: (template: ImportTemplateSummary) => void;
  onDelete: (template: ImportTemplateSummary) => void;
  onCreate: () => void;
  selectedDataType?: ImportDataType;
  onDataTypeChange?: (dataType: ImportDataType | undefined) => void;
}

const DATA_TYPE_LABELS: Record<ImportDataType, string> = {
  buildings: 'Buildings',
  units: 'Units',
  residents: 'Residents',
  financials: 'Financials',
  faults: 'Faults',
  documents: 'Documents',
  meters: 'Meters',
  votes: 'Votes',
  custom: 'Custom',
};

export function ImportTemplateList({
  templates,
  isLoading = false,
  onSelect,
  onEdit,
  onDownload,
  onDuplicate,
  onDelete,
  onCreate,
  selectedDataType,
  onDataTypeChange,
}: ImportTemplateListProps) {
  const [openMenuId, setOpenMenuId] = useState<string | null>(null);

  const toggleMenu = useCallback((id: string) => {
    setOpenMenuId((current) => (current === id ? null : id));
  }, []);

  const filteredTemplates = selectedDataType
    ? templates.filter((t) => t.dataType === selectedDataType)
    : templates;

  const systemTemplates = filteredTemplates.filter((t) => t.isSystemTemplate);
  const customTemplates = filteredTemplates.filter((t) => !t.isSystemTemplate);

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-lg font-medium text-gray-900">Import Templates</h2>
          <p className="mt-1 text-sm text-gray-500">
            Select a template to download or use for importing data.
          </p>
        </div>
        <button
          type="button"
          onClick={onCreate}
          className="inline-flex items-center rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
        >
          Create Template
        </button>
      </div>

      {/* Filters */}
      <div className="flex items-center gap-4">
        <label className="text-sm font-medium text-gray-700">Filter by type:</label>
        <select
          value={selectedDataType ?? ''}
          onChange={(e) => onDataTypeChange?.(e.target.value as ImportDataType || undefined)}
          className="rounded-md border-gray-300 text-sm shadow-sm focus:border-blue-500 focus:ring-blue-500"
        >
          <option value="">All types</option>
          {Object.entries(DATA_TYPE_LABELS).map(([value, label]) => (
            <option key={value} value={value}>
              {label}
            </option>
          ))}
        </select>
      </div>

      {isLoading ? (
        <div className="flex items-center justify-center py-12">
          <div className="h-8 w-8 animate-spin rounded-full border-2 border-blue-600 border-t-transparent" />
        </div>
      ) : (
        <>
          {/* System Templates */}
          {systemTemplates.length > 0 && (
            <div>
              <h3 className="mb-3 text-sm font-medium text-gray-500">System Templates</h3>
              <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                {systemTemplates.map((template) => (
                  <TemplateCard
                    key={template.id}
                    template={template}
                    isMenuOpen={openMenuId === template.id}
                    onToggleMenu={() => toggleMenu(template.id)}
                    onSelect={() => onSelect(template)}
                    onEdit={() => onEdit(template)}
                    onDownload={(format) => onDownload(template, format)}
                    onDuplicate={() => onDuplicate(template)}
                    onDelete={() => onDelete(template)}
                  />
                ))}
              </div>
            </div>
          )}

          {/* Custom Templates */}
          {customTemplates.length > 0 && (
            <div>
              <h3 className="mb-3 text-sm font-medium text-gray-500">Custom Templates</h3>
              <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                {customTemplates.map((template) => (
                  <TemplateCard
                    key={template.id}
                    template={template}
                    isMenuOpen={openMenuId === template.id}
                    onToggleMenu={() => toggleMenu(template.id)}
                    onSelect={() => onSelect(template)}
                    onEdit={() => onEdit(template)}
                    onDownload={(format) => onDownload(template, format)}
                    onDuplicate={() => onDuplicate(template)}
                    onDelete={() => onDelete(template)}
                  />
                ))}
              </div>
            </div>
          )}

          {/* Empty State */}
          {filteredTemplates.length === 0 && (
            <div className="rounded-lg border-2 border-dashed border-gray-300 p-12 text-center">
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
                  d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                />
              </svg>
              <h3 className="mt-4 text-sm font-medium text-gray-900">No templates found</h3>
              <p className="mt-1 text-sm text-gray-500">
                {selectedDataType
                  ? `No templates found for ${DATA_TYPE_LABELS[selectedDataType]}.`
                  : 'Get started by creating a new import template.'}
              </p>
              <button
                type="button"
                onClick={onCreate}
                className="mt-4 inline-flex items-center rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700"
              >
                Create Template
              </button>
            </div>
          )}
        </>
      )}
    </div>
  );
}

interface TemplateCardProps {
  template: ImportTemplateSummary;
  isMenuOpen: boolean;
  onToggleMenu: () => void;
  onSelect: () => void;
  onEdit: () => void;
  onDownload: (format: 'csv' | 'xlsx') => void;
  onDuplicate: () => void;
  onDelete: () => void;
}

function TemplateCard({
  template,
  isMenuOpen,
  onToggleMenu,
  onSelect,
  onEdit,
  onDownload,
  onDuplicate,
  onDelete,
}: TemplateCardProps) {
  return (
    <div className="relative rounded-lg border border-gray-200 bg-white p-4 shadow-sm hover:shadow-md transition-shadow">
      {/* System badge */}
      {template.isSystemTemplate && (
        <span className="absolute right-3 top-3 rounded bg-blue-100 px-2 py-0.5 text-xs font-medium text-blue-700">
          System
        </span>
      )}

      {/* Content */}
      <div className="pr-16">
        <h4 className="font-medium text-gray-900">{template.name}</h4>
        <p className="mt-1 text-sm text-gray-500 line-clamp-2">
          {template.description || 'No description'}
        </p>

        <div className="mt-3 flex items-center gap-2 text-xs text-gray-500">
          <span className="rounded bg-gray-100 px-2 py-0.5">
            {DATA_TYPE_LABELS[template.dataType]}
          </span>
          <span>{template.fieldCount} fields</span>
        </div>
      </div>

      {/* Actions */}
      <div className="mt-4 flex items-center justify-between border-t border-gray-100 pt-3">
        <button
          type="button"
          onClick={onSelect}
          className="text-sm font-medium text-blue-600 hover:text-blue-700"
        >
          Use Template
        </button>

        <div className="relative">
          <button
            type="button"
            onClick={onToggleMenu}
            className="rounded p-1 text-gray-400 hover:bg-gray-100 hover:text-gray-600"
          >
            <svg className="h-5 w-5" fill="currentColor" viewBox="0 0 20 20">
              <path d="M10 6a2 2 0 110-4 2 2 0 010 4zM10 12a2 2 0 110-4 2 2 0 010 4zM10 18a2 2 0 110-4 2 2 0 010 4z" />
            </svg>
          </button>

          {isMenuOpen && (
            <div className="absolute right-0 z-10 mt-1 w-48 rounded-md bg-white py-1 shadow-lg ring-1 ring-black ring-opacity-5">
              <button
                type="button"
                onClick={() => {
                  onDownload('csv');
                  onToggleMenu();
                }}
                className="block w-full px-4 py-2 text-left text-sm text-gray-700 hover:bg-gray-100"
              >
                Download CSV
              </button>
              <button
                type="button"
                onClick={() => {
                  onDownload('xlsx');
                  onToggleMenu();
                }}
                className="block w-full px-4 py-2 text-left text-sm text-gray-700 hover:bg-gray-100"
              >
                Download Excel
              </button>
              <hr className="my-1" />
              {!template.isSystemTemplate && (
                <button
                  type="button"
                  onClick={() => {
                    onEdit();
                    onToggleMenu();
                  }}
                  className="block w-full px-4 py-2 text-left text-sm text-gray-700 hover:bg-gray-100"
                >
                  Edit
                </button>
              )}
              <button
                type="button"
                onClick={() => {
                  onDuplicate();
                  onToggleMenu();
                }}
                className="block w-full px-4 py-2 text-left text-sm text-gray-700 hover:bg-gray-100"
              >
                Duplicate
              </button>
              {!template.isSystemTemplate && (
                <>
                  <hr className="my-1" />
                  <button
                    type="button"
                    onClick={() => {
                      onDelete();
                      onToggleMenu();
                    }}
                    className="block w-full px-4 py-2 text-left text-sm text-red-600 hover:bg-red-50"
                  >
                    Delete
                  </button>
                </>
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

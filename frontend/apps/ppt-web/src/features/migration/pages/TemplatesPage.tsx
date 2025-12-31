/**
 * Templates Page (Epic 66, Story 66.1).
 *
 * Manage import templates - create, edit, download, and organize.
 */

import { useCallback, useState } from 'react';
import {
  type ImportDataType,
  type ImportFieldMapping,
  ImportTemplateBuilder,
} from '../components/ImportTemplateBuilder';
import { ImportTemplateList, type ImportTemplateSummary } from '../components/ImportTemplateList';

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
  {
    id: '4',
    name: 'Financials Import',
    dataType: 'financials',
    description: 'Import financial transactions and balances',
    isSystemTemplate: true,
    fieldCount: 14,
    updatedAt: new Date().toISOString(),
  },
  {
    id: '5',
    name: 'Custom Buildings v2',
    dataType: 'buildings',
    description: 'Custom template with additional fields for our organization',
    isSystemTemplate: false,
    fieldCount: 16,
    updatedAt: new Date(Date.now() - 86400000).toISOString(),
  },
];

type PageMode = 'list' | 'create' | 'edit';

export function TemplatesPage() {
  const [mode, setMode] = useState<PageMode>('list');
  const [templates, setTemplates] = useState<ImportTemplateSummary[]>(MOCK_TEMPLATES);
  const [selectedTemplate, setSelectedTemplate] = useState<ImportTemplateSummary | null>(null);
  const [selectedDataType, setSelectedDataType] = useState<ImportDataType | undefined>();
  const [isLoading, setIsLoading] = useState(false);

  // Handle creating a new template
  const handleCreate = useCallback(() => {
    setSelectedTemplate(null);
    setMode('create');
  }, []);

  // Handle editing a template
  const handleEdit = useCallback((template: ImportTemplateSummary) => {
    setSelectedTemplate(template);
    setMode('edit');
  }, []);

  // Handle saving a template
  const handleSave = useCallback(
    async (templateData: {
      name: string;
      description?: string;
      dataType: ImportDataType;
      fieldMappings: ImportFieldMapping[];
    }) => {
      setIsLoading(true);
      try {
        // Simulate API call
        await new Promise((resolve) => setTimeout(resolve, 1000));

        if (mode === 'create') {
          const newTemplate: ImportTemplateSummary = {
            id: crypto.randomUUID(),
            name: templateData.name,
            dataType: templateData.dataType,
            description: templateData.description,
            isSystemTemplate: false,
            fieldCount: templateData.fieldMappings.length,
            updatedAt: new Date().toISOString(),
          };
          setTemplates((prev) => [...prev, newTemplate]);
        } else if (selectedTemplate) {
          setTemplates((prev) =>
            prev.map((t) =>
              t.id === selectedTemplate.id
                ? {
                    ...t,
                    name: templateData.name,
                    description: templateData.description,
                    dataType: templateData.dataType,
                    fieldCount: templateData.fieldMappings.length,
                    updatedAt: new Date().toISOString(),
                  }
                : t
            )
          );
        }

        setMode('list');
        setSelectedTemplate(null);
      } finally {
        setIsLoading(false);
      }
    },
    [mode, selectedTemplate]
  );

  // Handle cancel
  const handleCancel = useCallback(() => {
    setMode('list');
    setSelectedTemplate(null);
  }, []);

  // Handle template selection (for use)
  const handleSelect = useCallback((_template: ImportTemplateSummary) => {
    // TODO: Navigate to import page with template
  }, []);

  // Handle download
  const handleDownload = useCallback(
    (_template: ImportTemplateSummary, _format: 'csv' | 'xlsx') => {
      // TODO: Trigger file download from API
    },
    []
  );

  // Handle duplicate
  const handleDuplicate = useCallback((template: ImportTemplateSummary) => {
    const duplicated: ImportTemplateSummary = {
      ...template,
      id: crypto.randomUUID(),
      name: `${template.name} (Copy)`,
      isSystemTemplate: false,
      updatedAt: new Date().toISOString(),
    };
    setTemplates((prev) => [...prev, duplicated]);
  }, []);

  // Handle delete
  const handleDelete = useCallback((template: ImportTemplateSummary) => {
    if (template.isSystemTemplate) {
      return;
    }
    if (window.confirm(`Are you sure you want to delete "${template.name}"?`)) {
      setTemplates((prev) => prev.filter((t) => t.id !== template.id));
    }
  }, []);

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-xl font-semibold text-gray-900">Import Templates</h1>
        <p className="mt-1 text-sm text-gray-500">
          Manage templates for importing data into your organization.
        </p>
      </div>

      {/* Main Content */}
      <div className="rounded-lg border border-gray-200 bg-white p-6">
        {mode === 'list' && (
          <ImportTemplateList
            templates={templates}
            onSelect={handleSelect}
            onEdit={handleEdit}
            onDownload={handleDownload}
            onDuplicate={handleDuplicate}
            onDelete={handleDelete}
            onCreate={handleCreate}
            selectedDataType={selectedDataType}
            onDataTypeChange={setSelectedDataType}
          />
        )}

        {(mode === 'create' || mode === 'edit') && (
          <ImportTemplateBuilder
            initialTemplate={
              selectedTemplate && mode === 'edit'
                ? {
                    name: selectedTemplate.name,
                    description: selectedTemplate.description,
                    dataType: selectedTemplate.dataType,
                    fieldMappings: [], // In real implementation, fetch full template data
                  }
                : undefined
            }
            onSave={handleSave}
            onCancel={handleCancel}
            isLoading={isLoading}
          />
        )}
      </div>

      {/* Help Section */}
      {mode === 'list' && (
        <div className="grid gap-4 sm:grid-cols-2">
          <div className="rounded-lg bg-blue-50 p-4">
            <h3 className="text-sm font-medium text-blue-800">System Templates</h3>
            <p className="mt-1 text-sm text-blue-700">
              Pre-built templates for common data types. These cannot be modified, but you can
              duplicate them to create custom versions.
            </p>
          </div>
          <div className="rounded-lg bg-green-50 p-4">
            <h3 className="text-sm font-medium text-green-800">Custom Templates</h3>
            <p className="mt-1 text-sm text-green-700">
              Create your own templates with custom fields and validation rules to match your data
              format.
            </p>
          </div>
        </div>
      )}
    </div>
  );
}

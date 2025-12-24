/**
 * ReportBuilder component - Story 53.1
 *
 * Main component for creating and editing custom reports.
 */

import type {
  ChartType,
  CreateReportDefinition,
  DataSource,
  ReportField,
  ReportFilter,
  ReportGrouping,
  ReportResult,
} from '@ppt/api-client';
import { useState } from 'react';
import { FieldSelector } from './FieldSelector';
import { FilterBuilder } from './FilterBuilder';
import { GroupingConfig } from './GroupingConfig';
import { ReportPreview } from './ReportPreview';

interface ReportBuilderProps {
  dataSources: DataSource[];
  initialData?: Partial<CreateReportDefinition>;
  onSave: (data: CreateReportDefinition) => void;
  onPreview: (data: CreateReportDefinition) => Promise<ReportResult>;
  onCancel: () => void;
  isSaving?: boolean;
}

const CHART_TYPES: { value: ChartType; label: string; icon: string }[] = [
  { value: 'bar', label: 'Bar Chart', icon: '📊' },
  { value: 'line', label: 'Line Chart', icon: '📈' },
  { value: 'pie', label: 'Pie Chart', icon: '🥧' },
  { value: 'area', label: 'Area Chart', icon: '📉' },
  { value: 'stacked_bar', label: 'Stacked Bar', icon: '📊' },
];

export function ReportBuilder({
  dataSources,
  initialData,
  onSave,
  onPreview,
  onCancel,
  isSaving,
}: ReportBuilderProps) {
  const [name, setName] = useState(initialData?.name || '');
  const [description, setDescription] = useState(initialData?.description || '');
  const [dataSource, setDataSource] = useState(
    initialData?.data_source || dataSources[0]?.id || ''
  );
  const [selectedFields, setSelectedFields] = useState<ReportField[]>([]);
  const [filters, setFilters] = useState<ReportFilter[]>([]);
  const [groupings, setGroupings] = useState<ReportGrouping[]>([]);
  const [chartType, setChartType] = useState<ChartType | undefined>(initialData?.chart_type);
  const [isPublic, setIsPublic] = useState(initialData?.is_public || false);

  const [previewResult, setPreviewResult] = useState<ReportResult | null>(null);
  const [isPreviewLoading, setIsPreviewLoading] = useState(false);
  const [previewError, setPreviewError] = useState<string | undefined>();
  const [errors, setErrors] = useState<{ name?: string; fields?: string }>({});

  const currentDataSource = dataSources.find((ds) => ds.id === dataSource);
  const availableFields = currentDataSource?.available_fields || [];

  const handleDataSourceChange = (newSource: string) => {
    setDataSource(newSource);
    setSelectedFields([]);
    setFilters([]);
    setGroupings([]);
    setPreviewResult(null);
  };

  const validate = (): boolean => {
    const newErrors: { name?: string; fields?: string } = {};

    if (!name.trim()) {
      newErrors.name = 'Report name is required';
    }

    if (selectedFields.length === 0) {
      newErrors.fields = 'Select at least one field';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const buildReportDefinition = (): CreateReportDefinition => ({
    name,
    description: description || undefined,
    data_source: dataSource,
    fields: selectedFields.map((f) => ({
      name: f.name,
      type: f.type,
      source: f.source,
      aggregation: f.aggregation,
    })),
    filters: filters.length > 0 ? filters : undefined,
    groupings: groupings.length > 0 ? groupings : undefined,
    chart_type: chartType,
    is_public: isPublic,
  });

  const handlePreview = async () => {
    if (selectedFields.length === 0) {
      setPreviewError('Select at least one field to preview');
      return;
    }

    setIsPreviewLoading(true);
    setPreviewError(undefined);

    try {
      const result = await onPreview(buildReportDefinition());
      setPreviewResult(result);
    } catch (err) {
      setPreviewError(err instanceof Error ? err.message : 'Preview failed');
    } finally {
      setIsPreviewLoading(false);
    }
  };

  const handleSave = () => {
    if (!validate()) return;
    onSave(buildReportDefinition());
  };

  return (
    <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
      {/* Configuration Panel */}
      <div className="space-y-6">
        {/* Basic Info */}
        <div className="bg-white rounded-lg shadow p-6 space-y-4">
          <h3 className="text-lg font-medium text-gray-900">Report Details</h3>

          <div>
            <label htmlFor="report-name" className="block text-sm font-medium text-gray-700">
              Report Name *
            </label>
            <input
              type="text"
              id="report-name"
              value={name}
              onChange={(e) => {
                setName(e.target.value);
                setErrors({ ...errors, name: undefined });
              }}
              className={`mt-1 block w-full px-3 py-2 border rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 ${
                errors.name ? 'border-red-300' : 'border-gray-300'
              }`}
              placeholder="Monthly Revenue Report"
            />
            {errors.name && <p className="mt-1 text-sm text-red-600">{errors.name}</p>}
          </div>

          <div>
            <label htmlFor="report-description" className="block text-sm font-medium text-gray-700">
              Description
            </label>
            <textarea
              id="report-description"
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              rows={2}
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
              placeholder="Optional description of what this report shows"
            />
          </div>

          <div>
            <label htmlFor="data-source" className="block text-sm font-medium text-gray-700">
              Data Source
            </label>
            <select
              id="data-source"
              value={dataSource}
              onChange={(e) => handleDataSourceChange(e.target.value)}
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
            >
              {dataSources.map((ds) => (
                <option key={ds.id} value={ds.id}>
                  {ds.name}
                </option>
              ))}
            </select>
            {currentDataSource && (
              <p className="mt-1 text-sm text-gray-500">{currentDataSource.description}</p>
            )}
          </div>

          <div className="flex items-center gap-2">
            <input
              type="checkbox"
              id="is-public"
              checked={isPublic}
              onChange={(e) => setIsPublic(e.target.checked)}
              className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
            />
            <label htmlFor="is-public" className="text-sm text-gray-700">
              Make this report visible to other users
            </label>
          </div>
        </div>

        {/* Field Selection */}
        <div className="bg-white rounded-lg shadow p-6">
          <h3 className="text-lg font-medium text-gray-900 mb-4">Select Fields</h3>
          {errors.fields && <p className="mb-2 text-sm text-red-600">{errors.fields}</p>}
          <FieldSelector
            availableFields={availableFields}
            selectedFields={selectedFields}
            onFieldsChange={(fields) => {
              setSelectedFields(fields);
              setErrors({ ...errors, fields: undefined });
            }}
          />
        </div>

        {/* Filters */}
        <div className="bg-white rounded-lg shadow p-6">
          <FilterBuilder fields={selectedFields} filters={filters} onFiltersChange={setFilters} />
        </div>

        {/* Groupings */}
        <div className="bg-white rounded-lg shadow p-6">
          <GroupingConfig
            fields={selectedFields}
            groupings={groupings}
            onGroupingsChange={setGroupings}
          />
        </div>

        {/* Chart Type */}
        <div className="bg-white rounded-lg shadow p-6">
          <h3 className="text-lg font-medium text-gray-900 mb-4">Visualization</h3>
          <div className="grid grid-cols-5 gap-2">
            <button
              type="button"
              onClick={() => setChartType(undefined)}
              className={`p-3 text-center rounded-lg border-2 transition-colors ${
                chartType === undefined
                  ? 'border-blue-500 bg-blue-50'
                  : 'border-gray-200 hover:border-gray-300'
              }`}
            >
              <span className="text-xl">📋</span>
              <p className="text-xs mt-1">Table</p>
            </button>
            {CHART_TYPES.map((type) => (
              <button
                key={type.value}
                type="button"
                onClick={() => setChartType(type.value)}
                className={`p-3 text-center rounded-lg border-2 transition-colors ${
                  chartType === type.value
                    ? 'border-blue-500 bg-blue-50'
                    : 'border-gray-200 hover:border-gray-300'
                }`}
              >
                <span className="text-xl">{type.icon}</span>
                <p className="text-xs mt-1">{type.label}</p>
              </button>
            ))}
          </div>
        </div>

        {/* Actions */}
        <div className="flex justify-between gap-3">
          <button
            type="button"
            onClick={handlePreview}
            disabled={isPreviewLoading || selectedFields.length === 0}
            className="px-4 py-2 text-sm font-medium text-blue-700 bg-blue-100 rounded-md hover:bg-blue-200 disabled:opacity-50"
          >
            {isPreviewLoading ? 'Loading...' : 'Preview Report'}
          </button>
          <div className="flex gap-3">
            <button
              type="button"
              onClick={onCancel}
              disabled={isSaving}
              className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 disabled:opacity-50"
            >
              Cancel
            </button>
            <button
              type="button"
              onClick={handleSave}
              disabled={isSaving}
              className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 disabled:opacity-50"
            >
              {isSaving ? 'Saving...' : 'Save Report'}
            </button>
          </div>
        </div>
      </div>

      {/* Preview Panel */}
      <div className="lg:sticky lg:top-6 lg:self-start">
        <ReportPreview
          result={previewResult}
          fields={selectedFields}
          isLoading={isPreviewLoading}
          error={previewError}
        />
      </div>
    </div>
  );
}

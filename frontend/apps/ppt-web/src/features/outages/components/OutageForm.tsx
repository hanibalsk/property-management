/**
 * OutageForm component for creating and editing outages.
 * UC-12: Utility Outages
 */

import type { OutageCommodity, OutageSeverity } from '@ppt/api-client';
import { type FormEvent, useState } from 'react';
import { useTranslation } from 'react-i18next';

export interface OutageFormData {
  title: string;
  description: string;
  commodity: OutageCommodity;
  severity: OutageSeverity;
  buildingIds: string[];
  scheduledStart: string;
  scheduledEnd: string;
}

interface Building {
  id: string;
  name: string;
  address: string;
}

interface OutageFormProps {
  initialData?: Partial<OutageFormData>;
  buildings: Building[];
  isLoading?: boolean;
  onSubmit: (data: OutageFormData) => void;
  onCancel: () => void;
}

const commodityOptions: OutageCommodity[] = [
  'electricity',
  'gas',
  'water',
  'heating',
  'internet',
  'other',
];
const severityOptions: OutageSeverity[] = ['informational', 'minor', 'major', 'critical'];

export function OutageForm({
  initialData,
  buildings,
  isLoading,
  onSubmit,
  onCancel,
}: OutageFormProps) {
  const { t } = useTranslation();
  const [formData, setFormData] = useState<OutageFormData>({
    title: initialData?.title ?? '',
    description: initialData?.description ?? '',
    commodity: initialData?.commodity ?? 'electricity',
    severity: initialData?.severity ?? 'minor',
    buildingIds: initialData?.buildingIds ?? [],
    scheduledStart: initialData?.scheduledStart ?? '',
    scheduledEnd: initialData?.scheduledEnd ?? '',
  });
  const [errors, setErrors] = useState<Partial<Record<keyof OutageFormData, string>>>({});

  const validate = (): boolean => {
    const newErrors: Partial<Record<keyof OutageFormData, string>> = {};

    if (!formData.title.trim()) {
      newErrors.title = t('outages.errors.titleRequired');
    }
    if (!formData.description.trim()) {
      newErrors.description = t('outages.errors.descriptionRequired');
    }
    if (formData.buildingIds.length === 0) {
      newErrors.buildingIds = t('outages.errors.buildingsRequired');
    }
    if (!formData.scheduledStart) {
      newErrors.scheduledStart = t('outages.errors.startRequired');
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = (e: FormEvent) => {
    e.preventDefault();
    if (validate()) {
      onSubmit(formData);
    }
  };

  const handleBuildingToggle = (buildingId: string) => {
    setFormData((prev) => ({
      ...prev,
      buildingIds: prev.buildingIds.includes(buildingId)
        ? prev.buildingIds.filter((id) => id !== buildingId)
        : [...prev.buildingIds, buildingId],
    }));
  };

  const selectAllBuildings = () => {
    setFormData((prev) => ({
      ...prev,
      buildingIds: buildings.map((b) => b.id),
    }));
  };

  const clearAllBuildings = () => {
    setFormData((prev) => ({
      ...prev,
      buildingIds: [],
    }));
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      {/* Title */}
      <div>
        <label htmlFor="title" className="block text-sm font-medium text-gray-700">
          {t('outages.form.title')} *
        </label>
        <input
          type="text"
          id="title"
          value={formData.title}
          onChange={(e) => setFormData((prev) => ({ ...prev, title: e.target.value }))}
          className={`mt-1 block w-full px-3 py-2 border rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 ${
            errors.title ? 'border-red-300' : 'border-gray-300'
          }`}
          placeholder={t('outages.form.titlePlaceholder')}
        />
        {errors.title && <p className="mt-1 text-sm text-red-600">{errors.title}</p>}
      </div>

      {/* Description */}
      <div>
        <label htmlFor="description" className="block text-sm font-medium text-gray-700">
          {t('outages.form.description')} *
        </label>
        <textarea
          id="description"
          rows={4}
          value={formData.description}
          onChange={(e) => setFormData((prev) => ({ ...prev, description: e.target.value }))}
          className={`mt-1 block w-full px-3 py-2 border rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 ${
            errors.description ? 'border-red-300' : 'border-gray-300'
          }`}
          placeholder={t('outages.form.descriptionPlaceholder')}
        />
        {errors.description && <p className="mt-1 text-sm text-red-600">{errors.description}</p>}
      </div>

      {/* Commodity and Severity */}
      <div className="grid grid-cols-2 gap-4">
        <div>
          <label htmlFor="commodity" className="block text-sm font-medium text-gray-700">
            {t('outages.form.commodity')} *
          </label>
          <select
            id="commodity"
            value={formData.commodity}
            onChange={(e) =>
              setFormData((prev) => ({ ...prev, commodity: e.target.value as OutageCommodity }))
            }
            className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
          >
            {commodityOptions.map((commodity) => (
              <option key={commodity} value={commodity}>
                {t(`outages.commodity.${commodity}`)}
              </option>
            ))}
          </select>
        </div>

        <div>
          <label htmlFor="severity" className="block text-sm font-medium text-gray-700">
            {t('outages.form.severity')} *
          </label>
          <select
            id="severity"
            value={formData.severity}
            onChange={(e) =>
              setFormData((prev) => ({ ...prev, severity: e.target.value as OutageSeverity }))
            }
            className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
          >
            {severityOptions.map((severity) => (
              <option key={severity} value={severity}>
                {t(`outages.severity.${severity}`)}
              </option>
            ))}
          </select>
        </div>
      </div>

      {/* Scheduled Start and End */}
      <div className="grid grid-cols-2 gap-4">
        <div>
          <label htmlFor="scheduledStart" className="block text-sm font-medium text-gray-700">
            {t('outages.form.scheduledStart')} *
          </label>
          <input
            type="datetime-local"
            id="scheduledStart"
            value={formData.scheduledStart}
            onChange={(e) => setFormData((prev) => ({ ...prev, scheduledStart: e.target.value }))}
            className={`mt-1 block w-full px-3 py-2 border rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 ${
              errors.scheduledStart ? 'border-red-300' : 'border-gray-300'
            }`}
          />
          {errors.scheduledStart && (
            <p className="mt-1 text-sm text-red-600">{errors.scheduledStart}</p>
          )}
        </div>

        <div>
          <label htmlFor="scheduledEnd" className="block text-sm font-medium text-gray-700">
            {t('outages.form.scheduledEnd')}
          </label>
          <input
            type="datetime-local"
            id="scheduledEnd"
            value={formData.scheduledEnd}
            onChange={(e) => setFormData((prev) => ({ ...prev, scheduledEnd: e.target.value }))}
            className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
          />
        </div>
      </div>

      {/* Buildings */}
      <div>
        <div className="flex items-center justify-between">
          <label className="block text-sm font-medium text-gray-700">
            {t('outages.form.affectedBuildings')} *
          </label>
          <div className="flex gap-2">
            <button
              type="button"
              onClick={selectAllBuildings}
              className="text-xs text-blue-600 hover:text-blue-800"
            >
              {t('common.selectAll')}
            </button>
            <span className="text-gray-300">|</span>
            <button
              type="button"
              onClick={clearAllBuildings}
              className="text-xs text-blue-600 hover:text-blue-800"
            >
              {t('common.clearAll')}
            </button>
          </div>
        </div>
        <div
          className={`mt-2 max-h-48 overflow-y-auto border rounded-md p-2 ${
            errors.buildingIds ? 'border-red-300' : 'border-gray-300'
          }`}
        >
          {buildings.length === 0 ? (
            <p className="text-sm text-gray-500 py-2 text-center">
              {t('outages.form.noBuildings')}
            </p>
          ) : (
            buildings.map((building) => (
              <label
                key={building.id}
                className="flex items-center gap-2 py-1.5 px-2 hover:bg-gray-50 rounded cursor-pointer"
              >
                <input
                  type="checkbox"
                  checked={formData.buildingIds.includes(building.id)}
                  onChange={() => handleBuildingToggle(building.id)}
                  className="h-4 w-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
                />
                <span className="text-sm text-gray-900">{building.name}</span>
                <span className="text-xs text-gray-500">({building.address})</span>
              </label>
            ))
          )}
        </div>
        {errors.buildingIds && <p className="mt-1 text-sm text-red-600">{errors.buildingIds}</p>}
        <p className="mt-1 text-xs text-gray-500">
          {formData.buildingIds.length} {t('outages.form.buildingsSelected')}
        </p>
      </div>

      {/* Actions */}
      <div className="flex items-center justify-end gap-3 pt-4 border-t">
        <button
          type="button"
          onClick={onCancel}
          disabled={isLoading}
          className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 disabled:opacity-50"
        >
          {t('common.cancel')}
        </button>
        <button
          type="submit"
          disabled={isLoading}
          className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 disabled:opacity-50 flex items-center gap-2"
        >
          {isLoading && (
            <span className="animate-spin rounded-full h-4 w-4 border-b-2 border-white" />
          )}
          {t('common.save')}
        </button>
      </div>
    </form>
  );
}

/**
 * QuoteRequestForm component - form for requesting insurance quotes.
 * Insurance Management Feature (UC-35)
 */

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import type { CoverageType, PropertyType, QuoteRequest } from '../types';

interface QuoteRequestFormProps {
  buildings?: Array<{ id: string; name: string }>;
  isLoading?: boolean;
  onSubmit: (request: QuoteRequest) => void;
}

interface FormErrors {
  coverageType?: string;
  coverageAmount?: string;
  propertyType?: string;
  propertySize?: string;
  startDate?: string;
}

export function QuoteRequestForm({ buildings, isLoading, onSubmit }: QuoteRequestFormProps) {
  const { t } = useTranslation();

  const [coverageType, setCoverageType] = useState<CoverageType | ''>('');
  const [coverageAmount, setCoverageAmount] = useState<string>('');
  const [propertyType, setPropertyType] = useState<PropertyType | ''>('');
  const [propertySize, setPropertySize] = useState<string>('');
  const [startDate, setStartDate] = useState<string>('');
  const [buildingId, setBuildingId] = useState<string>('');
  const [errors, setErrors] = useState<FormErrors>({});

  const coverageTypeOptions: { value: CoverageType; label: string }[] = [
    { value: 'building', label: t('insurance.quotes.coverageTypeBuilding') },
    { value: 'contents', label: t('insurance.quotes.coverageTypeContents') },
    { value: 'liability', label: t('insurance.quotes.coverageTypeLiability') },
    { value: 'comprehensive', label: t('insurance.quotes.coverageTypeComprehensive') },
  ];

  const propertyTypeOptions: { value: PropertyType; label: string }[] = [
    { value: 'apartment', label: t('insurance.quotes.propertyTypeApartment') },
    { value: 'house', label: t('insurance.quotes.propertyTypeHouse') },
    { value: 'commercial', label: t('insurance.quotes.propertyTypeCommercial') },
    { value: 'mixed_use', label: t('insurance.quotes.propertyTypeMixedUse') },
  ];

  const validate = (): boolean => {
    const newErrors: FormErrors = {};

    if (!coverageType) {
      newErrors.coverageType = t('insurance.quotes.errors.coverageTypeRequired');
    }

    const amount = Number.parseFloat(coverageAmount);
    if (!coverageAmount || Number.isNaN(amount) || amount <= 0) {
      newErrors.coverageAmount = t('insurance.quotes.errors.coverageAmountRequired');
    }

    if (!propertyType) {
      newErrors.propertyType = t('insurance.quotes.errors.propertyTypeRequired');
    }

    const size = Number.parseFloat(propertySize);
    if (!propertySize || Number.isNaN(size) || size <= 0) {
      newErrors.propertySize = t('insurance.quotes.errors.propertySizeRequired');
    }

    if (!startDate) {
      newErrors.startDate = t('insurance.quotes.errors.startDateRequired');
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();

    if (!validate()) {
      return;
    }

    onSubmit({
      coverageType: coverageType as CoverageType,
      coverageAmount: Number.parseFloat(coverageAmount),
      propertyType: propertyType as PropertyType,
      propertySize: Number.parseFloat(propertySize),
      startDate,
      buildingId: buildingId || undefined,
    });
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        {/* Coverage Type */}
        <div>
          <label htmlFor="coverageType" className="block text-sm font-medium text-gray-700">
            {t('insurance.quotes.coverageType')} <span className="text-red-500">*</span>
          </label>
          <select
            id="coverageType"
            value={coverageType}
            onChange={(e) => setCoverageType(e.target.value as CoverageType | '')}
            className={`mt-1 block w-full rounded-md border ${
              errors.coverageType ? 'border-red-500' : 'border-gray-300'
            } px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500`}
          >
            <option value="">{t('common.select')}</option>
            {coverageTypeOptions.map((opt) => (
              <option key={opt.value} value={opt.value}>
                {opt.label}
              </option>
            ))}
          </select>
          {errors.coverageType && (
            <p className="mt-1 text-sm text-red-500">{errors.coverageType}</p>
          )}
        </div>

        {/* Coverage Amount */}
        <div>
          <label htmlFor="coverageAmount" className="block text-sm font-medium text-gray-700">
            {t('insurance.quotes.coverageAmount')} <span className="text-red-500">*</span>
          </label>
          <div className="mt-1 relative">
            <span className="absolute left-3 top-2 text-gray-500">$</span>
            <input
              type="number"
              id="coverageAmount"
              value={coverageAmount}
              onChange={(e) => setCoverageAmount(e.target.value)}
              placeholder={t('insurance.quotes.coverageAmountPlaceholder')}
              min="0"
              step="1000"
              className={`block w-full rounded-md border ${
                errors.coverageAmount ? 'border-red-500' : 'border-gray-300'
              } pl-8 pr-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500`}
            />
          </div>
          {errors.coverageAmount && (
            <p className="mt-1 text-sm text-red-500">{errors.coverageAmount}</p>
          )}
        </div>

        {/* Property Type */}
        <div>
          <label htmlFor="propertyType" className="block text-sm font-medium text-gray-700">
            {t('insurance.quotes.propertyType')} <span className="text-red-500">*</span>
          </label>
          <select
            id="propertyType"
            value={propertyType}
            onChange={(e) => setPropertyType(e.target.value as PropertyType | '')}
            className={`mt-1 block w-full rounded-md border ${
              errors.propertyType ? 'border-red-500' : 'border-gray-300'
            } px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500`}
          >
            <option value="">{t('common.select')}</option>
            {propertyTypeOptions.map((opt) => (
              <option key={opt.value} value={opt.value}>
                {opt.label}
              </option>
            ))}
          </select>
          {errors.propertyType && (
            <p className="mt-1 text-sm text-red-500">{errors.propertyType}</p>
          )}
        </div>

        {/* Property Size */}
        <div>
          <label htmlFor="propertySize" className="block text-sm font-medium text-gray-700">
            {t('insurance.quotes.propertySize')} <span className="text-red-500">*</span>
          </label>
          <div className="mt-1 relative">
            <input
              type="number"
              id="propertySize"
              value={propertySize}
              onChange={(e) => setPropertySize(e.target.value)}
              placeholder={t('insurance.quotes.propertySizePlaceholder')}
              min="0"
              step="10"
              className={`block w-full rounded-md border ${
                errors.propertySize ? 'border-red-500' : 'border-gray-300'
              } px-3 py-2 pr-12 focus:outline-none focus:ring-2 focus:ring-blue-500`}
            />
            <span className="absolute right-3 top-2 text-gray-500">
              {t('insurance.quotes.sqm')}
            </span>
          </div>
          {errors.propertySize && (
            <p className="mt-1 text-sm text-red-500">{errors.propertySize}</p>
          )}
        </div>

        {/* Start Date */}
        <div>
          <label htmlFor="startDate" className="block text-sm font-medium text-gray-700">
            {t('insurance.quotes.startDate')} <span className="text-red-500">*</span>
          </label>
          <input
            type="date"
            id="startDate"
            value={startDate}
            onChange={(e) => setStartDate(e.target.value)}
            min={new Date().toISOString().split('T')[0]}
            className={`mt-1 block w-full rounded-md border ${
              errors.startDate ? 'border-red-500' : 'border-gray-300'
            } px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500`}
          />
          {errors.startDate && <p className="mt-1 text-sm text-red-500">{errors.startDate}</p>}
        </div>

        {/* Building (optional) */}
        {buildings && buildings.length > 0 && (
          <div>
            <label htmlFor="buildingId" className="block text-sm font-medium text-gray-700">
              {t('insurance.quotes.building')}{' '}
              <span className="text-gray-400">({t('common.optional')})</span>
            </label>
            <select
              id="buildingId"
              value={buildingId}
              onChange={(e) => setBuildingId(e.target.value)}
              className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              <option value="">{t('insurance.form.allBuildings')}</option>
              {buildings.map((building) => (
                <option key={building.id} value={building.id}>
                  {building.name}
                </option>
              ))}
            </select>
          </div>
        )}
      </div>

      <div className="pt-4">
        <button
          type="submit"
          disabled={isLoading}
          className="w-full px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
        >
          {isLoading && (
            <svg className="animate-spin h-4 w-4" fill="none" viewBox="0 0 24 24">
              <circle
                className="opacity-25"
                cx="12"
                cy="12"
                r="10"
                stroke="currentColor"
                strokeWidth="4"
              />
              <path
                className="opacity-75"
                fill="currentColor"
                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
              />
            </svg>
          )}
          {isLoading ? t('insurance.quotes.gettingQuotes') : t('insurance.quotes.getQuotes')}
        </button>
      </div>
    </form>
  );
}

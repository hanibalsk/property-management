/**
 * PolicyForm component - form for creating/editing insurance policies.
 * Insurance Management Feature
 */

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import type { PolicyFormData, PolicyType } from '../types';

interface PolicyFormProps {
  initialData?: Partial<PolicyFormData>;
  buildings?: Array<{ id: string; name: string }>;
  isSubmitting?: boolean;
  onSubmit: (data: PolicyFormData) => void;
  onCancel: () => void;
}

export function PolicyForm({
  initialData,
  buildings = [],
  isSubmitting,
  onSubmit,
  onCancel,
}: PolicyFormProps) {
  const { t } = useTranslation();

  const policyTypeOptions: { value: PolicyType; label: string }[] = [
    { value: 'building', label: t('insurance.typeBuilding') },
    { value: 'liability', label: t('insurance.typeLiability') },
    { value: 'property', label: t('insurance.typeProperty') },
    { value: 'flood', label: t('insurance.typeFlood') },
    { value: 'earthquake', label: t('insurance.typeEarthquake') },
    { value: 'umbrella', label: t('insurance.typeUmbrella') },
    { value: 'directors_officers', label: t('insurance.typeDirectorsOfficers') },
    { value: 'workers_comp', label: t('insurance.typeWorkersComp') },
    { value: 'other', label: t('insurance.typeOther') },
  ];

  const frequencyOptions = [
    { value: 'monthly', label: t('insurance.frequency.monthly') },
    { value: 'quarterly', label: t('insurance.frequency.quarterly') },
    { value: 'annually', label: t('insurance.frequency.annually') },
  ];

  const [formData, setFormData] = useState<PolicyFormData>({
    buildingId: initialData?.buildingId || undefined,
    policyNumber: initialData?.policyNumber || '',
    policyType: initialData?.policyType || 'building',
    provider: initialData?.provider || '',
    providerContact: initialData?.providerContact || '',
    providerPhone: initialData?.providerPhone || '',
    providerEmail: initialData?.providerEmail || '',
    coverageAmount: initialData?.coverageAmount || 0,
    deductible: initialData?.deductible || 0,
    premium: initialData?.premium || 0,
    premiumFrequency: initialData?.premiumFrequency || 'annually',
    currency: initialData?.currency || 'USD',
    startDate: initialData?.startDate || '',
    endDate: initialData?.endDate || '',
    description: initialData?.description || '',
    coverageDetails: initialData?.coverageDetails || '',
  });

  const [errors, setErrors] = useState<Partial<Record<keyof PolicyFormData, string>>>({});

  const validate = (): boolean => {
    const newErrors: Partial<Record<keyof PolicyFormData, string>> = {};

    if (!formData.policyNumber.trim()) {
      newErrors.policyNumber = t('insurance.form.errors.policyNumberRequired');
    }
    if (!formData.provider.trim()) {
      newErrors.provider = t('insurance.form.errors.providerRequired');
    }
    if (formData.coverageAmount <= 0) {
      newErrors.coverageAmount = t('insurance.form.errors.coverageRequired');
    }
    if (formData.premium <= 0) {
      newErrors.premium = t('insurance.form.errors.premiumRequired');
    }
    if (!formData.startDate) {
      newErrors.startDate = t('insurance.form.errors.startDateRequired');
    }
    if (!formData.endDate) {
      newErrors.endDate = t('insurance.form.errors.endDateRequired');
    }
    if (formData.startDate && formData.endDate && formData.startDate >= formData.endDate) {
      newErrors.endDate = t('insurance.form.errors.endDateAfterStart');
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (validate()) {
      onSubmit(formData);
    }
  };

  const handleChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>
  ) => {
    const { name, value, type } = e.target;
    setFormData((prev) => ({
      ...prev,
      [name]: type === 'number' ? Number(value) : value || undefined,
    }));
    if (errors[name as keyof PolicyFormData]) {
      setErrors((prev) => ({ ...prev, [name]: undefined }));
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      {/* Building (optional) */}
      <div>
        <label htmlFor="buildingId" className="block text-sm font-medium text-gray-700">
          {t('insurance.form.building')} ({t('common.optional')})
        </label>
        <select
          id="buildingId"
          name="buildingId"
          value={formData.buildingId || ''}
          onChange={handleChange}
          className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          <option value="">{t('insurance.form.allBuildings')}</option>
          {buildings.map((b) => (
            <option key={b.id} value={b.id}>
              {b.name}
            </option>
          ))}
        </select>
      </div>

      {/* Policy Type */}
      <div>
        <label htmlFor="policyType" className="block text-sm font-medium text-gray-700">
          {t('insurance.form.policyType')} *
        </label>
        <select
          id="policyType"
          name="policyType"
          value={formData.policyType}
          onChange={handleChange}
          className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          {policyTypeOptions.map((opt) => (
            <option key={opt.value} value={opt.value}>
              {opt.label}
            </option>
          ))}
        </select>
      </div>

      {/* Policy Number */}
      <div>
        <label htmlFor="policyNumber" className="block text-sm font-medium text-gray-700">
          {t('insurance.form.policyNumber')} *
        </label>
        <input
          type="text"
          id="policyNumber"
          name="policyNumber"
          value={formData.policyNumber}
          onChange={handleChange}
          placeholder={t('insurance.form.policyNumberPlaceholder')}
          className={`mt-1 block w-full rounded-md border ${
            errors.policyNumber ? 'border-red-500' : 'border-gray-300'
          } px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500`}
        />
        {errors.policyNumber && <p className="mt-1 text-sm text-red-500">{errors.policyNumber}</p>}
      </div>

      {/* Provider */}
      <div>
        <label htmlFor="provider" className="block text-sm font-medium text-gray-700">
          {t('insurance.form.provider')} *
        </label>
        <input
          type="text"
          id="provider"
          name="provider"
          value={formData.provider}
          onChange={handleChange}
          placeholder={t('insurance.form.providerPlaceholder')}
          className={`mt-1 block w-full rounded-md border ${
            errors.provider ? 'border-red-500' : 'border-gray-300'
          } px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500`}
        />
        {errors.provider && <p className="mt-1 text-sm text-red-500">{errors.provider}</p>}
      </div>

      {/* Provider Contact Info */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div>
          <label htmlFor="providerContact" className="block text-sm font-medium text-gray-700">
            {t('insurance.form.contactName')}
          </label>
          <input
            type="text"
            id="providerContact"
            name="providerContact"
            value={formData.providerContact || ''}
            onChange={handleChange}
            className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
        <div>
          <label htmlFor="providerPhone" className="block text-sm font-medium text-gray-700">
            {t('insurance.form.phone')}
          </label>
          <input
            type="tel"
            id="providerPhone"
            name="providerPhone"
            value={formData.providerPhone || ''}
            onChange={handleChange}
            className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
        <div>
          <label htmlFor="providerEmail" className="block text-sm font-medium text-gray-700">
            {t('insurance.form.email')}
          </label>
          <input
            type="email"
            id="providerEmail"
            name="providerEmail"
            value={formData.providerEmail || ''}
            onChange={handleChange}
            className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
      </div>

      {/* Financial Details */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div>
          <label htmlFor="coverageAmount" className="block text-sm font-medium text-gray-700">
            {t('insurance.form.coverageAmount')} *
          </label>
          <input
            type="number"
            id="coverageAmount"
            name="coverageAmount"
            value={formData.coverageAmount}
            onChange={handleChange}
            min="0"
            step="1000"
            className={`mt-1 block w-full rounded-md border ${
              errors.coverageAmount ? 'border-red-500' : 'border-gray-300'
            } px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500`}
          />
          {errors.coverageAmount && (
            <p className="mt-1 text-sm text-red-500">{errors.coverageAmount}</p>
          )}
        </div>
        <div>
          <label htmlFor="deductible" className="block text-sm font-medium text-gray-700">
            {t('insurance.form.deductible')}
          </label>
          <input
            type="number"
            id="deductible"
            name="deductible"
            value={formData.deductible}
            onChange={handleChange}
            min="0"
            step="100"
            className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
        <div>
          <label htmlFor="currency" className="block text-sm font-medium text-gray-700">
            {t('insurance.form.currency')}
          </label>
          <select
            id="currency"
            name="currency"
            value={formData.currency}
            onChange={handleChange}
            className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <option value="USD">USD</option>
            <option value="EUR">EUR</option>
            <option value="GBP">GBP</option>
            <option value="CZK">CZK</option>
            <option value="SKK">EUR (SK)</option>
          </select>
        </div>
      </div>

      {/* Premium */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div>
          <label htmlFor="premium" className="block text-sm font-medium text-gray-700">
            {t('insurance.form.premium')} *
          </label>
          <input
            type="number"
            id="premium"
            name="premium"
            value={formData.premium}
            onChange={handleChange}
            min="0"
            step="10"
            className={`mt-1 block w-full rounded-md border ${
              errors.premium ? 'border-red-500' : 'border-gray-300'
            } px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500`}
          />
          {errors.premium && <p className="mt-1 text-sm text-red-500">{errors.premium}</p>}
        </div>
        <div>
          <label htmlFor="premiumFrequency" className="block text-sm font-medium text-gray-700">
            {t('insurance.form.premiumFrequency')}
          </label>
          <select
            id="premiumFrequency"
            name="premiumFrequency"
            value={formData.premiumFrequency}
            onChange={handleChange}
            className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            {frequencyOptions.map((opt) => (
              <option key={opt.value} value={opt.value}>
                {opt.label}
              </option>
            ))}
          </select>
        </div>
      </div>

      {/* Dates */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div>
          <label htmlFor="startDate" className="block text-sm font-medium text-gray-700">
            {t('insurance.form.startDate')} *
          </label>
          <input
            type="date"
            id="startDate"
            name="startDate"
            value={formData.startDate}
            onChange={handleChange}
            className={`mt-1 block w-full rounded-md border ${
              errors.startDate ? 'border-red-500' : 'border-gray-300'
            } px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500`}
          />
          {errors.startDate && <p className="mt-1 text-sm text-red-500">{errors.startDate}</p>}
        </div>
        <div>
          <label htmlFor="endDate" className="block text-sm font-medium text-gray-700">
            {t('insurance.form.endDate')} *
          </label>
          <input
            type="date"
            id="endDate"
            name="endDate"
            value={formData.endDate}
            onChange={handleChange}
            className={`mt-1 block w-full rounded-md border ${
              errors.endDate ? 'border-red-500' : 'border-gray-300'
            } px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500`}
          />
          {errors.endDate && <p className="mt-1 text-sm text-red-500">{errors.endDate}</p>}
        </div>
      </div>

      {/* Description */}
      <div>
        <label htmlFor="description" className="block text-sm font-medium text-gray-700">
          {t('insurance.form.description')}
        </label>
        <textarea
          id="description"
          name="description"
          value={formData.description || ''}
          onChange={handleChange}
          rows={3}
          placeholder={t('insurance.form.descriptionPlaceholder')}
          className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>

      {/* Coverage Details */}
      <div>
        <label htmlFor="coverageDetails" className="block text-sm font-medium text-gray-700">
          {t('insurance.form.coverageDetails')}
        </label>
        <textarea
          id="coverageDetails"
          name="coverageDetails"
          value={formData.coverageDetails || ''}
          onChange={handleChange}
          rows={4}
          placeholder={t('insurance.form.coverageDetailsPlaceholder')}
          className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>

      {/* Actions */}
      <div className="flex items-center justify-end gap-3 pt-4 border-t">
        <button
          type="button"
          onClick={onCancel}
          disabled={isSubmitting}
          className="px-4 py-2 text-gray-700 border border-gray-300 rounded-lg hover:bg-gray-50 disabled:opacity-50"
        >
          {t('common.cancel')}
        </button>
        <button
          type="submit"
          disabled={isSubmitting}
          className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 flex items-center gap-2"
        >
          {isSubmitting && (
            <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white" />
          )}
          {isSubmitting ? t('common.saving') : t('common.save')}
        </button>
      </div>
    </form>
  );
}

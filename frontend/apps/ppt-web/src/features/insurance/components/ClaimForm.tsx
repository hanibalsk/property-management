/**
 * ClaimForm component - form for filing insurance claims.
 * Insurance Management Feature
 */

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import type { ClaimFormData, ClaimType, InsurancePolicy } from '../types';

interface ClaimFormProps {
  initialData?: Partial<ClaimFormData>;
  policies: InsurancePolicy[];
  buildings?: Array<{ id: string; name: string }>;
  units?: Array<{ id: string; designation: string }>;
  isSubmitting?: boolean;
  onSubmit: (data: ClaimFormData) => void;
  onCancel: () => void;
}

export function ClaimForm({
  initialData,
  policies,
  buildings = [],
  units = [],
  isSubmitting,
  onSubmit,
  onCancel,
}: ClaimFormProps) {
  const { t } = useTranslation();

  const claimTypeOptions: { value: ClaimType; label: string }[] = [
    { value: 'property_damage', label: t('insurance.claimTypePropertyDamage') },
    { value: 'liability', label: t('insurance.claimTypeLiability') },
    { value: 'theft', label: t('insurance.claimTypeTheft') },
    { value: 'water_damage', label: t('insurance.claimTypeWaterDamage') },
    { value: 'fire_damage', label: t('insurance.claimTypeFireDamage') },
    { value: 'natural_disaster', label: t('insurance.claimTypeNaturalDisaster') },
    { value: 'personal_injury', label: t('insurance.claimTypePersonalInjury') },
    { value: 'other', label: t('insurance.claimTypeOther') },
  ];

  const [formData, setFormData] = useState<ClaimFormData>({
    policyId: initialData?.policyId || '',
    buildingId: initialData?.buildingId || undefined,
    unitId: initialData?.unitId || undefined,
    claimType: initialData?.claimType || 'property_damage',
    title: initialData?.title || '',
    description: initialData?.description || '',
    incidentDate: initialData?.incidentDate || '',
    claimAmount: initialData?.claimAmount || undefined,
  });

  const [errors, setErrors] = useState<Partial<Record<keyof ClaimFormData, string>>>({});

  const validate = (): boolean => {
    const newErrors: Partial<Record<keyof ClaimFormData, string>> = {};

    if (!formData.policyId) {
      newErrors.policyId = t('insurance.form.errors.policyRequired');
    }
    if (!formData.title.trim()) {
      newErrors.title = t('insurance.form.errors.titleRequired');
    } else if (formData.title.length > 255) {
      newErrors.title = t('insurance.form.errors.titleTooLong');
    }
    if (!formData.description.trim()) {
      newErrors.description = t('insurance.form.errors.descriptionRequired');
    }
    if (!formData.incidentDate) {
      newErrors.incidentDate = t('insurance.form.errors.incidentDateRequired');
    }
    if (formData.incidentDate && new Date(formData.incidentDate) > new Date()) {
      newErrors.incidentDate = t('insurance.form.errors.incidentDateFuture');
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
      [name]: type === 'number' ? (value ? Number(value) : undefined) : value || undefined,
    }));
    if (errors[name as keyof ClaimFormData]) {
      setErrors((prev) => ({ ...prev, [name]: undefined }));
    }
  };

  const activePolicies = policies.filter((p) => p.status === 'active');

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      {/* Policy Selection */}
      <div>
        <label htmlFor="policyId" className="block text-sm font-medium text-gray-700">
          {t('insurance.form.selectPolicy')} *
        </label>
        <select
          id="policyId"
          name="policyId"
          value={formData.policyId}
          onChange={handleChange}
          className={`mt-1 block w-full rounded-md border ${
            errors.policyId ? 'border-red-500' : 'border-gray-300'
          } px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500`}
        >
          <option value="">{t('insurance.form.selectPolicyPlaceholder')}</option>
          {activePolicies.map((p) => (
            <option key={p.id} value={p.id}>
              {p.provider} - {p.policyNumber} ({p.policyType})
            </option>
          ))}
        </select>
        {errors.policyId && <p className="mt-1 text-sm text-red-500">{errors.policyId}</p>}
      </div>

      {/* Claim Type */}
      <div>
        <label htmlFor="claimType" className="block text-sm font-medium text-gray-700">
          {t('insurance.form.claimType')} *
        </label>
        <select
          id="claimType"
          name="claimType"
          value={formData.claimType}
          onChange={handleChange}
          className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          {claimTypeOptions.map((opt) => (
            <option key={opt.value} value={opt.value}>
              {opt.label}
            </option>
          ))}
        </select>
      </div>

      {/* Building & Unit */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
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
            <option value="">{t('insurance.form.selectBuilding')}</option>
            {buildings.map((b) => (
              <option key={b.id} value={b.id}>
                {b.name}
              </option>
            ))}
          </select>
        </div>
        <div>
          <label htmlFor="unitId" className="block text-sm font-medium text-gray-700">
            {t('insurance.form.unit')} ({t('common.optional')})
          </label>
          <select
            id="unitId"
            name="unitId"
            value={formData.unitId || ''}
            onChange={handleChange}
            className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <option value="">{t('insurance.form.selectUnit')}</option>
            {units.map((u) => (
              <option key={u.id} value={u.id}>
                {u.designation}
              </option>
            ))}
          </select>
        </div>
      </div>

      {/* Title */}
      <div>
        <label htmlFor="title" className="block text-sm font-medium text-gray-700">
          {t('insurance.form.claimTitle')} *
        </label>
        <input
          type="text"
          id="title"
          name="title"
          value={formData.title}
          onChange={handleChange}
          placeholder={t('insurance.form.claimTitlePlaceholder')}
          className={`mt-1 block w-full rounded-md border ${
            errors.title ? 'border-red-500' : 'border-gray-300'
          } px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500`}
        />
        {errors.title && <p className="mt-1 text-sm text-red-500">{errors.title}</p>}
      </div>

      {/* Description */}
      <div>
        <label htmlFor="description" className="block text-sm font-medium text-gray-700">
          {t('insurance.form.claimDescription')} *
        </label>
        <textarea
          id="description"
          name="description"
          value={formData.description}
          onChange={handleChange}
          rows={5}
          placeholder={t('insurance.form.claimDescriptionPlaceholder')}
          className={`mt-1 block w-full rounded-md border ${
            errors.description ? 'border-red-500' : 'border-gray-300'
          } px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500`}
        />
        {errors.description && <p className="mt-1 text-sm text-red-500">{errors.description}</p>}
      </div>

      {/* Incident Date & Claim Amount */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div>
          <label htmlFor="incidentDate" className="block text-sm font-medium text-gray-700">
            {t('insurance.form.incidentDate')} *
          </label>
          <input
            type="date"
            id="incidentDate"
            name="incidentDate"
            value={formData.incidentDate}
            onChange={handleChange}
            max={new Date().toISOString().split('T')[0]}
            className={`mt-1 block w-full rounded-md border ${
              errors.incidentDate ? 'border-red-500' : 'border-gray-300'
            } px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500`}
          />
          {errors.incidentDate && (
            <p className="mt-1 text-sm text-red-500">{errors.incidentDate}</p>
          )}
        </div>
        <div>
          <label htmlFor="claimAmount" className="block text-sm font-medium text-gray-700">
            {t('insurance.form.estimatedAmount')} ({t('common.optional')})
          </label>
          <input
            type="number"
            id="claimAmount"
            name="claimAmount"
            value={formData.claimAmount || ''}
            onChange={handleChange}
            min="0"
            step="100"
            placeholder={t('insurance.form.estimatedAmountPlaceholder')}
            className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
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
          {isSubmitting ? t('insurance.form.submitting') : t('insurance.form.submitClaim')}
        </button>
      </div>
    </form>
  );
}
